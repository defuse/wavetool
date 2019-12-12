use crate::wavetable::{WaveTable, WaveCycle, WaveCyclePartials};

#[derive(Debug)]
pub struct SpectrogramArgs {
    pub input_file: String,
    pub phase: bool
}

pub fn run_spectrogram(args: &SpectrogramArgs) -> () {
    println!("{:?}", args);
}