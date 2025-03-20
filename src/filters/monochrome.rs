pub struct MonochromeFilter;

use std::collections::HashMap;

use crate::{
    filters::ChangeImage,
    cli::GLOBAL_CLI,
    color::Color
};

impl ChangeImage for MonochromeFilter{
    fn convert_image(&self,img:&mut image::ImageBuffer<image::Rgb<u8>,Vec<u8>>,cl: &[Color]){
        let cli = &*GLOBAL_CLI;
        let mut px_map: HashMap<Color, u32> = HashMap::new();
        for pixel in img.pixels(){
            let color = Color::from_arr(&pixel.0);
            if let Some(vl) = px_map.get(&color){
                px_map.insert(color, vl+1);
            }
            else{
                px_map.insert(color, 1);
            }
        }
        let mut max = (0, Color{r:0,g:0,b:0});
        for key in px_map.keys(){
            if let Some(vl) = px_map.get(key){
                if max.0 < *vl{
                    max = (*vl, *key);
                }
            }
        }
        max.1 = Color::from_arr(&max.1.change_color(&cl));
        drop(px_map);
        let maxlum = Color::lumination(&max.1);
        cli.debug(&format!("[monochrome.rs]: max = ({}, {}, {}), lumination: {}",max.1.r,max.1.g,max.1.b,maxlum));
        for pixel in img.pixels_mut(){
            let tmp_lum = maxlum / Color::lumination(&Color::from_arr(&pixel.0));
            *pixel = image::Rgb([pixel.0[0] * tmp_lum, pixel.0[1] * tmp_lum, pixel.0[2] * tmp_lum]);
        }
    }
}
