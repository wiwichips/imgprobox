use crate::image::Image;

pub fn padding_zero(image: &Image, x: i32, y: i32) -> (u8,u8,u8) {
    if x < 0 || y < 0  || x >= image.width || y >= image.height {
        return (0,0,0);
    }
    image.get_pixel_intensity_no_padding(x, y)
}

pub fn padding_circular(image: &Image, x: i32, y: i32) -> (u8,u8,u8) {
    if x >= 0 && y >= 0 && x < image.width && y < image.height {
        return image.get_pixel_intensity_no_padding(x, y);
    }
    image.get_pixel_intensity_no_padding(((x % image.width) + image.width) % image.width , ((y % image.height) + image.height) % image.height)
}

/*
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
*/
