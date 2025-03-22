pub struct StalinsortFilter(pub u16);

use image;

use crate::{
    filters::ChangeImage,
    color::Color
};

use std::collections::BTreeMap;

impl ChangeImage for StalinsortFilter{
    fn convert_image(&self,img: &mut image::ImageBuffer<image::Rgb<u8>,Vec<u8>>, cl_scheme: &[Color]){
        let mut color_map : BTreeMap<Color, u32> = BTreeMap::new();
        for pixel in img.pixels(){
            *color_map.entry(Color::from_arr(&pixel.0)).or_insert(1) += 1;
        }
        let mut i = 0;
        let mut filtered_vector : Vec<Color> = Vec::new();
        for c in color_map{
            if i < self.0{
                if cl_scheme.len() != 0{
                    filtered_vector.push(Color::change_color(&c.0, &cl_scheme));
                }
                else{
                    filtered_vector.push(c.0);
                }
                i += 1;
            }
        }

        for pixel in img.pixels_mut(){
            *pixel = image::Rgb(Color::from_arr(&pixel.0).change_color(&filtered_vector).to_arr());
        }
    }
}
