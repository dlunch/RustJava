use alloc::{boxed::Box, string::String};
use dyn_clone::clone_trait_object;

use crate::{Class, ClassInstance};

pub trait ArrayClass: Class {
    fn element_type_name(&self) -> String;
    fn instantiate_array(&self, length: usize) -> Box<dyn ClassInstance>;
}

clone_trait_object!(ArrayClass);
