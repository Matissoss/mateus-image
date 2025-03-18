pub struct ASCIIFilter(pub u16);

use image;

use crate::{
    filters::ChangeImage,
    color::Color
};

#[inline(always)]
fn quantize(cl: Color) -> u8{
    ((cl.red as u16 + cl.green as u16 + cl.blue as u16) * 94 / 765) as u8
}

impl ChangeImage for ASCIIFilter{
    fn convert_image(&self, img: &mut image::ImageBuffer<image::Rgb<u8>, Vec<u8>>, _: &[Color]){
        let param_usize = self.0.into();
        let (w,h) = img.dimensions();

        for y in (0..h).step_by(param_usize){
            print!("\n");
            for x in (0..w).step_by(param_usize){
                if let Some(pixel) = img.get_pixel_checked(x,y){
                    print!("{}",(quantize(Color{red:pixel.0[0],green:pixel.0[1],blue:pixel.0[2]}) + 33) as char);
                }
            }
        }
        std::process::exit(0);
    }
}
