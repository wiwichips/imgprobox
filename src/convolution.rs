use crate::image::Image;
use crate::helpers::{PaddingFn,padding_zero};

// Kernel Struct
pub struct Kernel {
    array: Vec<Vec<f64>>, // actual 2d kernel
    pub m: i32,           // rows / height
    pub n: i32,           // cols / width
}

impl Kernel {
    pub fn new(data: Vec<Vec<f64>>, m: i32, n: i32) -> Self {
        Kernel { array: data, m, n }
    }

    fn get_element(&self, x: i32, y: i32) -> f64 {
        return self.array[y as usize][x as usize];
    }

    pub fn get_sum_at_index_padding<P: PaddingFn>(&self, x: i32, y: i32, img: &Image, padding_fn: P) -> (u8, u8, u8) {
        (0,1,2)
    }

    pub fn get_sum_at_index(&self, x: i32, y: i32, img: &Image) -> (u8,u8,u8) {
        // use 0 padding by default
        return self.get_sum_at_index_padding(x,y,img, padding_zero);
    }
}
