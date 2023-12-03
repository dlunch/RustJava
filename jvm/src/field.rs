use downcast_rs::{impl_downcast, Downcast};

use crate::r#type::JavaType;

pub trait Field: Downcast {
    fn name(&self) -> &str;
    fn descriptor(&self) -> &str;
    fn is_static(&self) -> bool;
    fn r#type(&self) -> JavaType;
}
impl_downcast!(Field);
