const wasm = import("./pkg");

wasm.then((wasm) => {
  const context = new AudioContext();

  // get the audio element
  const audioElement = document.querySelector("audio");

  // pass it into the audio context
  const track = context.createMediaElementSource(audioElement);
  const granulatorProcessor = context.createScriptProcessor(512, 2, 2);
  const granulator = new wasm.Granulator();

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
    console.log("density", densitySlider.value);
    granulator.set_density(densitySlider.value);
  });

  const volumeSlider = document.getElementById("volume");
  volumeSlider.addEventListener("input", function () {
    console.log("volume", volumeSlider.value / 10);
    granulator.set_volume(volumeSlider.value / 10);
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
