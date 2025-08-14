#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Liquidation {
    none = 0x0_u8, 
    maker = 0x1_u8, 
    taker = 0x2_u8, 
    both = 0x3_u8, 
    NullVal = 0xff_u8, 
}
impl Default for Liquidation {
    #[inline]
    fn default() -> Self { Liquidation::NullVal }
}
impl From<u8> for Liquidation {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x0_u8 => Self::none, 
            0x1_u8 => Self::maker, 
            0x2_u8 => Self::taker, 
            0x3_u8 => Self::both, 
            _ => Self::NullVal,
        }
    }
}
