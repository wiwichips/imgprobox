use crate::image::Image;

/// Struct to hold the histogram data
pub struct Histogram {
    channel_red: Vec<f64>, // frequency of light intensity for each channel
    channel_green: Vec<f64>,
    channel_blue: Vec<f64>,
    num_pixels: u32, // number of pixels in the original image (used for normalization)
}

/// Histogram Methods
impl Histogram {
    /// Constructor for histogram - creates a new histogram based on the image
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

    /// Normalize the histogram
    pub fn normalize(&mut self) {
        // normalize the histogram by dividing each frequency by the total number of pixels
        for i in 0..256 {
            self.channel_red[i] = self.channel_red[i] / self.num_pixels as f64;
            self.channel_green[i] = self.channel_green[i] / self.num_pixels as f64;
            self.channel_blue[i] = self.channel_blue[i] / self.num_pixels as f64;
        }
    }

    /// Convert the histogram into a cumulative histogram
    pub fn cumulative(&mut self) {
        // calculate the cumulative distribution function for each channel
        for i in 1..256 {
            self.channel_red[i] += self.channel_red[i-1];
            self.channel_green[i] += self.channel_green[i-1];
            self.channel_blue[i] += self.channel_blue[i-1];
        }
    }

    /// Perform histogram equalization on the image
    /// this will require the historgram to be normalized and cumulative
    /// for best results. 
    pub fn equalize(&mut self, img: &mut Image) {
        // equalize the image by mapping the intensity of each pixel to the corresponding value in the CDF
        for i in 0..img.width {
            for j in 0..img.height {
                // for a given pixel intensity, get the corresponding value from the CDF and multiply by 255 to get the new intensity
                let (r, g, b) = img.get_pixel_intensity_no_padding(i, j);
                img.set_pixel_intensity(i, j, ((self.channel_red[r as usize] * 255.0) as u8, (self.channel_green[g as usize] * 255.0) as u8, (self.channel_blue[b as usize] * 255.0) as u8));
            }
        }
    }
}

