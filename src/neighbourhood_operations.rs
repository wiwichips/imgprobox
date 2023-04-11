use crate::image::Image;

pub fn min_filter(img: &Image, img_out: &mut Image, distance: i32, chessboard: bool) {
    for i in 0..img.width {
        for j in 0..img.height {
            let mut n = NeighbourHood::new(& img, i, j, distance);
            if (chessboard) {
                n.find_d8_neighbours();
            } else {
                n.find_d4_neighbours();
            }

            img_out.set_pixel_intensity(i, j, n.get_min());
        }
    }
}

pub fn median_filter(img: &Image, img_out: &mut Image, distance: i32, chessboard: bool) {
    for i in 0..img.width {
        for j in 0..img.height {
            let mut n = NeighbourHood::new(& img, i, j, distance);
            if (chessboard) {
                n.find_d8_neighbours();
            } else {
                n.find_d4_neighbours();
            }

            img_out.set_pixel_intensity(i, j, n.get_median());
        }
    }
}

pub fn max_filter(img: &Image, img_out: &mut Image, distance: i32, chessboard: bool) {
    for i in 0..img.width {
        for j in 0..img.height {
            let mut n = NeighbourHood::new(& img, i, j, distance);
            if (chessboard) {
                n.find_d8_neighbours();
            } else {
                n.find_d4_neighbours();
            }

            img_out.set_pixel_intensity(i, j, n.get_max());
        }
    }
}

// ------------------------------

pub struct NeighbourHood<'a> {
    pub elements: Vec<(u8, u8, u8)>,
    img: &'a Image,
    x: i32,
    y: i32,
    min: usize,
    median: usize,
    max: usize,
    distance: i32,
}

impl<'a> NeighbourHood<'a>{
    pub fn new(img: &'a Image, x: i32, y: i32, distance: i32) -> Self {
        NeighbourHood { elements: Vec::new(), img: img, min: 0, median: 0, max: 0, x, y, distance}
    }

    // TODO - this can be made much faster if you don't loop over the same elements again...
    // I also don't like the way this is written, it should make more sense

    pub fn find_d4_neighbours(&mut self) {
        for i in -self.distance..=self.distance {
            for j in -(self.distance - i.abs())..=(self.distance - i.abs()) {
                let pixel = self.img.get_pixel_intensity(self.x + j, self.y + i);
                self.elements.push(pixel);
                if (pixel < self.elements[self.min]) {
                    self.min = self.elements.len() - 1;
                } else if (pixel > self.elements[self.max]) {
                    self.max = self.elements.len() - 1;
                }
            }
        }
    }

    pub fn find_d8_neighbours(&mut self) {
        for i in -self.distance..=self.distance {
            for j in -self.distance..=self.distance {
                let pixel = self.img.get_pixel_intensity(self.x + j, self.y + i);
                self.elements.push(pixel);
                if (pixel < self.elements[self.min]) {
                    self.min = self.elements.len() - 1;
                } else if (pixel > self.elements[self.max]) {
                    self.max = self.elements.len() - 1;
                }
            }
        }
    }

    pub fn get_max(&self) -> (u8, u8, u8) {
        return self.elements[self.max];
    }

    pub fn get_min(&self) -> (u8, u8, u8) {
        return self.elements[self.min];
    }

    // TODO - use selection sort to find median O(n) instead of O(nlogn)
    pub fn get_median(&mut self) -> (u8, u8, u8) {
        self.elements.sort();
        return self.elements[self.elements.len() / 2];
    }

}
