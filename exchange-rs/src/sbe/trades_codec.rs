use crate::sbe::*;

pub use encoder::*;
pub use decoder::*;

pub const SBE_BLOCK_LENGTH: u16 = 4;
pub const SBE_TEMPLATE_ID: u16 = 1002;
pub const SBE_SCHEMA_ID: u16 = 1;
pub const SBE_SCHEMA_VERSION: u16 = 3;
pub const SBE_SEMANTIC_VERSION: &str = "";

pub mod encoder {
    use super::*;

    #[derive(Debug, Default)]
    pub struct TradesEncoder<'a> {
        buf: WriteBuf<'a>,
        initial_offset: usize,
        offset: usize,
        limit: usize,
    }

    impl<'a> Writer<'a> for TradesEncoder<'a> {
        #[inline]
        fn get_buf_mut(&mut self) -> &mut WriteBuf<'a> {
            &mut self.buf
        }
    }

    impl<'a> Encoder<'a> for TradesEncoder<'a> {
        #[inline]
        fn get_limit(&self) -> usize {
            self.limit
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.limit = limit;
        }
    }

    impl<'a> TradesEncoder<'a> {
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
        pub fn trades_list_encoder(self, count: u16, trades_list_encoder: TradesListEncoder<Self>) -> TradesListEncoder<Self> {
            trades_list_encoder.wrap(self, count)
        }

    }

    #[derive(Debug, Default)]
    pub struct TradesListEncoder<P> {
        parent: Option<P>,
        count: u16,
        index: usize,
        offset: usize,
        initial_limit: usize,
    }

    impl<'a, P> Writer<'a> for TradesListEncoder<P> where P: Writer<'a> + Default {
        #[inline]
        fn get_buf_mut(&mut self) -> &mut WriteBuf<'a> {
            if let Some(parent) = self.parent.as_mut() {
                parent.get_buf_mut()
            } else {
                panic!("parent was None")
            }
        }
    }

    impl<'a, P> Encoder<'a> for TradesListEncoder<P> where P: Encoder<'a> + Default {
        #[inline]
        fn get_limit(&self) -> usize {
            self.parent.as_ref().expect("parent missing").get_limit()
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.parent.as_mut().expect("parent missing").set_limit(limit);
        }
    }

    impl<'a, P> TradesListEncoder<P> where P: Encoder<'a> + Default {
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
            83
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
        pub fn direction(&mut self, value: Direction) {
            let offset = self.offset;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

        
        
        
        
        
        
        
        
        #[inline]
        pub fn price(&mut self, value: f64) {
            let offset = self.offset + 1;
            self.get_buf_mut().put_f64_at(offset, value);
        }

        
        
        
        
        
        
        
        
        #[inline]
        pub fn amount(&mut self, value: f64) {
            let offset = self.offset + 9;
            self.get_buf_mut().put_f64_at(offset, value);
        }

        
        
        
        
        
        
        
        
        #[inline]
        pub fn timestamp_ms(&mut self, value: u64) {
            let offset = self.offset + 17;
            self.get_buf_mut().put_u64_at(offset, value);
        }

        
        
        
        
        
        
        
        
        #[inline]
        pub fn mark_price(&mut self, value: f64) {
            let offset = self.offset + 25;
            self.get_buf_mut().put_f64_at(offset, value);
        }

        
        
        
        
        
        
        
        
        #[inline]
        pub fn index_price(&mut self, value: f64) {
            let offset = self.offset + 33;
            self.get_buf_mut().put_f64_at(offset, value);
        }

        
        
        
        
        
        
        
        
        #[inline]
        pub fn trade_seq(&mut self, value: u64) {
            let offset = self.offset + 41;
            self.get_buf_mut().put_u64_at(offset, value);
        }

        
        
        
        
        
        
        
        
        #[inline]
        pub fn trade_id(&mut self, value: u64) {
            let offset = self.offset + 49;
            self.get_buf_mut().put_u64_at(offset, value);
        }

        
        #[inline]
        pub fn tick_direction(&mut self, value: TickDirection) {
            let offset = self.offset + 57;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

        
        #[inline]
        pub fn liquidation(&mut self, value: Liquidation) {
            let offset = self.offset + 58;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

        
        
        
        
        
        
        
        
        #[inline]
        pub fn iv(&mut self, value: f64) {
            let offset = self.offset + 59;
            self.get_buf_mut().put_f64_at(offset, value);
        }

        
        
        
        
        
        
        
        
        #[inline]
        pub fn block_trade_id(&mut self, value: u64) {
            let offset = self.offset + 67;
            self.get_buf_mut().put_u64_at(offset, value);
        }

        
        
        
        
        
        
        
        
        #[inline]
        pub fn combo_trade_id(&mut self, value: u64) {
            let offset = self.offset + 75;
            self.get_buf_mut().put_u64_at(offset, value);
        }

    }

} 

pub mod decoder {
    use super::*;

    #[derive(Debug, Default)]
    pub struct TradesDecoder<'a> {
        buf: ReadBuf<'a>,
        initial_offset: usize,
        offset: usize,
        limit: usize,
        pub acting_block_length: u16,
        pub acting_version: u16,
    }

    impl<'a> Reader<'a> for TradesDecoder<'a> {
        #[inline]
        fn get_buf(&self) -> &ReadBuf<'a> {
            &self.buf
        }
    }

    impl<'a> Decoder<'a> for TradesDecoder<'a> {
        #[inline]
        fn get_limit(&self) -> usize {
            self.limit
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.limit = limit;
        }
    }

    impl<'a> TradesDecoder<'a> {
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
        pub fn trades_list_decoder(self) -> TradesListDecoder<Self> {
            let acting_version = self.acting_version;
            TradesListDecoder::default().wrap(self, acting_version as usize)
        }

    }

    #[derive(Debug, Default)]
    pub struct TradesListDecoder<P> {
        parent: Option<P>,
        block_length: usize,
        acting_version: usize,
        count: u16,
        index: usize,
        offset: usize,
    }

    impl<'a, P> Reader<'a> for TradesListDecoder<P> where P: Reader<'a> + Default {
        #[inline]
        fn get_buf(&self) -> &ReadBuf<'a> {
            self.parent.as_ref().expect("parent missing").get_buf()
        }
    }

    impl<'a, P> Decoder<'a> for TradesListDecoder<P> where P: Decoder<'a> + Default {
        #[inline]
        fn get_limit(&self) -> usize {
            self.parent.as_ref().expect("parent missing").get_limit()
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.parent.as_mut().expect("parent missing").set_limit(limit);
        }
    }

    impl<'a, P> TradesListDecoder<P> where P: Decoder<'a> + Default {
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
        pub fn direction(&self) -> Direction {
            self.get_buf().get_u8_at(self.offset).into()
        }

        
        #[inline]
        pub fn price(&self) -> f64 {
            self.get_buf().get_f64_at(self.offset + 1)
        }

        
        #[inline]
        pub fn amount(&self) -> f64 {
            self.get_buf().get_f64_at(self.offset + 9)
        }

        
        #[inline]
        pub fn timestamp_ms(&self) -> u64 {
            self.get_buf().get_u64_at(self.offset + 17)
        }

        
        #[inline]
        pub fn mark_price(&self) -> f64 {
            self.get_buf().get_f64_at(self.offset + 25)
        }

        
        #[inline]
        pub fn index_price(&self) -> f64 {
            self.get_buf().get_f64_at(self.offset + 33)
        }

        
        #[inline]
        pub fn trade_seq(&self) -> u64 {
            self.get_buf().get_u64_at(self.offset + 41)
        }

        
        #[inline]
        pub fn trade_id(&self) -> u64 {
            self.get_buf().get_u64_at(self.offset + 49)
        }

        
        #[inline]
        pub fn tick_direction(&self) -> TickDirection {
            self.get_buf().get_u8_at(self.offset + 57).into()
        }

        
        #[inline]
        pub fn liquidation(&self) -> Liquidation {
            self.get_buf().get_u8_at(self.offset + 58).into()
        }

        
        #[inline]
        pub fn iv(&self) -> Option<f64> {
            let value = self.get_buf().get_f64_at(self.offset + 59);
            if value.is_nan() {
                None
            } else {
                Some(value)
            }
        }

        
        #[inline]
        pub fn block_trade_id(&self) -> Option<u64> {
            let value = self.get_buf().get_u64_at(self.offset + 67);
            if value == 0xffffffffffffffff_u64 {
                None
            } else {
                Some(value)
            }
        }

        
        #[inline]
        pub fn combo_trade_id(&self) -> Option<u64> {
            let value = self.get_buf().get_u64_at(self.offset + 75);
            if value == 0xffffffffffffffff_u64 {
                None
            } else {
                Some(value)
            }
        }

    }

} 

