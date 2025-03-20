pub struct ASCIIFilter(pub u16);

use image;

use crate::{
    filters::ChangeImage,
    color::Color
};

impl ChangeImage for ASCIIFilter{
    fn convert_image(&self, img: &mut image::ImageBuffer<image::Rgb<u8>, Vec<u8>>, _: &[Color]){
        let param_usize = self.0.into();
        let (w,h) = img.dimensions();

        for y in (0..h).step_by(param_usize){
            print!("\n");
            for x in (0..w).step_by(param_usize){
                if let Some(pixel) = img.get_pixel_checked(x,y){
                    print!("{}",(Color::quantize(&Color::from_arr(&pixel.0), 94) + 33) as char);
                }
            }
        }
    }
}
