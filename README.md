# Granulator

## Some ideas

- different scheduling algorithm based on density
- randomly offset grain delay_time, maybe using osc
- modulate grains playback rate.
- add a possibility to set Granulator parameters, e.g set_grain_density
- Independent panning of each grain to create spatially diffused textures

- different durations?
- why did not I need interpolated sample look up?
- Ramping the source playback rate of each grain to create glissandi grains or ‘chirps.’

## References

https://gyng.github.io/synthrs/synthrs/filter/struct.DelayLine.html

https://github.com/RustAudio/dasp

https://doc.rust-lang.org/book/ch17-03-oo-design-patterns.html

https://github.com/irh/freeverb-rs/blob/master/freeverb/src/all_pass.rs
