use crate::sbe::*;

pub use encoder::*;
pub use decoder::*;

pub const SBE_BLOCK_LENGTH: u16 = 133;
pub const SBE_TEMPLATE_ID: u16 = 1003;
pub const SBE_SCHEMA_ID: u16 = 1;
pub const SBE_SCHEMA_VERSION: u16 = 3;
pub const SBE_SEMANTIC_VERSION: &str = "";

pub mod encoder {
    use super::*;

    #[derive(Debug, Default)]
    pub struct TickerEncoder<'a> {
        buf: WriteBuf<'a>,
        initial_offset: usize,
        offset: usize,
        limit: usize,
    }

    impl<'a> Writer<'a> for TickerEncoder<'a> {
        #[inline]
        fn get_buf_mut(&mut self) -> &mut WriteBuf<'a> {
            &mut self.buf
        }
    }

    impl<'a> Encoder<'a> for TickerEncoder<'a> {
        #[inline]
        fn get_limit(&self) -> usize {
            self.limit
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.limit = limit;
        }
    }

    impl<'a> TickerEncoder<'a> {
        pub fn wrap(mut self, buf: WriteBuf<'a>, offset: usize) -> Self {
            let limit = offset + SBE_BLOCK_LENGTH as usize;
            self.buf = buf;
            self.initial_offset = offset;
            self.offset = offset;
            self.limit = limit;
            self
        }

        #[inline]
        pub fn encoded_length(&self) -> usize {
            self.limit - self.offset
        }

        pub fn header(self, offset: usize) -> MessageHeaderEncoder<Self> {
            let mut header = MessageHeaderEncoder::default().wrap(self, offset);
            header.block_length(SBE_BLOCK_LENGTH);
            header.template_id(SBE_TEMPLATE_ID);
            header.schema_id(SBE_SCHEMA_ID);
            header.version(SBE_SCHEMA_VERSION);
            header
        }

        
        
        
        
        
        
        
        
        #[inline]
        pub fn instrument_id(&mut self, value: u32) {
            let offset = self.offset;
            self.get_buf_mut().put_u32_at(offset, value);
        }

        
        #[inline]
        pub fn instrument_state(&mut self, value: InstrumentState) {
            let offset = self.offset + 4;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

        
        
        
        
        
        
        
        
        #[inline]
        pub fn timestamp_ms(&mut self, value: u64) {
            let offset = self.offset + 5;
            self.get_buf_mut().put_u64_at(offset, value);
        }

        
        
        
        
        
        
        
        
        #[inline]
        pub fn open_interest(&mut self, value: f64) {
            let offset = self.offset + 13;
            self.get_buf_mut().put_f64_at(offset, value);
        }

        
        
        
        
        
        
        
        
        #[inline]
        pub fn min_sell_price(&mut self, value: f64) {
            let offset = self.offset + 21;
            self.get_buf_mut().put_f64_at(offset, value);
        }

        
        
        
        
        
        
        
        
        #[inline]
        pub fn max_buy_price(&mut self, value: f64) {
            let offset = self.offset + 29;
            self.get_buf_mut().put_f64_at(offset, value);
        }

        
        
        
        
        
        
        
        
        #[inline]
        pub fn last_price(&mut self, value: f64) {
            let offset = self.offset + 37;
            self.get_buf_mut().put_f64_at(offset, value);
        }

        
        
        
        
        
        
        
        
        #[inline]
        pub fn index_price(&mut self, value: f64) {
            let offset = self.offset + 45;
            self.get_buf_mut().put_f64_at(offset, value);
        }

        
        
        
        
        
        
        
        
        #[inline]
        pub fn mark_price(&mut self, value: f64) {
            let offset = self.offset + 53;
            self.get_buf_mut().put_f64_at(offset, value);
        }

        
        
        
        
        
        
        
        
        #[inline]
        pub fn best_bid_price(&mut self, value: f64) {
            let offset = self.offset + 61;
            self.get_buf_mut().put_f64_at(offset, value);
        }

        
        
        
        
        
        
        
        
        #[inline]
        pub fn best_bid_amount(&mut self, value: f64) {
            let offset = self.offset + 69;
            self.get_buf_mut().put_f64_at(offset, value);
        }

        
        
        
        
        
        
        
        
        #[inline]
        pub fn best_ask_price(&mut self, value: f64) {
            let offset = self.offset + 77;
            self.get_buf_mut().put_f64_at(offset, value);
        }

        
        
        
        
        
        
        
        
        #[inline]
        pub fn best_ask_amount(&mut self, value: f64) {
            let offset = self.offset + 85;
            self.get_buf_mut().put_f64_at(offset, value);
        }

        
        
        
        
        
        
        
        
        #[inline]
        pub fn current_funding(&mut self, value: f64) {
            let offset = self.offset + 93;
            self.get_buf_mut().put_f64_at(offset, value);
        }

        
        
        
        
        
        
        
        
        #[inline]
        pub fn funding_8h(&mut self, value: f64) {
            let offset = self.offset + 101;
            self.get_buf_mut().put_f64_at(offset, value);
        }

        
        
        
        
        
        
        
        
        #[inline]
        pub fn estimated_delivery_price(&mut self, value: f64) {
            let offset = self.offset + 109;
            self.get_buf_mut().put_f64_at(offset, value);
        }

        
        
        
        
        
        
        
        
        #[inline]
        pub fn delivery_price(&mut self, value: f64) {
            let offset = self.offset + 117;
            self.get_buf_mut().put_f64_at(offset, value);
        }

        
        
        
        
        
        
        
        
        #[inline]
        pub fn settlement_price(&mut self, value: f64) {
            let offset = self.offset + 125;
            self.get_buf_mut().put_f64_at(offset, value);
        }

    }

} 

