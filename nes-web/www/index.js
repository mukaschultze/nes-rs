import * as nes from "nes-web";

const context = nes.init();

const canvas = document.getElementById("canvas");
const canvasContext = canvas.getContext("2d");

const KEYMAPS = {
    "KeyZ": 1 << 0, // A,
    "KeyX": 1 << 1, // B,
    "Enter": 1 << 2, // SELECT,
    "Space": 1 << 3, // START,
    "ArrowUp": 1 << 4, // UP,
    "ArrowDown": 1 << 5, // DOWN,
    "ArrowLeft": 1 << 6, // LEFT,
    "ArrowRight": 1 << 7, // RIGHT,
};

document.addEventListener("keydown", (evt) => {
    if (KEYMAPS[evt.code])
        context.key_down(KEYMAPS[evt.code])
});
document.addEventListener("keyup", (evt) => {
    if (KEYMAPS[evt.code])
        context.key_up(KEYMAPS[evt.code])
});

const renderLoop = () => {
    context.nes_frame();
    requestAnimationFrame(renderLoop);

    const imageData = canvasContext.getImageData(0, 0, canvas.width, canvas.height);

    context.set_image_array(imageData.data, canvas.width, canvas.height);
    canvasContext.putImageData(imageData, 0, 0);
};

requestAnimationFrame(renderLoop);
