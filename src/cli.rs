use std::{
    sync::LazyLock,
    collections::{
        HashSet,
        HashMap
    }
};

pub static GLOBAL_CLI : LazyLock<Cli> = LazyLock::new(|| {
    return Cli::init();
});

#[derive(Hash, Eq, PartialEq, Clone)]
pub enum Flag{
    KeyValue(String, String),
    Flag(String),
    Value(String)
}

impl Flag{
    fn from_str(str: &str) -> Option<Flag>{
        if str.starts_with("-"){
            if let Some((key, value)) = str.split_once('='){
                return Some(Flag::KeyValue(key.to_string(), value.to_string()));
            }
            else {
                return Some(Flag::Flag(str.to_string()))
            }
        }
        else if str.len() != 0{
            return Some(Flag::Value(str.to_string()));
        }
        return None;
    }
}

pub struct Cli{
    keyvalue: HashMap<String, String>,
    args : HashSet<Flag>
}

impl Cli{
    pub fn init() -> Self{
        let args = std::env::args().collect::<Vec<String>>();
        let mut args_set = HashSet::new();
        let mut keyval_map = HashMap::new();
        for arg in args{
            if let Some((key, value)) = arg.split_once('='){
                keyval_map.insert(key.to_string(), value.to_string());
            }
            else{
                if let Some(flag) = Flag::from_str(&arg){
                    args_set.insert(flag);
                }
            }
        }
        return Cli{
            keyvalue: keyval_map,
            args: args_set,
        };
    }
    pub fn contains_flag(&self, input_str: &str) -> bool{
        if let Some(flag) = Flag::from_str(input_str){
            if self.args.contains(&flag){
                return true;
            }
        }
        if self.keyvalue.contains_key(input_str){
            return true;
        }
        else{
            GLOBAL_CLI.debug(&format!("[cli.rs]: `{}` couldn't be parsed to flag.", input_str));
        }
        return false;
    }
    pub fn get_flag(&self, flag_str: &str) -> Option<Flag>{
        if let Some(flag) = Flag::from_str(flag_str){
            if let Some(fg) = self.args.get(&flag){
                return Some(fg.clone());
            }
        }

        if let Some(value) = self.keyvalue.get(flag_str){
            return Some(Flag::KeyValue(flag_str.to_string(), value.to_string()));
        }

        return None;
    }
    pub fn debug(&self,msg: &str){
        if self.args.contains( &Flag::from_str("--debug").unwrap() ){
            println!("{}", msg);
        }
    }
}
