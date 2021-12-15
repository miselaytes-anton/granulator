const canvas = document.getElementById("canvas");
const ctx = canvas.getContext("2d");
const canvasWidth = canvas.width;
const canvasHeight = canvas.height;

const zoomLevel = 10;
const sampleRate = 41000;
const scalingCoefficient = (zoomLevel * canvasWidth) / sampleRate;
const minGrainDurationMs = 300;

// How often grains get redrawn.
const refreshRateMs = 40;
const grainDisappearMs = 1000;
const framesPerSecond = grainDisappearMs / refreshRateMs;
const advanceByX = sampleRate / refreshRateMs / zoomLevel;

const playLineX = Math.round(minGrainDurationMs * scalingCoefficient);
const grainTracks = 25;
const grains = [];

const drawPlayLine = () => {
  ctx.strokeStyle = "black";
  ctx.lineWidth = 1;

  ctx.beginPath();
  ctx.moveTo(playLineX, 0);
  ctx.lineTo(playLineX, canvasHeight);
  ctx.stroke();
};

const drawBackground = () => {
  ctx.clearRect(0, 0, canvasWidth, canvasHeight);
  ctx.fillStyle = "#bcece0";
  ctx.fillRect(0, 0, canvasWidth, canvasHeight);
};

const draw = () => {
  drawBackground();
  drawPlayLine();

  while (grains.length > 0 && grains[0].x > canvasWidth) {
    grains.shift();
  }

  grains.forEach((grain) => {
    ctx.fillStyle = "rgba(76,82,112,1)";

    ctx.fillRect(grain.x, grain.y, grain.width, grain.height);
    grain.x += advanceByX;
  });
};

setInterval(draw, framesPerSecond);

let totalGrainsAdded = 0;

export const addGrain = (duration) => {
  const width = Math.max(
    Math.round(parseInt(duration) * scalingCoefficient),
    1
  );
  const grainTrackIndex = totalGrainsAdded % grainTracks;
  const height = 10;

  grains.push({
    width,
    height,
    x: playLineX - width,
    y: height + grainTrackIndex * height * 3,
  });

  totalGrainsAdded++;
};
