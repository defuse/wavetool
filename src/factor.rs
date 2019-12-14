use crate::wavetable::{WaveTable, WaveCyclePartials, WAVE_SAMPLES, PARTIAL_COUNT};
use primes::PrimeSet;
use rustfft::num_complex::Complex;
use rustfft::num_traits::Zero;

const BIGGEST_USEFUL_PRIME : u64 = 43;

#[derive(Debug)]
#[derive(Clone)]
pub struct FactorArgs {
    pub input_file: String,
    pub normalize: bool,
    pub recursive: bool,
    pub shift: bool
}

impl WaveCyclePartials {
    fn filter_for_p(&self, p: u64) -> WaveCyclePartials {
        let mut filtered = WaveCyclePartials { partials: [Complex::zero(); PARTIAL_COUNT] };
        // leave DC component at zero
        for i in 1..filtered.partials.len() {
            if i as u64 % p == 0 && !divisible_by_prime_less_than(i as u64, p) {
                filtered.partials[i] = self.partials[i];
            } else {
                filtered.partials[i] = Complex::zero();
            }
        }
        filtered
    }
}

fn divisible_by_prime_less_than(n: u64, prime: u64) -> bool {
    let mut pset = PrimeSet::new();
    for p in pset.iter() {
        if p >= prime {
            break;
        }

        if n % p == 0 {
            return true;
        }
    }
    return false;
}

pub fn run_factor(args: &FactorArgs) -> () {
    let wt = WaveTable::load_from_wav(&args.input_file);
    
    let mut pset = PrimeSet::new();
    for p in pset.iter() {
        assert!(BIGGEST_USEFUL_PRIME < WAVE_SAMPLES as u64);
        if p > BIGGEST_USEFUL_PRIME as u64 {
            break;
        }

        let mut wtp = wt.clone();
        for i in 0..wtp.cycles.len() {

            let mut partials = wtp.cycles[i].fft();
            partials = partials.filter_for_p(p);

            if args.shift { 
                // leave DC component untouched
                for j in 1..partials.partials.len() {
                    let index = j * p as usize;
                    partials.partials[j] = if index < PARTIAL_COUNT {
                        partials.partials[index]
                    } else {
                        Complex::zero()
                    }
                }
            }

            wtp.cycles[i] = partials.fft();
        }

        if args.normalize {
            wtp = wtp.normalize();
        }

        assert!(wtp.cycles.len() == wt.cycles.len());

        let output_path = format!(
            "{}.{}{}p{:02}.wav", 
            &args.input_file,
            if args.shift { "s" } else { "u" },
            if args.normalize { "n" } else { "u" },
            p
        );
        wtp.save_to_wav(&output_path);
        if args.recursive {
            let mut recursive_args = args.clone();
            recursive_args.input_file = output_path;
            run_factor(&recursive_args);
        }
    }
}
