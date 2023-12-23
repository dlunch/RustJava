use alloc::string::String;

use crate::{as_any::AsAny, r#type::JavaType};

pub trait Field: AsAny {
    fn name(&self) -> String;
    fn descriptor(&self) -> String;
    fn is_static(&self) -> bool;
    fn r#type(&self) -> JavaType;
}
