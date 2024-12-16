use alloc::boxed::Box;
use core::{
    fmt::{self, Debug, Formatter},
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use dyn_clone::{clone_trait_object, DynClone};
use dyn_hash::{hash_trait_object, DynHash};

use crate::{as_any::AsAny, value::JavaValue, ArrayClassInstance, ClassDefinition, Field, Result};

#[async_trait::async_trait]
pub trait ClassInstance: Sync + Send + AsAny + Debug + DynHash + DynClone + 'static {
    fn destroy(self: Box<Self>);
    fn class_definition(&self) -> Box<dyn ClassDefinition>;
    fn equals(&self, other: &dyn ClassInstance) -> Result<bool>;
    fn get_field(&self, field: &dyn Field) -> Result<JavaValue>;
    fn put_field(&mut self, field: &dyn Field, value: JavaValue) -> Result<()>;
    fn as_array_instance(&self) -> Option<&dyn ArrayClassInstance> {
        None
    }
    fn as_array_instance_mut(&mut self) -> Option<&mut dyn ArrayClassInstance> {
        None
    }
}

clone_trait_object!(ClassInstance);
hash_trait_object!(ClassInstance);

impl Eq for dyn ClassInstance {}
impl PartialEq for dyn ClassInstance {
    fn eq(&self, other: &Self) -> bool {
        self.equals(other).unwrap()
    }
}

// array wrapper for ClassInstanceRef
pub struct Array<T>(PhantomData<T>);

// typesafe wrapper for ClassInstance
pub struct ClassInstanceRef<T> {
    pub instance: Option<Box<dyn ClassInstance>>,
    _phantom: PhantomData<T>,
}

impl<T> ClassInstanceRef<T> {
    pub fn new(instance: Option<Box<dyn ClassInstance>>) -> Self {
        Self {
            instance,
            _phantom: PhantomData,
        }
    }
}

impl<T> Clone for ClassInstanceRef<T> {
    fn clone(&self) -> Self {
        Self {
            instance: self.instance.clone(),
            _phantom: PhantomData,
        }
    }
}

impl<T> ClassInstanceRef<T> {
    pub fn is_null(&self) -> bool {
        self.instance.is_none()
    }
}

impl<T> Debug for ClassInstanceRef<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Some(x) = &self.instance {
            write!(f, "{:?}", x)
        } else {
            write!(f, "null")
        }
    }
}

impl<T> Deref for ClassInstanceRef<T> {
    type Target = Box<dyn ClassInstance>;
    fn deref(&self) -> &Self::Target {
        self.instance.as_ref().unwrap()
    }
}

impl<T> DerefMut for ClassInstanceRef<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.instance.as_mut().unwrap()
    }
}

impl<T> From<ClassInstanceRef<T>> for JavaValue {
    fn from(value: ClassInstanceRef<T>) -> Self {
        value.instance.into()
    }
}

impl<T> From<Box<dyn ClassInstance>> for ClassInstanceRef<T> {
    fn from(value: Box<dyn ClassInstance>) -> Self {
        Self {
            instance: Some(value),
            _phantom: PhantomData,
        }
    }
}

impl<T> From<Option<Box<dyn ClassInstance>>> for ClassInstanceRef<T> {
    fn from(value: Option<Box<dyn ClassInstance>>) -> Self {
        Self {
            instance: value,
            _phantom: PhantomData,
        }
    }
}

impl<T> From<JavaValue> for ClassInstanceRef<T> {
    fn from(val: JavaValue) -> Self {
        ClassInstanceRef {
            instance: val.into(),
            _phantom: PhantomData,
        }
    }
}

impl<T> From<ClassInstanceRef<T>> for Box<dyn ClassInstance> {
    fn from(value: ClassInstanceRef<T>) -> Self {
        value.instance.unwrap()
    }
}

impl<T> From<ClassInstanceRef<T>> for Option<Box<dyn ClassInstance>> {
    fn from(value: ClassInstanceRef<T>) -> Self {
        value.instance
    }
}
