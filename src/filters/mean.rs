pub struct MeanFilter(pub u16);

use image;
use crate::{
    filters::ChangeImage,
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
                        pxs.push(Color::from_arr(&cl.0));
                    }
                }
            }
            *pixel = image::Rgb(Color::avg_rgb(&pxs).change_color(&cl_scheme).to_arr());
        }
    }
}
