
pub fn hello_world() -> u32 {
    return 1;
}

// Image Struct
pub struct Image {
    array:  Vec<u8>,// canvas data (1d)
    m:      u32,    // rows / height
    n:      u32,    // cols / width
}

impl Image {
    pub fn new(data: Vec<u8>, m: u32, n: u32) -> Self {
        Image { array: data, m, n }
    }

    pub fn get_pixel_intensity(&self, x: u32, y: u32) -> (u8,u8,u8) {
        let index = self.get_pixel_index(x,y);
        return (self.array[index], self.array[index + 1], self.array[index + 2])
    }

    pub fn set_pixel_intensity(&mut self, x: u32, y: u32, rgb: (u8,u8,u8)) {
        let index = self.get_pixel_index(x,y);
        self.array[index]     = rgb.0;
        self.array[index + 1] = rgb.1;
        self.array[index + 2] = rgb.2;
    }

    pub fn get_pixel_index(&self, x: u32, y: u32) -> usize {
        ((self.n * y + x)) as usize
    }
}
