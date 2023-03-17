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

// cropping
pub fn crop_image(img_src: &Image, x1: i32, y1: i32, x2: i32, y2: i32) -> Image {
    let width = x2 - x1;
    let height = y2 - y1;
    let mut data = vec![255; (width * height * 4) as usize];
    let mut i = 0;

    for y in y1..y2 {
        for x in x1..x2 {
            let (r,g,b) = img_src.get_pixel_intensity(x, y);
            data[i]     = r;
            data[i + 1] = g;
            data[i + 2] = b;
            i += 4;
        }
    }
    return Image::new(data, width, height);
}


