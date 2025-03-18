#[derive(Debug, PartialEq, Clone, PartialOrd, Eq, Hash,Copy)]
pub struct Color{
    pub red: u8,
    pub green: u8,
    pub blue: u8
}

impl Color{
    pub fn from_hex(hex: &str) -> Option<Color>{
        if hex.starts_with('#'){
            return Self::from_hex(&hex[1..]);
        }
        else{
            if hex.len() == 6{
                let chrs = hex.chars().collect::<Vec<char>>();
                return Some(Color{
                    red     : from_hexchar(chrs[0]) * 16 + from_hexchar(chrs[1]),
                    green   : from_hexchar(chrs[2]) * 16 + from_hexchar(chrs[3]),
                    blue    : from_hexchar(chrs[4]) * 16 + from_hexchar(chrs[5]),
                });
            }
            else if hex.len() == 3{
                let chrs = hex.chars().collect::<Vec<char>>();
                let (red, green, blue) : (u8,u8,u8) = 
                (
                    from_hexchar(chrs[0]),
                    from_hexchar(chrs[1]),
                    from_hexchar(chrs[2]),
                );
                return Some(Color{red: red * red, green: green * green, blue: blue * blue});
            }
            else{
                return None;
            }
        }
    }
}

const fn from_hexchar(h: char) -> u8{
    return match h{
        '0'|'1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9' => h as u8 - '0' as u8,
        'a'|'A' => 10,
        'b'|'B' => 11,
        'c'|'C' => 12,
        'd'|'D' => 13,
        'e'|'E' => 14,
        'f'|'F' => 15,
        _       => 16
    }
}

const fn chararr_from_color(s: &'static str) -> [char;6]{
    let bytes = s.as_bytes();
    let mut to_ret : [char;6] = [' ';6];
    let mut i = 0;
    while i < bytes.len() && i < 6{
        to_ret[i] = bytes[i] as char;
        i+=1;
    }
    return to_ret;
}

impl Color{
    pub const fn ctime_hex(inp : &'static str) -> Color{
        let hex = chararr_from_color(inp);
        let (red, green, blue) : (u8,u8,u8) = 
        (
            from_hexchar(hex[0]) * 16 + from_hexchar(hex[1]),
            from_hexchar(hex[2]) * 16 + from_hexchar(hex[3]),
            from_hexchar(hex[4]) * 16 + from_hexchar(hex[5]),
        );
        return Color{red,green,blue};
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test(){
        let color = Color::ctime_hex("121212");
        assert!(color == Color{red:18,green:18,blue:18});
    }
}
