use alloc::boxed::Box;

use crate::{Class, JvmResult};

pub trait ClassLoader {
    fn load(&mut self, class_name: &str) -> JvmResult<Option<Box<dyn Class>>>;
}
