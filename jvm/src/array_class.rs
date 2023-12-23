use alloc::{boxed::Box, string::String};

use crate::{Class, ClassInstance};

pub trait ArrayClass: Class {
    fn element_type_name(&self) -> String;
    fn instantiate_array(&self, length: usize) -> Box<dyn ClassInstance>;
}
