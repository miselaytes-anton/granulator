const canvas = document.getElementById("canvas");
const ctx = canvas.getContext("2d");

let shouldInit = true;

const maxSimultaneousGrains = 100;
const grains = [];
const canvasWidth = 1500;
const canvasHeight = 1000;
const refreshRate = 40;
const framesPerSecond = 1000 / refreshRate;
const advanceByX = canvasWidth / refreshRate;
const advanceByY = canvasHeight / maxSimultaneousGrains;

let grainIndex = 0;

const drawGrains = () => {
  ctx.clearRect(0, 0, canvasWidth, canvasHeight);
  ctx.fillStyle = "#bcece0";
  ctx.fillRect(0, 0, canvasWidth, canvasHeight);

  while (grains[0].x > canvasWidth) {
    grains.shift();
  }

  grains.forEach((grain) => {
    grain.x += advanceByX;
    const opacity = parseFloat((1 - grain.x / canvasWidth).toFixed(3));
    ctx.fillStyle = `rgba(76,82,112,${opacity})`;

    ctx.fillRect(grain.x, grain.y, grain.width, grain.height);
  });
};

export const draw = (duration) => {
  const width = Math.max(Math.round(parseInt(duration) / 100), 1);
  const index = grainIndex % maxSimultaneousGrains;
  grains.push({
    width,
    height: 3,
    x: 0,
    y: index * 6,
  });

  if (shouldInit) {
    setInterval(drawGrains, framesPerSecond);

    shouldInit = false;
  }
  grainIndex++;
};
