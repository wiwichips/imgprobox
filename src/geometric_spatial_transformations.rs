use crate::image::Image;

type InterpolationFn = fn(img: &Image, x: f64, y: f64) -> (u8, u8, u8);

// interpolation functions
/// Nearest Neighbour Interpolation Function
pub fn nearest_neighbour_interpolation(img: &Image, x: f64, y: f64) -> (u8, u8, u8) {
    let x = x.round() as i32;
    let y = y.round() as i32;

    img.get_pixel_intensity(x, y)
}

/// Bilinear Interpolation Function
pub fn bilinear_interpolation(img: &Image, x: f64, y: f64) -> (u8, u8, u8) {
    let x1 = x.floor() as i32;
    let y1 = y.floor() as i32;
    let x2 = x1 + 1;
    let y2 = y1 + 1;

    // Get the four nearest pixels
    let q11 = img.get_pixel_intensity(x1, y1);
    let q12 = img.get_pixel_intensity(x1, y2);
    let q21 = img.get_pixel_intensity(x2, y1);
    let q22 = img.get_pixel_intensity(x2, y2);

    // Perform linear interpolation in the x direction
    let r1 = (
        ((x2 as f64 - x) * q11.0 as f64 + (x - x1 as f64) * q21.0 as f64) / (x2 as f64 - x1 as f64),
        ((x2 as f64 - x) * q11.1 as f64 + (x - x1 as f64) * q21.1 as f64) / (x2 as f64 - x1 as f64),
        ((x2 as f64 - x) * q11.2 as f64 + (x - x1 as f64) * q21.2 as f64) / (x2 as f64 - x1 as f64),
    );

    let r2 = (
        ((x2 as f64 - x) * q12.0 as f64 + (x - x1 as f64) * q22.0 as f64) / (x2 as f64 - x1 as f64),
        ((x2 as f64 - x) * q12.1 as f64 + (x - x1 as f64) * q22.1 as f64) / (x2 as f64 - x1 as f64),
        ((x2 as f64 - x) * q12.2 as f64 + (x - x1 as f64) * q22.2 as f64) / (x2 as f64 - x1 as f64),
    );

    // Perform linear interpolation in the y direction
    let result = (
        ((y2 as f64 - y) * r1.0 + (y - y1 as f64) * r2.0) / (y2 as f64 - y1 as f64),
        ((y2 as f64 - y) * r1.1 + (y - y1 as f64) * r2.1) / (y2 as f64 - y1 as f64),
        ((y2 as f64 - y) * r1.2 + (y - y1 as f64) * r2.2) / (y2 as f64 - y1 as f64),
    );

    return (result.0.round() as u8, result.1.round() as u8, result.2.round() as u8);
}

/// Horizontal flip in place
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

/// Vertical flip in place
pub fn flip_vertical(img: &mut Image) {
    for y in 0..img.height/2 {
        for x in 0..img.width {
            let top = img.get_pixel_intensity(x, y);
            let bottom = img.get_pixel_intensity(x, img.height - y - 1);
            img.set_pixel_intensity(x, y, bottom);
            img.set_pixel_intensity(x, img.height - y - 1, top);
        }
    }
}

/// Rotate an image by theta degrees
pub fn rotate(img: &Image, theta: f64, interpolation: InterpolationFn) -> Image {
    let theta_rad = theta.to_radians();
    let sin_theta = theta_rad.sin();
    let cos_theta = theta_rad.cos();

    let (mut x_min, mut x_max, mut y_min, mut y_max) = (0f64, 0f64, 0f64, 0f64);

    // first, find the corners of the rotated image and based off of them 
    // determine the new bounding max and minimum values for the new image.
    // the reason we need to do this is to ensure that the new image canvas
    // is large enough to fit the entire rotated image.
    let corners = [
        (0, 0),
        (img.width, 0),
        (0, img.height),
        (img.width, img.height),
    ];

    for &(x, y) in &corners {
        let x_rot = x as f64 * cos_theta - y as f64 * sin_theta;
        let y_rot = x as f64 * sin_theta + y as f64 * cos_theta;

        x_min = x_min.min(x_rot);
        x_max = x_max.max(x_rot);
        y_min = y_min.min(y_rot);
        y_max = y_max.max(y_rot);
    }

    let new_width = (x_max - x_min).ceil() as i32;
    let new_height = (y_max - y_min).ceil() as i32;

    let mut new_img = Image::new_blank(new_width, new_height);

    let center_x = img.width as f64 / 2.0;
    let center_y = img.height as f64 / 2.0;
    let new_center_x = new_width as f64 / 2.0;
    let new_center_y = new_height as f64 / 2.0;

    let translation_x = new_center_x - center_x;
    let translation_y = new_center_y - center_y;

    // now, we can iterate over the new image and perform the rotation
    for y in 0..new_height {
        for x in 0..new_width {
            // for each pixel in the new image, we need to find the
            // corresponding pixel in the original image. We do this by
            // performing the inverse rotation on the pixel in the new image
            // and then using the inverse translation to find the corresponding
            // pixel in the original image.
            let x = x as f64;
            let y = y as f64;

            let new_x = (x - new_center_x) * cos_theta + (y - new_center_y) * sin_theta + center_x;
            let new_y = -(x - new_center_x) * sin_theta + (y - new_center_y) * cos_theta + center_y;

            let rgb = interpolation(img, new_x, new_y); // use the interpolation type passed to the function
            new_img.set_pixel_intensity(x as i32, y as i32, rgb);
        }
    }

    new_img
}

/// Scale an image based on a scaling factor
pub fn scale(img: &mut Image, scale: f64, interpolation: InterpolationFn) -> Image{
    // generate a new image to hold the scaled image
    let new_width = (img.width as f64 * scale) as i32;
    let new_height = (img.height as f64 * scale) as i32;
    let mut new_img = Image::new_blank(new_width, new_height);

    // iterate over the new image and perform the scaling
    for y in 0..new_height {
        for x in 0..new_width {
            let new_x = (x as f64 / scale) as i32;
            let new_y = (y as f64 / scale) as i32;
            let rgb = interpolation(&img, new_x as f64, new_y as f64); // use the interpolation type passed to the function
            new_img.set_pixel_intensity(x, y, rgb);
        }
    }
    new_img
}
