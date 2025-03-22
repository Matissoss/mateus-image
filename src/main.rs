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
};

use cli::{
    GLOBAL_CLI,
    Flag
};

use image::{
    self,
    ImageBuffer,
    Rgb,
    ImageReader,
};

use std::{
    path::PathBuf,
    sync::LazyLock,
    process,
    fs
};

#[derive(Default, PartialEq)]
enum RegisteredFilters{
    #[default]
    Standard,
    Median,
    Mean,
    Binary,
    Ascii,
    Pixel,
    Stalinsort,
    Extra1
}

pub const DEF_SCHEME : [Color; 18] = 
[
    // bg 
    Color::ctime_hex("121212"),
    Color::ctime_hex("242424"),
    // fg
    Color::ctime_hex("C8C8CC"),
    Color::ctime_hex("AAAAAD"),
    // red
    Color::ctime_hex("C75147"),
    Color::ctime_hex("662A25"),
    // blue
    Color::ctime_hex("94D3E6"),
    Color::ctime_hex("6F9FAD"),
    // cyan
    Color::ctime_hex("91FFE9"),
    Color::ctime_hex("71C7B6"),
    // orange
    Color::ctime_hex("D99E64"),
    Color::ctime_hex("B88654"),
    // green
    Color::ctime_hex("9BD85F"),
    Color::ctime_hex("8DBF67"),
    // yellow
    Color::ctime_hex("D4B85D"),
    Color::ctime_hex("B39B4F"),
    // magenta
    Color::ctime_hex("B157D0"),
    Color::ctime_hex("713885")
];

pub static COLORSCHEME : LazyLock<Vec<Color>> = LazyLock::new(|| {
    if let Some(cfg) = &*config::CONFIG{
        cfg.colors.clone()
    }
    else{
        DEF_SCHEME.to_vec()
    }
});

pub const MAIN_HELP     : &str = include_str!("help/main.txt");
pub const CONFIG_HELP   : &str = include_str!("help/config.txt");

fn main() {
    let cli = &*GLOBAL_CLI;

    let mut inpath  : Option<PathBuf>           = None;
    let mut outpath : Option<PathBuf>           = None;
    let mut param   : Option<u16>               = None;

    if let Some(flag) = cli.get_flag("-i"){
        if let Flag::KeyValue(_, path) = flag{
            inpath = Some(PathBuf::from(path));
        }
    }
    if let Some(flag) = cli.get_flag("-o") {
        if let Flag::KeyValue(_, path) = flag{
            outpath = Some(PathBuf::from(path));
        }
    }
    if let Some(flag) = cli.get_flag("-p") {
        if let Flag::KeyValue(_, var) = flag{
            if let Ok(n) = var.trim().parse::<u16>() {param = Some(n)}
        }
    }

    if cli.contains_flag("help"){
        println!("{}", MAIN_HELP);
        process::exit(0);
    }
    if cli.contains_flag("confighelp"){
        println!("{}", CONFIG_HELP);
        process::exit(0);
    }

    cli.debug(&format!("[main.rs]: colorscheme: {:?}", &*COLORSCHEME));

    let filter : RegisteredFilters = 
    if      cli.contains_flag("standard")   {   RegisteredFilters::Standard     }
    else if cli.contains_flag("ascii")      {   RegisteredFilters::Ascii        }
    else if cli.contains_flag("pixel")      {   RegisteredFilters::Pixel        }
    else if cli.contains_flag("binary")     {   RegisteredFilters::Binary       }
    else if cli.contains_flag("extras-1")   {   RegisteredFilters::Extra1       }
    else if cli.contains_flag("median")     {   RegisteredFilters::Median       }
    else if cli.contains_flag("mean")       {   RegisteredFilters::Mean         }
    else if cli.contains_flag("stalinsort") {   RegisteredFilters::Stalinsort   }
    else                                    {   
        println!("[main.rs]: unknown option, use `help`"); process::exit(0)                    
    };

    convert_image([inpath.as_ref(), outpath.as_ref()], param, &filter, &*COLORSCHEME);
}

fn save_img(to_save: &ImageBuffer<Rgb<u8>, Vec<u8>>, path: PathBuf) -> !{
    match to_save.save(&path){
        Ok(_) => {
            println!("[main.rs:save_img] (SUCCESS): image succesfully saved to {:?}", path);
            process::exit(0);
        }
        Err(err) => {
            println!("[main.rs:save_img] (ERROR): {}", err);
            process::exit(1);
        }
    }
}

fn apply_filter(rgb8img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, filt: &RegisteredFilters, cls: &[Color], param: Option<u16>){
    match filt {
        &RegisteredFilters::Standard => {
            StandardFilter::convert_image(&StandardFilter, rgb8img, cls);
        }
        &RegisteredFilters::Pixel => {
            let uparam = param.unwrap_or(1);
            PixelFilter::convert_image(&PixelFilter((uparam & 0xFFFF) as u16), rgb8img, cls);
        }
        &RegisteredFilters::Mean => {
            let uparam = param.unwrap_or(1);
            MeanFilter::convert_image(&MeanFilter((uparam & 0xFFFF) as u16), rgb8img, cls);
        }
        &RegisteredFilters::Median => {
            let uparam = param.unwrap_or(1);
            MedianFilter::convert_image(&MedianFilter((uparam & 0xFFFF) as u16), rgb8img, cls);
        }
        &RegisteredFilters::Binary => {
            BinaryFilter::convert_image(&BinaryFilter, rgb8img, cls);
        }
        &RegisteredFilters::Ascii => {
            let uparam = param.unwrap_or(1);
            ASCIIFilter::convert_image(&ASCIIFilter((uparam & 0xFFFF) as u16), rgb8img, &[]);
            std::process::exit(0);
        }
        &RegisteredFilters::Stalinsort => {
            let uparam = param.unwrap_or(1);
            StalinsortFilter::convert_image(&StalinsortFilter((uparam & 0xFFFF) as u16), rgb8img, &cls);
        }
        &RegisteredFilters::Extra1 => {
            InversionFilter::convert_image(&InversionFilter, rgb8img, &cls);
        }
    }
}

fn convert_image(iopaths: [Option<&PathBuf>;2], param: Option<u16>, filt: &RegisteredFilters, cls: &[Color]) {
    if let Ok(true) = fs::exists(   &iopaths[0].unwrap_or(&PathBuf::from("."))  ){
        match ImageReader::open (   &iopaths[0].unwrap_or(&PathBuf::from("."))  ){
            Ok(dynimg) => {
                match dynimg.decode(){
                    Ok(decoded_img) => {
                        let mut rgb8_img = decoded_img.to_rgb8();
                        apply_filter(&mut rgb8_img, filt, cls, param);
                        save_img(&rgb8_img, iopaths[1].unwrap_or(&PathBuf::from(".")).to_path_buf());
                    }
                    Err(error) => {
                        println!("[main.rs:convert_image] (ERROR): {}", error);
                    }
                }
            },
            Err(error) => {
                return println!("[main.rs:convert_image] (ERROR): {}", error);
            }
        }
    }
    else{
        return println!("[main.rs:convert_image] (ERROR): file does not exist!");
    }
}
