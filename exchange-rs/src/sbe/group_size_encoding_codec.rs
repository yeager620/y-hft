use crate::sbe::*;
use crate::sbe::{Writer, WriteBuf, Reader, ReadBuf, SbeResult, SbeErr};

pub use encoder::*;
pub use decoder::*;

pub const ENCODED_LENGTH: usize = 8;

pub mod encoder {
    use super::*;

    #[derive(Debug, Default)]
    pub struct GroupSizeEncodingEncoder<P> {
        parent: Option<P>,
        offset: usize,
    }

    impl<'a, P> Writer<'a> for GroupSizeEncodingEncoder<P> where P: Writer<'a> + Default {
        #[inline]
        fn get_buf_mut(&mut self) -> &mut WriteBuf<'a> {
            if let Some(parent) = self.parent.as_mut() {
                parent.get_buf_mut()
            } else {
                panic!("parent was None")
            }
        }
    }

    impl<'a, P> GroupSizeEncodingEncoder<P> where P: Writer<'a> + Default {
        pub fn wrap(mut self, parent: P, offset: usize) -> Self {
            self.parent = Some(parent);
            self.offset = offset;
            self
        }

        #[inline]
        pub fn parent(&mut self) -> SbeResult<P> {
            self.parent.take().ok_or(SbeErr::ParentNotSet)
        }

        
        
        
        
        
        
        
        
        #[inline]
        pub fn block_length(&mut self, value: u16) {
            let offset = self.offset;
            self.get_buf_mut().put_u16_at(offset, value);
        }

        
        
        
        
        
        
        
        
        #[inline]
        pub fn num_in_group(&mut self, value: u16) {
            let offset = self.offset + 2;
            self.get_buf_mut().put_u16_at(offset, value);
        }

        
        
        
        
        
        
        
        
        #[inline]
        pub fn num_groups(&mut self, value: u16) {
            let offset = self.offset + 4;
            self.get_buf_mut().put_u16_at(offset, value);
        }

        
        
        
        
        
        
        
        
        #[inline]
        pub fn num_var_data_fields(&mut self, value: u16) {
            let offset = self.offset + 6;
            self.get_buf_mut().put_u16_at(offset, value);
        }

    }
} 

pub mod decoder {
    use super::*;

    #[derive(Debug, Default)]
    pub struct GroupSizeEncodingDecoder<P> {
        parent: Option<P>,
        offset: usize,
    }

    impl<'a, P> Reader<'a> for GroupSizeEncodingDecoder<P> where P: Reader<'a> + Default {
        #[inline]
        fn get_buf(&self) -> &ReadBuf<'a> {
            self.parent.as_ref().expect("parent missing").get_buf()
        }
    }

    impl<'a, P> GroupSizeEncodingDecoder<P> where P: Reader<'a> + Default {
        pub fn wrap(mut self, parent: P, offset: usize) -> Self {
            self.parent = Some(parent);
            self.offset = offset;
            self
        }

        #[inline]
        pub fn parent(&mut self) -> SbeResult<P> {
            self.parent.take().ok_or(SbeErr::ParentNotSet)
        }

        
        #[inline]
        pub fn block_length(&self) -> u16 {
            self.get_buf().get_u16_at(self.offset)
        }

        
        #[inline]
        pub fn num_in_group(&self) -> u16 {
            self.get_buf().get_u16_at(self.offset + 2)
        }

        
        #[inline]
        pub fn num_groups(&self) -> u16 {
            self.get_buf().get_u16_at(self.offset + 4)
        }

        
        #[inline]
        pub fn num_var_data_fields(&self) -> u16 {
            self.get_buf().get_u16_at(self.offset + 6)
        }

    }
} 
