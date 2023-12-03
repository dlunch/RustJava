use alloc::boxed::Box;

use crate::{Class, ClassInstance};

pub trait ArrayClass: Class {
    fn element_type_name(&self) -> &str;
    fn instantiate_array(&self, length: usize) -> Box<dyn ClassInstance>;
}
