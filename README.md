# wavetool

wavetool is a command-line processing/analysis tool for Serum wavetables. It can
be used to edit wavetables in various ways. The currently supported features
are:

- Filter harmonics:
    - Remove even/odd harmonics.
    - Specify which harmonics to keep with a bitmap.
    - Specify a repeating pattern of harmonics to keep.
    - Factor a wavetable into components at frequencies that are a prime
      multiple of the fundamental (experimental).

It can also generate cool-looking and informative spectrograms from your
wavetables like this:

<p align="center">
<img src="/docs/images/spectrum_1.png" height="500px" /> <img src="/docs/images/spectrum_2.png" height="500px" />
<img src="/docs/images/spectrum_3.png" height="500px" />
</p>

Amplitude information is encoded as brightness and phase information is encoded
in the color hue.

## Usage

Building:

```
cargo build --release
```

To generate a spectrogram:

```
wavetool spectrogram -p /path/to/wavetable.wav
```

This will save the spectrogram to `/path/to/wavetable.wav.spectrum.png`.

To apply a filter to a wavetable, e.g. specify a bitmap of harmonics to keep:

```
wavetool filter --bitmap 111000111000111000111000111 /path/to/wavetable.wav
```

The result will be saved to `/path/to/wavetable.wav.filtered.wav`.

## License

Licensed under either of

- Apache License, Version 2.0, (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)

at your option.
