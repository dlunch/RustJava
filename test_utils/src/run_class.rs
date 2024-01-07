use alloc::{
    boxed::Box,
    format,
    rc::Rc,
    string::{String, ToString},
    vec,
    vec::Vec,
};
use core::cell::RefCell;

use jvm::{ClassInstance, JavaValue, Jvm, JvmResult};

use crate::test_jvm;

async fn create_string(jvm: &mut Jvm, string: &str) -> JvmResult<Box<dyn ClassInstance>> {
    let chars = string.chars().map(|x| JavaValue::Char(x as _)).collect::<Vec<_>>();

    let mut array = jvm.instantiate_array("C", chars.len()).await?;
    jvm.store_array(&mut array, 0, chars)?;

    let instance = jvm.instantiate_class("java/lang/String").await?;
    jvm.invoke_virtual(&instance, "java/lang/String", "<init>", "([C)V", [array.into()])
        .await?;

    Ok(instance)
}

pub async fn run_class(name: &str, class: &[u8], args: &[&str]) -> JvmResult<String> {
    let printed = Rc::new(RefCell::new(String::new()));

    let printed1 = printed.clone();
    let println_handler = move |x: &str| printed1.borrow_mut().push_str(&format!("{}\n", x));

    let mut jvm = test_jvm(vec![(name.to_string(), class.to_vec())].into_iter().collect(), println_handler);

    let mut java_args = Vec::with_capacity(args.len());
    for arg in args {
        java_args.push(JavaValue::Object(Some(create_string(&mut jvm, arg).await?)));
    }
    let mut array = jvm.instantiate_array("Ljava/lang/String;", args.len()).await?;
    jvm.store_array(&mut array, 0, java_args).unwrap();

    jvm.invoke_static(name, "main", "([Ljava/lang/String;)V", [JavaValue::Object(Some(array))])
        .await?;

    let result = printed.borrow().to_string();
    Ok(result)
}
