use crate::image::Image;

// padding functions
pub trait Single_Pixel_Operation_Function {
    fn spo(&self, intensity: u8) -> u8;
}

pub fn apply_spo<F>(img: &mut Image, spo: F)
where
    F: Fn(u8) -> u8,
{
    for x in 0i32..img.width {
        for y in 0i32..img.height {
            let (r, g, b) = img.get_pixel_intensity(x,y);
            img.set_pixel_intensity(x,y, (spo(r), spo(g), spo(b))); 
        }
    }
}

pub fn generate_linear_mapping(a: i32, b: i32) -> impl Fn(u8) -> u8 {
    move |intensity: u8| -> u8 {
        (a * intensity as i32 + b) as u8
    }
}

pub fn generate_power_mapping(gamma: f64) -> impl Fn(u8) -> u8 {
    let L = 256.0;
    let y = gamma;
    move |intensity: u8| -> u8 {
        let base: f64 = intensity as f64/(L-1.0);
        ((L - 1.0) * base.powf(y)) as u8
    }
}

// hard coded single pixel operations that may be handy to have
pub fn invert(intensity: u8) -> u8 {
    (-1 * intensity as i32 + 255) as u8
}

