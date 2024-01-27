extern crate alloc;

mod runtime;

use alloc::string::String;
use core::future::ready;
use std::io::Write;

use bytemuck::cast_vec;

use java_runtime::{classes::java::lang::String as JavaString, Runtime};
use jvm::{JavaValue, Jvm, JvmResult};
use jvm_rust::{ClassImpl, JvmDetailImpl};

use runtime::RuntimeImpl;

pub async fn create_jvm<T>(stdout: T) -> JvmResult<Jvm>
where
    T: Write + 'static,
{
    let runtime = Box::new(RuntimeImpl::new(stdout)) as Box<dyn Runtime>;

    let jvm = Jvm::new(JvmDetailImpl::new()).await?;

    java_runtime::initialize(&jvm, |name, proto| {
        ready(Box::new(ClassImpl::from_class_proto(name, proto, runtime.clone())) as Box<_>)
    })
    .await?;

    Ok(jvm)
}

pub async fn load_class_file(jvm: &Jvm, class_name: &str, data: &[u8]) -> JvmResult<()> {
    let class_loader = jvm.get_system_class_loader().await?;

    let class_name = JavaString::from_rust_string(jvm, class_name).await?;

    let mut data_storage = jvm.instantiate_array("B", data.len()).await?;
    jvm.store_byte_array(&mut data_storage, 0, cast_vec(data.to_vec()))?;

    jvm.invoke_virtual(&class_loader, "addClassFile", "(Ljava/lang/String;[B)V", (class_name, data_storage))
        .await?;

    Ok(())
}

pub async fn load_jar_file(jvm: &Jvm, jar: &[u8]) -> JvmResult<String> {
    let class_loader = jvm.get_system_class_loader().await?;

    let mut data_storage = jvm.instantiate_array("B", jar.len()).await?;
    jvm.store_byte_array(&mut data_storage, 0, cast_vec(jar.to_vec()))?;

    let main_class_name = jvm
        .invoke_virtual(&class_loader, "addJarFile", "([B)Ljava/lang/String;", (data_storage,))
        .await?;

    JavaString::to_rust_string(jvm, &main_class_name)
}

pub async fn run_java_main(jvm: &Jvm, main_class_name: &str, args: &[String]) -> JvmResult<()> {
    let mut java_args = Vec::with_capacity(args.len());
    for arg in args {
        java_args.push(JavaString::from_rust_string(jvm, arg).await?);
    }
    let mut array = jvm.instantiate_array("Ljava/lang/String;", args.len()).await?;
    jvm.store_array(&mut array, 0, java_args).unwrap();

    jvm.invoke_static(main_class_name, "main", "([Ljava/lang/String;)V", [JavaValue::Object(Some(array))])
        .await?;

    Ok(())
}
