#![forbid(unsafe_code)]
#![allow(clippy::upper_case_acronyms)]
#![allow(non_camel_case_types)]
use ::core::{convert::TryInto};

pub mod group_size_encoding_codec;
pub mod snapshot_codec;
pub mod snapshot_start_codec;
pub mod instrument_kind;
pub mod instrument_v2_codec;
pub mod tick_direction;
pub mod rfq_codec;
pub mod instrument_type;
pub mod direction;
pub mod combo_legs_codec;
pub mod book_side;
pub mod trades_codec;
pub mod instrument_state;
pub mod message_header_codec;
pub mod option_type;
pub mod instrument_codec;
pub mod liquidation;
pub mod rfq_direction;
pub mod price_index_codec;
pub mod book_codec;
pub mod snapshot_end_codec;
pub mod yes_no;
pub mod ticker_codec;
pub mod period;
pub mod book_change;
pub mod var_string_codec;

pub use group_size_encoding_codec::*;
pub use snapshot_codec::*;
pub use snapshot_start_codec::*;
pub use instrument_kind::*;
pub use instrument_v2_codec::*;
pub use tick_direction::*;
pub use rfq_codec::*;
pub use instrument_type::*;
pub use direction::*;
pub use combo_legs_codec::*;
pub use book_side::*;
pub use trades_codec::*;
pub use instrument_state::*;
pub use message_header_codec::*;
pub use option_type::*;
pub use instrument_codec::*;
pub use liquidation::*;
pub use rfq_direction::*;
pub use price_index_codec::*;
pub use book_codec::*;
pub use snapshot_end_codec::*;
pub use yes_no::*;
pub use ticker_codec::*;
pub use period::*;
pub use book_change::*;
pub use var_string_codec::*;

pub type SbeResult<T> = core::result::Result<T, SbeErr>;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SbeErr {
    ParentNotSet,
}
impl core::fmt::Display for SbeErr {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl std::error::Error for SbeErr {}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Either<L, R> {
    Left(L),
    Right(R),
}

pub trait Writer<'a>: Sized {
    fn get_buf_mut(&mut self) -> &mut WriteBuf<'a>;
}

pub trait Encoder<'a>: Writer<'a> {
    fn get_limit(&self) -> usize;
    fn set_limit(&mut self, limit: usize);
}

pub trait Reader<'a>: Sized {
    fn get_buf(&self) -> &ReadBuf<'a>;
}

pub trait Decoder<'a>: Reader<'a> {
    fn get_limit(&self) -> usize;
    fn set_limit(&mut self, limit: usize);
}

#[derive(Debug, Default)]
pub struct ReadBuf<'a> {
    data: &'a [u8],
}
impl<'a> Reader<'a> for ReadBuf<'a> {
    #[inline]
    fn get_buf(&self) -> &ReadBuf<'a> {
        self
    }
}
impl<'a> ReadBuf<'a> {
    #[inline]
    pub fn new(data: &'a [u8]) -> Self {
        Self { data }
    }

    #[inline]
    fn get_bytes<const COUNT: usize>(slice: &[u8]) -> [u8; COUNT] {
        slice.try_into().expect("slice with incorrect length")
    }

    #[inline]
    fn get_bytes_at<const COUNT: usize>(slice: &[u8], index: usize) -> [u8; COUNT] {
        Self::get_bytes(slice.split_at(index).1.split_at(COUNT).0)
    }

    #[inline]
    pub fn get_u8_at(&self, index: usize) -> u8 {
        u8::from_le_bytes(Self::get_bytes_at(self.data, index))
    }

    #[inline]
    pub fn get_i8_at(&self, index: usize) -> i8 {
        i8::from_le_bytes(Self::get_bytes_at(self.data, index))
    }

    #[inline]
    pub fn get_i16_at(&self, index: usize) -> i16 {
        i16::from_le_bytes(Self::get_bytes_at(self.data, index))
    }

    #[inline]
    pub fn get_i32_at(&self, index: usize) -> i32 {
        i32::from_le_bytes(Self::get_bytes_at(self.data, index))
    }

    #[inline]
    pub fn get_i64_at(&self, index: usize) -> i64 {
        i64::from_le_bytes(Self::get_bytes_at(self.data, index))
    }

    #[inline]
    pub fn get_u16_at(&self, index: usize) -> u16 {
        u16::from_le_bytes(Self::get_bytes_at(self.data, index))
    }

    #[inline]
    pub fn get_u32_at(&self, index: usize) -> u32 {
        u32::from_le_bytes(Self::get_bytes_at(self.data, index))
    }

    #[inline]
    pub fn get_u64_at(&self, index: usize) -> u64 {
        u64::from_le_bytes(Self::get_bytes_at(self.data, index))
    }

    #[inline]
    pub fn get_f32_at(&self, index: usize) -> f32 {
        f32::from_le_bytes(Self::get_bytes_at(self.data, index))
    }

    #[inline]
    pub fn get_f64_at(&self, index: usize) -> f64 {
        f64::from_le_bytes(Self::get_bytes_at(self.data, index))
    }

    #[inline]
    pub fn get_slice_at(&self, index: usize, len: usize) -> &[u8] {
        self.data.split_at(index).1.split_at(len).0
    }

}

#[derive(Debug, Default)]
pub struct WriteBuf<'a> {
    data: &'a mut [u8],
}
impl<'a> WriteBuf<'a> {
    pub fn new(data: &'a mut [u8]) -> Self {
        Self { data }
    }

    #[inline]
    pub fn put_bytes_at<const COUNT: usize>(&mut self, index: usize, bytes: [u8; COUNT]) -> usize {
        for (i, byte) in bytes.iter().enumerate() {
            self.data[index + i] = *byte;
        }
        COUNT
    }

    #[inline]
    pub fn put_u8_at(&mut self, index: usize, value: u8) {
        self.put_bytes_at(index, u8::to_le_bytes(value));
    }

    #[inline]
    pub fn put_i8_at(&mut self, index: usize, value: i8) {
        self.put_bytes_at(index, i8::to_le_bytes(value));
    }

    #[inline]
    pub fn put_i16_at(&mut self, index: usize, value: i16) {
        self.put_bytes_at(index, i16::to_le_bytes(value));
    }

    #[inline]
    pub fn put_i32_at(&mut self, index: usize, value: i32) {
        self.put_bytes_at(index, i32::to_le_bytes(value));
    }

    #[inline]
    pub fn put_i64_at(&mut self, index: usize, value: i64) {
        self.put_bytes_at(index, i64::to_le_bytes(value));
    }

    #[inline]
    pub fn put_u16_at(&mut self, index: usize, value: u16) {
        self.put_bytes_at(index, u16::to_le_bytes(value));
    }

    #[inline]
    pub fn put_u32_at(&mut self, index: usize, value: u32) {
        self.put_bytes_at(index, u32::to_le_bytes(value));
    }

    #[inline]
    pub fn put_u64_at(&mut self, index: usize, value: u64) {
        self.put_bytes_at(index, u64::to_le_bytes(value));
    }

    #[inline]
    pub fn put_f32_at(&mut self, index: usize, value: f32) {
        self.put_bytes_at(index, f32::to_le_bytes(value));
    }

    #[inline]
    pub fn put_f64_at(&mut self, index: usize, value: f64) {
        self.put_bytes_at(index, f64::to_le_bytes(value));
    }

    #[inline]
    pub fn put_slice_at(&mut self, index: usize, src: &[u8]) -> usize {
        let len = src.len();
        let dest = self.data.split_at_mut(index).1.split_at_mut(len).0;
        dest.clone_from_slice(src);
        len
    }
}

