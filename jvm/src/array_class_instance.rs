use alloc::{boxed::Box, vec::Vec};

use crate::{ClassInstance, JavaValue, JvmResult};

pub trait ArrayClassInstance: ClassInstance {
    fn store(&mut self, offset: usize, values: Box<[JavaValue]>) -> JvmResult<()>;
    fn load(&self, offset: usize, count: usize) -> JvmResult<Vec<JavaValue>>;
    fn store_bytes(&mut self, offset: usize, values: Box<[i8]>) -> JvmResult<()>;
    fn load_bytes(&self, offset: usize, count: usize) -> JvmResult<Vec<i8>>;
    fn length(&self) -> usize;
}
