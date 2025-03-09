use image::ImageReader;

use std::{
    env,
    fs
};

#[derive(PartialEq, Eq, Debug)]
enum ImageFilter{
    Median(u16),
    Standard,
    Stalin(u16),
    Reverse,
    Mean(u16)
}

#[derive(PartialEq, Eq)]
enum Flag{
    KeyValue(String, String),
    LongFlag(String),
    ShortFlag(String)
}

#[derive(PartialEq, Clone, Debug)]
struct ColorFreq{
    quantity: u32,
    color: [u8;3]
}

impl From<(u32, [u8;3])> for ColorFreq{
    fn from(value: (u32, [u8;3])) -> Self{
        return ColorFreq{quantity:value.0, color:value.1};
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

const PALETTE : [[u8;3]; 8] = 
[
     [18, 18, 18],    // black
     [177, 66, 66],   // red
     [216, 124, 74],  // green 
     [228, 154, 68],  // yellow 
     [74, 139, 139],  // blue 
     [167, 167, 167], // magenta
     [180, 180, 180], // cyan
     [213, 213, 213]  // white
];

fn main() {
    let args = Flag::from_vec(&env::args().collect::<Vec<String>>());

    if args.contains(&Flag::LongFlag("--help".to_string())) || args.contains(&Flag::ShortFlag("-h".to_string())){
        println!(
"ashen-image v1.1
Flags:
--help/-h                 : prints this message
--input/-i=[IMAGE_PATH]
--output/-o=[OUTPUT_PATH]
--stalin/-S               : uses stalinsort-like filter to filter out rarest pixels
                            name comes from stalin's purges. Learn more at: https://en.wikipedia.org/wiki/Great_Purge
--median/-M               : uses median filter aka smooths image
--mean/-m                 : uses mean filter
--depth=[DEPTH]           : used by median filter; declares how much it needs to repeat
                            used by stalin filter; declares how much of top colors will be chosen
                            used by mean filter; same as median filter
--reverse/-r              : same as standard filter, but reversed
-----
made by matissoss and licensed under MIT License
");
        return;
    }
    let mut in_file_path = None;
    let mut out_file_path = None;
    
    let mut flag = ImageFilter::Standard;
    let mut depth = None;
    for arg in &args{
        if arg == &Flag::ShortFlag("-M".to_string()) || arg == &Flag::LongFlag("--median".to_string()){
            for arg1 in &args{
                match arg1{
                    Flag::KeyValue(key, value) => {
                        match key.as_str(){
                            "--depth" | "-D" => {
                                depth = value.trim().parse::<u16>().ok();
                            }
                            _ => continue
                        }
                    }
                    _ => continue
                }
            }
            flag = ImageFilter::Median(depth.unwrap_or(1));
        }
        else if arg == &Flag::ShortFlag("-r".to_string()) || arg == &Flag::LongFlag("--reverse".to_string()){
            flag = ImageFilter::Reverse;
        }
        else if arg == &Flag::ShortFlag("-m".to_string()) || arg == &Flag::LongFlag("--mean".to_string()){
            for arg1 in &args{
                match arg1{
                    Flag::KeyValue(key, value) => {
                        match key.as_str(){
                            "--depth" | "-D" => {
                                depth = value.trim().parse::<u16>().ok();
                            }
                            _ => continue
                        }
                    }
                    _ => continue
                }
            }
            flag = ImageFilter::Mean(depth.unwrap_or(1));
        }
        else if arg == &Flag::ShortFlag("-S".to_string()) || arg == &Flag::LongFlag("--stalin".to_string()){
            for arg1 in &args{
                match arg1{
                    Flag::KeyValue(key, value) => {
                        match key.as_str(){
                            "--depth" | "-D" => {
                                depth = value.trim().parse::<u16>().ok();
                            }
                            _ => continue
                        }
                    }
                    _ => continue
                }
            }
            flag = ImageFilter::Stalin(depth.unwrap_or(1));
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

fn change_color(color: [u8;3], palette: &[[u8;3]]) -> [u8; 3] {
    let mut chosen = (0, 0, 0);
    let mut distance: u16 = 255*3; // max rgb distance = 255 * 3
    for color_t in palette {
        let mut tmp_dist: u16 = 0;
        let (r,g,b) = (color_t[0], color_t[1], color_t[2]);
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

fn alt_change_color(color: [u8; 3], palette:&[[u8;3]]) -> [u8;3]{
    let mut chosen = (0, 0, 0);
    let mut distance: u16 = 0;
    for color_t in palette {
        let mut tmp_dist: u16 = 0;
        let (r,g,b) = (color_t[0], color_t[1], color_t[2]);
        let mut c_u16: u16 = r.abs_diff(color[0]).into();
        tmp_dist += c_u16;
        c_u16 = g.abs_diff(color[1]).into();
        tmp_dist += c_u16;
        c_u16 = b.abs_diff(color[2]).into();
        tmp_dist += c_u16;

        if tmp_dist > distance {
            distance = tmp_dist;
            chosen = (r, g, b);
        }
    }
    return [chosen.0, chosen.1, chosen.2];
}

fn get_pixel(img: &image::ImageBuffer<image::Rgb<u8>, Vec<u8>>, dim: (u32, u32), pos: (u32,u32)) -> Option<[u8; 3]>{
    if dim.0 > pos.0 && dim.1 > pos.1{
        return Some(img.get_pixel(pos.0, pos.1).0);
    }
    else{
        return None;
    }
}

fn quicksort<T>(input: &Vec<T>) -> Vec<T>
where T: PartialEq + std::cmp::PartialOrd + Clone{
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

fn convert_image(path: &str, outpath: &str, img_filter: ImageFilter){
    if let Ok(true) = fs::exists(path){
        if let Ok(img) = ImageReader::open(path){
            match img.decode(){
                Ok(dynimg) => {
                    let mut rgb8_img = dynimg.to_rgb8();
                    match img_filter{
                        ImageFilter::Standard => {
                            for pixel in rgb8_img.pixels_mut(){
                                *pixel = image::Rgb(change_color(pixel.0, &PALETTE));
                            }
                        }
                        ImageFilter::Mean(depth) => {
                            let (w, h) = rgb8_img.dimensions();
                            let cloned_img = rgb8_img.clone();
                            for _ in 0..depth{
                            for (x, y, pixel_mut) in rgb8_img.enumerate_pixels_mut(){
                                let pxs = [
                                        get_pixel(&cloned_img, (w,h), (x-1, y-1)), 
                                        get_pixel(&cloned_img, (w,h), (x-1, y  )), 
                                        get_pixel(&cloned_img, (w,h), (x-1, y+1)), 
                            
                                        Some(pixel_mut.0),
                                        get_pixel(&cloned_img, (w,h), (x  , y+1)), 
                                        get_pixel(&cloned_img, (w,h), (x  , y-1)),
                                        
                                        get_pixel(&cloned_img, (w,h), (x+1, y-1)), 
                                        get_pixel(&cloned_img, (w,h), (x+1, y  )), 
                                        get_pixel(&cloned_img, (w,h), (x+1, y+1)), 
                                ];
                                let mut mean : [u64;3] = [0;3];
                                let mut count = 1;
                                for px_op in pxs{
                                    if let Some(px) = px_op{
                                        let mut tmp : u64 = px[0].into();
                                        mean[0] += tmp;
                                        tmp = px[1].into();
                                        mean[1] += tmp;
                                        tmp = px[2].into();
                                        mean[2] += tmp;
                                        count+=1;
                                    } 
                                }
                                let color : [u8;3] = [
                                    (mean[0] / count & 0xFF).try_into().unwrap(),
                                    (mean[1] / count & 0xFF).try_into().unwrap(),
                                    (mean[2] / count & 0xFF).try_into().unwrap()
                                ];
                                *pixel_mut = image::Rgb(change_color(color, &PALETTE));
                            }
                            }
                        }
                        ImageFilter::Reverse => {
                            for pixel in rgb8_img.pixels_mut(){
                                *pixel = image::Rgb(alt_change_color(pixel.0, &PALETTE))
                            }
                        }
                        ImageFilter::Stalin(depth) => {
                            for pixel in rgb8_img.pixels_mut(){
                                *pixel = image::Rgb(change_color(pixel.0, &PALETTE));
                            }
                            // HashMap<[u8; 3], u32>
                            let mut hashmap : std::collections::HashMap<[u8;3], u32> = std::collections::HashMap::new();
                            for pixel in rgb8_img.pixels(){
                                if let Some(num) = hashmap.get(&pixel.0){
                                    hashmap.insert(pixel.0, num+1);
                                }
                                else{
                                    hashmap.insert(pixel.0, 1);
                                }
                            }
                            // change hashmap to filtered vector
                            let mut color_vec: Vec<ColorFreq> = vec![];
                            for key in hashmap.keys(){
                                let value = hashmap.get(key).unwrap();
                                color_vec.push(ColorFreq{quantity:*value, color:*key});
                            }
                            color_vec = quicksort(&color_vec);
                            let mut filtered_vec: Vec<[u8;3]> = vec![];

                            for i in 0..depth.into(){
                                if let Some(color) = color_vec.get(i){
                                    filtered_vec.push(color.color);
                                }
                            }
                            drop(color_vec);
                            
                            for pixel in rgb8_img.pixels_mut(){
                                *pixel = image::Rgb(change_color(pixel.0, filtered_vec.as_slice()))
                            }
                        }
                        ImageFilter::Median(depth) => {
                            for pixel in rgb8_img.pixels_mut(){
                                *pixel = image::Rgb(change_color(pixel.0, &PALETTE));
                            }
                            let (w, h) = rgb8_img.dimensions();
                            let cloned_img = rgb8_img.clone();
                            for _ in 0..depth{
                                for (x, y, pixel) in rgb8_img.enumerate_pixels_mut(){
                                    // HashMap<[u8;3], u32>
                                    let mut hashmap = std::collections::HashMap::new();
                                    let pxs = [
                                        get_pixel(&cloned_img, (w,h), (x-1, y-1)), 
                                        get_pixel(&cloned_img, (w,h), (x-1, y  )), 
                                        get_pixel(&cloned_img, (w,h), (x-1, y+1)), 
                                    
                                        Some(pixel.0),
                                        get_pixel(&cloned_img, (w,h), (x  , y+1)), 
                                        get_pixel(&cloned_img, (w,h), (x  , y-1)),

                                        get_pixel(&cloned_img, (w,h), (x+1, y-1)), 
                                        get_pixel(&cloned_img, (w,h), (x+1, y  )), 
                                        get_pixel(&cloned_img, (w,h), (x+1, y+1)), 
                                    ];
                                    for px_op in pxs{
                                        if let Some(px) = px_op{
                                            if let Some(num) = hashmap.get(&px){
                                                hashmap.insert(px, num+1);
                                            }
                                            else{
                                                hashmap.insert(px, 1);
                                            }
                                        }
                                    }
                                    let mut max : u8 = 0;
                                    let mut selected : [u8; 3] = [0;3];
                                    for key in hashmap.keys(){
                                        let tmp_value = hashmap.get(key).unwrap();
                                        if *tmp_value > max{
                                            max = *tmp_value;
                                            selected = *key;
                                        }
                                    }
                                    pixel.0 = selected
                                }
                            }
                        }
                    }
                    match rgb8_img.save(outpath){
                        Ok(_) => {
                            println!("Converted `{}`; outpath=`{}`", path, outpath);
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
