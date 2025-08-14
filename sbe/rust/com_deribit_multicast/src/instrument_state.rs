#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum InstrumentState {
    created = 0x0_u8, 
    open = 0x1_u8, 
    closed = 0x2_u8, 
    settled = 0x3_u8, 
    deactivated = 0x4_u8, 
    inactive = 0x5_u8, 
    started = 0x6_u8, 
    NullVal = 0xff_u8, 
}
impl Default for InstrumentState {
    #[inline]
    fn default() -> Self { InstrumentState::NullVal }
}
impl From<u8> for InstrumentState {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x0_u8 => Self::created, 
            0x1_u8 => Self::open, 
            0x2_u8 => Self::closed, 
            0x3_u8 => Self::settled, 
            0x4_u8 => Self::deactivated, 
            0x5_u8 => Self::inactive, 
            0x6_u8 => Self::started, 
            _ => Self::NullVal,
        }
    }
}
