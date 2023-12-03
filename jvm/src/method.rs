use downcast_rs::{impl_downcast, Downcast};

use crate::{JavaValue, Jvm, JvmResult};

pub trait Method: Downcast {
    fn name(&self) -> &str;
    fn descriptor(&self) -> &str;

    fn run(&self, jvm: &mut Jvm, args: &[JavaValue]) -> JvmResult<JavaValue>;
}
impl_downcast!(Method);
