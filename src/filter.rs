use crate::wavetable::{WaveTable, WaveCyclePartials, WAVE_SAMPLES, PARTIAL_COUNT};
use rustfft::num_complex::Complex;
use rustfft::num_traits::Zero;

#[derive(Debug)]
pub struct FilterArgs {
    pub input_file: String,
    pub keep_even: bool,
    pub keep_odd: bool,
    pub keep_bitmap: Option<String>,
    pub keep_pattern: Option<String>,
    pub protect_fundamental: bool,
    pub normalize: bool
}

pub struct Bitmap {
    bits: Vec<bool>
}

impl Bitmap {
    fn from_string(bitstring: &str) -> Bitmap {
        let mut bitmap = Bitmap { bits: Vec::<bool>::new() };
        for c in bitstring.chars() {
            match c {
                '0' => bitmap.bits.push(false),
                '1' => bitmap.bits.push(true),
                _ => panic!("Invalid bitstring given.")
            }
        }
        bitmap
    }
}

pub fn run_filter(args: &FilterArgs) -> () {
    let mut wtf = WaveTable::load_from_wav(&args.input_file);

    for i in 0..wtf.cycles.len() {
        let mut partials = wtf.cycles[i].fft();
        let fundamental_backup = partials.partials[1];

        if args.keep_even {
            // Kill off all the odd harmonics.
            for p in 0..partials.partials.len() {
                if p % 2 != 0 {
                    partials.partials[p] = Complex::zero();
                }
            }
        }

        if args.keep_odd {
            // Kill off all the even harmonics.
            for p in 0..partials.partials.len() {
                if p % 2 == 0 {
                    partials.partials[p] = Complex::zero();
                }
            }
        }

        if let Some(bitmap) = &args.keep_bitmap {
            let bitmap = Bitmap::from_string(&bitmap);
            for p in 0..partials.partials.len() {
                if p >= bitmap.bits.len() || !bitmap.bits[p] {
                    partials.partials[p] = Complex::zero();
                }
            }
        }

        if let Some(pattern) = &args.keep_pattern {
            let bitmap = Bitmap::from_string(&pattern);
            for p in 0..partials.partials.len() {
                if !bitmap.bits[p % bitmap.bits.len()] {
                    partials.partials[p] = Complex::zero();
                }
            }
        }

        if args.protect_fundamental {
            partials.partials[1] = fundamental_backup;
        }

        // Save our changes.
        wtf.cycles[i] = partials.fft();
    }

    if args.normalize {
        wtf = wtf.normalize();
    }

    let output_path = format!("{}.filtered.wav", &args.input_file);
    wtf.save_to_wav(&output_path);
}