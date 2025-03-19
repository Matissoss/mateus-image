pub struct InversionFilter;

use image;
use std::collections::BTreeMap;

use crate::{
    filters::ChangeImage,
    color::Color
};

impl ChangeImage for InversionFilter{
    fn convert_image(&self, img: &mut image::ImageBuffer<image::Rgb<u8>, Vec<u8>>, _ : &[Color]){
        let mut px_map: BTreeMap<Color, u32> = BTreeMap::new();
        let mut max = (0, Color::from_arr(&[0;3]));
        for pixel in img.pixels(){
            *px_map.entry(Color::from_arr(&pixel.0)).or_insert(1) += 1;
        }
        for key in px_map.keys(){
            if let Some(vl) = px_map.get(key){
                if max.0 < *vl{
                    max = (*vl, *key);
                }
            }
        }
        for pixel in img.pixels_mut(){
            if (pixel.0[0] as u16 + pixel.0[1] as u16 + pixel.0[2] as u16)
                < (max.1.red as u16 + max.1.green as u16 + max.1.blue as u16)
            {
                *pixel = image::Rgb([max.1.red-pixel.0[0], max.1.green-pixel.0[1],max.1.blue-pixel.0[2]]);
            }
            else{
                *pixel = image::Rgb([pixel.0[0] - max.1.red, pixel.0[1] - max.1.green, pixel.0[2] - max.1.blue]);
            }
        }
    }
}
