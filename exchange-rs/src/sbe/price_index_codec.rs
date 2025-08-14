use crate::sbe::*;

pub use encoder::*;
pub use decoder::*;

pub const SBE_BLOCK_LENGTH: u16 = 32;
pub const SBE_TEMPLATE_ID: u16 = 1008;
pub const SBE_SCHEMA_ID: u16 = 1;
pub const SBE_SCHEMA_VERSION: u16 = 3;
pub const SBE_SEMANTIC_VERSION: &str = "";

pub mod encoder {
    use super::*;

    #[derive(Debug, Default)]
    pub struct PriceIndexEncoder<'a> {
        buf: WriteBuf<'a>,
        initial_offset: usize,
        offset: usize,
        limit: usize,
    }

    impl<'a> Writer<'a> for PriceIndexEncoder<'a> {
        #[inline]
        fn get_buf_mut(&mut self) -> &mut WriteBuf<'a> {
            &mut self.buf
        }
    }

    impl<'a> Encoder<'a> for PriceIndexEncoder<'a> {
        #[inline]
        fn get_limit(&self) -> usize {
            self.limit
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.limit = limit;
        }
    }

    impl<'a> PriceIndexEncoder<'a> {
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
        pub fn index_name(&mut self, value: [u8; 16]) {
            let offset = self.offset;
            let buf = self.get_buf_mut();
            buf.put_u8_at(offset, value[0]);
            buf.put_u8_at(offset + 1, value[1]);
            buf.put_u8_at(offset + 2, value[2]);
            buf.put_u8_at(offset + 3, value[3]);
            buf.put_u8_at(offset + 4, value[4]);
            buf.put_u8_at(offset + 5, value[5]);
            buf.put_u8_at(offset + 6, value[6]);
            buf.put_u8_at(offset + 7, value[7]);
            buf.put_u8_at(offset + 8, value[8]);
            buf.put_u8_at(offset + 9, value[9]);
            buf.put_u8_at(offset + 10, value[10]);
            buf.put_u8_at(offset + 11, value[11]);
            buf.put_u8_at(offset + 12, value[12]);
            buf.put_u8_at(offset + 13, value[13]);
            buf.put_u8_at(offset + 14, value[14]);
            buf.put_u8_at(offset + 15, value[15]);
        }

        
        
        
        
        
        
        
        
        #[inline]
        pub fn price(&mut self, value: f64) {
            let offset = self.offset + 16;
            self.get_buf_mut().put_f64_at(offset, value);
        }

        
        
        
        
        
        
        
        
        #[inline]
        pub fn timestamp_ms(&mut self, value: u64) {
            let offset = self.offset + 24;
            self.get_buf_mut().put_u64_at(offset, value);
        }

    }

} 

pub mod decoder {
    use super::*;

    #[derive(Debug, Default)]
    pub struct PriceIndexDecoder<'a> {
        buf: ReadBuf<'a>,
        initial_offset: usize,
        offset: usize,
        limit: usize,
        pub acting_block_length: u16,
        pub acting_version: u16,
    }

    impl<'a> Reader<'a> for PriceIndexDecoder<'a> {
        #[inline]
        fn get_buf(&self) -> &ReadBuf<'a> {
            &self.buf
        }
    }

    impl<'a> Decoder<'a> for PriceIndexDecoder<'a> {
        #[inline]
        fn get_limit(&self) -> usize {
            self.limit
        }

        #[inline]
        fn set_limit(&mut self, limit: usize) {
            self.limit = limit;
        }
    }

    impl<'a> PriceIndexDecoder<'a> {
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
        pub fn index_name(&self) -> [u8; 16] {
            let buf = self.get_buf();
            [
                buf.get_u8_at(self.offset),
                buf.get_u8_at(self.offset + 1),
                buf.get_u8_at(self.offset + 2),
                buf.get_u8_at(self.offset + 3),
                buf.get_u8_at(self.offset + 4),
                buf.get_u8_at(self.offset + 5),
                buf.get_u8_at(self.offset + 6),
                buf.get_u8_at(self.offset + 7),
                buf.get_u8_at(self.offset + 8),
                buf.get_u8_at(self.offset + 9),
                buf.get_u8_at(self.offset + 10),
                buf.get_u8_at(self.offset + 11),
                buf.get_u8_at(self.offset + 12),
                buf.get_u8_at(self.offset + 13),
                buf.get_u8_at(self.offset + 14),
                buf.get_u8_at(self.offset + 15),
            ]
        }

        
        #[inline]
        pub fn price(&self) -> f64 {
            self.get_buf().get_f64_at(self.offset + 16)
        }

        
        #[inline]
        pub fn timestamp_ms(&self) -> u64 {
            self.get_buf().get_u64_at(self.offset + 24)
        }

    }

} 

