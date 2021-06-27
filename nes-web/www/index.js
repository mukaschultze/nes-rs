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

const KEYMAPS = {
  KeyZ: nes.ControllerKeys.A,
  KeyX: nes.ControllerKeys.B,
  Enter: nes.ControllerKeys.SELECT,
  Space: nes.ControllerKeys.START,
  ArrowUp: nes.ControllerKeys.UP,
  ArrowDown: nes.ControllerKeys.DOWN,
  ArrowLeft: nes.ControllerKeys.LEFT,
  ArrowRight: nes.ControllerKeys.RIGHT,
};

document.addEventListener("keydown", (evt) => {
  if (KEYMAPS[evt.code]) {
    context.key_down(KEYMAPS[evt.code]);
  }
});

document.addEventListener("keyup", (evt) => {
  if (KEYMAPS[evt.code]) {
    // This acts like a buffer for the keystrokes, making sure that an input is
    // registered even if it was released before an frame render
    runAfterNextFrame(() => {
      context.key_up(KEYMAPS[evt.code]);
    });
  }
});

const runAfterNextFrame = (func) =>
  requestAnimationFrame(() => setTimeout(func, 0));

let currentFrame = 0;
let fpsLog = 2000;

setInterval(() => {
  console.log(`${currentFrame / (fpsLog / 1000)} FPS`);
  currentFrame = 0;
}, fpsLog);

context.setup_canvas(canvas);

const renderLoop = () => {
  requestAnimationFrame(renderLoop);

  context.update_canvas(canvas);
  currentFrame++;
  background.style.backgroundColor = context.get_background_color();
};

requestAnimationFrame(renderLoop);
