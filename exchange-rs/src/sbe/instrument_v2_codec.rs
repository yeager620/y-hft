use crate::sbe::*;

pub use encoder::*;
pub use decoder::*;

pub const SBE_BLOCK_LENGTH: u16 = 139;
pub const SBE_TEMPLATE_ID: u16 = 1010;
pub const SBE_SCHEMA_ID: u16 = 1;
pub const SBE_SCHEMA_VERSION: u16 = 3;
pub const SBE_SEMANTIC_VERSION: &str = "";

pub mod encoder {
    use super::*;

    #[derive(Debug, Default)]
    pub struct InstrumentV2Encoder<'a> {
        buf: WriteBuf<'a>,
        initial_offset: usize,
        offset: usize,
        limit: usize,
    }

    impl<'a> Writer<'a> for InstrumentV2Encoder<'a> {
        #[inline]
        fn get_buf_mut(&mut self) -> &mut WriteBuf<'a> {
            &mut self.buf
        }
    }

    impl<'a> Encoder<'a> for InstrumentV2Encoder<'a> {
        #[inline]
        fn get_limit(&self) -> usize {
            self.limit
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.limit = limit;
        }
    }

    impl<'a> InstrumentV2Encoder<'a> {
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
        pub fn settlement_period(&mut self, value: Period) {
            let offset = self.offset + 8;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

        
        
        
        
        
        
        
        
        #[inline]
        pub fn settlement_period_count(&mut self, value: u16) {
            let offset = self.offset + 9;
            self.get_buf_mut().put_u16_at(offset, value);
        }

        
        
        
        
        
        
        
        
        
        #[inline]
        pub fn base_currency(&mut self, value: [u8; 8]) {
            let offset = self.offset + 11;
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
            let offset = self.offset + 19;
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
            let offset = self.offset + 27;
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
            let offset = self.offset + 35;
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
            let offset = self.offset + 43;
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
            let offset = self.offset + 51;
            self.get_buf_mut().put_u64_at(offset, value);
        }

        
        
        
        
        
        
        
        
        #[inline]
        pub fn expiration_timestamp_ms(&mut self, value: u64) {
            let offset = self.offset + 59;
            self.get_buf_mut().put_u64_at(offset, value);
        }

        
        
        
        
        
        
        
        
        #[inline]
        pub fn strike_price(&mut self, value: f64) {
            let offset = self.offset + 67;
            self.get_buf_mut().put_f64_at(offset, value);
        }

        
        
        
        
        
        
        
        
        #[inline]
        pub fn contract_size(&mut self, value: f64) {
            let offset = self.offset + 75;
            self.get_buf_mut().put_f64_at(offset, value);
        }

        
        
        
        
        
        
        
        
        #[inline]
        pub fn min_trade_amount(&mut self, value: f64) {
            let offset = self.offset + 83;
            self.get_buf_mut().put_f64_at(offset, value);
        }

        
        
        
        
        
        
        
        
        #[inline]
        pub fn tick_size(&mut self, value: f64) {
            let offset = self.offset + 91;
            self.get_buf_mut().put_f64_at(offset, value);
        }

        
        
        
        
        
        
        
        
        #[inline]
        pub fn maker_commission(&mut self, value: f64) {
            let offset = self.offset + 99;
            self.get_buf_mut().put_f64_at(offset, value);
        }

        
        
        
        
        
        
        
        
        #[inline]
        pub fn taker_commission(&mut self, value: f64) {
            let offset = self.offset + 107;
            self.get_buf_mut().put_f64_at(offset, value);
        }

        
        
        
        
        
        
        
        
        #[inline]
        pub fn block_trade_commission(&mut self, value: f64) {
            let offset = self.offset + 115;
            self.get_buf_mut().put_f64_at(offset, value);
        }

        
        
        
        
        
        
        
        
        #[inline]
        pub fn max_liquidation_commission(&mut self, value: f64) {
            let offset = self.offset + 123;
            self.get_buf_mut().put_f64_at(offset, value);
        }

        
        
        
        
        
        
        
        
        #[inline]
        pub fn max_leverage(&mut self, value: f64) {
            let offset = self.offset + 131;
            self.get_buf_mut().put_f64_at(offset, value);
        }

        
        #[inline]
        pub fn tick_steps_list_encoder(self, count: u16, tick_steps_list_encoder: TickStepsListEncoder<Self>) -> TickStepsListEncoder<Self> {
            tick_steps_list_encoder.wrap(self, count)
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

    #[derive(Debug, Default)]
    pub struct TickStepsListEncoder<P> {
        parent: Option<P>,
        count: u16,
        index: usize,
        offset: usize,
        initial_limit: usize,
    }

    impl<'a, P> Writer<'a> for TickStepsListEncoder<P> where P: Writer<'a> + Default {
        #[inline]
        fn get_buf_mut(&mut self) -> &mut WriteBuf<'a> {
            if let Some(parent) = self.parent.as_mut() {
                parent.get_buf_mut()
            } else {
                panic!("parent was None")
            }
        }
    }

    impl<'a, P> Encoder<'a> for TickStepsListEncoder<P> where P: Encoder<'a> + Default {
        #[inline]
        fn get_limit(&self) -> usize {
            self.parent.as_ref().expect("parent missing").get_limit()
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.parent.as_mut().expect("parent missing").set_limit(limit);
        }
    }

    impl<'a, P> TickStepsListEncoder<P> where P: Encoder<'a> + Default {
        #[inline]
        pub fn wrap(
            mut self,
            mut parent: P,
            count: u16,
        ) -> Self {
            let initial_limit = parent.get_limit();
            parent.set_limit(initial_limit + 8);
            parent.get_buf_mut().put_u16_at(initial_limit, Self::block_length());
            parent.get_buf_mut().put_u16_at(initial_limit + 2, count);
            self.parent = Some(parent);
            self.count = count;
            self.index = usize::MAX;
            self.offset = usize::MAX;
            self.initial_limit = initial_limit;
            self
        }

        #[inline]
        pub fn block_length() -> u16 {
            16
        }

        #[inline]
        pub fn parent(&mut self) -> SbeResult<P> {
            self.parent.take().ok_or(SbeErr::ParentNotSet)
        }

        
        #[inline]
        pub fn advance(&mut self) -> SbeResult<Option<usize>> {
            let index = self.index.wrapping_add(1);
            if index >= self.count as usize {
                return Ok(None);
            }
            if let Some(parent) = self.parent.as_mut() {
                self.offset = parent.get_limit();
                parent.set_limit(self.offset + Self::block_length() as usize);
                self.index = index;
                Ok(Some(index))
            } else {
                Err(SbeErr::ParentNotSet)
            }
        }

        
        
        
        
        
        
        
        
        #[inline]
        pub fn above_price(&mut self, value: f64) {
            let offset = self.offset;
            self.get_buf_mut().put_f64_at(offset, value);
        }

        
        
        
        
        
        
        
        
        #[inline]
        pub fn tick_size(&mut self, value: f64) {
            let offset = self.offset + 8;
            self.get_buf_mut().put_f64_at(offset, value);
        }

    }

} 

