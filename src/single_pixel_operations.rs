use crate::image::Image;

// padding functions
pub trait Single_Pixel_Operation_Function {
    fn spo(&self, intensity: u8) -> u8;
}

pub fn apply_spo(img: &mut Image, spo: fn(u8) -> u8) {
    for x in 0i32..img.width {
        for y in 0i32..img.height {
            let (r, g, b) = img.get_pixel_intensity(x,y);
            img.set_pixel_intensity(x,y, (spo(r), spo(g), spo(b))); 
        }
    }
}


pub fn invert(intensity: u8) -> u8 {
    let a: i32 = -1;
    let b: i32 = 255;
    let u: i32 = intensity as i32;
    return (a * u  + b) as u8;
}


