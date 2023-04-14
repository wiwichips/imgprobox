use crate::image::Image;

/// Min filtering function
pub fn min_filter(img: &Image, img_out: &mut Image, distance: i32, chessboard: bool) {
    for i in 0..img.width {
        for j in 0..img.height {
            // generate a new neighbourhood for each pixel and get the min value
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

/// Median filtering function
pub fn median_filter(img: &Image, img_out: &mut Image, distance: i32, chessboard: bool) {
    for i in 0..img.width {
        for j in 0..img.height {
            // generate a new neighbourhood for each pixel and get the median value
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

/// Max filtering function
pub fn max_filter(img: &Image, img_out: &mut Image, distance: i32, chessboard: bool) {
    for i in 0..img.width {
        for j in 0..img.height {
            // generate a new neighbourhood for each pixel and get the max value
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

// neighbourhood operations
/// Neighbourhood struct
pub struct NeighbourHood<'a> {
    pub elements: Vec<(u8, u8, u8)>, // elements in the neighbourhood
    img: &'a Image, // the image data associated with the neighbourhood
    x: i32, // x coordinate of the centre of the neighbourhood
    y: i32,  // y coordinate of the centre of the neighbourhood
    min: usize, // index of the minimum element in the neighbourhood
    median: usize, // index of the median element in the neighbourhood
    max: usize, // index of the maximum element in the neighbourhood
    distance: i32, // distance from the centre of the neighbourhood
}

// Neighbourhood methods
impl<'a> NeighbourHood<'a>{
    /// Constructor
    pub fn new(img: &'a Image, x: i32, y: i32, distance: i32) -> Self {
        NeighbourHood { elements: Vec::new(), img: img, min: 0, median: 0, max: 0, x, y, distance}
    }

    /// Find the city-block distance neighbours of the centre pixel
    pub fn find_d4_neighbours(&mut self) {
        // this for loop will iterate therough only each pixel in the nighbourhood
        // by traversing a diamond shaped pattern.
        // it iterates through each row of the neighbourhood and then only iterates
        // through the pixels in that row that are within the diamond shape.
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

    /// Find the chessboard distance neighbours of the centre pixel
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

    /// Get the max pixel value in the neighbourhood
    pub fn get_max(&self) -> (u8, u8, u8) {
        return self.elements[self.max];
    }

    /// Get the min pixel value in the neighbourhood
    pub fn get_min(&self) -> (u8, u8, u8) {
        return self.elements[self.min];
    }

    /// Get the median pixel value in the neighbourhood
    /// NOTE: this function could be made much more efficient if it
    /// were to find the median using "selection sort"
    /// https://en.wikipedia.org/wiki/Selection_sort
    pub fn get_median(&mut self) -> (u8, u8, u8) {
        // first sort the elements then find the median
        self.elements.sort();
        return self.elements[self.elements.len() / 2];
    }

}
