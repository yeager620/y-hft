#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum BookChange {
    created = 0x0_u8, 
    changed = 0x1_u8, 
    deleted = 0x2_u8, 
    NullVal = 0xff_u8, 
}
impl Default for BookChange {
    #[inline]
    fn default() -> Self { BookChange::NullVal }
}
impl From<u8> for BookChange {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x0_u8 => Self::created, 
            0x1_u8 => Self::changed, 
            0x2_u8 => Self::deleted, 
            _ => Self::NullVal,
        }
    }
}
