#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum InstrumentKind {
    future = 0x0_u8, 
    option = 0x1_u8, 
    future_combo = 0x2_u8, 
    option_combo = 0x3_u8, 
    spot = 0x4_u8, 
    NullVal = 0xff_u8, 
}
impl Default for InstrumentKind {
    #[inline]
    fn default() -> Self { InstrumentKind::NullVal }
}
impl From<u8> for InstrumentKind {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x0_u8 => Self::future, 
            0x1_u8 => Self::option, 
            0x2_u8 => Self::future_combo, 
            0x3_u8 => Self::option_combo, 
            0x4_u8 => Self::spot, 
            _ => Self::NullVal,
        }
    }
}
