use crate::image::Image;

use web_sys::console;
use js_sys::{ArrayBuffer, Uint8ClampedArray, Uint8Array};
use wasm_bindgen::prelude::*;


// Kernel Struct
pub struct Kernel {
    array: Vec<Vec<f64>>, // actual 2d kernel
    pub width: i32,           // cols / width
    pub height: i32,           // rows / height
}

impl Kernel {
    pub fn new(data: Vec<Vec<f64>>, width: i32, height: i32) -> Self {
        Kernel { array: data, width, height }
    }

    fn get_element(&self, x: i32, y: i32) -> f64 {
        return self.array[y as usize][x as usize];
    }

    pub fn get_sum_at_index_padding(&self, x: i32, y: i32, img: &Image) -> (u8, u8, u8) {
        let mut sum_r: f64 = 0.0;
        let mut sum_g: f64 = 0.0;
        let mut sum_b: f64 = 0.0;

        // initialize the part of the image to look at
        let left: i32 = -1 * (self.width - 1) / 2;
        let top:  i32 = -1 * (self.height - 1) / 2;

        // iterate through each element in the kernel and apply it to the image
        for i in 0i32..self.width {
            for j in 0i32..self.height {
                let (r,g,b) = img.get_pixel_intensity(x + i + left, y + j + top);

                sum_r += r as f64 * self.array[(self.height - j - 1) as usize][(self.width - i - 1) as usize];
                sum_g += g as f64 * self.array[(self.height - j - 1) as usize][(self.width - i - 1) as usize];
                sum_b += b as f64 * self.array[(self.height - j - 1) as usize][(self.width - i - 1) as usize];
            }
        }
        (sum_r.round() as u8, sum_g.round() as u8, sum_b.round() as u8)
    }

    pub fn get_sum_at_index(&self, x: i32, y: i32, img: &Image) -> (u8,u8,u8) {
        self.get_sum_at_index_padding(x,y,img)
    }

    pub fn sum(&self) -> f64 {
        let mut sum: f64 = 0.0;
        for j in 0i32..self.height {
            for i in 0i32..self.width {
                sum += self.array[j as usize][i as usize];
            }
        }
        sum
    }

    pub fn convolve(&self, img: &Image, img_out: &mut Image) {
        // Check if the kernel is separable, if it is, use the separable convolution algorithm
        if let Some((row_kernel, col_kernel)) = self.decompose_rank_one_combined() {
            let img_out_temporary = &mut Image::new_blank(img.width, img.height);
            row_kernel.convolve(img,img_out_temporary);
            col_kernel.convolve(img_out_temporary, img_out);
            return;
        }

        // iterate through each pixel in the image
        for y in 0i32..img.height {
            for x in 0i32..img.width {
                img_out.set_pixel_intensity(x,y,self.get_sum_at_index(x,y,img));
            }
        }
    }

    pub fn decompose_rank_one_combined(&self) -> Option<(Kernel, Kernel)> {
        let mut row_vec: Option<Vec<f64>> = None;

        for i in 0..self.height as usize {
            let scale = self.array[i][0];
            if scale == 0.0 {
                continue;
            }

            let scaled_row: Vec<f64> = self.array[0].iter().map(|v| v * scale).collect();
            if self.array[i] != scaled_row {
                return None;
            }

            if row_vec.is_none() {
                row_vec = Some(scaled_row);
            }
        }

        let row_vec = row_vec?;
        let col_vec: Vec<f64> = self.array.iter().map(|row| row[0]).collect();

        let row_kernel = Kernel::new(vec![row_vec.clone()], self.width, 1);
        let col_kernel = Kernel::new(col_vec.iter().map(|&val| vec![val]).collect(), 1, self.height);

        Some((row_kernel, col_kernel))
    }
}
