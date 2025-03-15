mod color;
mod filters;

use color::Color;
use filters::*;
use stalinsort as stalin;

use image;
use std::{
    env,
    fs
};


#[derive(PartialEq, Eq, Debug)]
enum ImageFilter{
    Median(u16),
    Standard,
    StalinSort(u16),
    Pixel(u16),
    Mean(u16)
}

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


pub const PALETTE : [Color; 11] = 
[
    // bg 
    Color::ctime_hex("121212"),
    Color::ctime_hex("242424"),
    // fg
    Color::ctime_hex("C8C8CC"),
    Color::ctime_hex("969699"),
    // red
    Color::ctime_hex("C75147"),
    // blue
    Color::ctime_hex("94D3E6"),
    // cyan
    Color::ctime_hex("A8FECB"),
    // orange
    Color::ctime_hex("FDA73C"),
    // green
    Color::ctime_hex("9BD85F"),
    // yellow
    Color::ctime_hex("E3D050"),
    // magenta
    Color::ctime_hex("B157D0"),
];

fn search_kvflag(args: &[Flag], searched: &str) -> Option<String>{
    for f in args{
        match f {
            Flag::KeyValue(key, value) => {
                if key.as_str() == searched{
                    return Some(value.to_string());
                }
            }
            _ => continue
        }
    }
    return None;
}

fn main() {
    let args = Flag::from_vec(&env::args().collect::<Vec<String>>());

    if args.contains(&Flag::LongFlag("--help".to_string())) || args.contains(&Flag::ShortFlag("-h".to_string())){
        println!(
"mateus-image v1.2
Flags:
--help      /-h                 : prints this message
--input     /-i=[IMAGE_PATH]
--output    /-o=[OUTPUT_PATH]
--standard  /-s                 : replaces unchanged pixel with closest one on palette
--stalinsort/-S                 : uses stalinsort-like filter to filter out rarest colors
--median    /-M                 : uses median pixel in area of (x,y) (DEPTH,DEPTH) from pixel
--mean      /-m                 : uses mathematic mean to choose pixel in area of (x,y) (DEPTH, DEPTH) 
                                  from pixel
--pixel     /-p                 : pixelates image; To get good quality image, it's recommended to use stalinsort
                                  filter after pixel one.
--depth=[DEPTH]                 : used by median filter;
                                  used by stalinsort filter; declares how much of top colors will be chosen
                                  used by mean filter;
                                  used by pixel filter; specifies changed pixel size (x, y) : (depth, depth)
-----
made by matissoss and licensed under MIT License
");
        return;
    }
    let mut in_file_path = None;
    let mut out_file_path = None;
    
    let mut flag = ImageFilter::Standard;
    for arg in &args{
        if arg == &Flag::ShortFlag("-M".to_string()) || arg == &Flag::LongFlag("--median".to_string()){
            let mut found = false;
            if let Some(d) = search_kvflag(&args, "--depth"){
                if let Ok(n) = d.trim().parse::<u16>(){
                    flag = ImageFilter::Median(n);
                    found = true;
                    break;
                }
            }
            if found {break}
            flag = ImageFilter::Median(1);
        }
        else if arg == &Flag::ShortFlag("-m".to_string()) || arg == &Flag::LongFlag("--mean".to_string()){
            let mut found = false;
            if let Some(d) = search_kvflag(&args, "--depth"){
                if let Ok(n) = d.trim().parse::<u16>(){
                    flag = ImageFilter::Mean(n);
                    found = true;
                    break;
                }
            }
            if found {break}
            flag = ImageFilter::Mean(1);
        }
        else if arg == &Flag::ShortFlag("-p".to_string()) || arg == &Flag::LongFlag("--pixel".to_string()){
            let mut found = false;
            if let Some(d) = search_kvflag(&args, "--depth"){
                if let Ok(n) = d.trim().parse::<u16>(){
                    flag = ImageFilter::Pixel(n);
                    found = true;
                    break;
                }
            }
            if found {break}
            flag = ImageFilter::Pixel(1);
        }
        else if arg == &Flag::ShortFlag("-S".to_string()) || arg == &Flag::LongFlag("--stalinsort".to_string()){
            let mut found = false;
            if let Some(d) = search_kvflag(&args, "--depth"){
                if let Ok(n) = d.trim().parse::<u16>(){
                    flag = ImageFilter::StalinSort(n);
                    found = true;
                    break;
                }
            }
            if found {break}
            flag = ImageFilter::StalinSort(1);
        }
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
    println!("Filter: {:?}", flag);
    if let Some(image_path) = in_file_path{
        if let Some(outpath) = out_file_path {
            convert_image(&image_path, &outpath, flag);
        }
        else{
            convert_image(&image_path, "output.png", flag);
        }
    }
}

fn convert_image(path: &str, outpath: &str, img_filter: ImageFilter){
    if let Ok(true) = fs::exists(path){
        if let Ok(img) = image::ImageReader::open(path){
            match img.decode(){
                Ok(dynimg) => {
                    let mut rgb8_img = dynimg.to_rgb8();
                    match img_filter{
                        ImageFilter::Standard => {
                            standard::StandardFilter::convert_image(&standard::StandardFilter, &mut rgb8_img)
                        }
                        ImageFilter::Mean(depth) => {
                            mean::MeanFilter::convert_image(&mean::MeanFilter(depth),&mut rgb8_img);
                        }
                        ImageFilter::StalinSort(depth) => {
                            stalin::StalinsortFilter::convert_image(&stalin::StalinsortFilter(depth),&mut rgb8_img);
                        }
                        ImageFilter::Pixel(depth) => {
                            pixel::PixelFilter::convert_image(&pixel::PixelFilter(depth), &mut rgb8_img);
                        }
                        ImageFilter::Median(depth) => {
                            median::MedianFilter::convert_image(&median::MedianFilter(depth),&mut rgb8_img);
                        }
                    }
                    match rgb8_img.save(outpath){
                        Ok(_) => {
                            println!("Converted `{}`; outpath=`{}`", path, outpath);
                        }
                        Err(e) => {
                            println!("Save Error: {}",e);
                        }
                    };
                }
                Err(imgerr) => {
                    println!("Decoding Error: {}", imgerr);
                }
            }
        };
    }
}
