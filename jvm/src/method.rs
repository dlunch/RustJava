use alloc::{boxed::Box, string::String};
use core::fmt::Debug;

use crate::{as_any::AsAny, JavaValue, Jvm, JvmResult};

#[async_trait::async_trait(?Send)]
pub trait Method: AsAny + Debug {
    fn name(&self) -> String;
    fn descriptor(&self) -> String;
    fn is_static(&self) -> bool;

    async fn run(&self, jvm: &mut Jvm, args: Box<[JavaValue]>) -> JvmResult<JavaValue>;
}
