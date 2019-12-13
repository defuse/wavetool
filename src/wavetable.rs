pub const WAVE_SAMPLES: usize = 2048;

use rustfft::FFTplanner;
use rustfft::num_complex::Complex;
use rustfft::num_traits::Zero;
use hound;

#[derive(Clone)]
pub struct WaveTable {
    pub cycles: Vec<WaveCycle>
}

#[derive(Clone)]
pub struct WaveCycle {
    pub samples: [f32; WAVE_SAMPLES]
}

#[derive(Clone)]
pub struct WaveCyclePartials {
    pub partials: [Complex<f32>; WAVE_SAMPLES]
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

        let mut n = 0;
        for cycle in self.cycles.iter() {
            for sample in cycle.samples.iter() {
                n += 1;
                writer.write_sample(*sample).unwrap();
            }
        }

        println!("n: {}", n);
    }

    pub fn normalize(&self) -> WaveTable {
        // TODO: implement normalization
        self.clone()
    }
}

impl WaveCycle {
    pub fn fft(&self) -> WaveCyclePartials {
        let mut input: Vec<Complex<f32>> = self.samples.iter().map(|s| Complex { re: *s, im: 0.0 } ).collect();

        let mut partials = WaveCyclePartials { partials: [Complex::zero(); WAVE_SAMPLES] };
        let mut planner = FFTplanner::new(false);
        let fft = planner.plan_fft(WAVE_SAMPLES);
        fft.process(&mut input, &mut partials.partials);

        // For some reason you have to divide by the number of samples.
        for i in 0..partials.partials.len() {
            partials.partials[i] /= WAVE_SAMPLES as f32;
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
        let mut input = self.partials.clone();
        let mut output = vec![Complex::zero(); WAVE_SAMPLES];
        
        let mut planner = FFTplanner::new(true);
        let fft = planner.plan_fft(WAVE_SAMPLES);
        fft.process(&mut input, &mut output);

        let mut cycle = WaveCycle { samples: [0.0; WAVE_SAMPLES] };
        // FIXME: after making edits in the frequency domain, imaginary components in the time domain show up

        let im_parts: Vec<f32> = output.iter().map(|c| c.im).collect();
        println!("IM COMPS {:?}", im_parts);
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

        for i in 1..WAVE_SAMPLES {
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