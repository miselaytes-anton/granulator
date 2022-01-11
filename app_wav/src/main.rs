use granulator::{Granulator, GranulatorOptions};
use rand::Rng;

use cpal;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use dasp::signal::{self, Signal};
use dasp::slice::ToFrameSliceMut;
use dasp::Sample;

fn main() -> Result<(), anyhow::Error> {
    // Find and load the wav.
    let assets = find_folder::Search::ParentsThenKids(5, 5)
        .for_folder("app_wav/assets")
        .unwrap();
    let reader = hound::WavReader::open(assets.join("piano.wav")).unwrap();
    let spec = reader.spec();
    println!("{:?}", spec);

    // Read the interleaved samples and convert them to a signal.
    let samples = reader
        .into_samples::<i16>()
        .filter_map(Result::ok)
        .map(|s| s.to_float_sample());

    let mut frames = signal::from_interleaved_samples_iter(samples).until_exhausted();

    // Initialise CPAL.
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("failed to find a default output device");

    // Create a stream config to match the wave format.
    //
    // NOTE: It's possible that the platform will not support the sample format, sample rate or
    // channel layout of the WAV file. In these cases, you may need to convert the data read from
    // the WAV file to a format compatible with one of the platform's supported stream
    // configurations.
    let config = cpal::StreamConfig {
        channels: spec.channels,
        sample_rate: cpal::SampleRate(spec.sample_rate),
        buffer_size: cpal::BufferSize::Default,
    };

    // A channel for indicating when playback has completed.
    let (complete_tx, complete_rx) = std::sync::mpsc::sync_channel(1);
    let _delay_time_seconds: usize = 2;
    let options = GranulatorOptions {
        ..GranulatorOptions::default()
    };
    let mut granulator = Granulator::new(options);
    //granulator.set_new_grain_hook(Some(|duration| println!("duration = {}\n", duration)));

    let mut counter = 0;
    let mut duration_counter = 0;

    // Create and run the CPAL stream.
    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);
    let data_fn = move |data: &mut [f32], _info: &cpal::OutputCallbackInfo| {
        let mut rng = rand::thread_rng();
        let buffer: &mut [[f32; 2]] = data.to_frame_slice_mut().unwrap();
        for out_frame in buffer {
            match frames.next() {
                Some(frame) => {
                    let processed = granulator.process(frame);
                    counter += 1;
                    if counter == 20500 {
                        counter = 0;
                        granulator.set_position(rng.gen_range(1000.0..41000.0));
                        granulator.set_density(rng.gen_range(1.0..100.0));
                    }
                    duration_counter += 1;
                    if duration_counter == 41000 {
                        duration_counter = 0;
                        let duration = rng.gen_range(1000.0..3000.0);
                        let pitch = rng.gen_range(0.5..2.0);
                        //println!("duration = {}, pitch = {}\n", duration, pitch);
                        granulator.set_duration(duration);
                        granulator.set_pitch(pitch);
                    }
                    *out_frame = processed
                }
                None => {
                    complete_tx.try_send(()).ok();
                    *out_frame = dasp::Frame::EQUILIBRIUM;
                }
            }
        }
    };
    let stream = device.build_output_stream(&config, data_fn, err_fn)?;
    stream.play().unwrap();

    // Block until playback completes.
    complete_rx.recv().unwrap();
    stream.pause().ok();
    Ok(())
}
