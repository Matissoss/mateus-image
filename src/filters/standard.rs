pub struct StandardFilter;

use image::*;
use crate::color::Color;
use crate::filters::{
    ChangeImage,
    change_color,
};

impl ChangeImage for StandardFilter{
    fn convert_image(&self,img: &mut ImageBuffer<Rgb<u8>,Vec<u8>>, cl_scheme : &[Color]){
        for pixel in img.pixels_mut(){
            *pixel = image::Rgb(change_color(Color{red:pixel.0[0],green:pixel.0[1],blue:pixel.0[2]},
            &cl_scheme, std::cmp::Ordering::Less));
        }
    }
}
