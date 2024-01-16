use alloc::{boxed::Box, string::String};
use core::fmt::Debug;

use dyn_clone::{clone_trait_object, DynClone};

use crate::{as_any::AsAny, ArrayClass, ClassInstance, Field, JavaValue, JvmResult, Method};

pub trait Class: AsAny + Debug + DynClone {
    fn name(&self) -> String;
    fn super_class_name(&self) -> Option<String>;
    fn instantiate(&self) -> Box<dyn ClassInstance>;
    fn method(&self, name: &str, descriptor: &str) -> Option<Box<dyn Method>>;
    fn field(&self, name: &str, descriptor: &str, is_static: bool) -> Option<Box<dyn Field>>;
    fn get_static_field(&self, field: &dyn Field) -> JvmResult<JavaValue>;
    fn put_static_field(&mut self, field: &dyn Field, value: JavaValue) -> JvmResult<()>;
    fn as_array_class(&self) -> Option<&dyn ArrayClass> {
        None
    }
}

clone_trait_object!(Class);
