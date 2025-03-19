use std::sync::LazyLock;

pub static GLOBAL_CLI : LazyLock<Cli> = LazyLock::new(|| {
    return Cli::init();
});

#[derive(Hash, Eq, PartialEq)]
pub enum Flag{
    KeyValue(String, String),
    Flag(String),
    Value(String)
}

impl Flag{
    fn is_equal(&self, flag: &str) -> bool{
        match self{
            Flag::Flag(str) => str == flag,
            Flag::KeyValue(key, _) => key == flag,
            Flag::Value(str) => flag == str
        }
    }
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
    pub args: Vec<Flag>,
    debug_mode : bool
}

impl Cli{
    pub fn init() -> Self{
        let args = std::env::args().collect::<Vec<String>>();
        let mut args_sh = Vec::new();
        let mut debug_mode = false;
        for arg in args{
            if let Some(flag) = Flag::from_str(&arg){
                if &arg == "--debug"{
                    debug_mode = true;
                }
                args_sh.push(flag);
            }
        }
        return Cli{
            args: args_sh,
            debug_mode
        };
    }
    pub fn contains_flag(&self, flag_str: &str) -> bool{
        for flag in &self.args{
            if flag.is_equal(flag_str){
                return true;
            }
        }
        return false;
    }
    pub fn get_flag(&self, flag_str: &str) -> Option<&Flag>{
        for flag in &self.args{
            if flag.is_equal(flag_str){
                return Some(flag);
            }
        }
        return None;
    }
    pub fn debug(&self,msg: &str){
        if self.debug_mode{
            println!("{}", msg);
        }
    }
}
