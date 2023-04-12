use crate::image::Image;

pub struct Histogram {
    channel_red: Vec<f64>,
    channel_green: Vec<f64>,
    channel_blue: Vec<f64>,
    num_pixels: u32,
}

impl Histogram {
    pub fn new(img: &Image) -> Histogram {
        let mut channel_red = vec![0.0; 256];
        let mut channel_green = vec![0.0; 256];
        let mut channel_blue = vec![0.0; 256];

        // add the frequency of light intensity for each channel to the histogram 
        for i in 0..img.width {
            for j in 0..img.height {
                let (r, g, b) = img.get_pixel_intensity_no_padding(i, j);
                channel_red[r as usize] += 1.0;
                channel_green[g as usize] += 1.0;
                channel_blue[b as usize] += 1.0;
            }
        }

        Histogram {
            channel_red, channel_green, channel_blue, num_pixels: (img.width * img.height) as u32
        }
    }

    pub fn normalize(&mut self) {
        // normalize the histogram by dividing each frequency by the total number of pixels
        for i in 0..256 {
            self.channel_red[i] = self.channel_red[i] / self.num_pixels as f64;
            self.channel_green[i] = self.channel_green[i] / self.num_pixels as f64;
            self.channel_blue[i] = self.channel_blue[i] / self.num_pixels as f64;
        }
    }

    pub fn cumulative(&mut self) {
        // calculate the cumulative distribution function for each channel
        for i in 1..256 {
            self.channel_red[i] += self.channel_red[i-1];
            self.channel_green[i] += self.channel_green[i-1];
            self.channel_blue[i] += self.channel_blue[i-1];
        }
    }

    pub fn equalize(&mut self, img: &mut Image) {
        // equalize the image by mapping the intensity of each pixel to the corresponding value in the CDF
        for i in 0..img.width {
            for j in 0..img.height {
                let (r, g, b) = img.get_pixel_intensity_no_padding(i, j);
                img.set_pixel_intensity(i, j, ((self.channel_red[r as usize] * 255.0) as u8, (self.channel_green[g as usize] * 255.0) as u8, (self.channel_blue[b as usize] * 255.0) as u8));
            }
        }
    }
}

