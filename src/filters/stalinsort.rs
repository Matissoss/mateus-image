pub struct StalinsortFilter(pub u16);

use image;

use crate::{
    filters::{
        ChangeImage,
        ColorFreq,
        change_color,
        quicksort,
        standard::StandardFilter
    },
    color::Color
};

use std::collections::HashMap;

impl ChangeImage for StalinsortFilter{
    fn convert_image(&self,img: &mut image::ImageBuffer<image::Rgb<u8>,Vec<u8>>, cl_scheme: &[Color]){
        StandardFilter::convert_image(&StandardFilter,img,cl_scheme);
        let mut color_hashmap : HashMap<Color, u32> = HashMap::new();
        for pixel in img.pixels(){
            *color_hashmap.entry(Color::from_arr(&pixel.0)).or_insert(1) += 1;
        }

        let mut color_vector : Vec<ColorFreq> = Vec::new();
        
        for (k,v) in color_hashmap{
            color_vector.push(ColorFreq{color:k,quantity:v});
        }
        
        color_vector = quicksort(&color_vector);
        let mut filtered_vector = Vec::new();
        let depth_usize : usize = self.0.into();
        for i in 0..depth_usize.min(color_vector.len()){
            filtered_vector.push(color_vector[i].color);
        }
        for pixel in img.pixels_mut(){
            *pixel = image::Rgb(change_color(Color::from_arr(&pixel.0), &filtered_vector, std::cmp::Ordering::Less));
        }
    }
}
