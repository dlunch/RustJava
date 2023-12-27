use alloc::{
    boxed::Box,
    string::{String, ToString},
    vec::Vec,
};

use crate::JavaValue;

#[derive(Eq, PartialEq, Clone)]
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
    Array(String),
    Method(Vec<Self>, Box<Self>),
}

impl JavaType {
    pub fn default(self) -> JavaValue {
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
        match descriptor {
            "V" => Self::Void,
            "I" => Self::Int,
            "J" => Self::Long,
            "F" => Self::Float,
            "D" => Self::Double,
            "C" => Self::Char,
            s => {
                if s.starts_with('L') && s.ends_with(';') {
                    Self::Class(s[1..s.len() - 1].to_string())
                } else if s.starts_with('[') {
                    Self::Array(s.to_string())
                } else if s.starts_with('(') {
                    let (params, ret) = Self::parse_method_type(s);
                    Self::Method(params, Box::new(ret))
                } else {
                    panic!("Invalid type descriptor: {}", s);
                }
            }
        }
    }

    fn parse_method_type(descriptor: &str) -> (Vec<Self>, Self) {
        let mut chars = descriptor.chars();
        if chars.next() != Some('(') {
            panic!("Invalid method descriptor: {}", descriptor);
        }

        let mut params = Vec::new();
        while let Some(c) = chars.next() {
            if c == ')' {
                break;
            }

            if c == 'L' {
                let class_name = chars.by_ref().take_while(|&x| x != ';').collect::<String>();
                params.push(Self::Class(class_name));
            } else {
                params.push(Self::parse(&c.to_string()));
            }
        }
        let ret = Self::parse(&chars.collect::<String>());

        (params, ret)
    }
}

#[cfg(test)]
mod test {
    use alloc::vec;

    use super::JavaType;

    #[test]
    fn test_parse_method_descriptor() {
        assert!(
            JavaType::parse_method_type("(Ljava/lang/String;I)V")
                == (vec![JavaType::Class("java/lang/String".into()), JavaType::Int], JavaType::Void)
        );
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
    }
}
