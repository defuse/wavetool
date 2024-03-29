#[macro_use]
extern crate clap;

mod spectrogram;
use spectrogram::{SpectrogramArgs, run_spectrogram};

mod factor;
use factor::{FactorArgs, run_factor};

mod filter;
use filter::{FilterArgs, run_filter};

mod wavetable;

fn main() {

   let matches = clap_app!(wavetool =>
        (version: "0.0.1")
        (author: "Taylor Hornby <taylor@defuse.ca>")
        (about: "Serum wavetable editing / analysis tool.")
        (@setting ArgRequiredElseHelp)
        (@subcommand spectrogram =>
            (about: "Generates a spectrogram from a wavetable.")
            (@arg INPUT: +required "The wavetable to analyze")
            (@arg phase: -p --phase "Include phase information as color")
        )
        (@subcommand factor =>
            (about: "Factors a wavetable into its prime-multiple-of-fundamental components.")
            (@arg INPUT: +required "The wavetable to factor")
            (@arg normalize: -n --normalize "Normalize the outputs")
            (@arg shift: -s --shift "Shift harmonics down to the fundamental")
            (@arg recursive: -r --recursive "Factor generated wavetables recursively")
        )
        (@subcommand filter =>
            (about: "Filter harmonics in various ways.")
            (@arg INPUT: +required "The wavetable to be filtered")
            (@arg even: -e --even "Keep only the even harmonics")
            (@arg odd: -o --odd "Keep only the odd harmonics")
            (@arg bitmap: -b --bitmap +takes_value "Keep only the harmonics specified by a bitmap")
            (@arg pattern: -p --pattern +takes_value "Repeat a bitmap up the spectrum")
            (@arg fundamental: -f --fundamental "Protect the fundamental (overrides other filters)")
            (@arg normalize: -n --normalize "Normalize the output")
        )
    )
    .get_matches();

    if let Some(matches) = matches.subcommand_matches("spectrogram") {
        let spectrogram_args = SpectrogramArgs {
            input_file: matches.value_of("INPUT").unwrap().to_string(),
            phase: matches.is_present("phase")
        };
        run_spectrogram(&spectrogram_args);
    } else if let Some(matches) = matches.subcommand_matches("factor") {
        let factor_args = FactorArgs {
            input_file: matches.value_of("INPUT").unwrap().to_string(),
            normalize: matches.is_present("normalize"),
            shift: matches.is_present("shift"),
            recursive: matches.is_present("recursive")
        };
        run_factor(&factor_args);
    } else if let Some(matches) = matches.subcommand_matches("filter") {
        let filter_args = FilterArgs {
            input_file: matches.value_of("INPUT").unwrap().to_string(),
            keep_even: matches.is_present("even"),
            keep_odd: matches.is_present("odd"),
            // These .map calls convert Option<&str> into Option<String>.
            keep_bitmap: matches.value_of("bitmap").map(String::from),
            keep_pattern: matches.value_of("pattern").map(String::from),
            protect_fundamental: matches.is_present("fundamental"),
            normalize: matches.is_present("normalize")
        };
        run_filter(&filter_args);
    }
}
