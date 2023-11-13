use alloc::{boxed::Box, vec::Vec};

use crate::{class::Class, class_loader::ClassLoader, JvmResult};

#[derive(Default)]
pub struct Jvm {
    class_loaders: Vec<Box<dyn ClassLoader>>,
}

impl Jvm {
    pub fn new() -> Jvm {
        Jvm { class_loaders: Vec::new() }
    }

    pub fn add_class_loader<T>(&mut self, class_loader: T)
    where
        T: ClassLoader + 'static,
    {
        self.class_loaders.push(Box::new(class_loader));
    }

    pub fn resolve_class(&mut self, class_name: &str) -> JvmResult<Option<Class>> {
        for class_loader in &mut self.class_loaders {
            if let Some(x) = class_loader.load(class_name)? {
                return Ok(Some(x));
            }
        }

        Ok(None)
    }
}
