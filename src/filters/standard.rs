pub struct StandardFilter;

use image::*;
use crate::filters::ChangeImage;
use crate::color::Color;

impl ChangeImage for StandardFilter{
    fn convert_image(&self,img: &mut ImageBuffer<Rgb<u8>,Vec<u8>>){
        for pixel in img.pixels_mut(){
            *pixel = image::Rgb
            (
                    crate::filters::change_color
                    (
                        Color{red:pixel.0[0],green:pixel.0[1],blue:pixel.0[2]},
                        &crate::PALETTE, 
                        std::cmp::Ordering::Less
                    )
            );
        }
    }
}
