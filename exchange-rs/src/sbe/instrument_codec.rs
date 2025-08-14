use crate::sbe::*;

pub use encoder::*;
pub use decoder::*;

pub const SBE_BLOCK_LENGTH: u16 = 140;
pub const SBE_TEMPLATE_ID: u16 = 1000;
pub const SBE_SCHEMA_ID: u16 = 1;
pub const SBE_SCHEMA_VERSION: u16 = 3;
pub const SBE_SEMANTIC_VERSION: &str = "";

pub mod encoder {
    use super::*;

    #[derive(Debug, Default)]
    pub struct InstrumentEncoder<'a> {
        buf: WriteBuf<'a>,
        initial_offset: usize,
        offset: usize,
        limit: usize,
    }

    impl<'a> Writer<'a> for InstrumentEncoder<'a> {
        #[inline]
        fn get_buf_mut(&mut self) -> &mut WriteBuf<'a> {
            &mut self.buf
        }
    }

    impl<'a> Encoder<'a> for InstrumentEncoder<'a> {
        #[inline]
        fn get_limit(&self) -> usize {
            self.limit
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.limit = limit;
        }
    }

    impl<'a> InstrumentEncoder<'a> {
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
        pub fn kind(&mut self, value: InstrumentKind) {
            let offset = self.offset + 5;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

        
        #[inline]
        pub fn instrument_type(&mut self, value: InstrumentType) {
            let offset = self.offset + 6;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

        
        #[inline]
        pub fn option_type(&mut self, value: OptionType) {
            let offset = self.offset + 7;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

        
        #[inline]
        pub fn rfq(&mut self, value: YesNo) {
            let offset = self.offset + 8;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

        
        #[inline]
        pub fn settlement_period(&mut self, value: Period) {
            let offset = self.offset + 9;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

        
        
        
        
        
        
        
        
        #[inline]
        pub fn settlement_period_count(&mut self, value: u16) {
            let offset = self.offset + 10;
            self.get_buf_mut().put_u16_at(offset, value);
        }

        
        
        
        
        
        
        
        
        
        #[inline]
        pub fn base_currency(&mut self, value: [u8; 8]) {
            let offset = self.offset + 12;
            let buf = self.get_buf_mut();
            buf.put_u8_at(offset, value[0]);
            buf.put_u8_at(offset + 1, value[1]);
            buf.put_u8_at(offset + 2, value[2]);
            buf.put_u8_at(offset + 3, value[3]);
            buf.put_u8_at(offset + 4, value[4]);
            buf.put_u8_at(offset + 5, value[5]);
            buf.put_u8_at(offset + 6, value[6]);
            buf.put_u8_at(offset + 7, value[7]);
        }

        
        
        
        
        
        
        
        
        
        #[inline]
        pub fn quote_currency(&mut self, value: [u8; 8]) {
            let offset = self.offset + 20;
            let buf = self.get_buf_mut();
            buf.put_u8_at(offset, value[0]);
            buf.put_u8_at(offset + 1, value[1]);
            buf.put_u8_at(offset + 2, value[2]);
            buf.put_u8_at(offset + 3, value[3]);
            buf.put_u8_at(offset + 4, value[4]);
            buf.put_u8_at(offset + 5, value[5]);
            buf.put_u8_at(offset + 6, value[6]);
            buf.put_u8_at(offset + 7, value[7]);
        }

        
        
        
        
        
        
        
        
        
        #[inline]
        pub fn counter_currency(&mut self, value: [u8; 8]) {
            let offset = self.offset + 28;
            let buf = self.get_buf_mut();
            buf.put_u8_at(offset, value[0]);
            buf.put_u8_at(offset + 1, value[1]);
            buf.put_u8_at(offset + 2, value[2]);
            buf.put_u8_at(offset + 3, value[3]);
            buf.put_u8_at(offset + 4, value[4]);
            buf.put_u8_at(offset + 5, value[5]);
            buf.put_u8_at(offset + 6, value[6]);
            buf.put_u8_at(offset + 7, value[7]);
        }

        
        
        
        
        
        
        
        
        
        #[inline]
        pub fn settlement_currency(&mut self, value: [u8; 8]) {
            let offset = self.offset + 36;
            let buf = self.get_buf_mut();
            buf.put_u8_at(offset, value[0]);
            buf.put_u8_at(offset + 1, value[1]);
            buf.put_u8_at(offset + 2, value[2]);
            buf.put_u8_at(offset + 3, value[3]);
            buf.put_u8_at(offset + 4, value[4]);
            buf.put_u8_at(offset + 5, value[5]);
            buf.put_u8_at(offset + 6, value[6]);
            buf.put_u8_at(offset + 7, value[7]);
        }

        
        
        
        
        
        
        
        
        
        #[inline]
        pub fn size_currency(&mut self, value: [u8; 8]) {
            let offset = self.offset + 44;
            let buf = self.get_buf_mut();
            buf.put_u8_at(offset, value[0]);
            buf.put_u8_at(offset + 1, value[1]);
            buf.put_u8_at(offset + 2, value[2]);
            buf.put_u8_at(offset + 3, value[3]);
            buf.put_u8_at(offset + 4, value[4]);
            buf.put_u8_at(offset + 5, value[5]);
            buf.put_u8_at(offset + 6, value[6]);
            buf.put_u8_at(offset + 7, value[7]);
        }

        
        
        
        
        
        
        
        
        #[inline]
        pub fn creation_timestamp_ms(&mut self, value: u64) {
            let offset = self.offset + 52;
            self.get_buf_mut().put_u64_at(offset, value);
        }

        
        
        
        
        
        
        
        
        #[inline]
        pub fn expiration_timestamp_ms(&mut self, value: u64) {
            let offset = self.offset + 60;
            self.get_buf_mut().put_u64_at(offset, value);
        }

        
        
        
        
        
        
        
        
        #[inline]
        pub fn strike_price(&mut self, value: f64) {
            let offset = self.offset + 68;
            self.get_buf_mut().put_f64_at(offset, value);
        }

        
        
        
        
        
        
        
        
        #[inline]
        pub fn contract_size(&mut self, value: f64) {
            let offset = self.offset + 76;
            self.get_buf_mut().put_f64_at(offset, value);
        }

        
        
        
        
        
        
        
        
        #[inline]
        pub fn min_trade_amount(&mut self, value: f64) {
            let offset = self.offset + 84;
            self.get_buf_mut().put_f64_at(offset, value);
        }

        
        
        
        
        
        
        
        
        #[inline]
        pub fn tick_size(&mut self, value: f64) {
            let offset = self.offset + 92;
            self.get_buf_mut().put_f64_at(offset, value);
        }

        
        
        
        
        
        
        
        
        #[inline]
        pub fn maker_commission(&mut self, value: f64) {
            let offset = self.offset + 100;
            self.get_buf_mut().put_f64_at(offset, value);
        }

        
        
        
        
        
        
        
        
        #[inline]
        pub fn taker_commission(&mut self, value: f64) {
            let offset = self.offset + 108;
            self.get_buf_mut().put_f64_at(offset, value);
        }

        
        
        
        
        
        
        
        
        #[inline]
        pub fn block_trade_commission(&mut self, value: f64) {
            let offset = self.offset + 116;
            self.get_buf_mut().put_f64_at(offset, value);
        }

        
        
        
        
        
        
        
        
        #[inline]
        pub fn max_liquidation_commission(&mut self, value: f64) {
            let offset = self.offset + 124;
            self.get_buf_mut().put_f64_at(offset, value);
        }

        
        
        
        
        
        
        
        
        #[inline]
        pub fn max_leverage(&mut self, value: f64) {
            let offset = self.offset + 132;
            self.get_buf_mut().put_f64_at(offset, value);
        }

        
        #[inline]
        pub fn instrument_name(&mut self, value: &[u8]) {
            let limit = self.get_limit();
            let data_length = value.len();
            self.set_limit(limit + 1 + data_length);
            self.get_buf_mut().put_u8_at(limit, data_length as u8);
            self.get_buf_mut().put_slice_at(limit + 1, value);
        }

    }

} 

pub mod decoder {
    use super::*;

    #[derive(Debug, Default)]
    pub struct InstrumentDecoder<'a> {
        buf: ReadBuf<'a>,
        initial_offset: usize,
        offset: usize,
        limit: usize,
        pub acting_block_length: u16,
        pub acting_version: u16,
    }

    impl<'a> Reader<'a> for InstrumentDecoder<'a> {
        #[inline]
        fn get_buf(&self) -> &ReadBuf<'a> {
            &self.buf
        }
    }

    impl<'a> Decoder<'a> for InstrumentDecoder<'a> {
        #[inline]
        fn get_limit(&self) -> usize {
            self.limit
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.limit = limit;
        }
    }

    impl<'a> InstrumentDecoder<'a> {
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
        pub fn kind(&self) -> InstrumentKind {
            self.get_buf().get_u8_at(self.offset + 5).into()
        }

        
        #[inline]
        pub fn instrument_type(&self) -> InstrumentType {
            self.get_buf().get_u8_at(self.offset + 6).into()
        }

        
        #[inline]
        pub fn option_type(&self) -> OptionType {
            self.get_buf().get_u8_at(self.offset + 7).into()
        }

        
        #[inline]
        pub fn rfq(&self) -> YesNo {
            self.get_buf().get_u8_at(self.offset + 8).into()
        }

        
        #[inline]
        pub fn settlement_period(&self) -> Period {
            self.get_buf().get_u8_at(self.offset + 9).into()
        }

        
        #[inline]
        pub fn settlement_period_count(&self) -> u16 {
            self.get_buf().get_u16_at(self.offset + 10)
        }

        #[inline]
        pub fn base_currency(&self) -> [u8; 8] {
            let buf = self.get_buf();
            [
                buf.get_u8_at(self.offset + 12),
                buf.get_u8_at(self.offset + 12 + 1),
                buf.get_u8_at(self.offset + 12 + 2),
                buf.get_u8_at(self.offset + 12 + 3),
                buf.get_u8_at(self.offset + 12 + 4),
                buf.get_u8_at(self.offset + 12 + 5),
                buf.get_u8_at(self.offset + 12 + 6),
                buf.get_u8_at(self.offset + 12 + 7),
            ]
        }

        #[inline]
        pub fn quote_currency(&self) -> [u8; 8] {
            let buf = self.get_buf();
            [
                buf.get_u8_at(self.offset + 20),
                buf.get_u8_at(self.offset + 20 + 1),
                buf.get_u8_at(self.offset + 20 + 2),
                buf.get_u8_at(self.offset + 20 + 3),
                buf.get_u8_at(self.offset + 20 + 4),
                buf.get_u8_at(self.offset + 20 + 5),
                buf.get_u8_at(self.offset + 20 + 6),
                buf.get_u8_at(self.offset + 20 + 7),
            ]
        }

        #[inline]
        pub fn counter_currency(&self) -> [u8; 8] {
            let buf = self.get_buf();
            [
                buf.get_u8_at(self.offset + 28),
                buf.get_u8_at(self.offset + 28 + 1),
                buf.get_u8_at(self.offset + 28 + 2),
                buf.get_u8_at(self.offset + 28 + 3),
                buf.get_u8_at(self.offset + 28 + 4),
                buf.get_u8_at(self.offset + 28 + 5),
                buf.get_u8_at(self.offset + 28 + 6),
                buf.get_u8_at(self.offset + 28 + 7),
            ]
        }

        #[inline]
        pub fn settlement_currency(&self) -> [u8; 8] {
            let buf = self.get_buf();
            [
                buf.get_u8_at(self.offset + 36),
                buf.get_u8_at(self.offset + 36 + 1),
                buf.get_u8_at(self.offset + 36 + 2),
                buf.get_u8_at(self.offset + 36 + 3),
                buf.get_u8_at(self.offset + 36 + 4),
                buf.get_u8_at(self.offset + 36 + 5),
                buf.get_u8_at(self.offset + 36 + 6),
                buf.get_u8_at(self.offset + 36 + 7),
            ]
        }

        #[inline]
        pub fn size_currency(&self) -> [u8; 8] {
            let buf = self.get_buf();
            [
                buf.get_u8_at(self.offset + 44),
                buf.get_u8_at(self.offset + 44 + 1),
                buf.get_u8_at(self.offset + 44 + 2),
                buf.get_u8_at(self.offset + 44 + 3),
                buf.get_u8_at(self.offset + 44 + 4),
                buf.get_u8_at(self.offset + 44 + 5),
                buf.get_u8_at(self.offset + 44 + 6),
                buf.get_u8_at(self.offset + 44 + 7),
            ]
        }

        
        #[inline]
        pub fn creation_timestamp_ms(&self) -> u64 {
            self.get_buf().get_u64_at(self.offset + 52)
        }

        
        #[inline]
        pub fn expiration_timestamp_ms(&self) -> u64 {
            self.get_buf().get_u64_at(self.offset + 60)
        }

        
        #[inline]
        pub fn strike_price(&self) -> Option<f64> {
            let value = self.get_buf().get_f64_at(self.offset + 68);
            if value.is_nan() {
                None
            } else {
                Some(value)
            }
        }

        
        #[inline]
        pub fn contract_size(&self) -> f64 {
            self.get_buf().get_f64_at(self.offset + 76)
        }

        
        #[inline]
        pub fn min_trade_amount(&self) -> f64 {
            self.get_buf().get_f64_at(self.offset + 84)
        }

        
        #[inline]
        pub fn tick_size(&self) -> f64 {
            self.get_buf().get_f64_at(self.offset + 92)
        }

        
        #[inline]
        pub fn maker_commission(&self) -> f64 {
            self.get_buf().get_f64_at(self.offset + 100)
        }

        
        #[inline]
        pub fn taker_commission(&self) -> f64 {
            self.get_buf().get_f64_at(self.offset + 108)
        }

        
        #[inline]
        pub fn block_trade_commission(&self) -> Option<f64> {
            let value = self.get_buf().get_f64_at(self.offset + 116);
            if value.is_nan() {
                None
            } else {
                Some(value)
            }
        }

        
        #[inline]
        pub fn max_liquidation_commission(&self) -> Option<f64> {
            let value = self.get_buf().get_f64_at(self.offset + 124);
            if value.is_nan() {
                None
            } else {
                Some(value)
            }
        }

        
        #[inline]
        pub fn max_leverage(&self) -> Option<f64> {
            let value = self.get_buf().get_f64_at(self.offset + 132);
            if value.is_nan() {
                None
            } else {
                Some(value)
            }
        }

        
        #[inline]
        pub fn instrument_name_decoder(&mut self) -> (usize, usize) {
            let offset = self.get_limit();
            let data_length = self.get_buf().get_u8_at(offset) as usize;
            self.set_limit(offset + 1 + data_length);
            (offset + 1, data_length)
        }

        #[inline]
        pub fn instrument_name_slice(&'a self, coordinates: (usize, usize)) -> &'a [u8] {
            debug_assert!(self.get_limit() >= coordinates.0 + coordinates.1);
            self.get_buf().get_slice_at(coordinates.0, coordinates.1)
        }

    }

} 

