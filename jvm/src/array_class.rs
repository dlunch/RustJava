use alloc::boxed::Box;

use crate::{Class, ClassInstance};

pub trait ArrayClass: Class {
    fn element_class(&self) -> &dyn Class;
    fn instantiate_array(&self, length: usize) -> Box<dyn ClassInstance>;
}
