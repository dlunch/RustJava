use alloc::string::String;
use core::fmt::Debug;

use java_constants::FieldAccessFlags;

use crate::as_any::AsAny;

pub trait Field: Sync + Send + AsAny + Debug {
    fn name(&self) -> String;
    fn descriptor(&self) -> String;
    fn access_flags(&self) -> FieldAccessFlags;
}
