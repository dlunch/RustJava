use alloc::{
    boxed::Box,
    string::{String, ToString},
    vec::Vec,
};

use nom::{
    bytes::complete::{take, take_until},
    character::complete::anychar,
    multi::many0,
    sequence::terminated,
    IResult,
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
        Self::parse_type(descriptor).unwrap().1
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
                let (remaining, class_name) = terminated(take_until(";"), take(1usize))(remaining)?;
                Ok((remaining, Self::Class(class_name.to_string())))
            }
            '[' => {
                let (remaining, element_type) = Self::parse_type(remaining)?;
                Ok((remaining, Self::Array(Box::new(element_type))))
            }
            '(' => {
                let (remaining, params) = terminated(take_until(")"), take(1usize))(remaining)?;
                let param_types = many0(Self::parse_type)(params)?.1;

                let (remaining, return_type) = Self::parse_type(remaining)?;

                Ok((remaining, Self::Method(param_types, Box::new(return_type))))
            }
            _ => panic!("Invalid type descriptor: {}", descriptor),
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
}
