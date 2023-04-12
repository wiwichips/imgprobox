use crate::padding::*;

// Image Struct
pub struct Image {
    array: Vec<u8>,// canvas data (1d)
    pub width: i32,    // cols / width
    pub height: i32,    // rows / height
    pad_fn: fn(&Image, i32, i32) -> (u8,u8,u8),
}

impl Image {
    pub fn new(data: Vec<u8>, width: i32, height: i32) -> Self {
        Image { array: data, width, height, pad_fn: padding_reflected }
    }

    pub fn new_blank(width: i32, height: i32) -> Self {
        Image { array: vec![255; (width * height * 4) as usize], width, height, pad_fn: padding_reflected }
    }

    pub fn set_padding(&mut self, padding: fn(&Image, i32, i32) -> (u8,u8,u8)) {
        self.pad_fn = padding;
    }

    pub fn get_pixel_intensity(&self, x: i32, y: i32) -> (u8,u8,u8) {
        (self.pad_fn)(&self, x, y)
    }

    pub fn get_pixel_intensity_no_padding(&self, x: i32, y: i32) -> (u8,u8,u8) {
        let index = self.get_pixel_index(x,y); 
        (self.array[index], self.array[index + 1], self.array[index + 2])
    }

    pub fn set_pixel_intensity(&mut self, x: i32, y: i32, rgb: (u8,u8,u8)) {
        let index = self.get_pixel_index(x,y);
        self.array[index]     = rgb.0;
        self.array[index + 1] = rgb.1;
        self.array[index + 2] = rgb.2;
    }

    pub fn get_pixel_index(&self, x: i32, y: i32) -> usize {
        ((self.width * y + x) * 4) as usize
    }

    pub fn get_array(&self) -> &Vec<u8> {
        &self.array
    }

    pub fn copy(&self) -> Image {
        let mut img = Image { array: vec![255; self.array.len()], height: self.height, width: self.width, pad_fn: self.pad_fn }; 
        for i in 0usize..self.array.len() {
            img.array[i] = self.array[i];
        }
        return img;
    }
}
