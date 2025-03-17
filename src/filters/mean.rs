pub struct MeanFilter(pub u16);

use image::*;
use crate::filters::ChangeImage;
use crate::filters::get_pixel;
use crate::filters::change_color;
use crate::PALETTE;
use crate::color::Color;

fn avg_rgb(values: &[Color]) -> Color {
    let mut avg = [0u16; 3];

    for (i, val) in values.iter().enumerate() {
        avg[0] += (val.red   as u16).wrapping_sub(avg[0]) / (i as u16 + 1);
        avg[1] += (val.blue  as u16).wrapping_sub(avg[1]) / (i as u16 + 1);
        avg[2] += (val.green as u16).wrapping_sub(avg[2]) / (i as u16 + 1);
    }

    Color{red:avg[0] as u8, green:avg[1] as u8, blue:avg[2] as u8}
}

impl ChangeImage for MeanFilter{
    fn convert_image(&self,img: &mut ImageBuffer<Rgb<u8>,Vec<u8>>){
        let cloned_img = img.clone();
        let depth : u32 = self.0.into();
        for (x,y,pixel) in img.enumerate_pixels_mut(){
            let mut pxs = vec![];
            for y1 in y-depth..y+depth{
                for x1 in x-depth..x+depth{
                    if let Some(cl) = get_pixel(&cloned_img, (x1,y1)){
                        pxs.push(Color{red:cl[0],green:cl[1],blue:cl[2]});
                    }
                }
            }
            *pixel = image::Rgb(change_color(avg_rgb(&pxs), &PALETTE, std::cmp::Ordering::Less));
        }
    }
}
