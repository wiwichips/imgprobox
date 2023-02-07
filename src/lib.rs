use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use web_sys::{CanvasRenderingContext2d, ImageData};
use wasm_bindgen::Clamped;

#[wasm_bindgen]
pub fn draw(ctx: &CanvasRenderingContext2d, width: u32, height: u32) -> Result<(), JsValue> {
    let currentImage = ctx.get_image_data(0.0, 0.0, width as f64, height as f64)?;
    let mut data = rotate_colours(width, height, currentImage);
    let data = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut data), width, height)?;
    ctx.put_image_data(&data, 0.1, 0.0)
}

pub fn clamp(val: u8, min: u8, max: u8) -> u8 {
    if val < min {
        return min;
    }
    if val > max {
        return max;
    }
    return val;
}

fn grayscale(width: u32, height: u32, img: ImageData) -> Vec<u8> {
    let mut data = Vec::new();
    let clamped = img.data();
    for i in (0..clamped.len()).step_by(4) {
        for j in 0..3 {
            data.push(clamp(255 - clamped[(i+j) as usize],0,255) as u8);
        }
        data.push(255 as u8);
    }
    data
}

fn rotate_colours(width: u32, height: u32, img: ImageData) -> Vec<u8> {
    let mut data = Vec::new();
    let clamped = img.data();
    for i in (0..clamped.len()).step_by(4) {
        data.push(clamped[i+2] as u8);
        data.push(clamped[i+0] as u8);
        data.push(clamped[i+1] as u8);
        data.push(255 as u8);
    }
    data
}


