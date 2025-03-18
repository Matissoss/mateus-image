pub struct BinaryFilter;

use image;

use std::collections::HashMap;

use crate::{
    filters::{
        ChangeImage,
        change_color
    },
    color::Color
};

impl ChangeImage for BinaryFilter{
    fn convert_image(&self, img: &mut image::ImageBuffer<image::Rgb<u8>, Vec<u8>>, _: &[Color]){
        let mut pixels_map : HashMap<Color, u32> = HashMap::new();
        for pixel in img.pixels(){
            if let Some(vl) = pixels_map.get(&Color{red:pixel.0[0],green:pixel.0[1],blue:pixel.0[2]}){
                pixels_map.insert(Color{red:pixel.0[0],green:pixel.0[1],blue:pixel.0[2]},vl+1);
            }
            else{
                pixels_map.insert(Color{red:pixel.0[0],green:pixel.0[1],blue:pixel.0[2]},1);
            }
        }
        let mut max_1 = (0,Color{red:0,green:0,blue:0});
        let mut max_2 = Color{red:0,green:0,blue:0};
        for key in pixels_map.keys(){
            if let Some(vl) = pixels_map.get(key){
                if max_1.0 < *vl{
                    max_2 = max_1.1;
                    max_1 = (*vl, *key);
                }
                else {continue;}
            }
        }
        drop(pixels_map);
        for pixel in img.pixels_mut(){
            *pixel = image::Rgb(
                change_color(Color{red:pixel.0[0],green:pixel.0[1],blue:pixel.0[2]}, &[max_1.1, max_2], std::cmp::Ordering::Less));
        }
    }
}
