use alloc::boxed::Box;

use crate::{as_any::AsAny, JavaValue, Jvm, JvmResult};

#[async_trait::async_trait(?Send)]
pub trait Method: AsAny {
    fn name(&self) -> &str;
    fn descriptor(&self) -> &str;

    async fn run(&self, jvm: &mut Jvm, args: &[JavaValue]) -> JvmResult<JavaValue>;
}
