use crate::sbe::*;

pub use encoder::*;
pub use decoder::*;

pub const SBE_BLOCK_LENGTH: u16 = 22;
pub const SBE_TEMPLATE_ID: u16 = 1009;
pub const SBE_SCHEMA_ID: u16 = 1;
pub const SBE_SCHEMA_VERSION: u16 = 3;
pub const SBE_SEMANTIC_VERSION: &str = "";

pub mod encoder {
    use super::*;

    #[derive(Debug, Default)]
    pub struct RfqEncoder<'a> {
        buf: WriteBuf<'a>,
        initial_offset: usize,
        offset: usize,
        limit: usize,
    }

    impl<'a> Writer<'a> for RfqEncoder<'a> {
        #[inline]
        fn get_buf_mut(&mut self) -> &mut WriteBuf<'a> {
            &mut self.buf
        }
    }

    impl<'a> Encoder<'a> for RfqEncoder<'a> {
        #[inline]
        fn get_limit(&self) -> usize {
            self.limit
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.limit = limit;
        }
    }

    impl<'a> RfqEncoder<'a> {
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
        pub fn state(&mut self, value: YesNo) {
            let offset = self.offset + 4;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

        
        #[inline]
        pub fn side(&mut self, value: RfqDirection) {
            let offset = self.offset + 5;
            self.get_buf_mut().put_u8_at(offset, value as u8)
        }

        
        
        
        
        
        
        
        
        #[inline]
        pub fn amount(&mut self, value: f64) {
            let offset = self.offset + 6;
            self.get_buf_mut().put_f64_at(offset, value);
        }

        
        
        
        
        
        
        
        
        #[inline]
        pub fn timestamp_ms(&mut self, value: u64) {
            let offset = self.offset + 14;
            self.get_buf_mut().put_u64_at(offset, value);
        }

    }

} 

pub mod decoder {
    use super::*;

    #[derive(Debug, Default)]
    pub struct RfqDecoder<'a> {
        buf: ReadBuf<'a>,
        initial_offset: usize,
        offset: usize,
        limit: usize,
        pub acting_block_length: u16,
        pub acting_version: u16,
    }

    impl<'a> Reader<'a> for RfqDecoder<'a> {
        #[inline]
        fn get_buf(&self) -> &ReadBuf<'a> {
            &self.buf
        }
    }

    impl<'a> Decoder<'a> for RfqDecoder<'a> {
        #[inline]
        fn get_limit(&self) -> usize {
            self.limit
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.limit = limit;
        }
    }

    impl<'a> RfqDecoder<'a> {
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
        pub fn state(&self) -> YesNo {
            self.get_buf().get_u8_at(self.offset + 4).into()
        }

        
        #[inline]
        pub fn side(&self) -> RfqDirection {
            self.get_buf().get_u8_at(self.offset + 5).into()
        }

        
        #[inline]
        pub fn amount(&self) -> f64 {
            self.get_buf().get_f64_at(self.offset + 6)
        }

        
        #[inline]
        pub fn timestamp_ms(&self) -> u64 {
            self.get_buf().get_u64_at(self.offset + 14)
        }

    }

} 

