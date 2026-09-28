use alloc::{collections::BTreeMap, string::String, sync::Arc, vec::Vec};
use core::char::REPLACEMENT_CHARACTER;

use nom::{
    IResult, Parser,
    bytes::complete::take,
    combinator::map_res,
    error::{Error, ErrorKind},
    number::complete::{be_f32, be_f64, be_i32, be_i64, be_u16, u8},
};

fn parse_utf8(data: &[u8]) -> IResult<&[u8], Arc<String>> {
    let (data, length) = be_u16(data)?;
    map_res(take(length as usize), |utf8: &[u8]| decode_modified_utf8(utf8).map(Arc::new)).parse(data)
}

// CONSTANT_Utf8 is *modified* UTF-8 (JVMS §4.4.7), not UTF-8: NUL is the two bytes C0 80, and a
// supplementary character is its UTF-16 surrogate pair with each half encoded as three bytes
// (ED A0..AF xx ED B0..BF xx). `String::from_utf8` rejects both, so any class holding "\0" or a
// supplementary character in a constant failed to load with ClassFormatError. Decoding goes
// through UTF-16 units, which is what the Java string is anyway.
//
// Strictly wider than before: input that is valid UTF-8 takes the old path unchanged. On the slow
// path the standard 4-byte form is accepted too (javac never emits it, but the old parser did
// accept it, so rejecting it now would be a regression), and a lone surrogate — legal in a Java
// string, unrepresentable in a Rust `String` — becomes U+FFFD instead of failing the whole class.
fn decode_modified_utf8(bytes: &[u8]) -> Result<String, ()> {
    if let Ok(x) = core::str::from_utf8(bytes) {
        return Ok(x.into());
    }

    let continuation = |i: usize| match bytes.get(i) {
        Some(&b) if b & 0xC0 == 0x80 => Ok((b & 0x3F) as u32),
        _ => Err(()),
    };

    let mut units = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        let b = bytes[i] as u32;
        let (code_point, length) = match bytes[i] {
            0x00..=0x7F => (b, 1),
            0xC0..=0xDF => (((b & 0x1F) << 6) | continuation(i + 1)?, 2),
            0xE0..=0xEF => (((b & 0x0F) << 12) | (continuation(i + 1)? << 6) | continuation(i + 2)?, 3),
            0xF0..=0xF4 => (
                ((b & 0x07) << 18) | (continuation(i + 1)? << 12) | (continuation(i + 2)? << 6) | continuation(i + 3)?,
                4,
            ),
            _ => return Err(()),
        };
        match char::from_u32(code_point) {
            Some(c) if code_point > 0xFFFF => units.extend_from_slice(c.encode_utf16(&mut [0; 2])),
            None if code_point > 0x10FFFF => return Err(()),
            _ => units.push(code_point as u16), // BMP, or one surrogate half (paired up below)
        }
        i += length;
    }

    Ok(char::decode_utf16(units).map(|x| x.unwrap_or(REPLACEMENT_CHARACTER)).collect())
}

#[derive(Debug)]
pub enum ConstantPoolItem {
    Utf8(Arc<String>),
    Integer(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    Class { name_index: u16 },
    String { string_index: u16 },
    Fieldref { class_index: u16, name_and_type_index: u16 },
    Methodref { class_index: u16, name_and_type_index: u16 },
    InterfaceMethodref { class_index: u16, name_and_type_index: u16 },
    NameAndType { name_index: u16, descriptor_index: u16 },
}

impl ConstantPoolItem {
    fn parse_tagged(data: &[u8], tag: u8) -> IResult<&[u8], Self> {
        match tag {
            1 => {
                let (data, utf8) = parse_utf8(data)?;
                Ok((data, Self::Utf8(utf8)))
            }
            3 => {
                let (data, value) = be_i32(data)?;
                Ok((data, Self::Integer(value)))
            }
            4 => {
                let (data, value) = be_f32(data)?;
                Ok((data, Self::Float(value)))
            }
            5 => {
                let (data, value) = be_i64(data)?;
                Ok((data, Self::Long(value)))
            }
            6 => {
                let (data, value) = be_f64(data)?;
                Ok((data, Self::Double(value)))
            }
            7 => {
                let (data, name_index) = be_u16(data)?;
                Ok((data, Self::Class { name_index }))
            }
            8 => {
                let (data, string_index) = be_u16(data)?;
                Ok((data, Self::String { string_index }))
            }
            9 => {
                let (data, class_index) = be_u16(data)?;
                let (data, name_and_type_index) = be_u16(data)?;
                Ok((
                    data,
                    Self::Fieldref {
                        class_index,
                        name_and_type_index,
                    },
                ))
            }
            10 => {
                let (data, class_index) = be_u16(data)?;
                let (data, name_and_type_index) = be_u16(data)?;
                Ok((
                    data,
                    Self::Methodref {
                        class_index,
                        name_and_type_index,
                    },
                ))
            }
            11 => {
                let (data, class_index) = be_u16(data)?;
                let (data, name_and_type_index) = be_u16(data)?;
                Ok((
                    data,
                    Self::InterfaceMethodref {
                        class_index,
                        name_and_type_index,
                    },
                ))
            }
            12 => {
                let (data, name_index) = be_u16(data)?;
                let (data, descriptor_index) = be_u16(data)?;
                Ok((
                    data,
                    Self::NameAndType {
                        name_index,
                        descriptor_index,
                    },
                ))
            }
            _ => Err(nom::Err::Error(Error::new(data, ErrorKind::Switch))),
        }
    }

