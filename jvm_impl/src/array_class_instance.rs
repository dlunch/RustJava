use alloc::{
    string::{String, ToString},
    vec,
    vec::Vec,
};
use jvm::{ArrayClass, ArrayClassInstance, Class, ClassInstance, JavaType, JavaValue};

use crate::array_class::ArrayClassImpl;

pub struct ArrayClassInstanceImpl {
    class_name: String,
    _length: usize,
    _elements: Vec<JavaValue>,
}

impl ArrayClassInstanceImpl {
    pub fn new(class: &ArrayClassImpl, length: usize) -> Self {
        let element_type = class.element_type_name();
        let default_value = JavaType::parse(element_type).default();

        Self {
            class_name: class.name().to_string(),
            _length: length,
            _elements: vec![default_value; length],
        }
    }
}

impl ClassInstance for ArrayClassInstanceImpl {
    fn class_name(&self) -> &str {
        &self.class_name
    }
}

impl ArrayClassInstance for ArrayClassInstanceImpl {}
