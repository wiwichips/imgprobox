
use crate::image::Image;

pub trait PaddingFn {
    fn pad(&self, image: &Image, x: i32, y: i32) -> (u8,u8,u8);
}

pub fn padding_zero(image: &Image, x: i32, y: i32) -> (u8,u8,u8) {
    if x < 0 || y < 0 {
        return (0,0,0);
    }
    if x >= image.n || y >= image.m {
        return (0,0,0);
    }
    return image.get_pixel_intensity(x, y);
}

pub fn padding_circular(image: &Image, x: i32, y: i32) -> (u8,u8,u8) {
    // implementation of padding_circular
    return (0,1,2);
}

pub fn padding_reflected(image: &Image, x: i32, y: i32) -> (u8,u8,u8) {
    // implementation of padding_reflected
    return (0,1,2);
}
