use crate::color::Color;
use std::cmp::Ordering;

pub mod standard    ;
pub mod mean        ;
pub mod median      ;
pub mod stalinsort  ;
pub mod pixel       ;
pub mod ascii       ;
pub mod binary      ;
pub mod inversion   ;
pub mod monochrome  ;

pub trait ChangeImage{
    fn convert_image(&self, img: &mut image::ImageBuffer<image::Rgb<u8>, Vec<u8>>, cl_scheme:&[Color]);
}

pub fn quicksort<T>(input: &Vec<T>) -> Vec<T>
where T: PartialEq + std::cmp::PartialOrd + Clone
{
    if input.len() <= 1 {return input.to_vec()}
    let pivot = &input[input.len()-1];
    let mut veca :Vec<T> = vec![];
    let mut vecb :Vec<T> = vec![];
    for num in &input[0..input.len()-1]{
        if pivot < num{
            veca.push(num.clone());
        }
        else{
            vecb.push(num.clone());
        }
    }
    veca = quicksort(&veca);
    vecb = quicksort(&vecb);
    veca.push(pivot.clone());
    for e in vecb{
        veca.push(e);
    }
    return veca;
}

pub fn change_color(color: Color, palette: &[Color], ordering: Ordering) -> [u8; 3] {
    let mut chosen = (0, 0, 0);
    match ordering {
        Ordering::Equal|Ordering::Less => {
            let mut distance: u16 = 255*3; // max rgb distance = 255 * 3
            for color_t in palette {
                let mut tmp_dist: u16 = 0;
                let (r,g,b) = (color_t.red, color_t.green, color_t.blue);
                let mut c_u16: u16 = r.abs_diff(color.red).into();
                tmp_dist += c_u16;
                c_u16 = g.abs_diff(color.blue).into();
                tmp_dist += c_u16;
                c_u16 = b.abs_diff(color.green).into();
                tmp_dist += c_u16;
                if tmp_dist < distance {
                    distance = tmp_dist;
                    chosen = (r, g, b);
                }
            }
        }
        Ordering::Greater => {
            let mut distance: u16 = 255*3;
            for color_t in palette {
                let mut tmp_dist: u16 = 0;
                let (r,g,b) = (color_t.red, color_t.green, color_t.blue);
                let mut c_u16: u16 = r.abs_diff(color.red).into();
                tmp_dist += c_u16;
                c_u16 = g.abs_diff(color.blue).into();
                tmp_dist += c_u16;
                c_u16 = b.abs_diff(color.green).into();
                tmp_dist += c_u16;
                if tmp_dist > distance {
                    distance = tmp_dist;
                    chosen = (r, g, b);
                }
            }
        }
    }
    return [chosen.0, chosen.1, chosen.2];
}

#[derive(PartialEq, Clone, Debug)]
struct ColorFreq{
    quantity: u32,
    color: Color
}

impl From<(u32, Color)> for ColorFreq{
    fn from(value: (u32, Color)) -> Self{
        return ColorFreq{quantity:value.0, color: value.1};
    }
}
impl std::cmp::PartialOrd for ColorFreq{
    fn partial_cmp(&self, value: &ColorFreq) -> Option<std::cmp::Ordering>{
        if self.quantity < value.quantity{
            return Some(std::cmp::Ordering::Less);
        }
        else if self.quantity == value.quantity{
            return Some(std::cmp::Ordering::Equal);
        }
        else{
            return Some(std::cmp::Ordering::Greater);
        }
    }
}
