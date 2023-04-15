use crate::padding::*;

// Image Struct
#[derive(Clone)]
pub struct Image {
    /// canvas represented as (1d array of pixels)
    array: Vec<u8>,
    /// number of columns
    pub width: i32,
    /// number of rows
    pub height: i32,
    /// function used for padding
    pub pad_fn: fn(&Image, i32, i32) -> (u8,u8,u8),
}

// Image Methods
impl Image {
    // constructors
    // Default constructor
    pub fn new(data: Vec<u8>, width: i32, height: i32) -> Self {
        Image { array: data, width, height, pad_fn: padding_reflected }
    }

    /// Constructor with padding function
    pub fn new_with_padding(data: Vec<u8>, width: i32, height: i32, padding: fn(&Image, i32, i32) -> (u8,u8,u8)) -> Self {
        Image { array: data, width, height, pad_fn: padding }
    }

    /// Blank image constructor
    pub fn new_blank(width: i32, height: i32) -> Self {
        Image { array: vec![255; (width * height * 4) as usize], width, height, pad_fn: padding_reflected }
    }

    /// Blank image constructor with padding function
    pub fn new_blank_with_padding(width: i32, height: i32, padding: fn(&Image, i32, i32) -> (u8,u8,u8)) -> Self {
        Image { array: vec![255; (width * height * 4) as usize], width, height, pad_fn: padding }
    }

    /// Set padding function to another type of padding
    pub fn set_padding(&mut self, padding: fn(&Image, i32, i32) -> (u8,u8,u8)) {
        self.pad_fn = padding;
    }

    /// Get light intensity of pixel at (x,y)
    pub fn get_pixel_intensity(&self, x: i32, y: i32) -> (u8,u8,u8) {
        (self.pad_fn)(&self, x, y)
    }

    /// Get light intensity of pixel at (x,y) without calling padding function
    pub fn get_pixel_intensity_no_padding(&self, x: i32, y: i32) -> (u8,u8,u8) {
        let index = self.get_pixel_index(x,y); 
        (self.array[index], self.array[index + 1], self.array[index + 2])
    }

    /// Set light intensity of pixel at (x,y)
    pub fn set_pixel_intensity(&mut self, x: i32, y: i32, rgb: (u8,u8,u8)) {
        let index = self.get_pixel_index(x,y);
        self.array[index]     = rgb.0;
        self.array[index + 1] = rgb.1;
        self.array[index + 2] = rgb.2;
    }

    /// Get the 1d index of pixel at (x,y)
    pub fn get_pixel_index(&self, x: i32, y: i32) -> usize {
        ((self.width * y + x) * 4) as usize
    }

    /// Get the 1d array of pixel data
    pub fn get_array(&self) -> &Vec<u8> {
        &self.array
    }

    /// Make a deep copy of the image
    /// TODO: use https://doc.rust-lang.org/std/clone/trait.Clone.html
    /// and https://doc.rust-lang.org/std/ops/trait.Index.html to simplify
    /// copy code.
    pub fn copy(&self) -> Image {
        self.clone()
    }
}
