pub struct MedianFilter(pub u16);

use image::*;

use crate::Color;
use crate::filters::change_color;
use crate::filters::ChangeImage;
use crate::filters::get_pixel;
use crate::filters::quicksort;

impl ChangeImage for MedianFilter{
    fn convert_image(&self,img: &mut ImageBuffer<Rgb<u8>,Vec<u8>>, cl_scheme: &[Color]){
        let cloned_img = img.clone();
        for (x,y,pixel) in img.enumerate_pixels_mut(){
            let depth : u32 = self.0.into();
            let mut pxs : Vec<Color> = Vec::new();
            for y1 in y-depth..y+depth{
                for x1 in x-depth..x+depth{
                    if let Some(px) = get_pixel(&cloned_img, (x1,y1)){
                        pxs.push(Color{red:px[0],blue:px[1],green:px[2]});
                    }
                }
            }
            let sorted = quicksort(&pxs);
            if let Some(px) = sorted.get(sorted.len()/2){
                *pixel = image::Rgb(change_color(*px, &cl_scheme, std::cmp::Ordering::Less));
            }
            else{
                *pixel = image::Rgb(change_color(Color{red:pixel.0[0],green:pixel.0[1],blue:pixel.0[2]}, &cl_scheme, 
                        std::cmp::Ordering::Less));
            }
        }
    }
}
