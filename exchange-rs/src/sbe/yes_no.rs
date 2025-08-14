#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum YesNo {
    no = 0x0_u8, 
    yes = 0x1_u8, 
    NullVal = 0xff_u8, 
}
impl Default for YesNo {
    #[inline]
    fn default() -> Self { YesNo::NullVal }
}
impl From<u8> for YesNo {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x0_u8 => Self::no, 
            0x1_u8 => Self::yes, 
            _ => Self::NullVal,
        }
    }
}
