use crate::wavetable::{WaveTable, PARTIAL_COUNT};
use std::path::Path;
use std::fs::File;
use std::io::BufWriter;

const PIXEL_SIZE: u32 = 10;
// Serum only lets you edit up to 512, so the rest don't matter (+ DC offset)
// i.e. up to 15kHz when the wave is played at 30Hz.
const NUM_PARTIALS_WE_CARE_ABOUT: usize = 513;

#[derive(Debug)]
pub struct SpectrogramArgs {
    pub input_file: String,
    pub phase: bool
}

pub fn run_spectrogram(args: &SpectrogramArgs) -> () {
    let wt = WaveTable::load_from_wav(&args.input_file);
    let spectrogram = wt.to_spectrogram();

    let mut imgbuf = image::ImageBuffer::new(
        spectrogram.cycles.len() as u32 * PIXEL_SIZE, 
        NUM_PARTIALS_WE_CARE_ABOUT as u32 * PIXEL_SIZE
    );

    for (i, cycle) in spectrogram.cycles.iter().enumerate() {
        for (j, ampl) in cycle.partials.iter().map(|c| c.norm()).enumerate() {
            if j >= NUM_PARTIALS_WE_CARE_ABOUT {
                break;
            }
            for k in 0..PIXEL_SIZE {
                for l in 0..PIXEL_SIZE {
                    imgbuf.put_pixel(
                        i as u32 * PIXEL_SIZE + k,
                        // flip it so it goes low->high from bottom->top
                        (NUM_PARTIALS_WE_CARE_ABOUT as u32 - j as u32 - 1) * PIXEL_SIZE + l,
                        image::Rgb([get_red(ampl),
                        get_green(ampl),
                        get_blue(ampl)])
                    );
                }
            }
        }
    }

    let path_str = format!("{}.spectrum.png", args.input_file);
    imgbuf.save(path_str).unwrap();
}

fn get_red(ampl: f32) -> u8 {
    get_pixel(ampl)
}

fn get_green(ampl: f32) -> u8 {
    get_pixel(ampl)
}

fn get_blue(ampl: f32) -> u8 {
    get_pixel(ampl)
}

fn get_pixel(ampl: f32) -> u8 {
    // brightness decreases by 2 for every -1db, will saturate at 0 below -127db
    // FIXME: this is fucked
    (255.0 + 20.0*(ampl*ampl).log10()*2.0).max(0.0) as u8
}