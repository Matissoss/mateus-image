pub struct StalinsortFilter(pub u16);

use image::*;
use crate::color::Color;
use crate::filters::change_color;
use crate::filters::ColorFreq;
use crate::filters::ChangeImage;
use crate::filters::quicksort;
use crate::filters::standard::StandardFilter;
use std::collections::HashMap;

impl ChangeImage for StalinsortFilter{
    fn convert_image(&self,img: &mut ImageBuffer<Rgb<u8>,Vec<u8>>, cl_scheme: &[Color]){
        StandardFilter::convert_image(&StandardFilter,img,cl_scheme);
        let mut color_hashmap : HashMap<Color, u32> = HashMap::new();
        for pixel in img.pixels(){
            let color = Color{red:pixel.0[0],green:pixel.0[1],blue:pixel.0[2]};
            if let Some(cl) = color_hashmap.get(&color){
                color_hashmap.insert(color, cl+1);
            }
            else{
                color_hashmap.insert(color, 1);
            }
        }
        let mut color_vector : Vec<ColorFreq> = Vec::new();
        for k in color_hashmap.keys(){
            color_vector.push(ColorFreq{color:k.clone(),quantity:*color_hashmap.get(k).unwrap()});
        }
        drop(color_hashmap);
        color_vector = quicksort(&color_vector);
        let mut filtered_vector = Vec::new();
        let depth_usize : usize = self.0.into();
        for i in 0..depth_usize.min(color_vector.len()){
            filtered_vector.push(color_vector[i].color.clone());
        }
        drop(color_vector);
        for pixel in img.pixels_mut(){
            *pixel = image::Rgb(change_color(Color{red:pixel.0[0],green:pixel.0[1],blue:pixel.0[2]}, &filtered_vector, 
            std::cmp::Ordering::Less));
        }
    }
}
