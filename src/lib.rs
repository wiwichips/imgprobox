use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, ImageData};
use wasm_bindgen::Clamped;
//use web_sys::console;
//use js_sys::{ArrayBuffer, Uint8ClampedArray, Uint8Array};

mod image;
use image::Image;

mod padding;
//use helpers::PaddingFn;

mod convolution;
use convolution::Kernel;

mod single_pixel_operations;
use single_pixel_operations::{apply_spo, apply_multi_channel_spo, invert, grayscale, linear_mapping, power_law_mapping, apply_spo_chain, single_to_tri, generate_linear_mapping, fn_to_opaque, generate_power_mapping, SinglePixelOperation};

#[wasm_bindgen]
pub fn draw(ctx: &CanvasRenderingContext2d, width: u32, height: u32) -> Result<(), JsValue> {
    let current_image = ctx.get_image_data(0.0, 0.0, width as f64, height as f64)?;
    let clamped_data = current_image.data();

    let mut my_image = Image::new(clamped_data.to_vec(), width as i32, height as i32);
    //let mut img_out = Image::new(vec![255; my_image.get_array().len()], my_image.height, my_image.width);

    //let mut data = cool_effect_02(&mut my_image);
    //convo_test_02(&mut my_image, &mut img_out);
    test_spo_chain(&mut my_image);
    let mut data = my_image.get_array();

    let data = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut data), width, height)?;

/*
    let js: JsValue = width.into();
    console::log_2(&"print\t".into(), &js);
*/


    ctx.put_image_data(&data, 0.1, 0.0)
}

fn test_spo_chain(img: &mut Image) {
    let spo = single_to_tri(generate_linear_mapping(-1, 255));
    let spo_pm = single_to_tri(generate_power_mapping(5.0)); 
    let spo_g = fn_to_opaque(grayscale);

    //let spo_arr = vec![&spo, &spo, &spo, &spo, &spo];
    let spo_arr: Vec<Box<dyn SinglePixelOperation>> = vec![Box::new(spo), Box::new(spo_pm), Box::new(spo_g)];

    apply_spo_chain(img, spo_arr);
}

fn test_gray_scale(img: &mut Image) {
    apply_multi_channel_spo(img, grayscale);
}

fn test_power_law(img: &mut Image) {
    power_law_mapping(img, 0.5);
}

fn test_spo(img: &mut Image) {
    linear_mapping(img, -1, 255);
}

fn convo_test_01(img: &mut Image, img_out: &mut Image) {
    let matrix = vec![
        vec![0.33,0.33,0.33],
        vec![0.33,0.33,0.33],
        vec![0.33,0.33,0.33],
    ]; 
    let h = Kernel::new(matrix, 3,3);
    h.convolve(img, img_out);
}

fn convo_test_02(img: &mut Image, img_out: &mut Image) {
    let matrix = vec![
        vec![1.0],
        vec![0.0],
        vec![0.0],
        vec![0.0],
        vec![-1.0],
    ]; 
    let h = Kernel::new(matrix, 1,5);
    h.convolve(img, img_out);
}

fn convo_test_03(img: &mut Image, img_out: &mut Image) {
    let matrix = vec![
        vec![1.0, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00],
        vec![0.0, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00],
        vec![0.0, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00],
        vec![0.0, 0.00, 0.00, 0.00, 0.00, 0.00, -1.00],
        vec![0.0, 0.00, 0.00, 0.00, 0.00, 0.00, 0.00],
        vec![0.0, 1.00, 0.00, 0.00, 0.00, 0.00, 0.00],
        vec![0.0, 0.00, 0.00, 0.00, 0.00, 0.00, 0.0],
    ]; 
    let h = Kernel::new(matrix, 7, 7);
    h.convolve(img, img_out);
}

fn cool_effect_02(img: &mut Image) -> &Vec<u8> {
    for y in 0i32..img.height {
        for x in 0i32..img.width {
            let (r, g, b) = img.get_pixel_intensity(x, y);
            img.set_pixel_intensity(x,y, (b,g,r));
        }
    }
    return img.get_array();
}

