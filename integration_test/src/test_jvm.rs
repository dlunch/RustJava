use alloc::{boxed::Box, collections::BTreeMap, rc::Rc, string::String, vec::Vec};
use core::time::Duration;

use java_class_proto::JavaClassProto;
use java_runtime::{get_class_proto, Runtime};
use jvm::{Class, Jvm, JvmCallback, JvmResult};
use jvm_rust::{ClassImpl, JvmDetailImpl};

fn get_class_loader(class_files: BTreeMap<String, Vec<u8>>, runtime: Box<dyn Runtime>) -> impl Fn(&str) -> JvmResult<Option<Box<dyn Class>>> {
    move |class_name| {
        let runtime_proto = get_class_proto(class_name);
        if let Some(x) = runtime_proto {
            Ok(Some(Box::new(ClassImpl::from_class_proto(class_name, x, runtime.clone()))))
        } else if class_files.contains_key(class_name) {
            Ok(Some(Box::new(ClassImpl::from_classfile(class_files.get(class_name).unwrap())?)))
        } else {
            Ok(None)
        }
    }
}

#[derive(Clone)]
#[allow(clippy::type_complexity)]
struct TestRuntime {
    println_handler: Rc<Box<dyn Fn(&str)>>,
}

#[async_trait::async_trait(?Send)]
impl Runtime for TestRuntime {
    async fn sleep(&self, _duration: Duration) {
        todo!()
    }

    async fn r#yield(&self) {
        todo!()
    }

    fn define_class(&self, _name: &str, _data: &[u8]) -> Box<dyn Class> {
        todo!()
    }

    fn define_class_proto(&self, _name: &str, _proto: JavaClassProto<dyn Runtime>) -> Box<dyn Class> {
        todo!()
    }

    fn spawn(&self, _callback: Box<dyn JvmCallback>) {
        todo!()
    }

    fn now(&self) -> u64 {
        todo!()
    }

    fn encode_str(&self, _s: &str) -> Vec<u8> {
        todo!()
    }

    fn decode_str(&self, _bytes: &[u8]) -> String {
        todo!()
    }

    fn load_resource(&self, _name: &str) -> Option<Vec<u8>> {
        todo!()
    }

    fn println(&self, s: &str) {
        (self.println_handler)(s)
    }
}

pub fn test_jvm<T>(classes: BTreeMap<String, Vec<u8>>, println_handler: T) -> Jvm
where
    T: Fn(&str) + 'static,
{
    let platform = TestRuntime {
        println_handler: Rc::new(Box::new(println_handler)),
    };

    Jvm::new(JvmDetailImpl::new(get_class_loader(classes, Box::new(platform))))
}
