// Serum et al. use 2048 samples to represent a single cycle.
pub const WAVE_SAMPLES: usize = 2048;
// DC offset + 1024 partials.
pub const PARTIAL_COUNT: usize = 1025;

use rustfft::FFTplanner;
use rustfft::num_complex::Complex;
use rustfft::num_traits::Zero;
use hound;
use std::io::Read;
use std::io::Write;

#[derive(Clone)]
pub struct WaveTable {
    pub cycles: Vec<WaveCycle>,
    clm_chunk: Option<Vec<u8>>
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
        let file = std::fs::File::open(path).unwrap();
        let buf_reader = std::io::BufReader::new(file);
        let mut chunk_reader = hound::ChunksReader::new(buf_reader).unwrap();

        let mut clm_chunk: Option<Vec<u8>> = None;

        // This only works if the 'clm ' chunk comes before the data.
        // FIXME: Fix that. I was running into problems with the borrow checker and gave up.
        while let Ok(Some(chunk)) = chunk_reader.next() {
            match chunk {
                hound::Chunk::Data => {
                    break;
                }
                hound::Chunk::Fmt(_) => { },
                hound::Chunk::Fact => { },
                hound::Chunk::Unknown(chars, mut reader) => {
                    if (chars[0],chars[1],chars[2],chars[3]) == ('c' as u8, 'l' as u8, 'm' as u8, ' ' as u8) {
                        let mut buffer = [0u8; 128];
                        let length = reader.read(&mut buffer).unwrap();
                        clm_chunk = Some(buffer[0..length].to_vec());
                    }
                }
            };
        }

        // Will throw an error if the above loop didn't leave it in a data section
        let samples: Vec<f32> = chunk_reader.samples::<f32>().map(Result::unwrap).collect();

        // I'm basically copying the implementation of WavReader here.
        if let Some(spec_ex) = chunk_reader.spec_ex {
            if spec_ex.spec.channels != 1 {
                panic!("Invalid wavetable: file is not mono.");
            }
        } else {
            panic!("Wave file has no fmt header.");
        }

        let mut wavetable = WaveTable {
            cycles: Vec::<WaveCycle>::new(),
            clm_chunk: clm_chunk
        };

        if samples.len() == 0 || (samples.len() % WAVE_SAMPLES) != 0 {
            panic!("Invalid wavetable: bad number of samples (empty or not a multiple of 2048)");
        }
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

        let file = std::fs::File::create(path).unwrap();
        let buf_writer = std::io::BufWriter::new(file);
        let mut chunks_writer = hound::ChunksWriter::new(buf_writer).unwrap();
        chunks_writer.write_fmt(spec).unwrap();

        if let Some(clm_chunk) = &self.clm_chunk {
            println!("writing serum data");
            let mut writer = chunks_writer.start_chunk(['c' as u8, 'l' as u8, 'm' as u8, ' ' as u8]).unwrap();
            let written_len = writer.write(&clm_chunk).unwrap();
            assert!(written_len == clm_chunk.len());
            writer.finalize().unwrap();
        }

        chunks_writer.start_data_chunk().unwrap();

        for cycle in self.cycles.iter() {
            for sample in cycle.samples.iter() {
                chunks_writer.write_sample(*sample).unwrap();
            }
        }

    }

    pub fn normalize(&self) -> WaveTable {
        let mut normalized = self.clone();
        let mut gain = std::f32::INFINITY;

        // Find the smallest gain that will bring at least one sample to unity.
        for cycle in normalized.cycles.iter() {
            for sample in cycle.samples.iter() {
                // FIXME: This can probably cause slight clipping due to floating point error.
                let gain_for_sample = 1.0 / sample;
                if gain_for_sample < gain {
                    gain = gain_for_sample;
                }
            }
        }

        assert!(gain != std::f32::INFINITY);

        // Apply the gain
        for cycle in &mut normalized.cycles {
            for i in 0..cycle.samples.len() {
                cycle.samples[i] *= gain;
            }
        }

        normalized
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
        // We compute the FFT, then throw away the redundant part, keeping only the partials for easy editing.
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