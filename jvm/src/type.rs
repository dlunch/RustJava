use alloc::{
    string::{String, ToString},
    vec::Vec,
};

use crate::JavaValue;

#[derive(Eq, PartialEq, Clone)]
pub enum JavaType {
    Void,
    Integer,
    Long,
    Float,
    Double,
    Char,
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
            JavaType::Char => JavaValue::Char('\0'),
            JavaType::Object(_) => JavaValue::Object(None),
        }
    }

    pub fn parse(descriptor: &str) -> Self {
        match descriptor {
            "V" => JavaType::Void,
            "I" => JavaType::Integer,
            "J" => JavaType::Long,
            "F" => JavaType::Float,
            "D" => JavaType::Double,
            "C" => JavaType::Char,
            s => {
                if s.starts_with('L') && s.ends_with(';') {
                    JavaType::Object(s[1..s.len() - 1].to_string())
                } else {
                    panic!("Invalid descriptor: {}", s);
                }
            }
        }
    }

    pub fn parse_method_descriptor(descriptor: &str) -> (Vec<JavaType>, JavaType) {
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
                params.push(JavaType::Object(class_name));
            } else {
                params.push(JavaType::parse(&c.to_string()));
            }
        }
        let ret = JavaType::parse(&chars.collect::<String>());

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
            JavaType::parse_method_descriptor("(Ljava/lang/String;I)V")
                == (vec![JavaType::Object("java/lang/String".into()), JavaType::Integer], JavaType::Void)
        );
    }

    #[test]
    fn test_parse() {
        assert!(JavaType::parse("V") == JavaType::Void);
        assert!(JavaType::parse("I") == JavaType::Integer);
        assert!(JavaType::parse("J") == JavaType::Long);
        assert!(JavaType::parse("F") == JavaType::Float);
        assert!(JavaType::parse("D") == JavaType::Double);
        assert!(JavaType::parse("C") == JavaType::Char);
        assert!(JavaType::parse("Ljava/lang/String;") == JavaType::Object("java/lang/String".into()));
    }
}
