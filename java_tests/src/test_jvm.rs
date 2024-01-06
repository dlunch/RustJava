use alloc::{boxed::Box, collections::BTreeMap, string::String, vec::Vec};
use core::time::Duration;

use java_runtime::get_class_proto;
use jvm::{Class, Jvm, JvmCallback, JvmResult, Platform};
use jvm_impl::{ClassImpl, JvmDetailImpl};

fn get_class_loader(class_files: BTreeMap<String, Vec<u8>>) -> impl Fn(&str) -> JvmResult<Option<Box<dyn Class>>> {
    move |class_name| {
        let runtime_proto = get_class_proto(class_name);
        if let Some(x) = runtime_proto {
            Ok(Some(Box::new(ClassImpl::from_class_proto(class_name, x))))
        } else if class_files.contains_key(class_name) {
            Ok(Some(Box::new(ClassImpl::from_classfile(class_files.get(class_name).unwrap())?)))
        } else {
            Ok(None)
        }
    }
}

struct TestPlatform {
    println_handler: Box<dyn Fn(&str)>,
}

#[async_trait::async_trait(?Send)]
impl Platform for TestPlatform {
    async fn sleep(&self, _duration: Duration) {
        todo!()
    }

    async fn r#yield(&self) {
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
    let platform = TestPlatform {
        println_handler: Box::new(println_handler),
    };

    Jvm::new(JvmDetailImpl::new(get_class_loader(classes)), platform)
}
