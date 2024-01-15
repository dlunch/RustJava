use alloc::{boxed::Box, collections::BTreeMap, rc::Rc, string::String, vec::Vec};
use core::time::Duration;

use bytemuck::cast_vec;

use java_runtime::{classes::java::lang::String as JavaString, Runtime};
use jvm::{Jvm, JvmCallback, JvmResult};
use jvm_rust::{ClassImpl, JvmDetailImpl};

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

pub async fn test_jvm<T>(classes: BTreeMap<String, Vec<u8>>, println_handler: T) -> JvmResult<Jvm>
where
    T: Fn(&str) + 'static,
{
    let runtime = Box::new(TestRuntime {
        println_handler: Rc::new(Box::new(println_handler)),
    });

    let mut jvm = Jvm::new(JvmDetailImpl::new()).await?;

    java_runtime::initialize(&mut jvm, |name, proto| {
        Box::new(ClassImpl::from_class_proto(name, proto, runtime.clone() as Box<_>))
    })
    .await?;

    let class_loader = jvm.get_system_class_loader().clone();

    for (name, data) in classes {
        let class_name = JavaString::from_rust_string(&mut jvm, &name).await?;

        let mut data_storage = jvm.instantiate_array("B", data.len()).await?;
        jvm.store_byte_array(&mut data_storage, 0, cast_vec(data))?;

        jvm.invoke_virtual(
            &class_loader,
            "rustjava/ClassPathClassLoader",
            "addClassFile",
            "(Ljava/lang/String;[B)V",
            (class_name, data_storage),
        )
        .await?;
    }

    Ok(jvm)
}
