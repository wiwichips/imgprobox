use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, ImageData};
use wasm_bindgen::Clamped;
//use web_sys::console;
//use js_sys::{ArrayBuffer, Uint8ClampedArray, Uint8Array};

mod image;
use image::Image;

mod helpers;
//use helpers::PaddingFn;

mod convolution;
//use convolution::Kernel;

#[wasm_bindgen]
pub fn draw(ctx: &CanvasRenderingContext2d, width: u32, height: u32) -> Result<(), JsValue> {
    let current_image = ctx.get_image_data(0.0, 0.0, width as f64, height as f64)?;
    let clamped_data = current_image.data();
    let mut my_image = Image::new(clamped_data.to_vec(), width as i32, height as i32);
    let mut data = cool_effect_02(&mut my_image);
    let data = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut data), width, height)?;

/*
    let js: JsValue = width.into();
    console::log_2(&"print\t".into(), &js);
*/


    ctx.put_image_data(&data, 0.1, 0.0)
}

fn cool_effect_02(img: &mut Image) -> &Vec<u8> {
    for y in 0i32..img.m {
        for x in 0i32..img.n {
            let (r, g, b) = img.get_pixel_intensity(x, y);
            img.set_pixel_intensity(x,y, (b,g,r));
        }
    }
    return img.get_array();
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

fn get_pixel(img_data: &Vec<u8>, width: u32, height: u32, x: u32, y: u32, outside: u8) -> Vec<u8>{
    // outside 0: zero padding; 1: circular indexing; 2: reflected indexing
    let mut pixel = vec![255;4];

    // check if the 

    let offset = get_pixel_idx(width, height, x, y);
    for i in 0..4 {
        pixel[(offset + i) as usize] = img_data[(offset + i) as usize];
    }
    return pixel;
}

// image processing functions
fn grayscale(width: u32, height: u32, img: &ImageData) -> Vec<u8> {
    let mut data = Vec::new();
    let clamped = img.data();
    for i in (0..clamped.len()).step_by(4) {
        let mut avg:u32 = 0;
        for j in 0..3 {
            avg += clamped[(i+j) as usize] as u32;
        }
        if avg != 0 {
            avg = avg / 3;
        }
        for j in 0..3 {
            data.push(avg as u8);
        }
        data.push(255 as u8);
    }
    data
}

fn rotate_colours(width: u32, height: u32, img: &ImageData) -> Vec<u8> {
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

fn cool_effect_01(width: u32, height: u32, img: &ImageData) -> Vec<u8> {
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

fn flip_horizontal(width: u32, height: u32, img: &ImageData) -> Vec<u8> {
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

fn flip_vertical(width: u32, height: u32, img: &ImageData) -> Vec<u8> {
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

fn avg_pixel(height: u32, width: u32, img_data: &Vec<u8>) -> Vec<u8> {

    let mut new_pixel = vec![255;4];
    let mut r = 0;
    let mut g = 0;
    let mut b = 0;
    for i in (0..img_data.len()).step_by(4) {
        r += img_data[i] as u32;
        g += img_data[i+1] as u32;
        b += img_data[i+2] as u32;
    
/*
    let js: JsValue = img_data[0].into();
    console::log_2(&"Logging arbitrary values looks like".into(), &js);
*/


    }
    new_pixel[0] = (r/(height*width)) as u8;
    new_pixel[1] = (g/(height*width)) as u8;
//    new_pixel[2] = (b/(height*width)) as u8;
    new_pixel[2] = (b/(height*width)) as u8;
    new_pixel[3] = 255;

    new_pixel
}

// domain of definition is x1..x2 x y1..y2
fn crop_image (width: u32, height: u32, img_data: &Vec<u8>, x1: u32, y1: u32, x2: u32, y2: u32, outside: u8) -> Vec<u8> {
    let mut data = vec![255;((x2 - x1 + 1)*(y2 - y1 + 1)*4) as usize];
    for y in 0..3 {
        for x in 0..3 {
            let idx_img = get_pixel_idx(width, height, x1 + x, y2 + y);
            let idx_crop = get_pixel_idx(3, 3, x, y);
            data[0 as usize] = img_data[(idx_img) as usize];
            data[(0 + 1) as usize] = img_data[(0 + 1) as usize];
            data[(0 + 2) as usize] = img_data[(0 + 2) as usize];
        }
    }
    data
}

fn blur_nearest_neighbours(width: u32, height: u32, img: &ImageData) -> Vec<u8> {
    let mut data = vec![255;img.data().len()];
    let clamped = img.data();
    for y in 0..height {
        for x in 0..width {
            let a_pixel = avg_pixel(3, 3, &crop_image(width, height, &clamped, x - 1, y - 1, x + 1, y + 1, 0));
            let idx = get_pixel_idx(height, width, x, y) as usize;
            data[idx] = a_pixel[0];
            data[idx+1] = a_pixel[1];
            data[idx+2] = a_pixel[2];
            data[idx+3] = 255;
        }
    }
    data
}

fn rotate_90(width: u32, height: u32, img: &ImageData) -> Vec<u8> {
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

