use alloc::{boxed::Box, vec::Vec};

use crate::{class::Class, class_loader::ClassLoader, interpreter::Interpreter, stack_frame::StackFrame, JvmResult};

#[derive(Default)]
pub struct Jvm {
    class_loaders: Vec<Box<dyn ClassLoader>>,
    stack: Vec<StackFrame>, // TODO move to thread
}

impl Jvm {
    pub fn new() -> Jvm {
        Jvm {
            class_loaders: Vec::new(),
            stack: Vec::new(),
        }
    }

    pub fn add_class_loader<T>(&mut self, class_loader: T)
    where
        T: ClassLoader + 'static,
    {
        self.class_loaders.push(Box::new(class_loader));
    }

    pub fn invoke_static_method(&mut self, class_name: &str, name: &str, signature: &str) -> JvmResult<()> {
        let class = self.resolve_class(class_name)?.unwrap();
        let method = class.method(name, signature).unwrap();

        self.stack.push(StackFrame::new());

        let last = self.stack.len() - 1;
        Interpreter::run(&self.stack[last], &method.body)
    }

    fn resolve_class(&mut self, class_name: &str) -> JvmResult<Option<Class>> {
        for class_loader in &mut self.class_loaders {
            if let Some(x) = class_loader.load(class_name)? {
                return Ok(Some(x));
            }
        }

        Ok(None)
    }
}
