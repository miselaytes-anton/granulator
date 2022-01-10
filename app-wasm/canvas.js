const canvas = document.getElementById("canvas");
const ctx = canvas.getContext("2d");
const canvasWidth = canvas.width;
const canvasHeight = canvas.height;

const zoomLevel = 20;
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

const getRandomColor = () =>
  "#000000".replace(/0/g, () => (~~(Math.random() * 16)).toString(16));

let isColor1 = true;
const getColor = () => {
  isColor1 = !isColor1;
  if (isColor1) {
    return "grey";
  }

  return "black";
};

const drawPlayLine = () => {
  ctx.strokeStyle = "rgba(76,82,112,1)";
  ctx.lineWidth = 1;

  ctx.beginPath();
  ctx.moveTo(playLineX, 0);
  ctx.lineTo(playLineX, canvasHeight);
  ctx.stroke();
};

const drawBackground = () => {
  ctx.clearRect(0, 0, canvasWidth, canvasHeight);
  ctx.fillStyle = "white";
  ctx.fillRect(0, 0, canvasWidth, canvasHeight);
};

const draw = () => {
  drawBackground();
  drawPlayLine();

  while (grains.length > 0 && grains[0].x > canvasWidth) {
    grains.shift();
  }

  grains.forEach((grain) => {
    ctx.fillStyle = grain.color;

    ctx.fillRect(grain.x, grain.y, grain.width, grain.height);
    grain.x += advanceByX;
  });
};

setInterval(draw, framesPerSecond);

let totalGrainsAdded = 0;
let color = getRandomColor();

export const addGrain = (duration) => {
  const width = Math.max(
    Math.round(parseInt(duration) * scalingCoefficient),
    1
  );
  if (totalGrainsAdded === grainTracks) {
    totalGrainsAdded = 0;
    color = getRandomColor();
  }
  const grainTrackIndex = totalGrainsAdded % grainTracks;
  const height = 10;

  grains.push({
    color,
    width,
    height,
    x: playLineX - width,
    y: height + grainTrackIndex * height * 3,
  });

  totalGrainsAdded++;
};
