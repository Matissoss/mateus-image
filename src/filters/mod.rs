use crate::color::Color;

pub mod standard    ;
pub mod mean        ;
pub mod median      ;
pub mod stalinsort  ;
pub mod pixel       ;
pub mod ascii       ;
pub mod binary      ;
pub mod inversion   ;
pub mod monochrome  ;

pub fn quicksort<T>(arr: &[T]) -> Vec<T>
where T: PartialEq + PartialOrd + Clone{
    if arr.len() <= 1 {return Vec::new()}
    let pivot = &arr[arr.len()-1];

    let mut less_than : Vec<T> = Vec::new();
    let mut more_than : Vec<T> = Vec::new();

    for n in &arr[0..arr.len()-1usize]{
        if n < pivot{
            less_than.push(n.clone());
        }
        else{
            more_than.push(n.clone());
        }
    }
    quicksort(&mut less_than);
    less_than.push(pivot.clone());
    quicksort(&mut more_than);
    less_than.extend(more_than);
    return less_than;
}

pub trait ChangeImage{
    fn convert_image(&self, img: &mut image::ImageBuffer<image::Rgb<u8>, Vec<u8>>, cl_scheme:&[Color]);
}

