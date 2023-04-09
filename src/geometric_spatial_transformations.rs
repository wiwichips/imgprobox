use crate::image::Image;

// horizontal flip in place
pub fn flip_horizontal(img: &mut Image) {
    for y in 0..img.height {
        for x in 0..img.width/2 {
            let left = img.get_pixel_intensity(x, y);
            let right = img.get_pixel_intensity(img.width - x - 1, y);
            img.set_pixel_intensity(x, y, right);
            img.set_pixel_intensity(img.width - x - 1, y, left);
        }
    }
}

// vertical flip in place
// TODO broken only for video webcam 
pub fn flip_vertical(img: &mut Image) {
    for y in 0..img.height/2 {
        for x in 0..img.width {
            let top = img.get_pixel_intensity(x, y);
            let bottom = img.get_pixel_intensity(x, img.height - y - 1);
            img.set_pixel_intensity(x, y, bottom);
            img.set_pixel_intensity(x, img.height - y - 1, top);
            if x % 100 == 0 {
                img.set_pixel_intensity(x, y, (255,0,0));
                img.set_pixel_intensity(x, img.height - y - 1, (0,255,0));
            }
        }
    }
}

// rotate by theta degrees
pub fn rotate(img: &mut Image, theta: f64) {
    let mut new_img = img.copy();
    let theta = theta.to_radians();
    let cos_theta = theta.cos();
    let sin_theta = theta.sin();
    let center_x = img.width as f64 / 2.0;
    let center_y = img.height as f64 / 2.0;
    for y in 0..img.height {
        for x in 0..img.width {
            let x = x as f64;
            let y = y as f64;
            let new_x = (x - center_x) * cos_theta - (y - center_y) * sin_theta + center_x;
            let new_y = (x - center_x) * sin_theta + (y - center_y) * cos_theta + center_y;
            let new_x = new_x as i32;
            let new_y = new_y as i32;
            if new_x >= 0 && new_x < img.width && new_y >= 0 && new_y < img.height {
                let rgb = new_img.get_pixel_intensity(new_x, new_y);
                img.set_pixel_intensity(x as i32, y as i32, rgb);
            }
        }
    }
}

// scale by nearest neighbor
pub fn scale_nearest_neighbor(img: &mut Image, scale: f64) {
    let mut new_img = img.copy();
    let new_width = (img.width as f64 * scale) as i32;
    let new_height = (img.height as f64 * scale) as i32;
    for y in 0..new_height {
        for x in 0..new_width {
            let new_x = (x as f64 / scale) as i32;
            let new_y = (y as f64 / scale) as i32;
            let rgb = new_img.get_pixel_intensity(new_x, new_y);
            img.set_pixel_intensity(x, y, rgb);
        }
    }
}

// scale by bilinear interpolation
pub fn scale_bilinear(img: &mut Image, scale: f64) {
    let mut new_img = img.copy();
    let new_width = (img.width as f64 * scale) as i32;
    let new_height = (img.height as f64 * scale) as i32;
    img.width = new_width;
    img.height = new_height;
    for y in 0..new_height {
        for x in 0..new_width {
            let new_x = (x as f64 / scale) as i32;
            let new_y = (y as f64 / scale) as i32;
            let rgb = new_img.get_pixel_intensity(new_x, new_y);
            img.set_pixel_intensity(x, y, rgb);
        }
    }
}
