use alloc::vec::Vec;

use crate::{class_definition::ClassDefinition, JavaValue};

pub struct Class {
    pub class_definition: ClassDefinition,
    pub storage: Vec<JavaValue>,
}
