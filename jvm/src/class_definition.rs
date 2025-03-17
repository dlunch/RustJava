use alloc::{boxed::Box, string::String, vec::Vec};
use core::fmt::Debug;

use dyn_clone::{DynClone, clone_trait_object};

use crate::{ArrayClassDefinition, ClassInstance, Field, JavaValue, Method, Result, as_any::AsAny};

#[async_trait::async_trait]
pub trait ClassDefinition: Sync + Send + AsAny + Debug + DynClone {
    fn name(&self) -> String;
    fn super_class_name(&self) -> Option<String>;
    fn instantiate(&self) -> Result<Box<dyn ClassInstance>>;
    fn method(&self, name: &str, descriptor: &str, is_static: bool) -> Option<Box<dyn Method>>;
    fn field(&self, name: &str, descriptor: &str, is_static: bool) -> Option<Box<dyn Field>>;
    fn fields(&self) -> Vec<Box<dyn Field>>;
    fn get_static_field(&self, field: &dyn Field) -> Result<JavaValue>; // TODO do we need to split class? or rename classdefinition?
    fn put_static_field(&mut self, field: &dyn Field, value: JavaValue) -> Result<()>;
    fn as_array_class_definition(&self) -> Option<&dyn ArrayClassDefinition> {
        None
    }
}

clone_trait_object!(ClassDefinition);
