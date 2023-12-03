use crate::{ClassInstance, JavaValue, JvmResult};

pub trait ArrayClassInstance: ClassInstance {
    fn store(&mut self, offset: usize, values: &[JavaValue]) -> JvmResult<()>;
}
