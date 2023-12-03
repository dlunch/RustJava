use crate::{as_any::AsAny, JavaValue, Jvm, JvmResult};

pub trait Method: AsAny {
    fn name(&self) -> &str;
    fn descriptor(&self) -> &str;

    fn run(&self, jvm: &mut Jvm, args: &[JavaValue]) -> JvmResult<JavaValue>;
}
