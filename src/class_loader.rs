use crate::{class::Class, JvmResult};

pub trait ClassLoader {
    fn load(&mut self, class_name: &str) -> JvmResult<Option<Class>>;
}
