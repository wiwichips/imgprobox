use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, ImageData, HtmlCanvasElement};
use wasm_bindgen::{Clamped, JsCast};
use web_sys::console;
use js_sys::{ArrayBuffer, Uint8ClampedArray, Uint8Array, Array, Object};

mod image;
use image::Image;

mod padding;
use padding::*;

mod convolution;
use convolution::Kernel;

mod single_pixel_operations;
use single_pixel_operations::*;

mod geometric_spatial_transformations;
use geometric_spatial_transformations::*;

mod neighbourhood_operations;
use neighbourhood_operations::*;

mod histogram;
use histogram::*;

//mod binary_spo;

#[wasm_bindgen]
pub fn draw(
    ctx: &CanvasRenderingContext2d,
    width: u32,
    height: u32,
    options: Array,
    // convolutions
    custom_convolution: Array,
    // single pixel operations
    spo_array_options: Array,
    // geometric spatial transformations
    rotate_theta: f64,
    scale_factor: f64,
    flip_horizontal_option: bool,
    flip_vertical_option: bool,
    crop_data: Array,
    interpolation_method: String,
    // filtering
    salt_ratio: f64,
    pepper_ratio: f64,
    filtering: String,
    neighbourhood_size: u32,
    distance_type: String,
    // padding
    padding_method: String,
    // random
    random_seed: u32,
) -> Result<(), JsValue> {
    // get current image data
    let current_image = ctx.get_image_data(0.0, 0.0, width as f64, height as f64)?;
    let clamped_data = current_image.data();

    // variables for handle arguments
    let spo_options = js_spo_array_to_vec(&spo_array_options);
    let mut do_convolution = false;
    let mut spo_array: Vec<Box<dyn SinglePixelOperation>> = vec![];
    let mut img_out;
    let mut data;
    let mut padding: fn(&Image, i32, i32) -> (u8,u8,u8) = padding_zero;

    // set padding method
    let mut padding = if padding_method == "reflected" {
        padding_reflected
    } else if padding_method == "zero" {
        padding_zero
    } else if padding_method == "circular" {
        padding_circular
    } else {
        padding_zero
    };
    let mut my_image = Image::new_with_padding(clamped_data.to_vec(), width as i32, height as i32, padding);

    // set interpolation method
    let interpolate_fn = if interpolation_method == "nearest" {
        nearest_neighbour_interpolation
    } else if interpolation_method == "bilinear" {
        bilinear_interpolation
    } else {
        nearest_neighbour_interpolation
    };

    // crop image
    if false {
        my_image = crop_helper(&my_image, 100, 100, 400, 149, & ctx);
    }

    // mirror image
    if (flip_horizontal_option) {
        flip_horizontal(&mut my_image);
    }
    if (flip_vertical_option) {
        flip_vertical(&mut my_image);
    }

    // rotate image
    if rotate_theta > 1.0 && rotate_theta < 357.0 {
        my_image = rotate(&my_image, rotate_theta, interpolate_fn);

        if let Some(canvas) = ctx.canvas() {
            canvas.set_width(my_image.width as u32);
            canvas.set_height(my_image.height as u32);
        }
    }

    // scale image
    if scale_factor < 0.995 || scale_factor > 1.005{
        my_image = scale(&mut my_image, scale_factor, interpolate_fn);
        if let Some(canvas) = ctx.canvas() {
            canvas.set_width(my_image.width as u32);
            canvas.set_height(my_image.height as u32);
        }
    }

    // add single pixel operations to list of single pixel operations for computation
    for spo in &spo_options {
        if (spo.op_type == "inverse") {
            spo_array.push(Box::new(single_to_tri(invert)));
        } else if (spo.op_type == "sepia") {
            spo_array.push(Box::new(sepia));
        } else if (spo.op_type == "grayscale") {
            spo_array.push(Box::new(grayscale));
        } else if (spo.op_type == "threshold") {
            spo_array.push(Box::new(single_to_tri(generate_threshold_mapping(spo.a as i32))));
        } else if (spo.op_type == "linear") {
            spo_array.push(Box::new(single_to_tri(generate_linear_mapping(spo.a as f64, spo.b as f64))));
        } else if (spo.op_type == "powerLaw") {
            spo_array.push(Box::new(single_to_tri(generate_power_mapping(spo.a))));
        } else if (spo.op_type == "histogram_equalization") {
            let mut h = Histogram::new(&my_image);
            h.normalize();
            h.cumulative();
            h.equalize(&mut my_image);
        }
    }

    // apply single pixel operations to image
    apply_spo_chain(&mut my_image, spo_array);

    // check if convolution is selected
    for i in 0..options.length() {
        if let Some(option_str) = options.get(i).as_string() {
            if option_str == "convolutionDemo" {
                do_convolution = true;
            }
        }
    }

    // Convolution
    if do_convolution {
        // Convert the JavaScript 2D array into a Rust Vec<Vec<f64>>.
        let kernel_matrix = js_2d_array_to_vec(&custom_convolution);
        let kernel_width = kernel_matrix[0].len() as i32;
        let kernel_height = kernel_matrix.len() as i32;

        // Create a new Kernel using the converted data.
        let kernel = Kernel::new(kernel_matrix, kernel_width, kernel_height);

        // create new image to put result in and convolve
        img_out = Image::new(vec![255; my_image.get_array().len()], my_image.width, my_image.height);
        kernel.convolve(&my_image, &mut img_out);
        my_image = img_out;
    } 

    // Noise
    if (salt_ratio>0.00001) {
        noise(&mut my_image, salt_ratio, random_seed, true);
    }
    console::log_1(&format!("pep ratio: {}", pepper_ratio).into());
    if (pepper_ratio>0.00001) {
        noise(&mut my_image, pepper_ratio, random_seed + 1, false);
    }

    // filtering
    /*
    img_out = Image::new(vec![255; my_image.get_array().len()], my_image.width, my_image.height);
    median_filter(&mut my_image, &mut img_out, 2, false);
    my_image = img_out;
    */

    // paint new image to canvas
    data = my_image.get_array();
    let data = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut data), (my_image.width + 0) as u32, (my_image.height + 0) as u32)?;
    ctx.put_image_data(&data, 0.0, 0.0)
}

pub fn crop_helper(img: &Image, x1: u32, y1: u32, x2: u32, y2: u32, ctx: & CanvasRenderingContext2d) -> Image {
    let new_width = x2 - x1 + 1;
    let new_height = y2 - y1 + 1;
    if let Some(canvas) = ctx.canvas() {
        canvas.set_width(new_width);
        canvas.set_height(new_height);
    } else {
        console::log_1(&format!("Error: Canvas undefined during crop - reverting to original image").into());
        return img.copy();
    }
    let mut img_out = Image::new(vec![255; (new_width*new_height*4) as usize], new_width as i32, new_height as i32);

    for i in 0..new_width {
        for j in 0..new_height {
            img_out.set_pixel_intensity(i as i32, j as i32, img.get_pixel_intensity((x1 + i) as i32, (y1 + j) as i32));
        }
    }
    img_out
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

