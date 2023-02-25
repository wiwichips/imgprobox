use crate::image::Image;
use crate::helpers::padding_zero;

// Kernel Struct
pub struct Kernel {
    array: Vec<Vec<f64>>, // actual 2d kernel
    pub m: i32,           // rows / height
    pub n: i32,           // cols / width
}

impl Kernel {
    pub fn new(data: Vec<Vec<f64>>, m: i32, n: i32) -> Self {
        Kernel { array: data, m, n }
    }

    fn get_element(&self, x: i32, y: i32) -> f64 {
        return self.array[y as usize][x as usize];
    }

    pub fn get_sum_at_index_padding(&self, x: i32, y: i32, img: &Image, pad: fn(&Image, i32, i32) -> (u8,u8,u8) ) -> (u8, u8, u8) {
        let mut sum_r: f64 = 0.0;
        let mut sum_g: f64 = 0.0;
        let mut sum_b: f64 = 0.0;

        // initialize the part of the image to look at
        let left: i32 = -(self.n - 1) / 2;
        let top:  i32 = -(self.m - 1) / 2;

        // iterate through each element in the kernel and apply it to the image
        for j in 0i32..self.n {
            for i in 0i32..self.m {
                let (r,g,b) = pad(&img, x + i - left, y + j - top);
                sum_r += r as f64 * self.array[(self.n - j - 1) as usize][(self.m - i - 1) as usize];
                sum_g += g as f64 * self.array[(self.n - j - 1) as usize][(self.m - i - 1) as usize];
                sum_b += b as f64 * self.array[(self.n - j - 1) as usize][(self.m - i - 1) as usize];
            }
        }

        (sum_r.round() as u8, sum_g.round() as u8, sum_b.round() as u8)
    }

    pub fn get_sum_at_index(&self, x: i32, y: i32, img: &Image) -> (u8,u8,u8) {
        // use 0 padding by default
        return self.get_sum_at_index_padding(x,y,img, padding_zero);
    }

    pub fn sum(&self) -> f64 {
        let mut sum: f64 = 0.0;
        for j in 0i32..self.n {
            for i in 0i32..self.m {
                sum += self.array[j as usize][i as usize];
            }
        }
        sum
    }

    pub fn convolve(&self, img: &Image) -> Image {
        let mut img_out = Image::new(vec![255; self.array.len()], img.m, img.n);

        for y in 0i32..img.n {
            for x in 0i32..self.m {
                img_out.set_pixel_intensity(x,y,self.get_sum_at_index(x,y,img));
            }
        }
        return img_out;
    }
}
