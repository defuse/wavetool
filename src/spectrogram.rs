use crate::wavetable::{WaveTable, PARTIAL_COUNT};
use hsl::HSL;
use rustfft::num_complex::Complex32;

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
        for (j, partial) in cycle.partials.iter().enumerate() {
            assert!(NUM_PARTIALS_WE_CARE_ABOUT <= PARTIAL_COUNT);
            if j >= NUM_PARTIALS_WE_CARE_ABOUT {
                break;
            }
            for k in 0..PIXEL_SIZE {
                for m in 0..PIXEL_SIZE {
                    let (r,g,b) = if args.phase {
                        get_pixel_with_phase(partial)
                    } else {
                        // delete phase information if they don't want it
                        get_pixel_no_phase(partial)
                    };
                    
                    imgbuf.put_pixel(
                        i as u32 * PIXEL_SIZE + k,
                        // flip it so it goes low->high from bottom->top
                        (NUM_PARTIALS_WE_CARE_ABOUT as u32 - j as u32 - 1) * PIXEL_SIZE + m,
                        image::Rgb([r,g,b])
                    );
                }
            }
        }
    }

    let path_str = format!("{}.spectrum.png", args.input_file);
    imgbuf.save(path_str).unwrap();
}


fn get_pixel_with_phase(partial: &Complex32) -> (u8, u8, u8) {
    let luminosity = get_luminosity(partial);
    let hue = partial.arg() / std::f32::consts::PI * 180.0;
    let color = HSL { h: hue as f64, s: 1_f64, l: luminosity as f64};
    color.to_rgb()
}

fn get_pixel_no_phase(partial: &Complex32) -> (u8, u8, u8) {
    let luminosity = get_luminosity(partial);
    let color = HSL { h: 0_f64, s: 0_f64, l: luminosity as f64};
    color.to_rgb()
}

fn get_luminosity(partial: &Complex32) -> f32 {
    // brightness decreases by 2 for every -1db, will saturate at black below -127db
    // there's no real reason here, just fuck with the code until it looks right
    let power = partial.norm() * partial.norm();
    (255.0 + 20.0*(power).log10()*2.0).max(0.0) / 255.0
}