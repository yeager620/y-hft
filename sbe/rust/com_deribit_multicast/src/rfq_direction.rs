#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum RfqDirection {
    buy = 0x0_u8, 
    sell = 0x1_u8, 
    no_direction = 0x2_u8, 
    NullVal = 0xff_u8, 
}
impl Default for RfqDirection {
    #[inline]
    fn default() -> Self { RfqDirection::NullVal }
}
impl From<u8> for RfqDirection {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x0_u8 => Self::buy, 
            0x1_u8 => Self::sell, 
            0x2_u8 => Self::no_direction, 
            _ => Self::NullVal,
        }
    }
}
