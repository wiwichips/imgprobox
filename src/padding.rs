use crate::image::Image;

// padding functions
pub trait PaddingFn {
    fn pad(&self, image: &Image, x: i32, y: i32) -> (u8,u8,u8);
}

pub fn padding_zero(image: &Image, x: i32, y: i32) -> (u8,u8,u8) {
    if x < 0 || y < 0 {
        return (0,0,0);
    }
    if x >= image.width || y >= image.height {
        return (0,0,0);
    }
    return image.get_pixel_intensity(x, y);
}

pub fn padding_circular(image: &Image, x: i32, y: i32) -> (u8,u8,u8) {
    return image.get_pixel_intensity(x % image.width, y % image.height);
}

pub fn padding_reflected(image: &Image, x: i32, y: i32) -> (u8,u8,u8) {
    let x_reflected = if x < 0 {
        -x - 1
    } else if x >= image.width {
        2 * image.width - x - 1
    } else {
        x
    };

    let y_reflected = if y < 0 {
        -y - 1
    } else if y >= image.height {
        2 * image.height - y - 1
    } else {
        y
    };

    return image.get_pixel_intensity(x_reflected, y_reflected);
}

