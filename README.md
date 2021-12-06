# Granulator

## Some ideas

- global pitch control
- brake into multiple packages: granulator, process_wav, web
- compile to web assembly and play around with knobs
- Independent panning of each grain to create spatially diffused textures

- different durations?
- why did not I need interpolated sample look up?
- Ramping the source playback rate of each grain to create glissandi grains or ‘chirps.’

## References

https://gyng.github.io/synthrs/synthrs/filter/struct.DelayLine.html

https://github.com/RustAudio/dasp

https://doc.rust-lang.org/book/ch17-03-oo-design-patterns.html

https://github.com/irh/freeverb-rs/blob/master/freeverb/src/all_pass.rs

## Comparisons

Mutable instruments clouds:

- https://mutable-instruments.net/modules/beads/
- https://mutable-instruments.net/modules/clouds/manual/
- https://github.com/pichenettes/eurorack/tree/master/clouds/dsp

  - Grain generation time base: periodical, randomized, or externally clocked.
  - The POSITION knob selects from which part of the recording buffer the grains are extracted.
  - Grain size from 16ms to 1s.
  - Up to 40 to 60 concurrent grains (depending on recording buffer resolution).
  - Transposition from -2 octaves to +2 octaves, with V/O tracking.
  - Grain envelope continuously variable between boxcar, triangle and Hann functions.
  - CV inputs for all grain parameters, individually sampled and held by each grain. For stochastic, Xenakis-style explorations, try feeding random voltages to those!
  - Diffusion network with 4 All-pass filters to post process the granularized signals.

  - A diffuser (network of all-pass filters - like a reverb without tail) can be applied to smear transients.
  - Very dense clouds sound the best when at least one parameter (pitch or position) receives random modulations. Otherwise, the many identical “echoes” created by the repeating grains will sound like a very resonant feedback comb filter.

  Blend:

  - Dry/wet balance.
  - Stereo spread (amount of random panning/balance applied to the grains).
  - Feedback amount.
  - Reverberation amount.
