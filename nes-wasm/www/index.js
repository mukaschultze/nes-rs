import { ControllerKeys, NesWebContext } from "nes-wasm";

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

const canvas = document.getElementById("canvas");
const background = document.getElementById("background");
const romInput = document.getElementById("rom");
const debugText = document.getElementById("debug");

const KEYMAPS = {
  KeyZ: ControllerKeys.A,
  KeyX: ControllerKeys.B,
  Enter: ControllerKeys.SELECT,
  Space: ControllerKeys.START,
  ArrowUp: ControllerKeys.UP,
  ArrowDown: ControllerKeys.DOWN,
  ArrowLeft: ControllerKeys.LEFT,
  ArrowRight: ControllerKeys.RIGHT,
};

let joypad = 0;

document.addEventListener("keydown", (evt) => {
  if (evt.code === "KeyQ") {
    joypad = joypad === 0 ? 1 : 0;
  }
  if (KEYMAPS[evt.code]) {
    context.key_down(KEYMAPS[evt.code], joypad);
  }
});

document.addEventListener("keyup", (evt) => {
  if (KEYMAPS[evt.code]) {
    // This acts like a buffer for the keystrokes, making sure that an input is
    // registered even if it was released before an frame render
    runAfterNextFrame(() => {
      context.key_up(KEYMAPS[evt.code], joypad);
    });
  }
});

let zapper_trigger = false;
let mouse_pixel_x = 0;
let mouse_pixel_y = 0;

document.addEventListener("contextmenu", (event) => event.preventDefault());

canvas.addEventListener("mousemove", (evt) => {
  mouse_pixel_x = ~~((evt.offsetX / canvas.clientWidth) * 256);
  mouse_pixel_y = ~~((evt.offsetY / canvas.clientHeight) * 240);
});

canvas.addEventListener("mousedown", (evt) => {
  zapper_trigger = true;
  setTimeout(() => {
    zapper_trigger = false;
  }, 100);
});

const runAfterNextFrame = (func) =>
  requestAnimationFrame(() => setTimeout(func, 0));

let currentFrame = 0;
let framerate = 0;
let lastframe = 0;
let fpsLog = 500;
let timings = [];

setInterval(() => {
  framerate = (currentFrame - lastframe) / (fpsLog / 1000);
  lastframe = currentFrame;
}, fpsLog);

const context = new NesWebContext();
context.setup_canvas(canvas);
context.attach_joypad(0);
// context.attach_zapper_gun(1);

romInput.addEventListener("input", async (evt) => {
  const file = evt.target.files[0];
  const buffer = await file.arrayBuffer();
  const arr = new Uint8Array(buffer);
  context.insert_cartridge(arr);
  context.reset();
  console.log(`${file.name} loaded`);
});

console.log("Input 0:", context.get_input_type(0));
console.log("Input 1:", context.get_input_type(1));

const renderLoop = () => {
  requestAnimationFrame(renderLoop);
  // setTimeout(renderLoop, 1000 / 60);

  // const brightness = context.brigthness_at(mouse_pixel_x, mouse_pixel_y);
  // const sensor = brightness > 0.9;
  // context.zapper_gun_input(zapper_trigger, zapper_trigger && sensor, 1);

  const t0 = performance.now();
  context.simulate();
  context.update_canvas(canvas);
  const t1 = performance.now();

  currentFrame++;
  background.style.backgroundColor = context.get_background_color();

  const diff = t1 - t0;
  timings[currentFrame % 120] = diff;

  const low = Math.min(...timings).toFixed(2);
  const high = Math.max(...timings).toFixed(2);
  const avg = (timings.reduce((a, b) => a + b) / timings.length).toFixed(2);
  const ideal = (1000 / 60).toFixed(2);

  if (currentFrame % 15 === 0) {
    debugText.innerText = `${framerate} FPS / Render: ${diff.toFixed(
      2
    )}ms / Low ${low}ms / Avg ${avg}ms / High ${high}ms / Ideal <${ideal}ms`;
  }
};

requestAnimationFrame(renderLoop);
// setInterval(renderLoop, Math.floor(1000 / 60));
