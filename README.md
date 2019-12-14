# wavetool

wavetool is a processing/analysis tool for Serum wavetables. It can generate
pretty spectrograms from your wavetables like this:

|                                           |                                            |
|-------------------------------------------|--------------------------------------------|
| <img src="/docs/images/spectrum_1.png" height="500px" /> | <img src="/docs/images/spectrum_2.png" height="500px" /> |
| <img src="/docs/images/spectrum_3.png" height="500px" /> | <img src="/docs/images/spectrum_3.png" height="500px" /> |

- Generate a spectrogram from a wavetable.
- Filter a wavetable's harmonics in various ways:
    - Keep only the even/odd harmonics.
    - Specify a bitmap of harmonics to keep.
    - Specify a repeating pattern of harmonics to keep.
- Factor a wavetable into its prime-multiple-of-fundamental components.

