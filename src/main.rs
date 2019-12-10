
#[macro_use]
extern crate clap;

fn main() {

   let matches = clap_app!(wavetool =>
        (version: "0.0.1")
        (author: "Taylor Hornby <taylor@defuse.ca>")
        (about: "Serum wavetable editing / analysis tool.")
        (@subcommand spectrogram =>
            (about: "Generates a spectrogram from a wavetable")
            (@arg INPUT: +required "The wavetable to analyze")
        )
        (@subcommand factor =>
            (about: "Factors a wavetable into its prime-multiple-of-fundamental components")
            (@arg INPUT: +required "The wavetable to factor")
            (@arg normalize: -n --normalize "Normalize the outputs")
            (@arg shift: -s --shift "Shift harmonics down to the fundamental")
            (@arg recursive: -r --recursive "Factor generated wavetables recursively")
        )
        (@subcommand filter =>
            (about: "Filter harmonics in various ways.")
            (@arg INPUT: +required "The wavetable to be filtered")
            (@arg OUTPUT: +required "Output file")
            (@arg even: -e --even "Keep only the even harmonics")
            (@arg odd: -o --odd "Keep only the odd harmonics")
            (@arg BITMAP: -b --bitmap +takes_value "Keep only the harmonics specified by a bitmap.")
            (@arg PATTERN: -p --pattern +takes_value "Repeat a bitmap up the spectrum")
            (@arg KEEP_PRIMES: -k --keepprimes +takes_value "Keep only a set of prime-factorization tree branches")
            (@arg REMOVE_PRIMES: -r --removeprimes +takes_value "Remove a set of prime-factorization tree branches")
            (@arg fundamental: -f --fundamental "Protect the fundamental (overrides other filters)")
            (@arg normalize: -n --normalize "Normalize the output")
        )
    )
    .get_matches();

    if let Some(matches) = matches.subcommand_matches("factor") {
        println!("Factoring!");
    }
}