pub mod decoder {
    use super::*;

    #[derive(Debug, Default)]
    pub struct TickerDecoder<'a> {
        buf: ReadBuf<'a>,
        initial_offset: usize,
        offset: usize,
        limit: usize,
        pub acting_block_length: u16,
        pub acting_version: u16,
    }

    impl<'a> Reader<'a> for TickerDecoder<'a> {
        #[inline]
        fn get_buf(&self) -> &ReadBuf<'a> {
            &self.buf
        }
    }

    impl<'a> Decoder<'a> for TickerDecoder<'a> {
        #[inline]
        fn get_limit(&self) -> usize {
            self.limit
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.limit = limit;
        }
    }

    impl<'a> TickerDecoder<'a> {
        pub fn wrap(
            mut self,
            buf: ReadBuf<'a>,
            offset: usize,
            acting_block_length: u16,
            acting_version: u16,
        ) -> Self {
            let limit = offset + acting_block_length as usize;
            self.buf = buf;
            self.initial_offset = offset;
            self.offset = offset;
            self.limit = limit;
            self.acting_block_length = acting_block_length;
            self.acting_version = acting_version;
            self
        }

        #[inline]
        pub fn encoded_length(&self) -> usize {
            self.limit - self.offset
        }

        pub fn header(self, mut header: MessageHeaderDecoder<ReadBuf<'a>>) -> Self {
            debug_assert_eq!(SBE_TEMPLATE_ID, header.template_id());
            let acting_block_length = header.block_length();
            let acting_version = header.version();

            self.wrap(
                header.parent().unwrap(),
                message_header_codec::ENCODED_LENGTH,
                acting_block_length,
                acting_version,
            )
        }

        
        #[inline]
        pub fn instrument_id(&self) -> u32 {
            self.get_buf().get_u32_at(self.offset)
        }

        
        #[inline]
        pub fn instrument_state(&self) -> InstrumentState {
            self.get_buf().get_u8_at(self.offset + 4).into()
        }

        
        #[inline]
        pub fn timestamp_ms(&self) -> u64 {
            self.get_buf().get_u64_at(self.offset + 5)
        }

        
        #[inline]
        pub fn open_interest(&self) -> Option<f64> {
            let value = self.get_buf().get_f64_at(self.offset + 13);
            if value.is_nan() {
                None
            } else {
                Some(value)
            }
        }

        
        #[inline]
        pub fn min_sell_price(&self) -> f64 {
            self.get_buf().get_f64_at(self.offset + 21)
        }

        
        #[inline]
        pub fn max_buy_price(&self) -> f64 {
            self.get_buf().get_f64_at(self.offset + 29)
        }

        
        #[inline]
        pub fn last_price(&self) -> Option<f64> {
            let value = self.get_buf().get_f64_at(self.offset + 37);
            if value.is_nan() {
                None
            } else {
                Some(value)
            }
        }

        
        #[inline]
        pub fn index_price(&self) -> f64 {
            self.get_buf().get_f64_at(self.offset + 45)
        }

        
        #[inline]
        pub fn mark_price(&self) -> f64 {
            self.get_buf().get_f64_at(self.offset + 53)
        }

        
        #[inline]
        pub fn best_bid_price(&self) -> f64 {
            self.get_buf().get_f64_at(self.offset + 61)
        }

        
        #[inline]
        pub fn best_bid_amount(&self) -> f64 {
            self.get_buf().get_f64_at(self.offset + 69)
        }

        
        #[inline]
        pub fn best_ask_price(&self) -> f64 {
            self.get_buf().get_f64_at(self.offset + 77)
        }

        
        #[inline]
        pub fn best_ask_amount(&self) -> f64 {
            self.get_buf().get_f64_at(self.offset + 85)
        }

        
        #[inline]
        pub fn current_funding(&self) -> Option<f64> {
            let value = self.get_buf().get_f64_at(self.offset + 93);
            if value.is_nan() {
                None
            } else {
                Some(value)
            }
        }

        
        #[inline]
        pub fn funding_8h(&self) -> Option<f64> {
            let value = self.get_buf().get_f64_at(self.offset + 101);
            if value.is_nan() {
                None
            } else {
                Some(value)
            }
        }

        
        #[inline]
        pub fn estimated_delivery_price(&self) -> Option<f64> {
            let value = self.get_buf().get_f64_at(self.offset + 109);
            if value.is_nan() {
                None
            } else {
                Some(value)
            }
        }

        
        #[inline]
        pub fn delivery_price(&self) -> Option<f64> {
            let value = self.get_buf().get_f64_at(self.offset + 117);
            if value.is_nan() {
                None
            } else {
                Some(value)
            }
        }

        
        #[inline]
        pub fn settlement_price(&self) -> Option<f64> {
            let value = self.get_buf().get_f64_at(self.offset + 125);
            if value.is_nan() {
                None
            } else {
                Some(value)
            }
        }

    }

} 

