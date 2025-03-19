mod filters ;
mod color   ;
mod cli     ;
mod config  ;

use color::Color;

use filters::{
    ChangeImage,
    ascii       ::ASCIIFilter       ,
    median      ::MedianFilter      ,
    stalinsort  ::StalinsortFilter  ,
    pixel       ::PixelFilter       ,
    mean        ::MeanFilter        ,
    standard    ::StandardFilter    ,
    binary      ::BinaryFilter      ,
    inversion   ::InversionFilter   ,
    monochrome  ::MonochromeFilter  ,
};

use cli::{
    GLOBAL_CLI,
    Flag
};

use image::{
    self,
    ImageReader
};

use std::{
    path::PathBuf,
    process,
    fs
};

pub const DEF_SCHEME : [Color; 11] = 
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

pub const MAIN_HELP     : &str = include_str!("help/help.txt");

fn main() {
    let cli = &*GLOBAL_CLI;
    cli.debug("[main.rs]: initialized CLI");

    let mut inpath  : Option<PathBuf>           = None;
    let mut outpath : Option<PathBuf>           = None;
    let mut param   : Option<u16>               = None;

    if let Some(flag) = cli.get_flag("--input"){
        if let Flag::KeyValue(_, path) = flag{
            cli.debug("[main.rs]: found inpath; --input variant");
            inpath = Some(PathBuf::from(path));
        }
    }
    else if let Some(flag) = cli.get_flag("-i"){
        if let Flag::KeyValue(_, path) = flag{
            cli.debug("[main.rs]: found inpath; -i variant");
            inpath = Some(PathBuf::from(path));
        }
    }

    if let Some(flag) = cli.get_flag("--output") {
        if let Flag::KeyValue(_, path) = flag{
            cli.debug("[main.rs]: found outpath; --output variant");
            outpath = Some(PathBuf::from(path));
        }
    }
    else if let Some(flag) = cli.get_flag("-o"){
        if let Flag::KeyValue(_, path) = flag{
            cli.debug("[main.rs]: found outpath; -o variant");
            outpath = Some(PathBuf::from(path));
        }
    }


    if let Some(flag) = cli.get_flag("--param") {
        if let Flag::KeyValue(_, var) = flag{
            cli.debug("[main.rs]: found param; --param variant");
            if let Ok(n) = var.trim().parse::<u16>() {param = Some(n)}
        }
    }
    else if let Some(flag) = cli.get_flag("-p"){
        if let Flag::KeyValue(_, var) = flag{
            cli.debug("[main.rs]: found param; -p variant");
            if let Ok(n) = var.trim().parse::<u16>() {param = Some(n)}
        }
    }

    if cli.contains_flag("help"){
        println!("{}", MAIN_HELP);
        process::exit(0);
    }
    
    let cl_scheme : Vec<Color> = if let Some(cfg) = &*config::CONFIG{
        cfg.colors.clone()
    }
    else{
        DEF_SCHEME.to_vec()
    };

    cli.debug(&format!("[main.rs]: color scheme\n{:?}",cl_scheme));
    
    if let (Some(inpath), Some(param)) = (&inpath, param){
        if cli.contains_flag("ascii"){
            convert_image(&inpath, &PathBuf::new(), ASCIIFilter(param), &[]);
            process::exit(0);
        }
    }

    if let (Some(inpath), Some(outpath)) = (inpath, outpath){
        if cli.contains_flag("standard"){
            convert_image(&inpath, &outpath, StandardFilter, &cl_scheme);
        }
        else if cli.contains_flag("binary"){
            convert_image(&inpath, &outpath, BinaryFilter, &cl_scheme);
        }
        else if cli.contains_flag("monochrome"){
            convert_image(&inpath, &outpath, MonochromeFilter, &cl_scheme);
        }
        else if cli.contains_flag("extras-1"){
            convert_image(&inpath, &outpath, InversionFilter, &[]);
        }
        else{
            if let Some(param) = param{
                if      cli.contains_flag("median"){
                    convert_image(&inpath, &outpath, MedianFilter(param), &cl_scheme)
                }
                else if cli.contains_flag("mean") {
                    convert_image(&inpath, &outpath, MeanFilter(param), &cl_scheme);
                }
                else if cli.contains_flag("pixel"){
                    convert_image(&inpath, &outpath, PixelFilter(param), &cl_scheme);
                }
                else if cli.contains_flag("stalinsort"){
                    convert_image(&inpath, &outpath, StalinsortFilter(param), &cl_scheme);
                }
            }
            else{
                println!("[main.rs]: --param isn't specified and you tried to use filter that uses it");
                process::exit(-1);
            }
        }
    }
    else{
        println!("[main.rs]: either input path or output path or both haven't been provided. use `help`");
        process::exit(-1);
    }
}

fn convert_image(inpath: &PathBuf, outpath: &PathBuf, filter: impl ChangeImage, color_scheme: &[Color]){
    let cli = &*GLOBAL_CLI;
    if let Ok(true) = fs::exists(inpath){
        if let Ok(img) = ImageReader::open(inpath){
            match img.decode(){
                Ok(dynimg) => {
                    let mut conv_img = dynimg.to_rgb8();
                    filter.convert_image(&mut conv_img, color_scheme);
                    if let Err(err) = conv_img.save(outpath){
                        cli.debug(&format!("[main.rs]: couldn't save image, error:\n{}",err));
                    } else {
                        cli.debug(&format!("[main.rs]: sucessfully saved image to {:?}",outpath));
                    }
                }
                Err(err) => {
                    println!("[main.rs]: failed to decode image:\n{}", err);
                    process::exit(1);
                }
            }
        }
        else{
            println!("[main.rs]: ImageReader couldn't open file {:?}", inpath);
            process::exit(1);
        }
    }
    else{
        println!("[main.rs]: image in path {:?} doesn't exist", inpath);
        process::exit(1);
    }
}
