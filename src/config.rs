use crate::{
    cli::{
        GLOBAL_CLI,
        Flag
    },
    color::Color
};

use std::{
    env,
    fs,
    sync::LazyLock,
    path::PathBuf
};

#[derive(Default,PartialEq,Debug)]
pub struct Config{
    pub colors: Vec<Color>,
}

pub static CONFIG_PATH : LazyLock<Option<PathBuf>> = LazyLock::new(|| {
    if cfg!(windows){
        return None;
    }
    else{
        #[allow(deprecated)]
        if let Some(hdir) = env::home_dir(){
            Some(hdir.join(PathBuf::from(".config/mateus-image")))
        }
        else{
            GLOBAL_CLI.debug("[config.rs]: couldn't locate user directory");
            return None;
        }
    }
});

pub static CONFIG : LazyLock<Option<Config>> = LazyLock::new(|| {
    let tmp_cli = &*GLOBAL_CLI;
    if let Some(flg) = tmp_cli.get_flag("--color"){
        if let Flag::KeyValue(_, cols) = flg{
            return Some(Config{colors:parse_colors_from_csv(&cols)})
        }
        else{
            return None;
        }
    }
    else if let Some(flg) = tmp_cli.get_flag("-c"){
        if let Flag::KeyValue(_, cols) = flg{
            return Some(Config{colors:parse_colors_from_csv(&cols)})
        }
        else{
            return None;
        }
    }
    else{
        return Some(get_config());
    }
});

pub fn parse_csv(csv_str: &str) -> Vec<String>{
    let mut tmp_buf : Vec<char> = Vec::new();
    let chrs = &csv_str.chars().collect::<Vec<char>>();
    let mut skip = false;
    let mut to_return : Vec<String> = Vec::new();
    for (ind,chr) in csv_str.chars().enumerate(){
        if skip { skip = false; continue; }
        if chr == '\\' && chrs[ind+1] == ','{
            skip = true;
            continue;
        }
        else if chr == ','{
            to_return.push(String::from_iter(tmp_buf.iter()));
            tmp_buf = Vec::new();
            continue;
        }
        else{
            tmp_buf.push(chr);
        }
    }

    if !tmp_buf.is_empty(){
        to_return.push(String::from_iter(tmp_buf.iter()));
    }
    return to_return;
}

pub fn parse_colors_from_csv(csv_str: &str) -> Vec<Color>{
    let mut colors = Vec::new();
    for color_str in parse_csv(csv_str){
        if let Some(col) = Color::from_hex(&color_str){
            colors.push(col);
        }
    }
    return colors;
}

pub fn parse_cfgstr(conf_str: &str) -> Config{
    let mut to_return = Config::default();
    for line in conf_str.lines().collect::<Vec<&str>>(){
        if line.starts_with(";"){
            continue;
        }
        if let Some((key,value)) = line.split_once('='){
            if key == "colors_path"{
                let pathbuf = if let Some(ph) = &*CONFIG_PATH{
                    ph.join(PathBuf::from(value))
                }else{
                    PathBuf::from(value)
                };
                if let Ok(true) = fs::exists(&pathbuf){
                    if let Ok(csv_str) = fs::read_to_string(&pathbuf){
                        to_return.colors = parse_colors_from_csv(&csv_str);
                    }
                }
            }
            else if key == "colors" {
                to_return.colors = parse_colors_from_csv(&value);
            }
        }
    }
    return to_return;
}

pub fn get_config() -> Config{
    if let Some(pathbuf) = &*CONFIG_PATH{
        if let Ok(true) = fs::exists(pathbuf.join(PathBuf::from("conf.ini"))){
            if let Ok(conf_str) = fs::read_to_string(pathbuf.join(PathBuf::from("conf.ini"))){
                return parse_cfgstr(&conf_str);
            }
        }
        else{
            GLOBAL_CLI.debug("[config.rs]: config doesn't exist, create mateus-image directory or conf.ini file");
        }
    }
    return Config{colors: crate::DEF_SCHEME.to_vec()};
}
