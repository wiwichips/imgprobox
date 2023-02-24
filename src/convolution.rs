
// Kernel Struct
pub struct Kernel {
    array: Vec<Vec<f64>>, // actual 2d kernel
    pub m: u32,           // rows / height
    pub n: u32,           // cols / width
}

impl Kernel {
    pub fn new(data: Vec<Vec<f64>>, m: u32, n: u32) -> Self {
        Kernel { array: data, m, n }
    }

    fn get_element(&self, x: u32, y: u32) -> f64 {
        return self.array[y as usize][x as usize];
    }

/*
    fn get_sum_at_index(&self, x: u32, y: u32) -> u8 {
        
    }
*/

}
