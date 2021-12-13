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

  track.connect(granulatorProcessor);
  granulatorProcessor.connect(context.destination);
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
  densitySlider.addEventListener("input", function () {
    const value = parseFloat(densitySlider.value);
    console.log("density", value);
    granulator.set_density(value);
  });

  const volumeSlider = document.getElementById("volume");
  volumeSlider.addEventListener("input", function () {
    const value = parseFloat(volumeSlider.value / 10);
    console.log("volume", value);
    granulator.set_volume(value);
  });

  const positionsSlider = document.getElementById("position");
  positionsSlider.addEventListener("input", function () {
    const value = parseInt(positionsSlider.value);
    console.log("position", value);
    granulator.set_position(value);
  });

  const durationSlider = document.getElementById("duration");
  durationSlider.addEventListener("input", function () {
    const value = parseInt(durationSlider.value);
    console.log("duration", value);
    granulator.set_duration(value);
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
