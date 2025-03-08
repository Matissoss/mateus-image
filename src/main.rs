use image::ImageReader;

use std::{
    env,
    fs
};

#[derive(PartialEq, Eq)]
enum Flag{
    KeyValue(String, String),
    LongFlag(String),
    ShortFlag(String)
}

impl Flag{
    fn from_vec(input: &[String]) -> Vec<Flag>{
        let mut to_return = vec![];
        for flag in input{
            if flag.starts_with("--"){
                if let Some((key,value)) = flag.split_once('='){
                    to_return.push(Flag::KeyValue(key.to_string(), value.to_string()))
                }
                else{
                    to_return.push(Flag::LongFlag(flag.to_string()));
                }
            }
            else if flag.starts_with("-"){
                if let Some((key,value)) = flag.split_once('='){
                    to_return.push(Flag::KeyValue(key.to_string(), value.to_string()))
                }
                else{
                    to_return.push(Flag::ShortFlag(flag.to_string()));
                }
            }
        }
        return to_return;
    }
}

const PALETTE : [(u8,u8,u8); 8] = 
[
    (18, 18, 18),    // black
    (177, 66, 66),   // red
    (216, 124, 74),  // green 
    (228, 154, 68),  // yellow 
    (74, 139, 139),  // blue 
    (167, 167, 167), // magenta
    (180, 180, 180), // cyan
    (213, 213, 213)  // white
];

fn main() {
    let args = Flag::from_vec(&env::args().collect::<Vec<String>>());

    if args.contains(&Flag::LongFlag("--help".to_string())) || args.contains(&Flag::ShortFlag("-h".to_string())){
        println!(
"ashen-image v1.0
Flags:
--help/-h                 : prints this message
--input/-i=[IMAGE_PATH]
--output/-o=[OUTPUT_PATH]

-----
made by matissoss and licensed under MIT License
");
        return;
    }
    let mut in_file_path = None;
    let mut out_file_path = None;
    for arg in args{
        match arg{ 
            Flag::KeyValue(key, value) => {
                match key.as_str() {
                    "--input" | "-i" => {
                        in_file_path = Some(value);
                    }
                    "--output" | "-o" => {
                        out_file_path = Some(value);
                    }
                    _ => continue
                }
            }
            _ => continue
        }
    }
    
    if let Some(image_path) = in_file_path{
        if let Some(outpath) = out_file_path {
            convert_image(&image_path, &outpath);
        }
        else{
            convert_image(&image_path, "output.png");
        }
    }
}

fn change_color(color: [u8;3]) -> [u8; 3] {
    let mut chosen = (0, 0, 0);
    let mut distance: u16 = 255*3; // max rgb distance = 255 * 3
    for color_t in PALETTE {
        let mut tmp_dist: u16 = 0;
        let (r,g,b) = color_t;
        let mut c_u16: u16 = r.abs_diff(color[0]).into();
        tmp_dist += c_u16;
        c_u16 = g.abs_diff(color[1]).into();
        tmp_dist += c_u16;
        c_u16 = b.abs_diff(color[2]).into();
        tmp_dist += c_u16;

        if tmp_dist < distance {
            distance = tmp_dist;
            chosen = (r, g, b);
        }
    }
    return [chosen.0, chosen.1, chosen.2];
}

fn convert_image(path: &str, outpath: &str){
    if let Ok(true) = fs::exists(path){
        if let Ok(img) = ImageReader::open(path){
            match img.decode(){
                Ok(dynimg) => {
                    let mut rgb8_img = dynimg.to_rgb8();
                    for mut pixel in rgb8_img.pixels_mut(){
                        *pixel = image::Rgb(change_color(pixel.0));
                    }
                    match rgb8_img.save(outpath){
                        Ok(_) => {
                            println!("Succesfully converted `{}`. check {} for results", path, outpath);
                        }
                        Err(_) => {
                            println!("Error occured while saving to {}", outpath);
                        }
                    };
                }
                Err(imgerr) => {
                    println!("Error Occured: {}", imgerr);
                }
            }
        };
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn change_color_test(){
        assert_eq!([18, 18, 18], change_color([17, 17, 17]))
    }
}
