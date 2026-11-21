use alloc::{
    boxed::Box,
    string::{String, ToString},
    vec::Vec,
};

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
    pub fn parse(descriptor: &str) -> Self {
        Self::try_parse(descriptor).expect("invalid Java type descriptor")
    }

    // a field descriptor, a method descriptor, or V (JVMS 4.3)
    pub fn try_parse(descriptor: &str) -> Option<Self> {
        let (r#type, rest) = match descriptor.strip_prefix('(') {
            Some(mut params) => {
                let mut param_types = Vec::new();
                while !params.starts_with(')') {
                    let (param_type, rest) = Self::parse_field_type(params)?;
                    param_types.push(param_type);
                    params = rest;
                }
                let (return_type, rest) = Self::parse_return_type(&params[1..])?;

                (Self::Method(param_types, Box::new(return_type)), rest)
            }
            None => Self::parse_return_type(descriptor)?,
        };

        rest.is_empty().then_some(r#type)
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

    // V or the field type at the start of `descriptor`, with the rest of it
    fn parse_return_type(descriptor: &str) -> Option<(Self, &str)> {
        match descriptor.strip_prefix('V') {
            Some(rest) => Some((Self::Void, rest)),
            None => Self::parse_field_type(descriptor),
        }
    }

    // the field type at the start of `descriptor`, with the rest of it
    fn parse_field_type(descriptor: &str) -> Option<(Self, &str)> {
        let element = descriptor.trim_start_matches('[');
        let dimensions = descriptor.len() - element.len();
        // JVMS 4.3.2: an array type has at most 255 dimensions
        if dimensions > 255 {
            return None;
        }

        let (element_type, rest) = match element.split_at_checked(1)? {
            ("Z", rest) => (Self::Boolean, rest),
            ("B", rest) => (Self::Byte, rest),
            ("C", rest) => (Self::Char, rest),
            ("S", rest) => (Self::Short, rest),
            ("I", rest) => (Self::Int, rest),
            ("J", rest) => (Self::Long, rest),
            ("F", rest) => (Self::Float, rest),
            ("D", rest) => (Self::Double, rest),
            ("L", rest) => {
                let (class_name, rest) = rest.split_once(';')?;
                if class_name.is_empty() || class_name.contains(['.', '[']) {
                    return None;
                }
                (Self::Class(class_name.to_string()), rest)
            }
            _ => return None,
        };

        Some(((0..dimensions).fold(element_type, |r#type, _| Self::Array(Box::new(r#type))), rest))
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
        assert!(JavaType::try_parse("()").is_none());
        assert!(JavaType::try_parse(&("[".repeat(256) + "I")).is_none());
        assert!(JavaType::try_parse(&("[".repeat(255) + "I")).is_some());
    }
}
