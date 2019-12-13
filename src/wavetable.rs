// Serum et al. use 2048 samples to represent a single cycle.
pub const WAVE_SAMPLES: usize = 2048;
// DC offset + 1024 partials.
pub const PARTIAL_COUNT: usize = 1025;

use rustfft::FFTplanner;
use rustfft::num_complex::Complex;
use rustfft::num_traits::Zero;
use hound;

#[derive(Clone)]
pub struct WaveTable {
    pub cycles: Vec<WaveCycle>
}

pub struct WaveSpectrogram {
    pub cycles: Vec<WaveCyclePartials>
}

#[derive(Clone)]
pub struct WaveCycle {
    pub samples: [f32; WAVE_SAMPLES]
}

#[derive(Clone)]
pub struct WaveCyclePartials {
    pub partials: [Complex<f32>; PARTIAL_COUNT]
}

impl WaveTable {
    pub fn load_from_wav(path: &str) -> WaveTable {
        let mut reader = hound::WavReader::open(path).unwrap();
        let spec = reader.spec();
        if spec.channels != 1 {
            panic!("Invalid wavetable: file is not mono.");
        }

        let samples : Vec<f32> = reader.samples::<f32>().map(Result::unwrap).collect();
        if samples.len() == 0 || (samples.len() % WAVE_SAMPLES) != 0 {
            panic!("Invalid wavetable: bad number of samples (empty or not a multiple of 2048)");
        }

        let mut wavetable = WaveTable { cycles: Vec::<WaveCycle>::new() };

        let num_cycles = samples.len() / WAVE_SAMPLES;
        for cycle in 0..num_cycles {
            let mut wavecycle = WaveCycle { samples: [0.0; WAVE_SAMPLES] };

            for sample in 0..WAVE_SAMPLES {
                wavecycle.samples[sample] = samples[cycle * WAVE_SAMPLES + sample];
            }

            wavetable.cycles.push(wavecycle);
        }

        wavetable
    }

    pub fn save_to_wav(&self, path: &str) -> () {
        let spec = hound::WavSpec {
            channels: 1,
            sample_rate: 44100,
            bits_per_sample: 32,
            sample_format: hound::SampleFormat::Float,
        };

        let mut writer = hound::WavWriter::create(path, spec).unwrap();

        for cycle in self.cycles.iter() {
            for sample in cycle.samples.iter() {
                writer.write_sample(*sample).unwrap();
            }
        }

    }

    pub fn normalize(&self) -> WaveTable {
        // TODO: implement normalization (same gain applied across the whole table)
        self.clone()
    }

    pub fn to_spectrogram(&self) -> WaveSpectrogram {
        WaveSpectrogram { cycles: self.cycles.iter().map(|cycle| cycle.fft()).collect() }
    }
}

impl WaveCycle {
    pub fn fft(&self) -> WaveCyclePartials {

        // When working with real signals, the output of an FFT has some redundancy, see:
        // https://math.stackexchange.com/questions/867337/how-to-interpret-the-imaginary-part-of-an-inverse-fourier-transform
        //
        // We compute the FFT, then throw away the redundant part, keeping only the partials for easy editing.WAVE_SAMPLES
        // The part we're throwing away is just the conjugate of the part we're keeping, so we can recover it later.

        let mut input: Vec<Complex<f32>> = self.samples.iter().map(|s| Complex { re: *s, im: 0.0 } ).collect();

        let mut planner = FFTplanner::new(false);
        let fft = planner.plan_fft(WAVE_SAMPLES);
        let mut frequency_domain = [Complex::<f32>::zero(); WAVE_SAMPLES];
        fft.process(&mut input, &mut frequency_domain);

        let mut partials = WaveCyclePartials { partials: [Complex::<f32>::zero(); PARTIAL_COUNT]};
        for i in 0..partials.partials.len() {
             // We have to normalize by dividing by the number of samples.
            partials.partials[i] = frequency_domain[i] / (WAVE_SAMPLES as f32);
        }
        
        partials
    }

    pub fn print(&self) {
        for s in self.samples.iter() {
            print!("{},", s)
        }
        println!("");
    }
}

impl WaveCyclePartials {
    pub fn fft(&self) -> WaveCycle {
        let mut input = self.partials.to_vec();
        let mut output = vec![Complex::zero(); WAVE_SAMPLES];

        // Rebuild the symmetric/conjugate part of the frequency domain which we threw away.
        for i in 1..WAVE_SAMPLES/2 {
            input.push(input[WAVE_SAMPLES/2-i].conj());
        }
        
        let mut planner = FFTplanner::new(true);
        let fft = planner.plan_fft(WAVE_SAMPLES);
        fft.process(&mut input, &mut output);

        let mut cycle = WaveCycle { samples: [0.0; WAVE_SAMPLES] };

        // There should never be a significant imaginary component.
        for c in output.iter() {
            assert!(c.im.abs() < 0.000001);
        }

        let real_parts: Vec<f32> = output.iter().map(|c| c.re).collect();
        cycle.samples.copy_from_slice(&real_parts[..WAVE_SAMPLES]);

        cycle
    }

    pub fn print(&self) {
        for s in self.partials.iter() {
            print!("{},", s)
        }
        println!("");
    }
}

#[cfg(test)]
mod test {
    use crate::wavetable::*;
    use float_cmp::approx_eq;

    #[test]
    fn test_fft_fft() {
        let saw = &WaveTable::load_from_wav("wavetables/saw.wav").cycles[0];
        let saw2 = saw.fft().fft();
        for (a, b) in saw.samples.iter().zip(saw2.samples.iter()) {
            assert!(approx_eq!(f32, *a, *b, epsilon=0.0001));
        }
    }

    #[test]
    fn test_fft_edit_fft() {
        let saw = &WaveTable::load_from_wav("wavetables/saw.wav").cycles[0];
        let mut partials = saw.fft();

        for i in 1..partials.partials.len() {
            if i % 3 == 0 || i % 2 == 0 {
                partials.partials[i] = Complex::zero();
            }
        }

        let partials2 = partials.fft().fft();

        for (a, b) in partials.partials.iter().zip(partials2.partials.iter()) {
            println!("{}, {}", a.norm(), b.norm());
            assert!(approx_eq!(f32, a.norm(), b.norm(), epsilon=0.0001));
        }
    }

}