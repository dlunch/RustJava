use alloc::{boxed::Box, vec::Vec};

use crate::{ClassInstance, JavaValue, JvmResult};

pub trait ArrayClassInstance: ClassInstance {
    fn store(&mut self, offset: usize, values: Box<[JavaValue]>) -> JvmResult<()>;
    fn load(&self, offset: usize, count: usize) -> JvmResult<Vec<JavaValue>>;
    fn length(&self) -> usize;
}
