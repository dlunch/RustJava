use alloc::{
    boxed::Box,
    collections::BTreeMap,
    string::{String, ToString},
};

use dyn_clone::clone_box;

use jvm::{ArrayClass, Class, JvmDetail, JvmResult, ThreadContext, ThreadId};

use crate::{array_class::ArrayClassImpl, thread::ThreadContextImpl};

type ClassLoader = dyn Fn(&str) -> JvmResult<Option<Box<dyn Class>>>;

pub struct JvmDetailImpl {
    class_loader: Box<ClassLoader>,
    classes: BTreeMap<String, Box<dyn Class>>,
    thread_contexts: BTreeMap<ThreadId, Box<dyn ThreadContext>>,
}

impl JvmDetailImpl {
    pub fn new<T>(class_loader: T) -> Self
    where
        T: Fn(&str) -> JvmResult<Option<Box<dyn Class>>> + 'static,
    {
        Self {
            class_loader: Box::new(class_loader),
            classes: BTreeMap::new(),
            thread_contexts: BTreeMap::new(),
        }
    }
}

#[async_trait::async_trait(?Send)]
impl JvmDetail for JvmDetailImpl {
    async fn load_class(&mut self, class_name: &str) -> JvmResult<Option<Box<dyn Class>>> {
        let class = (self.class_loader)(class_name)?;

        if let Some(x) = class {
            self.classes.insert(class_name.to_string(), clone_box(&*x));

            Ok(Some(x))
        } else {
            Ok(None)
        }
    }

    async fn load_array_class(&mut self, element_type_name: &str) -> JvmResult<Option<Box<dyn ArrayClass>>> {
        Ok(Some(Box::new(ArrayClassImpl::new(element_type_name))))
    }

    fn thread_context(&mut self, thread_id: ThreadId) -> &mut dyn ThreadContext {
        let thread_context = self
            .thread_contexts
            .entry(thread_id)
            .or_insert_with(|| Box::new(ThreadContextImpl::new()));

        thread_context.as_mut()
    }
}
