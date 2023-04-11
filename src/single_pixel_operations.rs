use crate::image::Image;
use web_sys::console;
use js_sys::{ArrayBuffer, Uint8ClampedArray, Uint8Array};
use wasm_bindgen::prelude::*;


// generic single pixel operation -----------------------------------
// Define a new trait for the single pixel operation
pub trait SinglePixelOperation {
    fn apply(&self, r: u8, g: u8, b: u8) -> (u8, u8, u8);
}

// Implement the trait for function pointers with the matching signature
impl<F> SinglePixelOperation for F
where
    F: Fn(u8, u8, u8) -> (u8, u8, u8),
{
    fn apply(&self, r: u8, g: u8, b: u8) -> (u8, u8, u8) {
        (self)(r, g, b)
    }
}

// Modify the apply_spo_chain function to accept Vec<Box<dyn SinglePixelOperation>>
pub fn apply_spo_chain(img: &mut Image, spo_array: Vec<Box<dyn SinglePixelOperation>>) {
    for x in 0i32..img.width {
        for y in 0i32..img.height {
            let (mut r, mut g, mut b) = img.get_pixel_intensity(x, y);
            for spo in spo_array.iter() {
                (r, g, b) = spo.apply(r, g, b);
            }
            img.set_pixel_intensity(x, y, (r, g, b));
        }
    }
}

pub fn apply_multi_channel_spo<F>(img: &mut Image, tri_chan_spo: F)
where
    F: Fn(u8, u8, u8) -> (u8, u8, u8),
{
    for x in 0i32..img.width {
        for y in 0i32..img.height {
            let (r, g, b) = img.get_pixel_intensity(x,y);
            img.set_pixel_intensity(x,y, tri_chan_spo(r,g,b)); 
        }
    }
}

pub fn apply_multi_channel_mutable_spo<F>(img: &mut Image, mut tri_chan_spo: F)
where
    F: FnMut(u8, u8, u8) -> (u8, u8, u8),
{
    for x in 0i32..img.width {
        for y in 0i32..img.height {
            let (r, g, b) = img.get_pixel_intensity(x,y);
            img.set_pixel_intensity(x,y, tri_chan_spo(r,g,b)); 
        }
    }
}

pub fn apply_spo<F>(img: &mut Image, spo: F)
where
    F: Fn(u8) -> u8,
{
    let multi_channel_spo = single_to_tri(spo);
    apply_multi_channel_spo(img, multi_channel_spo);
}

pub fn single_to_tri<F>(spo: F) -> impl Fn(u8, u8, u8) -> (u8, u8, u8)
where 
    F: Fn(u8) -> u8,
{
    move |r: u8, g: u8, b: u8| -> (u8, u8, u8) {
        (spo(r), spo(g), spo(b))
    }
}

pub fn fn_to_opaque<F>(spo: F) -> impl Fn(u8, u8, u8) -> (u8, u8, u8)
where 
    F: Fn(u8, u8, u8) -> (u8, u8, u8),
{
    move |r: u8, g: u8, b: u8| -> (u8, u8, u8) {
        spo(r,g,b)
    }
}

// linear mappings --------------------------------------------------
pub fn generate_linear_mapping(a: i32, b: i32) -> impl Fn(u8) -> u8 {
    move |intensity: u8| -> u8 {
        (a * intensity as i32 + b) as u8
    }
}

pub fn linear_mapping(img: &mut Image, a: i32, b: i32) {
    apply_spo(img, generate_linear_mapping(-1, 255));
}

// thresholding -----------------------------------------------------
// TODO fix thresholding
pub fn generate_threshold_mapping(u: i32) -> impl Fn(u8) -> u8 {
    generate_linear_mapping(255, (-256) * u)
}

// power law mappings -----------------------------------------------
pub fn generate_power_mapping(gamma: f64) -> impl Fn(u8) -> u8 {
    let l = 256.0;
    let y = gamma;

    // Precompute the lookup table
    //  base.powf is really slow... so calculating it each time for
    //  every pixel in an image takes a long time. Therefore its
    //  useful to precalculate all possible power law mappings for
    //  the inputted gamma into a table that can be accessed.
    let lookup_table: Vec<u8> = (0..256)
        .map(|intensity| {
            let base: f64 = intensity as f64 / (l - 1.0);
            ((l - 1.0) * base.powf(y)) as u8
        })
        .collect();

    move |intensity: u8| -> u8 {
        // Use the lookup table to get the result
        lookup_table[intensity as usize]
    }
}

pub fn power_law_mapping(img: &mut Image, gamma: f64) {
    apply_spo(img, generate_power_mapping(gamma));
}

// noise ------------------------------------------------------------
pub fn generate_noise_function(noise_ratio: f64, seed: u32, noise_value: u8) -> impl FnMut(u8,u8,u8) -> (u8,u8,u8) {
    // In order to avoid calling the JavaScript random number
    // generator for every pixel in the image, we use a simple
    // linear congruential generator to generate a random number
    // for each pixel. This is a lot faster than calling the
    // JavaScript random number generator for every pixel.
    let mut current_seed = seed;
    move |r: u8, g: u8, b: u8| -> (u8, u8, u8) {

        // Update the seed using a simple LCG formula
        current_seed = (current_seed * 1664525 + 1013904223) & 0xFFFFFFFF;

        // Generate a random float between 0 and 1 using the updated seed
        let random_float = (current_seed as f64) / (u32::MAX as f64);

        if random_float > noise_ratio {
            (noise_value, noise_value, noise_value)
        } else {
            (r,g,b)
        }
    }
}

pub fn noise(img: &mut Image, noise_ratio: f64, seed: u32, salt: bool) {
    let noise_value = if salt { 255 } else { 0 };
    let noise_function = generate_noise_function(noise_ratio, seed, noise_value);
    apply_multi_channel_mutable_spo(img, noise_function);
}


// hard coded single pixel operations that may be handy to have -----
pub fn invert(intensity: u8) -> u8 {
    (-1 * intensity as i32 + 255) as u8
}

pub fn grayscale(r: u8, g: u8, b: u8) -> (u8, u8, u8) {
    let sum: f64 = r as f64 + g as f64 + b as f64;
    let avg = sum / 3.0;
    (avg as u8, avg as u8, avg as u8)
}

pub fn sepia(r: u8, g: u8, b: u8) -> (u8, u8, u8) {
    let tr = 0.393 * r as f64 + 0.769 * g as f64 + 0.189 * b as f64;
    let tg = 0.349 * r as f64 + 0.686 * g as f64 + 0.168 * b as f64;
    let tb = 0.272 * r as f64 + 0.534 * g as f64 + 0.131 * b as f64;
    (tr as u8, tg as u8, tb as u8)
}

