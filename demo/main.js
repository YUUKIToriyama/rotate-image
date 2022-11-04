import init, { ImageConverter } from "./pkg/rotate_image.js";

await init();

const canvasInput = document.getElementById("canvasInput");
const canvasOutput = document.getElementById("canvasOutput");

document.getElementById("inputNumber").addEventListener("change", event => {
    const radian = parseInt(event.target.value) * Math.PI / 180;
    const image = ImageConverter.from_canvas(canvasInput);
    image.rotate(radian);
    image.to_canvas(canvasOutput);
});

document.getElementById("inputFile").addEventListener("change", event => {
    const file = event.target.files[0];
    loadImageToCanvas(
        URL.createObjectURL(file),
        canvasInput
    );
});

const loadImageToCanvas = (url, canvasElement) => {
    let context = canvasElement.getContext("2d");
    let img = new Image();
    img.addEventListener("load", () => {
        canvasElement.width = img.width;
        canvasElement.height = img.height;
        context.drawImage(img, 0, 0, img.width, img.height);
    });
    img.crossOrigin = "anonymous";
    img.src = url;
}