    pub fn parse_all(data: &[u8]) -> IResult<&[u8], BTreeMap<u16, Self>> {
        let (remaining, count) = be_u16(data)?;
        if count == 0 {
            return Err(nom::Err::Error(Error::new(remaining, ErrorKind::Verify)));
        }
        if count == 1 {
            return Ok((remaining, BTreeMap::new()));
        }

        let mut data = remaining;
        let mut result = BTreeMap::new();
        let mut i = 1;
        loop {
            let (remaining, item) = Self::parse_with_tag(data)?;
            let is_double_entry = match &item {
                Self::Long(_) | Self::Double(_) => {
                    // long or double constant takes two constant pool entries....
                    true
                }
                _ => false,
            };
            result.insert(i, item);

            data = remaining;
            i += 1;
            if is_double_entry {
                i += 1;
            }

            if i > count {
                return Err(nom::Err::Error(Error::new(data, ErrorKind::Verify)));
            }
            if i == count {
                break;
            }
        }

        Ok((data, result))
    }

    pub fn parse_with_tag(data: &[u8]) -> IResult<&[u8], Self> {
        let (data, tag) = u8(data)?;
        Self::parse_tagged(data, tag)
    }

    pub fn utf8(&self) -> Option<Arc<String>> {
        if let ConstantPoolItem::Utf8(x) = self { Some(x.clone()) } else { None }
    }

    pub fn class_name_index(&self) -> Option<u16> {
        if let ConstantPoolItem::Class { name_index } = self {
            Some(*name_index)
        } else {
            None
        }
    }

    pub fn name_and_type(&self) -> Option<(u16, u16)> {
        if let ConstantPoolItem::NameAndType {
            name_index,
            descriptor_index,
        } = self
        {
            Some((*name_index, *descriptor_index))
        } else {
            None
        }
    }
}

#[derive(Clone, Debug)]
pub enum ConstantPoolReference {
    Integer(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    String(Arc<String>),
    Class(Arc<String>),
    Method(FieldMethodref),
    InterfaceMethodref(FieldMethodref),
    Field(FieldMethodref),
}

impl ConstantPoolReference {
    pub fn from_constant_pool(constant_pool: &BTreeMap<u16, ConstantPoolItem>, index: u16) -> Option<Self> {
        match constant_pool.get(&index)? {
            ConstantPoolItem::Integer(x) => Some(Self::Integer(*x)),
            ConstantPoolItem::Float(x) => Some(Self::Float(*x)),
            ConstantPoolItem::Long(x) => Some(Self::Long(*x)),
            ConstantPoolItem::Double(x) => Some(Self::Double(*x)),
            ConstantPoolItem::String { string_index } => Some(Self::String(constant_pool.get(string_index)?.utf8()?)),
            ConstantPoolItem::Class { name_index } => Some(Self::Class(constant_pool.get(name_index)?.utf8()?)),
            ConstantPoolItem::Methodref {
                class_index,
                name_and_type_index,
            } => Some(Self::Method(FieldMethodref::from_reference_info(
                constant_pool,
                *class_index,
                *name_and_type_index,
            )?)),
            ConstantPoolItem::Fieldref {
                class_index,
                name_and_type_index,
            } => Some(Self::Field(FieldMethodref::from_reference_info(
                constant_pool,
                *class_index,
                *name_and_type_index,
            )?)),
            ConstantPoolItem::InterfaceMethodref {
                class_index,
                name_and_type_index,
            } => Some(Self::InterfaceMethodref(FieldMethodref::from_reference_info(
                constant_pool,
                *class_index,
                *name_and_type_index,
            )?)),
            _ => None,
        }
    }

