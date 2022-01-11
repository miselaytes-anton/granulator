import SimpleReverb from "./reverb";
const wasm = import("./pkg");

wasm.then((wasm) => {
  const context = new AudioContext();

  // get the audio element
  const audioElement = document.querySelector("audio");

  // pass it into the audio context
  const track = context.createMediaElementSource(audioElement);
  const granulatorProcessor = context.createScriptProcessor(512, 2, 2);
  const granulator = new wasm.Granulator();
  granulator.set_new_grain_hook();

  const reverb = new SimpleReverb(context, {
    seconds: 1.0,
    decay: 1.0,
    reverse: 0,
  });

  track.connect(granulatorProcessor);
  granulatorProcessor.connect(reverb.input);
  //granulatorProcessor.connect(context.destination);

  reverb.connect(context.destination);

  context.suspend();

  granulatorProcessor.onaudioprocess = function (event) {
    const input = event.inputBuffer;
    const output = event.outputBuffer;
    granulator.process(
      input.getChannelData(0),
      input.getChannelData(1),
      output.getChannelData(0),
      output.getChannelData(1)
    );
  };

  const densitySlider = document.getElementById("density");
  densitySlider.value = granulatorProcessor.density;
  densitySlider.addEventListener("input", function () {
    const value = parseFloat(densitySlider.value);
    console.log("density", value);
    granulator.set_density(value);
  });

  const volumeSlider = document.getElementById("volume");
  densitySlider.value = granulatorProcessor.volume;
  volumeSlider.addEventListener("input", function () {
    const value = parseFloat(volumeSlider.value);
    console.log("volume", value);
    granulator.set_volume(value);
  });

  const positionsSlider = document.getElementById("position");
  densitySlider.value = granulatorProcessor.position;
  positionsSlider.addEventListener("input", function () {
    const value = parseInt(positionsSlider.value);
    console.log("position", value);
    granulator.set_position(value);
  });

  const durationSlider = document.getElementById("duration");
  densitySlider.value = granulatorProcessor.duration;
  durationSlider.addEventListener("input", function () {
    const value = parseInt(durationSlider.value);
    console.log("duration", value);
    granulator.set_duration(value);
  });

  const pitchSlider = document.getElementById("pitch");
  pitchSlider.value = granulatorProcessor.pitch;
  pitchSlider.addEventListener("input", function () {
    const value = pitchSlider.value;
    console.log("pitch", value);
    granulator.set_pitch(value);
  });

  const feedbackSlider = document.getElementById("feedback");
  feedbackSlider.value = granulatorProcessor.feedback;
  feedbackSlider.addEventListener("input", function () {
    const value = parseFloat(feedbackSlider.value);
    console.log("feedback", value);
    granulator.set_feedback(value);
  });

  const wetDrySlider = document.getElementById("wet-dry");
  wetDrySlider.value = granulatorProcessor.wet_dry;
  wetDrySlider.addEventListener("input", function () {
    const value = parseFloat(wetDrySlider.value);
    console.log("wet-dry", value);
    granulator.set_wet_dry(value);
  });

  // select our play button
  const playButton = document.querySelector("button");

  playButton.addEventListener(
    "click",
    function () {
      console.log("playButton");

      // check if context is in suspended state (autoplay policy)
      if (context.state === "suspended") {
        context.resume();
      }

      // play or pause track depending on state
      if (this.dataset.playing === "false") {
        audioElement.play();
        this.dataset.playing = "true";
      } else if (this.dataset.playing === "true") {
        audioElement.pause();
        this.dataset.playing = "false";
      }
    },
    false
  );
});
