use alloc::string::{String, ToString};

use crate::JavaValue;

pub enum JavaType {
    Void,
    Integer,
    Long,
    Float,
    Double,
    Object(String),
}

impl JavaType {
    pub fn default(self) -> JavaValue {
        match self {
            JavaType::Void => JavaValue::Void,
            JavaType::Integer => JavaValue::Integer(0),
            JavaType::Long => JavaValue::Long(0),
            JavaType::Float => JavaValue::Float(0.0),
            JavaType::Double => JavaValue::Double(0.0),
            JavaType::Object(_) => JavaValue::Object(None),
        }
    }

    pub fn parse_field_descriptor(descriptor: &str) -> Self {
        match descriptor {
            "V" => JavaType::Void,
            "I" => JavaType::Integer,
            "J" => JavaType::Long,
            "F" => JavaType::Float,
            "D" => JavaType::Double,
            s => {
                if s.starts_with('L') && s.ends_with(';') {
                    JavaType::Object(s[1..s.len() - 1].to_string())
                } else {
                    panic!("Invalid descriptor: {}", s);
                }
            }
        }
    }
}
