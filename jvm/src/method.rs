use alloc::{boxed::Box, string::String};

use crate::{as_any::AsAny, JavaValue, Jvm, JvmResult};

#[async_trait::async_trait(?Send)]
pub trait Method: AsAny {
    fn name(&self) -> String;
    fn descriptor(&self) -> String;

    async fn run(&self, jvm: &mut Jvm, args: &[JavaValue]) -> JvmResult<JavaValue>;
}
