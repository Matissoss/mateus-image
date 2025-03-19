pub struct MedianFilter(pub u16);

use image;

use crate::{
    filters::{
        ChangeImage,
        change_color,
        quicksort
    },
    color::Color
};

impl ChangeImage for MedianFilter{
    fn convert_image(&self,img: &mut image::ImageBuffer<image::Rgb<u8>,Vec<u8>>, cl_scheme: &[Color]){
        let cloned_img = img.clone();
        for (x,y,pixel) in img.enumerate_pixels_mut(){
            let depth : u32 = self.0.into();
            let mut pxs : Vec<Color> = Vec::new();
            for y1 in y-depth..y+depth{
                for x1 in x-depth..x+depth{
                    if let Some(px) = cloned_img.get_pixel_checked(x1,y1){
                        pxs.push(Color::from_arr(&px.0));
                    }
                }
            }
            let sorted = quicksort(&pxs);
            if let Some(px) = sorted.get(sorted.len()/2){
                *pixel = image::Rgb(change_color(*px, &cl_scheme, std::cmp::Ordering::Less));
            }
            else{
                *pixel = image::Rgb(change_color(Color::from_arr(&pixel.0), &cl_scheme, std::cmp::Ordering::Less));
            }
        }
    }
}
