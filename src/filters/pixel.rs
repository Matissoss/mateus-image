pub struct PixelFilter(pub u16);

use image::*;
use crate::filters::ChangeImage;
use crate::filters::get_pixel;
use crate::filters::get_pixel_mut;
use crate::filters::change_color;
use crate::color::Color;
use crate::PALETTE;

fn avg_rgb(values: &[Color]) -> Color {
    let mut avg = [0u16; 3];

    for (i, val) in values.iter().enumerate() {
        avg[0] += (val.red   as u16).wrapping_sub(avg[0]) / (i as u16 + 1);
        avg[1] += (val.blue  as u16).wrapping_sub(avg[1]) / (i as u16 + 1);
        avg[2] += (val.green as u16).wrapping_sub(avg[2]) / (i as u16 + 1);
    }

    Color{red:avg[0] as u8, green:avg[1] as u8, blue:avg[2] as u8}
}

impl ChangeImage for PixelFilter{
    fn convert_image(&self,img: &mut ImageBuffer<Rgb<u8>,Vec<u8>>){
        let depth_usize : usize = self.0.into();
        let depth_u32 : u32 = self.0.into();
        let cloned_img = img.clone();
        let (w,h) = img.dimensions();

        for y in (0..h).step_by(depth_usize){
            for x in (0..w).step_by(depth_usize){
                let mut pxs = Vec::new();
                for y1 in y-depth_u32..y{
                    for x1 in x-depth_u32..x{
                        if let Some(cl) = get_pixel(&cloned_img, (x1,y1)){
                            pxs.push(Color{red:cl[0],green:cl[1],blue:cl[2]});
                        }
                    }
                }
                let avg = avg_rgb(&pxs);
                for y1 in y-depth_u32..y{
                    for x1 in x-depth_u32..x{
                        if let Some(px) = get_pixel_mut(img, (x1,y1)){
                            *px = image::Rgb(change_color(avg, &PALETTE, std::cmp::Ordering::Less));
                        }
                    }
                }
            }
        }

    }
}
