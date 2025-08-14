use crate::*;

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

        /// primitive field 'instrumentId'
        /// - min value: 0
        /// - max value: 4294967294
        /// - null value: 4294967295
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 0
        /// - encodedLength: 4
        #[inline]
        pub fn instrument_id(&mut self, value: u32) {
            let offset = self.offset;
            self.get_buf_mut().put_u32_at(offset, value);
        }

        /// REQUIRED enum
        #[inline]
        pub fn instrument_state(&mut self, value: InstrumentState) {
            let offset = self.offset + 4;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

        /// REQUIRED enum
        #[inline]
        pub fn kind(&mut self, value: InstrumentKind) {
            let offset = self.offset + 5;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

        /// REQUIRED enum
        #[inline]
        pub fn instrument_type(&mut self, value: InstrumentType) {
            let offset = self.offset + 6;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

        /// REQUIRED enum
        #[inline]
        pub fn option_type(&mut self, value: OptionType) {
            let offset = self.offset + 7;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

        /// REQUIRED enum
        #[inline]
        pub fn rfq(&mut self, value: YesNo) {
            let offset = self.offset + 8;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

        /// REQUIRED enum
        #[inline]
        pub fn settlement_period(&mut self, value: Period) {
            let offset = self.offset + 9;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

        /// primitive field 'settlementPeriodCount'
        /// - min value: 0
        /// - max value: 65534
        /// - null value: 65535
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 10
        /// - encodedLength: 2
        #[inline]
        pub fn settlement_period_count(&mut self, value: u16) {
            let offset = self.offset + 10;
            self.get_buf_mut().put_u16_at(offset, value);
        }

        /// primitive array field 'baseCurrency'
        /// - min value: 32
        /// - max value: 126
        /// - null value: 0
        /// - characterEncoding: US-ASCII
        /// - semanticType: null
        /// - encodedOffset: 12
        /// - encodedLength: 8
        /// - version: 0
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

        /// primitive array field 'quoteCurrency'
        /// - min value: 32
        /// - max value: 126
        /// - null value: 0
        /// - characterEncoding: US-ASCII
        /// - semanticType: null
        /// - encodedOffset: 20
        /// - encodedLength: 8
        /// - version: 0
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

        /// primitive array field 'counterCurrency'
        /// - min value: 32
        /// - max value: 126
        /// - null value: 0
        /// - characterEncoding: US-ASCII
        /// - semanticType: null
        /// - encodedOffset: 28
        /// - encodedLength: 8
        /// - version: 0
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

        /// primitive array field 'settlementCurrency'
        /// - min value: 32
        /// - max value: 126
        /// - null value: 0
        /// - characterEncoding: US-ASCII
        /// - semanticType: null
        /// - encodedOffset: 36
        /// - encodedLength: 8
        /// - version: 0
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

        /// primitive array field 'sizeCurrency'
        /// - min value: 32
        /// - max value: 126
        /// - null value: 0
        /// - characterEncoding: US-ASCII
        /// - semanticType: null
        /// - encodedOffset: 44
        /// - encodedLength: 8
        /// - version: 0
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

        /// primitive field 'creationTimestampMs'
        /// - min value: 0
        /// - max value: -2
        /// - null value: -1
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 52
        /// - encodedLength: 8
        #[inline]
        pub fn creation_timestamp_ms(&mut self, value: u64) {
            let offset = self.offset + 52;
            self.get_buf_mut().put_u64_at(offset, value);
        }

        /// primitive field 'expirationTimestampMs'
        /// - min value: 0
        /// - max value: -2
        /// - null value: -1
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 60
        /// - encodedLength: 8
        #[inline]
        pub fn expiration_timestamp_ms(&mut self, value: u64) {
            let offset = self.offset + 60;
            self.get_buf_mut().put_u64_at(offset, value);
        }

        /// primitive field 'strikePrice'
        /// - min value: 4.9E-324
        /// - max value: 1.7976931348623157E308
        /// - null value: NaN
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 68
        /// - encodedLength: 8
        #[inline]
        pub fn strike_price(&mut self, value: f64) {
            let offset = self.offset + 68;
            self.get_buf_mut().put_f64_at(offset, value);
        }

        /// primitive field 'contractSize'
        /// - min value: 4.9E-324
        /// - max value: 1.7976931348623157E308
        /// - null value: NaN
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 76
        /// - encodedLength: 8
        #[inline]
        pub fn contract_size(&mut self, value: f64) {
            let offset = self.offset + 76;
            self.get_buf_mut().put_f64_at(offset, value);
        }

        /// primitive field 'minTradeAmount'
        /// - min value: 4.9E-324
        /// - max value: 1.7976931348623157E308
        /// - null value: NaN
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 84
        /// - encodedLength: 8
        #[inline]
        pub fn min_trade_amount(&mut self, value: f64) {
            let offset = self.offset + 84;
            self.get_buf_mut().put_f64_at(offset, value);
        }

        /// primitive field 'tickSize'
        /// - min value: 4.9E-324
        /// - max value: 1.7976931348623157E308
        /// - null value: NaN
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 92
        /// - encodedLength: 8
        #[inline]
        pub fn tick_size(&mut self, value: f64) {
            let offset = self.offset + 92;
            self.get_buf_mut().put_f64_at(offset, value);
        }

        /// primitive field 'makerCommission'
        /// - min value: 4.9E-324
        /// - max value: 1.7976931348623157E308
        /// - null value: NaN
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 100
        /// - encodedLength: 8
        #[inline]
        pub fn maker_commission(&mut self, value: f64) {
            let offset = self.offset + 100;
            self.get_buf_mut().put_f64_at(offset, value);
        }

        /// primitive field 'takerCommission'
        /// - min value: 4.9E-324
        /// - max value: 1.7976931348623157E308
        /// - null value: NaN
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 108
        /// - encodedLength: 8
        #[inline]
        pub fn taker_commission(&mut self, value: f64) {
            let offset = self.offset + 108;
            self.get_buf_mut().put_f64_at(offset, value);
        }

        /// primitive field 'blockTradeCommission'
        /// - min value: 4.9E-324
        /// - max value: 1.7976931348623157E308
        /// - null value: NaN
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 116
        /// - encodedLength: 8
        #[inline]
        pub fn block_trade_commission(&mut self, value: f64) {
            let offset = self.offset + 116;
            self.get_buf_mut().put_f64_at(offset, value);
        }

        /// primitive field 'maxLiquidationCommission'
        /// - min value: 4.9E-324
        /// - max value: 1.7976931348623157E308
        /// - null value: NaN
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 124
        /// - encodedLength: 8
        #[inline]
        pub fn max_liquidation_commission(&mut self, value: f64) {
            let offset = self.offset + 124;
            self.get_buf_mut().put_f64_at(offset, value);
        }

        /// primitive field 'maxLeverage'
        /// - min value: 4.9E-324
        /// - max value: 1.7976931348623157E308
        /// - null value: NaN
        /// - characterEncoding: null
        /// - semanticType: null
        /// - encodedOffset: 132
        /// - encodedLength: 8
        #[inline]
        pub fn max_leverage(&mut self, value: f64) {
            let offset = self.offset + 132;
            self.get_buf_mut().put_f64_at(offset, value);
        }

        /// VAR_DATA ENCODER - character encoding: 'None'
        #[inline]
        pub fn instrument_name(&mut self, value: &[u8]) {
            let limit = self.get_limit();
            let data_length = value.len();
            self.set_limit(limit + 1 + data_length);
            self.get_buf_mut().put_u8_at(limit, data_length as u8);
            self.get_buf_mut().put_slice_at(limit + 1, value);
        }

    }

} // end encoder

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

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn instrument_id(&self) -> u32 {
            self.get_buf().get_u32_at(self.offset)
        }

        /// REQUIRED enum
        #[inline]
        pub fn instrument_state(&self) -> InstrumentState {
            self.get_buf().get_u8_at(self.offset + 4).into()
        }

        /// REQUIRED enum
        #[inline]
        pub fn kind(&self) -> InstrumentKind {
            self.get_buf().get_u8_at(self.offset + 5).into()
        }

        /// REQUIRED enum
        #[inline]
        pub fn instrument_type(&self) -> InstrumentType {
            self.get_buf().get_u8_at(self.offset + 6).into()
        }

        /// REQUIRED enum
        #[inline]
        pub fn option_type(&self) -> OptionType {
            self.get_buf().get_u8_at(self.offset + 7).into()
        }

        /// REQUIRED enum
        #[inline]
        pub fn rfq(&self) -> YesNo {
            self.get_buf().get_u8_at(self.offset + 8).into()
        }

        /// REQUIRED enum
        #[inline]
        pub fn settlement_period(&self) -> Period {
            self.get_buf().get_u8_at(self.offset + 9).into()
        }

        /// primitive field - 'REQUIRED'
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

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn creation_timestamp_ms(&self) -> u64 {
            self.get_buf().get_u64_at(self.offset + 52)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn expiration_timestamp_ms(&self) -> u64 {
            self.get_buf().get_u64_at(self.offset + 60)
        }

        /// primitive field - 'OPTIONAL' { null_value: 'NaN' }
        #[inline]
        pub fn strike_price(&self) -> Option<f64> {
            let value = self.get_buf().get_f64_at(self.offset + 68);
            if value.is_nan() {
                None
            } else {
                Some(value)
            }
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn contract_size(&self) -> f64 {
            self.get_buf().get_f64_at(self.offset + 76)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn min_trade_amount(&self) -> f64 {
            self.get_buf().get_f64_at(self.offset + 84)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn tick_size(&self) -> f64 {
            self.get_buf().get_f64_at(self.offset + 92)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn maker_commission(&self) -> f64 {
            self.get_buf().get_f64_at(self.offset + 100)
        }

        /// primitive field - 'REQUIRED'
        #[inline]
        pub fn taker_commission(&self) -> f64 {
            self.get_buf().get_f64_at(self.offset + 108)
        }

        /// primitive field - 'OPTIONAL' { null_value: 'NaN' }
        #[inline]
        pub fn block_trade_commission(&self) -> Option<f64> {
            let value = self.get_buf().get_f64_at(self.offset + 116);
            if value.is_nan() {
                None
            } else {
                Some(value)
            }
        }

        /// primitive field - 'OPTIONAL' { null_value: 'NaN' }
        #[inline]
        pub fn max_liquidation_commission(&self) -> Option<f64> {
            let value = self.get_buf().get_f64_at(self.offset + 124);
            if value.is_nan() {
                None
            } else {
                Some(value)
            }
        }

        /// primitive field - 'OPTIONAL' { null_value: 'NaN' }
        #[inline]
        pub fn max_leverage(&self) -> Option<f64> {
            let value = self.get_buf().get_f64_at(self.offset + 132);
            if value.is_nan() {
                None
            } else {
                Some(value)
            }
        }

        /// VAR_DATA DECODER - character encoding: 'None'
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

} // end decoder

