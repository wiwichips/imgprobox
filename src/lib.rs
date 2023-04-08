use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, ImageData};
use wasm_bindgen::{Clamped, JsCast};
use web_sys::console;
use js_sys::{ArrayBuffer, Uint8ClampedArray, Uint8Array, Array, Object};

mod image;
use image::Image;

mod padding;
//use helpers::PaddingFn;

mod convolution;
use convolution::Kernel;

mod single_pixel_operations;
use single_pixel_operations::*;

//mod binary_spo;

#[wasm_bindgen]
pub fn draw(
    ctx: &CanvasRenderingContext2d,
    width: u32,
    height: u32,
    options: Array,
    custom_convolution: Array,
    spo_array_options: Array
) -> Result<(), JsValue> {
    let current_image = ctx.get_image_data(0.0, 0.0, width as f64, height as f64)?;
    let clamped_data = current_image.data();

    let spo_options = js_spo_array_to_vec(&spo_array_options);
    let mut doConv = false;
    let mut my_image = Image::new(clamped_data.to_vec(), width as i32, height as i32);
    let mut spo_array: Vec<Box<dyn SinglePixelOperation>> = vec![];
    let mut img_out;
    let mut data;

    for spo in &spo_options {
        console::log_1(&format!("{:?}", spo.op_type).into());
        if (spo.op_type == "threshold") {
            spo_array.push(Box::new(single_to_tri(generate_threshold_mapping(spo.a as i32))));
        } else if (spo.op_type == "linear") {
            spo_array.push(Box::new(single_to_tri(generate_linear_mapping(spo.a as i32, spo.b as i32))));
        } else if (spo.op_type == "powerLaw") {
            spo_array.push(Box::new(single_to_tri(generate_power_mapping(spo.a))));
        } else if (spo.op_type == "histogram_equalization") {
            // TODO
            // not implemented yet
        }
    }

    for i in 0..options.length() {
        if let Some(option_str) = options.get(i).as_string() {
            if option_str == "convolutionDemo" {
                doConv = true;
            }
        }
    }

    apply_spo_chain(&mut my_image, spo_array);

    // do convolutions after 
    if doConv {
        // Convert the JavaScript 2D array into a Rust Vec<Vec<f64>>.
        let kernel_matrix = js_2d_array_to_vec(&custom_convolution);
        let kernel_width = kernel_matrix[0].len() as i32;
        let kernel_height = kernel_matrix.len() as i32;

        // Create a new Kernel using the converted data.
        let kernel = Kernel::new(kernel_matrix, kernel_width, kernel_height);

        // create new image to put result in and convolve
        img_out = Image::new(vec![255; my_image.get_array().len()], my_image.width, my_image.height);
        kernel.convolve(&my_image, &mut img_out);
        data = img_out.get_array();
    } else {
        data = my_image.get_array();
    }

    let data = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut data), width, height)?;
    ctx.put_image_data(&data, 0.1, 0.0)
}

// Single Pixel Operation Options object
#[derive(Debug)]
pub struct SinglePixelOption {
    op_type: String,
    a: f64,
    b: f64,
}

impl SinglePixelOption {
    pub fn new(op_type: String, a: f64, b: f64) -> Self {
        SinglePixelOption { op_type, a, b }
    }
}


fn js_spo_array_to_vec(js_array: &Array) -> Vec<SinglePixelOption> {
    let len = js_array.length() as usize;
    let mut vec = Vec::with_capacity(len);
    for i in 0..len {
        let spo_js_object = js_array.get(i as u32).dyn_into::<Object>().unwrap();
        let entries_array = Object::entries(&spo_js_object);
        let op_type = entries_array.get(0).dyn_into::<Array>().unwrap().get(1).as_string().unwrap();
        let a = entries_array.get(1).dyn_into::<Array>().unwrap().get(1).as_f64().unwrap() as f64;
        let b = entries_array.get(2).dyn_into::<Array>().unwrap().get(1).as_f64().unwrap() as f64;
        vec.push(SinglePixelOption::new(op_type, a, b));
    }
    vec
}


fn js_2d_array_to_vec(js_array: &Array) -> Vec<Vec<f64>> {
    let outer_len = js_array.length() as usize;
    let mut vec = Vec::with_capacity(outer_len);
    for i in 0..outer_len {
        let inner_js_array = js_array.get(i as u32).dyn_into::<Array>().unwrap();
        let inner_len = inner_js_array.length() as usize;
        let mut inner_vec = Vec::with_capacity(inner_len);
        for j in 0..inner_len {
            let value = inner_js_array.get(j as u32).as_f64().unwrap();
            inner_vec.push(value);
        }
        vec.push(inner_vec);
    }
    vec
}

fn test_spo_thresh(img: &mut Image) {
    let spo = generate_threshold_mapping(50);
    apply_spo(img, spo);
}

fn test_spo_chain(img: &mut Image) {
    let spo = single_to_tri(generate_linear_mapping(-1, 255));
    let spo_pm = single_to_tri(generate_power_mapping(5.0)); 
    let spo_g = fn_to_opaque(grayscale);

    let mut spo_array: Vec<Box<dyn SinglePixelOperation>> = vec![];
    spo_array.push(Box::new(spo));
    spo_array.push(Box::new(spo_pm));
    spo_array.push(Box::new(spo_g));

    apply_spo_chain(img, spo_array);
}

fn test_gray_scale(img: &mut Image) {
    apply_multi_channel_spo(img, grayscale);
}

fn test_power_law(img: &mut Image) {
    power_law_mapping(img, 0.5);
}


