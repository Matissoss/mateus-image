pub struct StandardFilter;

use image::*;
use crate::color::Color;
use crate::filters::ChangeImage;

impl ChangeImage for StandardFilter{
    fn convert_image(&self,img: &mut ImageBuffer<Rgb<u8>,Vec<u8>>, cl_scheme : &[Color]){
        for pixel in img.pixels_mut(){
            *pixel = image::Rgb(Color::from_arr(&pixel.0).change_color(&cl_scheme).to_arr());
        }
    }
}
