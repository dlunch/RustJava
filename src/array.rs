use alloc::{format, string::String, vec};

use crate::ClassDefinition;

pub struct ArrayClass {}

impl ArrayClass {
    pub fn class_definition(element_type_name: &str) -> ClassDefinition {
        ClassDefinition {
            name: Self::class_name(element_type_name),
            methods: vec![],
            fields: vec![],
        }
    }

    pub fn class_name(element_type: &str) -> String {
        format!("[{}", element_type)
    }
}
