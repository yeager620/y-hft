use crate::fix::error::ParseError;

const SOH: u8 = 0x01;

#[derive(Debug, Clone)]
pub struct RawField<'a> {
    pub tag: &'a [u8],
    pub value: &'a [u8],
}

pub struct RawParser;

impl RawParser {
    pub fn new() -> Self {
        Self
    }

    pub fn parse<'a>(&self, data: &'a [u8]) -> Result<Vec<RawField<'a>>, ParseError> {
        if data.is_empty() {
            return Err(ParseError::InvalidFormat);
        }

        let mut fields = Vec::with_capacity(32);
        let mut pos = 0;

        while pos < data.len() {
            let field_start = pos;
            
            let equals_pos = self.find_byte(data, pos, b'=')?;
            let tag = &data[pos..equals_pos];
            
            pos = equals_pos + 1;
            let soh_pos = self.find_byte(data, pos, SOH)?;
            let value = &data[pos..soh_pos];
            
            fields.push(RawField { tag, value });
            pos = soh_pos + 1;
        }

        if fields.is_empty() {
            return Err(ParseError::InvalidFormat);
        }

        Ok(fields)
    }

    pub fn validate_checksum(&self, data: &[u8]) -> Result<(), ParseError> {
        if data.len() < 7 {
            return Err(ParseError::InvalidFormat);
        }

        let checksum_start = self.find_last_checksum_position(data)?;
        let message_body = &data[..checksum_start];
        let checksum_field = &data[checksum_start..];

        let calculated_checksum = self.calculate_checksum(message_body);
        let expected_checksum = self.extract_checksum(checksum_field)?;

        if calculated_checksum != expected_checksum {
            return Err(ParseError::InvalidChecksum {
                expected: expected_checksum,
                actual: calculated_checksum,
            });
        }

        Ok(())
    }

    fn find_byte(&self, data: &[u8], start: usize, byte: u8) -> Result<usize, ParseError> {
        for i in start..data.len() {
            if data[i] == byte {
                return Ok(i);
            }
        }
        match byte {
            SOH => Err(ParseError::MissingSoh),
            _ => Err(ParseError::InvalidFormat),
        }
    }

    fn find_last_checksum_position(&self, data: &[u8]) -> Result<usize, ParseError> {
        let mut last_soh = None;
        for (i, &byte) in data.iter().enumerate().rev() {
            if byte == SOH {
                if let Some(prev_soh) = last_soh {
                    if i + 4 < data.len() && &data[i + 1..i + 4] == b"10=" {
                        return Ok(i + 1);
                    }
                }
                last_soh = Some(i);
            }
        }
        Err(ParseError::InvalidFormat)
    }

    fn calculate_checksum(&self, data: &[u8]) -> u8 {
        data.iter().fold(0u8, |acc, &byte| acc.wrapping_add(byte))
    }

    fn extract_checksum(&self, checksum_field: &[u8]) -> Result<u8, ParseError> {
        if checksum_field.len() < 6 || &checksum_field[0..3] != b"10=" {
            return Err(ParseError::InvalidFormat);
        }

        let checksum_value = &checksum_field[3..6];
        let checksum_str = std::str::from_utf8(checksum_value)
            .map_err(|_| ParseError::InvalidFormat)?;
        
        u8::from_str_radix(checksum_str, 10)
            .map_err(|_| ParseError::InvalidFormat)
    }

    pub fn validate_body_length(&self, data: &[u8]) -> Result<(), ParseError> {
        let fields = self.parse(data)?;
        
        if fields.len() < 2 {
            return Err(ParseError::InvalidFormat);
        }

        let body_length_field = &fields[1];
        if body_length_field.tag != b"9" {
            return Err(ParseError::InvalidFormat);
        }

        let body_length_str = std::str::from_utf8(body_length_field.value)
            .map_err(|_| ParseError::InvalidFormat)?;
        let declared_length: usize = body_length_str.parse()
            .map_err(|_| ParseError::InvalidFormat)?;

        let begin_string_len = fields[0].tag.len() + 1 + fields[0].value.len() + 1;
        let body_length_len = body_length_field.tag.len() + 1 + body_length_field.value.len() + 1;
        let header_len = begin_string_len + body_length_len;
        
        let trailer_len = 7;
        let actual_length = data.len() - header_len - trailer_len;

        if declared_length != actual_length {
            return Err(ParseError::InvalidBodyLength {
                expected: declared_length,
                actual: actual_length,
            });
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_message() {
        let parser = RawParser::new();
        let data = b"8=FIX.4.4\x019=40\x0135=D\x0149=SENDER\x0156=TARGET\x0110=161\x01";
        
        let fields = parser.parse(data).unwrap();
        assert_eq!(fields.len(), 6);
        assert_eq!(fields[0].tag, b"8");
        assert_eq!(fields[0].value, b"FIX.4.4");
        assert_eq!(fields[1].tag, b"9");
        assert_eq!(fields[1].value, b"40");
    }

    #[test]
    fn test_checksum_validation() {
        let parser = RawParser::new();
        let data = b"8=FIX.4.4\x019=40\x0135=D\x0149=SENDER\x0156=TARGET\x0110=194\x01";
        
        assert!(parser.validate_checksum(data).is_ok());
    }
}