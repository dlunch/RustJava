use alloc::{
    boxed::Box,
    string::{String, ToString},
    vec::Vec,
};

use nom::{
    IResult, Parser,
    bytes::complete::{take, take_until},
    character::complete::anychar,
    error::{Error, ErrorKind},
    sequence::terminated,
};

use crate::JavaValue;

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum JavaType {
    Void,
    Boolean,
    Byte,
    Char,
    Short,
    Int,
    Long,
    Float,
    Double,
    Class(String),
    Array(Box<Self>),
    Method(Vec<Self>, Box<Self>),
}

impl JavaType {
    pub fn default(&self) -> JavaValue {
        match self {
            Self::Void => panic!("Cannot create default value for void"),
            Self::Boolean => JavaValue::Boolean(false),
            Self::Byte => JavaValue::Byte(0),
            Self::Char => JavaValue::Char(0),
            Self::Short => JavaValue::Short(0),
            Self::Int => JavaValue::Int(0),
            Self::Long => JavaValue::Long(0),
            Self::Float => JavaValue::Float(0.0),
            Self::Double => JavaValue::Double(0.0),
            Self::Class(_) => JavaValue::Object(None),
            Self::Array(_) => JavaValue::Object(None),
            Self::Method(_, _) => panic!("Cannot create default value for method"),
        }
    }

    pub fn parse(descriptor: &str) -> Self {
        Self::try_parse(descriptor).expect("invalid Java type descriptor")
    }

    pub fn try_parse(descriptor: &str) -> Option<Self> {
        let (remaining, r#type) = Self::parse_type(descriptor).ok()?;
        if remaining.is_empty() { Some(r#type) } else { None }
    }

    // a CONSTANT_Class_info name (JVMS 4.4.1): a class binary name in internal form (java/lang/String)
    // or an array type descriptor ([Ljava/lang/String;, [I)
    pub fn from_class_name(name: &str) -> Self {
        if name.starts_with('[') {
            Self::parse(name)
        } else {
            Self::Class(name.to_string())
        }
    }

    pub fn as_method(&self) -> (&[Self], &Self) {
        if let Self::Method(params, return_type) = self {
            (params, return_type)
        } else {
            panic!("Invalid type");
        }
    }

    fn parse_type(descriptor: &str) -> IResult<&str, Self> {
        let (remaining, type_char) = anychar(descriptor)?;

        match type_char {
            'V' => Ok((remaining, Self::Void)),
            'Z' => Ok((remaining, Self::Boolean)),
            'B' => Ok((remaining, Self::Byte)),
            'C' => Ok((remaining, Self::Char)),
            'S' => Ok((remaining, Self::Short)),
            'I' => Ok((remaining, Self::Int)),
            'J' => Ok((remaining, Self::Long)),
            'F' => Ok((remaining, Self::Float)),
            'D' => Ok((remaining, Self::Double)),
            'L' => {
                let (remaining, class_name) = terminated(take_until(";"), take(1usize)).parse(remaining)?;
                if class_name.is_empty() || class_name.contains(['.', '[', ';']) {
                    return Err(nom::Err::Error(Error::new(descriptor, ErrorKind::Verify)));
                }
                Ok((remaining, Self::Class(class_name.to_string())))
            }
            '[' => {
                let (remaining, element_type) = Self::parse_type(remaining)?;
                if matches!(element_type, Self::Void | Self::Method(_, _)) {
                    return Err(nom::Err::Error(Error::new(descriptor, ErrorKind::Verify)));
                }
                Ok((remaining, Self::Array(Box::new(element_type))))
            }
            '(' => {
                let (remaining, params) = terminated(take_until(")"), take(1usize)).parse(remaining)?;
                let mut param_types = Vec::new();
                let mut params = params;
                while !params.is_empty() {
                    let (remaining_params, param_type) = Self::parse_type(params)?;
                    if remaining_params.len() >= params.len() || matches!(param_type, Self::Void | Self::Method(_, _)) {
                        return Err(nom::Err::Error(Error::new(descriptor, ErrorKind::Verify)));
                    }
                    param_types.push(param_type);
                    params = remaining_params;
                }

                let (remaining, return_type) = Self::parse_type(remaining)?;
                if matches!(return_type, Self::Method(_, _)) {
                    return Err(nom::Err::Error(Error::new(descriptor, ErrorKind::Verify)));
                }

                Ok((remaining, Self::Method(param_types, Box::new(return_type))))
            }
            _ => Err(nom::Err::Error(Error::new(descriptor, ErrorKind::Verify))),
        }
    }
}

#[cfg(test)]
mod test {
    use alloc::{boxed::Box, vec};

    use super::JavaType;

    #[test]
    fn test_parse_method_descriptor() {
        assert!(
            JavaType::parse("(Ljava/lang/String;I)V")
                == JavaType::Method(vec![JavaType::Class("java/lang/String".into()), JavaType::Int], Box::new(JavaType::Void))
        );
    }

    #[test]
    fn test_parse_method_descriptor_array() {
        assert!(
            JavaType::parse("([CI)V") == JavaType::Method(vec![JavaType::Array(Box::new(JavaType::Char)), JavaType::Int], Box::new(JavaType::Void))
        )
    }

    #[test]
    fn test_parse() {
        assert!(JavaType::parse("V") == JavaType::Void);
        assert!(JavaType::parse("I") == JavaType::Int);
        assert!(JavaType::parse("J") == JavaType::Long);
        assert!(JavaType::parse("F") == JavaType::Float);
        assert!(JavaType::parse("D") == JavaType::Double);
        assert!(JavaType::parse("C") == JavaType::Char);
        assert!(JavaType::parse("Ljava/lang/String;") == JavaType::Class("java/lang/String".into()));
        assert!(JavaType::parse("[Ljava/lang/String;") == JavaType::Array(Box::new(JavaType::Class("java/lang/String".into()))));
        assert!(
            JavaType::parse("[[Ljava/lang/String;")
                == JavaType::Array(Box::new(JavaType::Array(Box::new(JavaType::Class("java/lang/String".into())))))
        );
    }

    #[test]
    fn test_try_parse_rejects_malformed_descriptors() {
        assert!(JavaType::try_parse("").is_none());
        assert!(JavaType::try_parse("Igarbage").is_none());
        assert!(JavaType::try_parse("[V").is_none());
        assert!(JavaType::try_parse("(V)V").is_none());
        assert!(JavaType::try_parse("(I").is_none());
        assert!(JavaType::try_parse("L;").is_none());
    }
}
