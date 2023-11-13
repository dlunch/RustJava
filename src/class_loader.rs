use crate::{class::Class, JvmResult};

use cafebabe::parse_class;

pub trait ClassLoader {
    fn parse_class(data: &[u8]) -> JvmResult<Class>
    where
        Self: Sized,
    {
        let _class = parse_class(data)?;

        Ok(Class {})
    }

    fn load(&mut self, class_name: &str) -> JvmResult<Option<Class>>;
}
