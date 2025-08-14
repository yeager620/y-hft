use crate::fix::parser::{FixField, field_parser::{FieldParser, FieldValue}, raw_parser::RawField};
use crate::fix::error::{ParseError, FixError};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct RepeatingGroup {
    pub count: u32,
    pub instances: Vec<HashMap<u32, FixField>>,
}

pub struct GroupParser {
    field_parser: FieldParser,
}

impl GroupParser {
    pub fn new() -> Self {
        Self {
            field_parser: FieldParser::new(),
        }
    }
    
    pub fn parse_repeating_group(
        &self,
        raw_fields: &[RawField<'_>],
        group_count_tag: u32,
        first_field_tag: u32,
        group_fields: &[u32],
    ) -> Result<Option<RepeatingGroup>, FixError> {
        
        let count_field = self.find_field_by_tag(raw_fields, group_count_tag)?;
        if count_field.is_none() {
            return Ok(None); 
        }
        
        let count_field = count_field.unwrap();
        let parsed_count = self.field_parser.parse_field(count_field.clone())?;
        let count = parsed_count.as_int()
            .ok_or_else(|| ParseError::InvalidFieldValue {
                tag: group_count_tag,
                value: format!("{:?}", parsed_count.value),
            })? as u32;
            
        if count == 0 {
            return Ok(Some(RepeatingGroup {
                count: 0,
                instances: Vec::new(),
            }));
        }
        
        
        let count_pos = self.find_field_position(raw_fields, group_count_tag)?;
        let mut instances = Vec::with_capacity(count as usize);
        let mut current_pos = count_pos + 1;
        
        
        for i in 0..count {
            let mut instance = HashMap::new();
            
            
            let delimiter_pos = self.find_next_occurrence(raw_fields, current_pos, first_field_tag)?;
            if delimiter_pos.is_none() {
                return Err(ParseError::InvalidFormat.into());
            }
            
            let delimiter_pos = delimiter_pos.unwrap();
            current_pos = delimiter_pos;
            
            
            for &field_tag in group_fields {
                if let Some(field_pos) = self.find_field_in_range(raw_fields, current_pos, field_tag, &group_fields) {
                    let raw_field = &raw_fields[field_pos];
                    let parsed_field = self.field_parser.parse_field(raw_field.clone())?;
                    instance.insert(field_tag, parsed_field);
                    
                    if field_pos >= current_pos {
                        current_pos = field_pos + 1;
                    }
                }
            }
            
            instances.push(instance);
        }
        
        Ok(Some(RepeatingGroup {
            count,
            instances,
        }))
    }
    
    fn find_field_by_tag<'a>(&self, raw_fields: &'a [RawField<'a>], tag: u32) -> Result<Option<RawField<'a>>, ParseError> {
        let tag_str = tag.to_string();
        let tag_bytes = tag_str.as_bytes();
        
        for field in raw_fields {
            if field.tag == tag_bytes {
                return Ok(Some(field.clone()));
            }
        }
        
        Ok(None)
    }
    
    fn find_field_position(&self, raw_fields: &[RawField<'_>], tag: u32) -> Result<usize, ParseError> {
        let tag_str = tag.to_string();
        let tag_bytes = tag_str.as_bytes();
        
        for (i, field) in raw_fields.iter().enumerate() {
            if field.tag == tag_bytes {
                return Ok(i);
            }
        }
        
        Err(ParseError::InvalidFormat)
    }
    
    fn find_next_occurrence(&self, raw_fields: &[RawField<'_>], start_pos: usize, tag: u32) -> Result<Option<usize>, ParseError> {
        let tag_str = tag.to_string();
        let tag_bytes = tag_str.as_bytes();
        
        for i in start_pos..raw_fields.len() {
            if raw_fields[i].tag == tag_bytes {
                return Ok(Some(i));
            }
        }
        
        Ok(None)
    }
    
    fn find_field_in_range(
        &self,
        raw_fields: &[RawField<'_>],
        start_pos: usize,
        target_tag: u32,
        group_fields: &[u32],
    ) -> Option<usize> {
        let target_tag_str = target_tag.to_string();
        let target_tag_bytes = target_tag_str.as_bytes();
        
        
        let group_tags: std::collections::HashSet<String> = group_fields
            .iter()
            .map(|tag| tag.to_string())
            .collect();
        
        for i in start_pos..raw_fields.len() {
            let field = &raw_fields[i];
            let field_tag_str = String::from_utf8_lossy(field.tag);
            
            if field.tag == target_tag_bytes {
                return Some(i);
            }
            
            
            if !group_tags.contains(&field_tag_str.to_string()) {
                
                if let Ok(parsed_tag) = field_tag_str.parse::<u32>() {
                    if parsed_tag == group_fields[0] {
                        
                        break;
                    }
                }
                
                break;
            }
        }
        
        None
    }
}

impl Default for GroupParser {
    fn default() -> Self {
        Self::new()
    }
}


pub struct GroupDefinitions;

impl GroupDefinitions {
    
    pub const PARTIES_GROUP: GroupDef = GroupDef {
        count_tag: 453,
        delimiter_tag: 448, 
        fields: &[448, 447, 452, 802], 
    };
    
    
    pub const SECURITY_ALT_ID_GROUP: GroupDef = GroupDef {
        count_tag: 454,
        delimiter_tag: 455, 
        fields: &[455, 456], 
    };
    
    
    pub const MD_ENTRIES_GROUP: GroupDef = GroupDef {
        count_tag: 268,
        delimiter_tag: 269, 
        fields: &[269, 270, 15, 271, 272, 273, 274, 275, 336, 625], 
    };
}

#[derive(Debug, Clone)]
pub struct GroupDef {
    pub count_tag: u32,
    pub delimiter_tag: u32,
    pub fields: &'static [u32],
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fix::parser::raw_parser::{RawParser, RawField};
    
    #[test]
    fn test_parse_simple_repeating_group() {
        let group_parser = GroupParser::new();
        let raw_parser = RawParser::new();
        
        
        let data = b"8=FIX.4.4\x019=50\x01453=2\x01448=PARTY1\x01447=D\x01448=PARTY2\x01447=D\x0110=123\x01";
        let raw_fields = raw_parser.parse(data).unwrap();
        
        let group = group_parser.parse_repeating_group(
            &raw_fields,
            453, 
            448, 
            &[448, 447], 
        ).unwrap();
        
        assert!(group.is_some());
        let group = group.unwrap();
        assert_eq!(group.count, 2);
        assert_eq!(group.instances.len(), 2);
    }
}