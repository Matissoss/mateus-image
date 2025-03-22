pub struct PixelFilter(pub u16);

use image;

use crate::{
    filters::ChangeImage,
    color::Color,
};

impl ChangeImage for PixelFilter{
    fn convert_image(&self,img: &mut image::ImageBuffer<image::Rgb<u8>,Vec<u8>>, cl_scheme: &[Color]){
        let depth_usize : usize = self.0.into();
        let depth_u32 : u32 = self.0.into();
        let (w,h) = img.dimensions();

        // i do not know why this even works, but it does :)
        for x in (0..=w+depth_u32).step_by(depth_usize){
            for y in (0..=h+depth_u32).step_by(depth_usize){
                let mut pxs = Vec::new();
                for x1 in x-depth_u32..x{
                    for y1 in y-depth_u32..y{
                        if let Some(cl) = img.get_pixel_checked(x1,y1){
                            pxs.push(Color::from_arr(&cl.0));
                        }
                    }
                }
                let avg = Color::avg_rgb(&pxs);
                let pxcolor = image::Rgb(avg.change_color(&cl_scheme).to_arr());
                for y1 in y-depth_u32..y{
                    for x1 in x-depth_u32..x{
                        if let Some(px) = img.get_pixel_mut_checked(x1,y1){
                            *px = pxcolor;
                        }
                    }
                }
            }
        }
    }
}
