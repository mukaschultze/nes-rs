import * as nes from "nes-web";

let upscale = false;

const context = nes.init();

const canvas = document.getElementById("canvas");
const background = document.getElementById("background");
const canvasContext = canvas.getContext("2d");

const ControllerInput = {
  A: 1 << 0,
  B: 1 << 1,
  SELECT: 1 << 2,
  START: 1 << 3,
  UP: 1 << 4,
  DOWN: 1 << 5,
  LEFT: 1 << 6,
  RIGHT: 1 << 7,
};

const KEYMAPS = {
  KeyZ: ControllerInput.A,
  KeyX: ControllerInput.B,
  Enter: ControllerInput.SELECT,
  Space: ControllerInput.START,
  ArrowUp: ControllerInput.UP,
  ArrowDown: ControllerInput.DOWN,
  ArrowLeft: ControllerInput.LEFT,
  ArrowRight: ControllerInput.RIGHT,
};

document.addEventListener("keydown", (evt) => {
  if (KEYMAPS[evt.code]) context.key_down(KEYMAPS[evt.code]);
});
document.addEventListener("keyup", (evt) => {
  if (KEYMAPS[evt.code]) context.key_up(KEYMAPS[evt.code]);
  if (evt.code === "KeyQ") upscale = !upscale;
});

let frames = 0;
let fpsLog = 2000;

setInterval(() => {
  console.log(`${frames / (fpsLog / 1000)} FPS`);
  frames = 0;
}, fpsLog);

const renderLoop = () => {
  frames++;
  requestAnimationFrame(renderLoop);

  canvas.width = upscale ? 512 : 256;
  canvas.height = upscale ? 480 : 240;

  context.nes_frame();

  const imageData = canvasContext.getImageData(
    0,
    0,
    canvas.width,
    canvas.height
  );

  if (upscale) context.set_image_array_upscale(imageData.data);
  else context.set_image_array(imageData.data);

  canvasContext.putImageData(imageData, 0, 0);
  background.style.backgroundColor = context.get_background_color();
};

requestAnimationFrame(renderLoop);
