use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, ImageData};
use wasm_bindgen::Clamped;
use std::convert::TryInto;

#[wasm_bindgen]
pub fn draw(ctx: &CanvasRenderingContext2d, width: u32, height: u32) -> Result<(), JsValue> {
    let currentImage = ctx.get_image_data(0.0, 0.0, width as f64, height as f64)?;
    let mut data = grayscale(width, height, currentImage);
    let data = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut data), width, height)?;
    ctx.put_image_data(&data, 0.1, 0.0)
}

// helper functions
fn clip(val: u8, min: u8, max: u8) -> u8 {
    if val < min {
        return min;
    }
    if val > max {
        return max;
    }
    return val;
}

fn get_pixel_idx(width: u32, height: u32, x: u32, y: u32) -> u32 {
    ((width * y + x) * 4) as u32
}

// image processing functions
fn grayscale(width: u32, height: u32, img: ImageData) -> Vec<u8> {
    let mut data = Vec::new();
    let clamped = img.data();
    for i in (0..clamped.len()).step_by(4) {
        let mut avg:u32 = 0;
        for j in 0..3 {
            avg += clamped[(i+j) as usize] as u32;
        }
        if (avg != 0) {
            avg = avg / 3;
        }
        for j in 0..3 {
            data.push(avg as u8);
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

fn cool_effect_01(width: u32, height: u32, img: ImageData) -> Vec<u8> {
    let mut data = Vec::new();
    let clamped = img.data();
    for y in 0..height {
        for x in 0..width {
            data.push(clamped[get_pixel_idx(width, height, x, y) as usize] as u8);
            data.push(clamped[(y*height+x) as usize] as u8);
            data.push(clamped[(x*width+y) as usize] as u8);
            data.push(255 as u8);
        }
    }
    data
}

fn flip_horizontal(width: u32, height: u32, img: ImageData) -> Vec<u8> {
    let mut data = Vec::new();
    let clamped = img.data();
    for y in 0..height {
        for x in 0..width {
            data.push(clamped[get_pixel_idx(width, height, width-1-x, y) as usize] as u8);
            data.push(clamped[(get_pixel_idx(width, height, width-1-x, y) + 1) as usize] as u8);
            data.push(clamped[(get_pixel_idx(width, height, width-1-x, y) + 2) as usize] as u8);
            data.push(255 as u8);
        }
    }
    data
}

fn flip_vertical(width: u32, height: u32, img: ImageData) -> Vec<u8> {
    let mut data = Vec::new();
    let clamped = img.data();
    for y in 0..height {
        for x in 0..width {
            data.push(clamped[get_pixel_idx(width, height, x, height-1-y) as usize] as u8);
            data.push(clamped[(get_pixel_idx(width, height, x, height-1-y) + 1) as usize] as u8);
            data.push(clamped[(get_pixel_idx(width, height, x, height-1-y) + 2) as usize] as u8);
            data.push(255 as u8);
        }
    }
    data
}

fn blur_nearest_neighbours(width: u32, height: u32, img: ImageData) -> Vec<u8> {
    let mut data = Vec::new();
    let clamped = img.data();
    for y in 0..height {
        for x in 0..width {
            data.push(clamped[get_pixel_idx(width, height, x, height) as usize] as u8);
            data.push(clamped[(get_pixel_idx(width, height, x, height) + 1) as usize] as u8);
            data.push(clamped[(get_pixel_idx(width, height, x, height) + 2) as usize] as u8);
            data.push(255 as u8);
        }
    }
    data
}

fn rotate_90(width: u32, height: u32, img: ImageData) -> Vec<u8> {
    // initialize every pixel to white
    let mut data = vec![255;img.data().len()];
    let clamped = img.data();
    for y in 0..height {
        for x in 0..width {
            let old_idx = get_pixel_idx(height, width, y, x) as usize;
            let new_idx = get_pixel_idx(width, height, x, height-1-y) as usize;
            data[new_idx] = clamped[old_idx];
            data[new_idx+1] = clamped[old_idx+1];
            data[new_idx+2] = clamped[old_idx+2];
        }
    }
    data
}

