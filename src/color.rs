#[derive(Ord,Debug, PartialEq, Clone, PartialOrd, Eq, Hash,Copy)]
pub struct Color{
    pub red: u8,
    pub green: u8,
    pub blue: u8
}

impl Color{
    #[inline(always)]
    pub fn lumination(&self) -> u8{
        return ((self.red as f32 * 0.299 + self.green as f32 * 0.587 + self.blue as f32 * 0.114)
            .floor() as u32 & 0xFF).try_into().unwrap();
    }

    pub fn avg_rgb(values: &[Color]) -> Color {
        let mut avg = [0u16; 3];

        for (i, val) in values.iter().enumerate() {
            avg[0] += (val.red   as u16).wrapping_sub(avg[0]) / (i as u16 + 1);
            avg[1] += (val.blue  as u16).wrapping_sub(avg[1]) / (i as u16 + 1);
            avg[2] += (val.green as u16).wrapping_sub(avg[2]) / (i as u16 + 1);
        }

        Color{red:avg[0] as u8, green:avg[1] as u8, blue:avg[2] as u8}
    }
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
    pub fn from_arr(arr: &[u8;3]) -> Color{
        return Color{red:arr[0],green:arr[1],blue:arr[2]};
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
        _       => 0
    }
}

impl Color{
    pub const fn ctime_hex(inp : &'static str) -> Color{
        let hex = inp.as_bytes();
        let (red, green, blue) : (u8,u8,u8) = 
        (
            from_hexchar(hex[0] as char) * 16 + from_hexchar(hex[1] as char),
            from_hexchar(hex[2] as char) * 16 + from_hexchar(hex[3] as char),
            from_hexchar(hex[4] as char) * 16 + from_hexchar(hex[5] as char),
        );
        return Color{red,green,blue};
    }
}
