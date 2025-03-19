pub struct MeanFilter(pub u16);

use image;
use crate::{
    filters::{
        ChangeImage,
        change_color
    },
    color::Color
};

impl ChangeImage for MeanFilter{
    fn convert_image(&self,img: &mut image::ImageBuffer<image::Rgb<u8>,Vec<u8>>, cl_scheme: &[Color]){
        let cloned_img = img.clone();
        let depth : u32 = self.0.into();
        for (x,y,pixel) in img.enumerate_pixels_mut(){
            let mut pxs = vec![];
            for y1 in y-depth..y+depth{
                for x1 in x-depth..x+depth{
                    if let Some(cl) = cloned_img.get_pixel_checked(x1,y1){
                        pxs.push(Color{red:cl[0],green:cl[1],blue:cl[2]});
                    }
                }
            }
            *pixel = image::Rgb(change_color(Color::avg_rgb(&pxs), &cl_scheme, std::cmp::Ordering::Less));
        }
    }
}
