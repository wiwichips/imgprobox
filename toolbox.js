//import init, { add, greet } from './pkg/without_a_bundler.js';
//import { send_example_to_js, receive_example_from_js, process_image } from "./pkg/without_a_bundler.js";
import init, { draw } from './pkg/without_a_bundler.js';

export function grayscale(img) {
  ctx.drawImage(img, 0, 0);
  const imageData = ctx.getImageData(0, 0, canvas.width, canvas.height);
  const data = imageData.data;
  for (let i = 0; i < data.length; i += 4) {
    const avg = (data[i] + data[i + 1] + data[i + 2]) / 3;
    data[i] = avg; // red
    data[i + 1] = avg; // green
    data[i + 2] = avg; // blue
  }
  ctx.putImageData(imageData, 0, 0);
};

export function changeColour(img) {
  ctx.drawImage(img, 0, 0);
  const imageData = ctx.getImageData(0, 0, canvas.width, canvas.height);
  const data = imageData.data;
  for (let i = 0; i < data.length; i += 4) {
    const r = data[i];
    const g = data[i+1];
    const b = data[i+2];
    data[i]     = Math.min((255 - r), 255); // red
    data[i + 1] = Math.min(r&b + (i/200)%256, 255) // green
    data[i + 2] = Math.min(b, 255); // blue
  }
  ctx.putImageData(imageData, 0, 0);
};

var loop = 2;
export function crazyColour(imageData) {
  loop = loop + 370;
  const data = imageData.data;
  for (let i = 0; i < data.length; i += 4) {
    const r = data[i];
    const g = data[i+1];
    const b = data[i+2];
    data[i]     = 255 - r; // red
    data[i + 1] = (loop/g)%256; // green
    data[i + 2] = b; // blue
    for (let j = 0; j < 3; j++) {
        data[j] = Math.max(1, Math.min(data[j], 255));
    }
  }
  return imageData;
};

export function rustImage(ctx) {
    draw(ctx, 600, 600, -0.15, 0.65);
}