    pub fn as_class(&self) -> &str {
        if let Self::Class(x) = self {
            x
        } else {
            panic!("Invalid constant pool item");
        }
    }

    pub fn as_field_ref(&self) -> &FieldMethodref {
        if let Self::Field(x) = self {
            x
        } else {
            panic!("Invalid constant pool item");
        }
    }

    pub fn as_method_ref(&self) -> &FieldMethodref {
        if let Self::Method(x) = self {
            x
        } else {
            panic!("Invalid constant pool item");
        }
    }

    pub fn as_interface_method_ref(&self) -> &FieldMethodref {
        if let Self::InterfaceMethodref(x) = self {
            x
        } else {
            panic!("Invalid constant pool item");
        }
    }
}

#[derive(Clone, Debug)]
pub struct FieldMethodref {
    pub class: Arc<String>,
    pub name: Arc<String>,
    pub descriptor: Arc<String>,
}

impl FieldMethodref {
    pub fn from_reference_info(constant_pool: &BTreeMap<u16, ConstantPoolItem>, class_index: u16, name_and_type_index: u16) -> Option<Self> {
        let class_name_index = constant_pool.get(&class_index)?.class_name_index()?;
        let class_name = constant_pool.get(&class_name_index)?.utf8()?;

        let (name_index, descriptor_index) = constant_pool.get(&name_and_type_index)?.name_and_type()?;
        let name = constant_pool.get(&name_index)?.utf8()?;
        let descriptor = constant_pool.get(&descriptor_index)?.utf8()?;

        Some(Self {
            class: class_name,
            name,
            descriptor,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::ConstantPoolItem;

    #[test]
    fn empty_constant_pool_is_valid() {
        let (remaining, constant_pool) = ConstantPoolItem::parse_all(&[0x00, 0x01, 0xff]).unwrap();

        assert!(constant_pool.is_empty());
        assert_eq!(remaining, &[0xff]);
    }

    #[test]
    fn long_must_fit_in_two_constant_pool_slots() {
        assert!(ConstantPoolItem::parse_all(&[0x00, 0x02, 0x05, 0, 0, 0, 0, 0, 0, 0, 0]).is_err());
    }
}

#[cfg(test)]
mod modified_utf8_tests {
    use super::parse_utf8;

    fn constant(bytes: &[u8]) -> Option<alloc::string::String> {
        let mut data = (bytes.len() as u16).to_be_bytes().to_vec();
        data.extend_from_slice(bytes);
        parse_utf8(&data).ok().map(|(rest, x)| {
            assert!(rest.is_empty());
            (*x).clone()
        })
    }

    #[test]
    fn nul_is_c0_80() {
        // javac's encoding of "MTR\0"
        assert_eq!(constant(b"MTR\xC0\x80").as_deref(), Some("MTR\0"));
    }

    #[test]
    fn supplementary_is_a_surrogate_pair() {
        // javac's encoding of "\uD83D\uDC0D" (U+1F40D) — six bytes, two 3-byte halves
        assert_eq!(constant(b"a\xED\xA0\xBD\xED\xB0\x8Db").as_deref(), Some("a\u{1F40D}b"));
    }

    #[test]
    fn ascii_and_hangul_are_unchanged() {
        assert_eq!(constant(b"java/lang/Object").as_deref(), Some("java/lang/Object"));
        assert_eq!(constant("한글 5개".as_bytes()).as_deref(), Some("한글 5개"));
        assert_eq!(constant(b"").as_deref(), Some(""));
    }

    #[test]
    fn edges_of_the_wider_decoder() {
        // standard 4-byte UTF-8 next to C0 80: the old parser accepted the first alone
        assert_eq!(constant(b"\xF0\x9F\x90\x8D\xC0\x80").as_deref(), Some("\u{1F40D}\0"));
        // a lone surrogate half cannot live in a Rust String
        assert_eq!(constant(b"x\xED\xA0\xBD").as_deref(), Some("x\u{FFFD}"));
        // still malformed: truncated sequence, stray continuation byte, invalid lead byte
        assert_eq!(constant(b"\xC0"), None);
        assert_eq!(constant(b"\x80"), None);
        assert_eq!(constant(b"\xFF"), None);
    }
}
