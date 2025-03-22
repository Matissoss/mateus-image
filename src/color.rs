use std::{
    fmt::{
        self,
        Formatter,
        Debug
    },
    cmp::Ordering
};

#[derive(Clone, Hash, Copy)]
pub struct Color{
    pub r: u8,
    pub g: u8,
    pub b: u8
}

impl Eq for Color{}

impl PartialEq for Color{
    fn eq(&self, other: &Color) -> bool{
        self.r == other.r && self.g == other.g && self.b == other.b
    }
}

impl Debug for Color{
    fn fmt(&self, form: &mut Formatter<'_>) -> Result<(), fmt::Error>{
        writeln!(form, "Color({},{},{})", self.r,self.g,self.b)
    }
}

impl PartialOrd for Color{
    fn partial_cmp(&self, othr: &Color) -> Option<Ordering> {
        let grouped_self  : u16 = self.r as u16 + self.g as u16 + self.b as u16;
        let grouped_other : u16 = othr.r as u16 + othr.g as u16 + othr.b as u16;

        if grouped_self < grouped_other{
            return Some(Ordering::Less);
        }
        else if grouped_self == grouped_other {
            return Some(Ordering::Equal);
        }
        else {
            return Some(Ordering::Greater);
        }
    }
}

impl Ord for Color{
    fn cmp(&self, othr: &Self) -> Ordering{
        let grouped_self  : u16 = self.r as u16 + self.g as u16 + self.b as u16;
        let grouped_other : u16 = othr.r as u16 + othr.g as u16 + othr.b as u16;

        if grouped_self < grouped_other{
            return Ordering::Less;
        }
        else if grouped_self == grouped_other {
            return Ordering::Equal;
        }
        else {
            return Ordering::Greater;
        }
    }
}

impl ToString for Color{
    fn to_string(&self) -> String{
        return format!("({}, {}, {})", self.r, self.g, self.b)
    }
}

impl Color{
    pub fn new() -> Self{
        return Self {r:0,g:0,b:0};
    }
    
    pub fn avg_rgb(colors: &[Color]) -> Color {
        let mut avg = [0u64; 3];

        for val in colors{
            avg[0] += val.r as u64;
            avg[1] += val.g as u64;
            avg[2] += val.b as u64;
        }
        let len = if colors.len() == 0 {1} else {colors.len()};
        avg[0] /= len as u64;
        avg[1] /= len as u64;
        avg[2] /= len as u64;

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
    

    pub fn change_color(&self, palette: &[Color]) -> Color {
        let mut chosen = Color::new();
        let mut distance: u16 = 255*3;
        for c in palette {
            let c_dist = c.distance_from(self);
            if c_dist < distance {
                distance = c_dist;
                chosen = *c;
            }
        }
        return chosen;
    }
    
    #[inline(always)]
    pub fn from_arr(arr: &[u8;3]) -> Color{
        return Color{r:arr[0],g:arr[1],b:arr[2]};
    }
    
    #[inline(always)]
    pub fn from_tuple(tup: &(u8,u8,u8)) -> Color{
        return Color{r:tup.0,g:tup.1,b:tup.2}
    }
    
    #[inline(always)]
    pub fn to_arr(&self) -> [u8;3]{
        return [self.r, self.g, self.b];
    }

    #[inline(always)]
    pub fn to_tuple(&self) -> (u8,u8,u8){
        return (self.r,self.g,self.b);
    }

    #[inline(always)]
    pub fn distance_from(&self, c: &Color) -> u16{
        return c.r.abs_diff(self.r) as u16 + c.g.abs_diff(self.g) as u16 + c.b.abs_diff(self.b) as u16;
    }
    
    #[inline(always)]
    pub fn lumination(&self) -> u32{
        return ((self.r as f32 * 0.2126) + (self.g as f32 * 0.7152) + (self.b as f32 * 0.0722)) as u32
    }
    
    #[inline(always)]
    pub fn quantize(&self, num: u16) -> u8{
        ((self.r as u16 + self.g as u16 + self.b as u16) * num / 765) as u8
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
