#[derive(Ord, Debug, PartialEq, Clone, PartialOrd, Eq, Hash, Copy)]
pub struct Color{
    pub r: u8,
    pub g: u8,
    pub b: u8
}

impl ToString for Color{
    fn to_string(&self) -> String{
        return format!("({}, {}, {})", self.r, self.g, self.b)
    }
}

impl Color{
    #[inline(always)]
    pub fn lumination(&self) -> u8{
        return ((self.r as f32 * 0.299 + self.g as f32 * 0.587 + self.b as f32 * 0.114)
            .floor() as u32 & 0xFF).try_into().unwrap();
    }
    #[inline(always)]
    pub fn quantize(&self, num: u16) -> u8{
        ((self.r as u16 + self.g as u16 + self.b as u16) * num / 765) as u8
    }
    pub fn avg_rgb(values: &[Color]) -> Color {
        let mut avg = [0u16; 3];

        for (i, val) in values.iter().enumerate() {
            avg[0] += (val.r    as u16).wrapping_sub(avg[0]) / (i as u16 + 1);
            avg[1] += (val.b    as u16).wrapping_sub(avg[1]) / (i as u16 + 1);
            avg[2] += (val.g    as u16).wrapping_sub(avg[2]) / (i as u16 + 1);
        }

        Color{r:avg[0] as u8, g:avg[1] as u8, b:avg[2] as u8}
    }
    pub fn from_hex(hex: &str) -> Option<Color>{
        if hex.starts_with('#'){
            return Self::from_hex(&hex[1..]);
        }
        else{
            if hex.len() == 6{
                let chrs = hex.as_bytes();
                return Some(Color{
                    r       : hexchar(chrs[0] as char) * 16 + hexchar(chrs[1] as char),
                    g       : hexchar(chrs[2] as char) * 16 + hexchar(chrs[3] as char),
                    b       : hexchar(chrs[4] as char) * 16 + hexchar(chrs[5] as char),
                });
            }
            else if hex.len() == 3{
                let chrs = hex.as_bytes();
                let (r, g, b) : (u8,u8,u8) = 
                (
                    hexchar(chrs[0] as char),
                    hexchar(chrs[1] as char),
                    hexchar(chrs[2] as char),
                );
                return Some(Color{r: r * r, g: g * g, b: b * b});
            }
            else{
                return None;
            }
        }
    }
    pub fn from_arr(arr: &[u8;3]) -> Color{
        return Color{r:arr[0],g:arr[1],b:arr[2]};
    }
    pub fn from_tuple(tup: &(u8,u8,u8)) -> Color{
        return Color{r:tup.0,g:tup.1,b:tup.2}
    }
    pub fn into_arr(&self) -> [u8;3]{
        return [self.r, self.g, self.b];
    }
    pub const fn ctime_hex(inp : &'static str) -> Color{
        let hex = inp.as_bytes();
        let (red, green, blue) : (u8,u8,u8) = 
        (
            hexchar(hex[0] as char) * 16 + hexchar(hex[1] as char),
            hexchar(hex[2] as char) * 16 + hexchar(hex[3] as char),
            hexchar(hex[4] as char) * 16 + hexchar(hex[5] as char),
        );
        return Color{r:red,g:green,b:blue};
    }
    #[inline(always)]
    pub fn distance_from(&self, c: &Color) -> u16{
        return c.r.abs_diff(self.r) as u16 + c.g.abs_diff(self.g) as u16 + c.b.abs_diff(self.b) as u16;
    }
    pub fn change_color(&self, palette: &[Color]) -> [u8; 3] {
        let mut chosen = (0, 0, 0);
        let mut distance: u16 = 255*3; // max rgb distance = 255 * 3
        for c in palette {
            let c_dist = c.distance_from(self);
            if c_dist < distance {
                distance = c_dist;
                chosen = (c.r, c.g, c.b);
            }
        }
        return [chosen.0, chosen.1, chosen.2];
    }
}

const fn hexchar(h: char) -> u8{
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
