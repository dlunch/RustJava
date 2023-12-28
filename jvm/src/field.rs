use alloc::string::String;
use core::fmt::Debug;

use crate::{as_any::AsAny, r#type::JavaType};

pub trait Field: AsAny + Debug {
    fn name(&self) -> String;
    fn descriptor(&self) -> String;
    fn is_static(&self) -> bool;
    fn r#type(&self) -> JavaType;
}
