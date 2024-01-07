use alloc::{
    format,
    rc::Rc,
    string::{String, ToString},
    vec,
    vec::Vec,
};
use core::cell::RefCell;

use jvm::{JavaValue, JvmResult};

use crate::JavaLangString;

pub async fn run_class(name: &str, class: &[u8], args: &[&str]) -> JvmResult<String> {
    let printed = Rc::new(RefCell::new(String::new()));

    let printed1 = printed.clone();
    let println_handler = move |x: &str| printed1.borrow_mut().push_str(&format!("{}\n", x));

    let mut jvm = crate::test_jvm(vec![(name.to_string(), class.to_vec())].into_iter().collect(), println_handler);

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
