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

|                                           |                                            |
|-------------------------------------------|--------------------------------------------|
| <img src="/docs/images/spectrum_1.png" height="500px" /> | <img src="/docs/images/spectrum_2.png" height="500px" /> |
| <img src="/docs/images/spectrum_3.png" height="500px" /> | <img src="/docs/images/spectrum_4.png" height="500px" /> |

Amplitude information is encoded as brightness and phase information is encoded
in the color hue.
