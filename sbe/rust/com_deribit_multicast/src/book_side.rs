#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum BookSide {
    ask = 0x0_u8, 
    bid = 0x1_u8, 
    NullVal = 0xff_u8, 
}
impl Default for BookSide {
    #[inline]
    fn default() -> Self { BookSide::NullVal }
}
impl From<u8> for BookSide {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x0_u8 => Self::ask, 
            0x1_u8 => Self::bid, 
            _ => Self::NullVal,
        }
    }
}
