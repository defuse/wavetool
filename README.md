# wavetool

wavetool is a processing/analysis tool for Serum wavetables. It can generate
pretty spectrograms from your wavetables like this:

|                                           |                                            |
|-------------------------------------------|--------------------------------------------|
| ![Spectrum 1](/docs/images/spectrum_1.png)| ![Spectrum 2](/docs/images/spectrum_2.png) |
! [Spectrum 3](/docs/images/spectrum_3.png) | ![Spectrum 4](/docs/images/spectrum_4.png) |

- Generate a spectrogram from a wavetable.
- Filter a wavetable's harmonics in various ways:
    - Keep only the even/odd harmonics.
    - Specify a bitmap of harmonics to keep.
    - Specify a repeating pattern of harmonics to keep.
- Factor a wavetable into its prime-multiple-of-fundamental components.

