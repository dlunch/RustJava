use crate::{as_any::AsAny, r#type::JavaType};

pub trait Field: AsAny {
    fn name(&self) -> &str;
    fn descriptor(&self) -> &str;
    fn is_static(&self) -> bool;
    fn r#type(&self) -> JavaType;
}
