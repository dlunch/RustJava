#![no_std]

extern crate alloc;

use alloc::{
    boxed::Box,
    collections::BTreeMap,
    format,
    rc::Rc,
    string::{String, ToString},
    vec,
    vec::Vec,
};
use core::{cell::RefCell, time::Duration};

use java_runtime::get_class_proto;
use jvm::{runtime::JavaLangString, Class, JavaValue, Jvm, JvmCallback, JvmResult, Platform};
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

pub async fn run_class(name: &str, class: &[u8], args: &[&str]) -> JvmResult<String> {
    let printed = Rc::new(RefCell::new(String::new()));

    let printed1 = printed.clone();
    let println_handler = move |x: &str| printed1.borrow_mut().push_str(&format!("{}\n", x));

    let platform = TestPlatform {
        println_handler: Box::new(println_handler),
    };

    let mut jvm = Jvm::new(
        JvmDetailImpl::new(get_class_loader(vec![(name.to_string(), class.to_vec())].into_iter().collect())),
        platform,
    );

    let mut java_args = Vec::with_capacity(args.len());
    for arg in args {
        java_args.push(JavaValue::Object(Some(JavaLangString::new(&mut jvm, arg).await?.instance)));
    }
    let mut array = jvm.instantiate_array("Ljava/lang/String;", args.len()).await?;
    jvm.store_array(&mut array, 0, java_args).unwrap();

    jvm.invoke_static(name, "main", "([Ljava/lang/String;)V", [JavaValue::Object(Some(array))])
        .await?;

    let result = printed.borrow().to_string();
    Ok(result)
}
