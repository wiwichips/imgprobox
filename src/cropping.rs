use crate::image::Image;

/// Crop an image and return a new cropped image object
/// Pass in a left top corner point and a right bottom point of the reigion to crop
pub fn crop_image(img_src: &Image, x1: i32, y1: i32, x2: i32, y2: i32) -> Image {
    let width = x2 - x1;
    let height = y2 - y1;
    let mut data = vec![255; (width * height * 4) as usize];
    let mut i = 0;

    // iterate through the region and copy the pixels to the new output image
    for y in y1..y2 {
        for x in x1..x2 {
            let (r,g,b) = img_src.get_pixel_intensity(x, y);
            data[i]     = r;
            data[i + 1] = g;
            data[i + 2] = b;
            i += 4;
        }
    }
    return Image::new(data, width, height);
}
