use alloc::string::String;
use core::fmt::Debug;

use java_constants::FieldAccessFlags;

use crate::{as_any::AsAny, r#type::JavaType};

pub trait Field: AsAny + Debug {
    fn name(&self) -> String;
    fn descriptor(&self) -> String;
    fn r#type(&self) -> JavaType;
    fn access_flags(&self) -> FieldAccessFlags;
}
