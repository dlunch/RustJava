use alloc::boxed::Box;
use core::fmt::Debug;

use dyn_clone::{clone_trait_object, DynClone};

use crate::{as_any::AsAny, ArrayClassInstance, Class, Field, JavaValue, JvmResult};

pub trait ClassInstance: AsAny + Debug + DynClone + 'static {
    fn destroy(self: Box<Self>);
    fn class(&self) -> Box<dyn Class>;
    fn get_field(&self, field: &dyn Field) -> JvmResult<JavaValue>;
    fn put_field(&mut self, field: &dyn Field, value: JavaValue) -> JvmResult<()>;
    fn as_array_instance(&self) -> Option<&dyn ArrayClassInstance>; // is there a better way to do this?
    fn as_array_instance_mut(&mut self) -> Option<&mut dyn ArrayClassInstance>; // is there a better way to do this?
}

clone_trait_object!(ClassInstance);
