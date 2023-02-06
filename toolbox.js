import init, { add, greet } from './pkg/without_a_bundler.js';
import { send_example_to_js, receive_example_from_js } from "./pkg/without_a_bundler.js";


export async function run() {
  // First up we need to actually load the wasm file, so we use the
  await init();

  // And afterwards we can use all the functionality defined in wasm.
  const result = add(1, 2);
  greet(`1 + 2 = ${result}`);
  if (result !== 3)
    throw new Error("wasm addition doesn't work!");

  let example = send_example_to_js(); // get the example from wasm

  console.log(example);

  example.field2.push([5, 6]); // add another vec element to the end of the vec array
  receive_example_from_js(example); // send it back to rust wasm
}

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