pub mod decoder {
    use super::*;

    #[derive(Debug, Default)]
    pub struct InstrumentV2Decoder<'a> {
        buf: ReadBuf<'a>,
        initial_offset: usize,
        offset: usize,
        limit: usize,
        pub acting_block_length: u16,
        pub acting_version: u16,
    }

    impl<'a> Reader<'a> for InstrumentV2Decoder<'a> {
        #[inline]
        fn get_buf(&self) -> &ReadBuf<'a> {
            &self.buf
        }
    }

    impl<'a> Decoder<'a> for InstrumentV2Decoder<'a> {
        #[inline]
        fn get_limit(&self) -> usize {
            self.limit
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.limit = limit;
        }
    }

    impl<'a> InstrumentV2Decoder<'a> {
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
        pub fn settlement_period(&self) -> Period {
            self.get_buf().get_u8_at(self.offset + 8).into()
        }

        
        #[inline]
        pub fn settlement_period_count(&self) -> u16 {
            self.get_buf().get_u16_at(self.offset + 9)
        }

        #[inline]
        pub fn base_currency(&self) -> [u8; 8] {
            let buf = self.get_buf();
            [
                buf.get_u8_at(self.offset + 11),
                buf.get_u8_at(self.offset + 11 + 1),
                buf.get_u8_at(self.offset + 11 + 2),
                buf.get_u8_at(self.offset + 11 + 3),
                buf.get_u8_at(self.offset + 11 + 4),
                buf.get_u8_at(self.offset + 11 + 5),
                buf.get_u8_at(self.offset + 11 + 6),
                buf.get_u8_at(self.offset + 11 + 7),
            ]
        }

        #[inline]
        pub fn quote_currency(&self) -> [u8; 8] {
            let buf = self.get_buf();
            [
                buf.get_u8_at(self.offset + 19),
                buf.get_u8_at(self.offset + 19 + 1),
                buf.get_u8_at(self.offset + 19 + 2),
                buf.get_u8_at(self.offset + 19 + 3),
                buf.get_u8_at(self.offset + 19 + 4),
                buf.get_u8_at(self.offset + 19 + 5),
                buf.get_u8_at(self.offset + 19 + 6),
                buf.get_u8_at(self.offset + 19 + 7),
            ]
        }

        #[inline]
        pub fn counter_currency(&self) -> [u8; 8] {
            let buf = self.get_buf();
            [
                buf.get_u8_at(self.offset + 27),
                buf.get_u8_at(self.offset + 27 + 1),
                buf.get_u8_at(self.offset + 27 + 2),
                buf.get_u8_at(self.offset + 27 + 3),
                buf.get_u8_at(self.offset + 27 + 4),
                buf.get_u8_at(self.offset + 27 + 5),
                buf.get_u8_at(self.offset + 27 + 6),
                buf.get_u8_at(self.offset + 27 + 7),
            ]
        }

        #[inline]
        pub fn settlement_currency(&self) -> [u8; 8] {
            let buf = self.get_buf();
            [
                buf.get_u8_at(self.offset + 35),
                buf.get_u8_at(self.offset + 35 + 1),
                buf.get_u8_at(self.offset + 35 + 2),
                buf.get_u8_at(self.offset + 35 + 3),
                buf.get_u8_at(self.offset + 35 + 4),
                buf.get_u8_at(self.offset + 35 + 5),
                buf.get_u8_at(self.offset + 35 + 6),
                buf.get_u8_at(self.offset + 35 + 7),
            ]
        }

        #[inline]
        pub fn size_currency(&self) -> [u8; 8] {
            let buf = self.get_buf();
            [
                buf.get_u8_at(self.offset + 43),
                buf.get_u8_at(self.offset + 43 + 1),
                buf.get_u8_at(self.offset + 43 + 2),
                buf.get_u8_at(self.offset + 43 + 3),
                buf.get_u8_at(self.offset + 43 + 4),
                buf.get_u8_at(self.offset + 43 + 5),
                buf.get_u8_at(self.offset + 43 + 6),
                buf.get_u8_at(self.offset + 43 + 7),
            ]
        }

        
        #[inline]
        pub fn creation_timestamp_ms(&self) -> u64 {
            self.get_buf().get_u64_at(self.offset + 51)
        }

        
        #[inline]
        pub fn expiration_timestamp_ms(&self) -> u64 {
            self.get_buf().get_u64_at(self.offset + 59)
        }

        
        #[inline]
        pub fn strike_price(&self) -> Option<f64> {
            let value = self.get_buf().get_f64_at(self.offset + 67);
            if value.is_nan() {
                None
            } else {
                Some(value)
            }
        }

        
        #[inline]
        pub fn contract_size(&self) -> f64 {
            self.get_buf().get_f64_at(self.offset + 75)
        }

        
        #[inline]
        pub fn min_trade_amount(&self) -> f64 {
            self.get_buf().get_f64_at(self.offset + 83)
        }

        
        #[inline]
        pub fn tick_size(&self) -> f64 {
            self.get_buf().get_f64_at(self.offset + 91)
        }

        
        #[inline]
        pub fn maker_commission(&self) -> f64 {
            self.get_buf().get_f64_at(self.offset + 99)
        }

        
        #[inline]
        pub fn taker_commission(&self) -> f64 {
            self.get_buf().get_f64_at(self.offset + 107)
        }

        
        #[inline]
        pub fn block_trade_commission(&self) -> Option<f64> {
            let value = self.get_buf().get_f64_at(self.offset + 115);
            if value.is_nan() {
                None
            } else {
                Some(value)
            }
        }

        
        #[inline]
        pub fn max_liquidation_commission(&self) -> Option<f64> {
            let value = self.get_buf().get_f64_at(self.offset + 123);
            if value.is_nan() {
                None
            } else {
                Some(value)
            }
        }

        
        #[inline]
        pub fn max_leverage(&self) -> Option<f64> {
            let value = self.get_buf().get_f64_at(self.offset + 131);
            if value.is_nan() {
                None
            } else {
                Some(value)
            }
        }

        
        #[inline]
        pub fn tick_steps_list_decoder(self) -> TickStepsListDecoder<Self> {
            let acting_version = self.acting_version;
            TickStepsListDecoder::default().wrap(self, acting_version as usize)
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

    #[derive(Debug, Default)]
    pub struct TickStepsListDecoder<P> {
        parent: Option<P>,
        block_length: usize,
        acting_version: usize,
        count: u16,
        index: usize,
        offset: usize,
    }

    impl<'a, P> Reader<'a> for TickStepsListDecoder<P> where P: Reader<'a> + Default {
        #[inline]
        fn get_buf(&self) -> &ReadBuf<'a> {
            self.parent.as_ref().expect("parent missing").get_buf()
        }
    }

    impl<'a, P> Decoder<'a> for TickStepsListDecoder<P> where P: Decoder<'a> + Default {
        #[inline]
        fn get_limit(&self) -> usize {
            self.parent.as_ref().expect("parent missing").get_limit()
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.parent.as_mut().expect("parent missing").set_limit(limit);
        }
    }

    impl<'a, P> TickStepsListDecoder<P> where P: Decoder<'a> + Default {
        pub fn wrap(
            mut self,
            mut parent: P,
            acting_version: usize,
        ) -> Self {
            let initial_offset = parent.get_limit();
            let block_length = parent.get_buf().get_u16_at(initial_offset) as usize;
            let count = parent.get_buf().get_u16_at(initial_offset + 2);
            parent.set_limit(initial_offset + 8);
            self.parent = Some(parent);
            self.block_length = block_length;
            self.acting_version = acting_version;
            self.count = count;
            self.index = usize::MAX;
            self.offset = 0;
            self
        }

        
        #[inline]
        pub fn parent(&mut self) -> SbeResult<P> {
            self.parent.take().ok_or(SbeErr::ParentNotSet)
        }

        #[inline]
        pub fn count(&self) -> u16 {
            self.count
        }

        
        pub fn advance(&mut self) -> SbeResult<Option<usize>> {
            let index = self.index.wrapping_add(1);
            if index >= self.count as usize {
                 return Ok(None);
            }
            if let Some(parent) = self.parent.as_mut() {
                self.offset = parent.get_limit();
                parent.set_limit(self.offset + self.block_length as usize);
                self.index = index;
                Ok(Some(index))
            } else {
                Err(SbeErr::ParentNotSet)
            }
        }

        
        #[inline]
        pub fn above_price(&self) -> f64 {
            self.get_buf().get_f64_at(self.offset)
        }

        
        #[inline]
        pub fn tick_size(&self) -> f64 {
            self.get_buf().get_f64_at(self.offset + 8)
        }

    }

} 

