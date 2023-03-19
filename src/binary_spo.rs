use crate::image::Image;

// pass two images with equal sizes  -->  output img highlighting diff
pub fn img_dif(img_src: &mut Image, img_dest: &mut Image) {
    for x in 0i32..img_src.width {
        for y in 0i32..img_src.height {
            let (r1, g1, b1) = img_src.get_pixel_intensity(x,y);
            let (mut r, mut g, mut b) = img_dest.get_pixel_intensity(x,y);
            r = r1 - r;
            g = g1 - g;
            b = b1 - b;
            img_dest.set_pixel_intensity(x,y, (r,g,b)); 
        }
    }
}

