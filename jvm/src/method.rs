use alloc::{borrow::Cow, boxed::Box};
use core::fmt::Debug;

use jvm_types::MethodAccessFlags;

use crate::{JavaValue, Jvm, Result, as_any::AsAny};

#[async_trait::async_trait]
pub trait Method: Sync + Send + AsAny + Debug {
    fn name(&self) -> Cow<'_, str>;
    fn descriptor(&self) -> Cow<'_, str>;
    fn access_flags(&self) -> MethodAccessFlags;

    async fn run(&self, jvm: &Jvm, args: Box<[JavaValue]>) -> Result<JavaValue>;
}
