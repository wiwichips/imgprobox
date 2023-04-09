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
