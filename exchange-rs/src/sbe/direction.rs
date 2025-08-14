#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Direction {
    buy = 0x0_u8, 
    sell = 0x1_u8, 
    NullVal = 0xff_u8, 
}
impl Default for Direction {
    #[inline]
    fn default() -> Self { Direction::NullVal }
}
impl From<u8> for Direction {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x0_u8 => Self::buy, 
            0x1_u8 => Self::sell, 
            _ => Self::NullVal,
        }
    }
}
