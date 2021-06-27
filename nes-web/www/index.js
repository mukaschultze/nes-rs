import * as nes from "nes-web";

function bytesToSize(bytes) {
  var sizes = ["Bytes", "KB", "MB", "GB", "TB"];
  if (bytes == 0) return "0 Byte";
  var i = parseInt(Math.floor(Math.log(bytes) / Math.log(1024)));
  return Math.round(bytes / Math.pow(1024, i), 2) + " " + sizes[i];
}

let last = window.performance.memory;
const logMemoryChange = () => {
  const current = window.performance.memory;
  const change = current.usedJSHeapSize - last.usedJSHeapSize;

  if (change > 0) console.log(`Allocated ${bytesToSize(change)} of heap`);
  else console.log(`Freed ${bytesToSize(-change)} of heap`);

  last = current;
};

setInterval(logMemoryChange, 5000);

const context = nes.init();

const canvas = document.getElementById("canvas");
const background = document.getElementById("background");

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
});

let frames = 0;
let fpsLog = 2000;

setInterval(() => {
  console.log(`${frames / (fpsLog / 1000)} FPS`);
  frames = 0;
}, fpsLog);

context.setup_canvas(canvas);

const renderLoop = () => {
  frames++;
  requestAnimationFrame(renderLoop);

  context.update_canvas(canvas);
  background.style.backgroundColor = context.get_background_color();
};

requestAnimationFrame(renderLoop);
