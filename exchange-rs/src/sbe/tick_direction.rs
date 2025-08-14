#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum TickDirection {
    plus = 0x0_u8, 
    zeroplus = 0x1_u8, 
    minus = 0x2_u8, 
    zerominus = 0x3_u8, 
    NullVal = 0xff_u8, 
}
impl Default for TickDirection {
    #[inline]
    fn default() -> Self { TickDirection::NullVal }
}
impl From<u8> for TickDirection {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x0_u8 => Self::plus, 
            0x1_u8 => Self::zeroplus, 
            0x2_u8 => Self::minus, 
            0x3_u8 => Self::zerominus, 
            _ => Self::NullVal,
        }
    }
}
