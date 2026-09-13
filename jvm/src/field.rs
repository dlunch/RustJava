use alloc::borrow::Cow;
use core::fmt::Debug;

use jvm_types::FieldAccessFlags;

use crate::as_any::AsAny;

pub trait Field: Sync + Send + AsAny + Debug {
    fn name(&self) -> Cow<'_, str>;
    fn descriptor(&self) -> Cow<'_, str>;
    fn access_flags(&self) -> FieldAccessFlags;
}
