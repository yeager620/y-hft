#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum OptionType {
    not_applicable = 0x0_u8, 
    call = 0x1_u8, 
    put = 0x2_u8, 
    NullVal = 0xff_u8, 
}
impl Default for OptionType {
    #[inline]
    fn default() -> Self { OptionType::NullVal }
}
impl From<u8> for OptionType {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            0x0_u8 => Self::not_applicable, 
            0x1_u8 => Self::call, 
            0x2_u8 => Self::put, 
            _ => Self::NullVal,
        }
    }
}
