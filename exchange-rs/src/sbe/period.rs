#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Period {
    perpetual = 0x0_u8, 
    minute = 0x1_u8, 
    hour = 0x2_u8, 
    day = 0x3_u8, 
    week = 0x4_u8, 
    month = 0x5_u8, 
    year = 0x6_u8, 
    NullVal = 0xff_u8, 
}
impl Default for Period {
    #[inline]
    fn default() -> Self { Period::NullVal }
}
impl From<u8> for Period {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x0_u8 => Self::perpetual, 
            0x1_u8 => Self::minute, 
            0x2_u8 => Self::hour, 
            0x3_u8 => Self::day, 
            0x4_u8 => Self::week, 
            0x5_u8 => Self::month, 
            0x6_u8 => Self::year, 
            _ => Self::NullVal,
        }
    }
}
