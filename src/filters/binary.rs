pub struct BinaryFilter;

use image;

use std::collections::BTreeMap;

use crate::{
    filters::{
        ChangeImage,
        change_color,
        quicksort
    },
    cli::GLOBAL_CLI,
    color::Color
};

impl ChangeImage for BinaryFilter{
    fn convert_image(&self, img: &mut image::ImageBuffer<image::Rgb<u8>, Vec<u8>>, cls: &[Color]){
        let cli = &*GLOBAL_CLI;
        let mut pixels_map : BTreeMap<Color, u32> = BTreeMap::new();
        for pixel in img.pixels(){
            *pixels_map.entry(Color::from_arr(&pixel.0)).or_insert(1) += 1;
        }

        let mut max_1 = (0, Color::from_arr(&[0;3]));
        let mut max_2 = Color::from_arr(&[0;3]);
        
        if cls.len() != 2{
            cli.debug(&format!("[binary.rs]: cls.len() == {}", cls.len()));
            for cl in pixels_map.keys(){
                if let Some(vl) = pixels_map.get(cl){
                    if max_1.0 < *vl{
                        max_2 = max_1.1;
                        max_1 = (*vl, *cl);
                    }
                }
            }
        }
        else{
            let cls_sorted = quicksort(&cls.to_vec());
            max_1.1 = cls_sorted[0];
            max_2   = cls_sorted[1];
        }

        cli.debug(&format!("[binary.rs]: max_1 = ({}, {}, {}); max_2 = ({}, {}, {})",
            max_1.1.red,max_1.1.green,max_1.1.blue,max_2.red,max_2.green,max_2.blue));
        for pixel in img.pixels_mut(){
            *pixel = image::Rgb(
                change_color(Color::from_arr(&pixel.0), &[max_1.1, max_2], std::cmp::Ordering::Less));
        }
    }
}
