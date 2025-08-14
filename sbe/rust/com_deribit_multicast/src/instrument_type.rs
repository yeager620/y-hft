#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum InstrumentType {
    not_applicable = 0x0_u8, 
    reversed = 0x1_u8, 
    linear = 0x2_u8, 
    NullVal = 0xff_u8, 
}
impl Default for InstrumentType {
    #[inline]
    fn default() -> Self { InstrumentType::NullVal }
}
impl From<u8> for InstrumentType {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x0_u8 => Self::not_applicable, 
            0x1_u8 => Self::reversed, 
            0x2_u8 => Self::linear, 
            _ => Self::NullVal,
        }
    }
}
