extern crate alloc;

use alloc::{format, rc::Rc, string::String, vec};
use core::cell::RefCell;

use jvm_tests::TestClassLoader;

use jvm::{runtime::JavaLangString, JavaValue, Jvm};
use jvm_impl::ThreadContextProviderImpl;

// #[test]
pub fn test_hello() -> anyhow::Result<()> {
    let odd_even = include_bytes!("../../test_data/OddEven.class");

    let printed = Rc::new(RefCell::new(String::new()));

    let printed1 = printed.clone();
    let println_handler = move |x: &str| printed1.borrow_mut().push_str(&format!("{}\n", x));

    let mut jvm = Jvm::new(
        TestClassLoader::new(
            vec![("OddEven".to_string(), odd_even.to_vec())].into_iter().collect(),
            Box::new(println_handler),
        ),
        &ThreadContextProviderImpl {},
    );

    let param = JavaLangString::new(&mut jvm, "1234")?;
    jvm.invoke_static_method("OddEven", "main", "([Ljava/lang/String;)V", &[JavaValue::Object(Some(param.instance))])?;

    assert_eq!(printed.borrow().as_str(), "i is even\n");

    let param = JavaLangString::new(&mut jvm, "1233")?;
    jvm.invoke_static_method("OddEven", "main", "([Ljava/lang/String;)V", &[JavaValue::Object(Some(param.instance))])?;

    assert_eq!(printed.borrow().as_str(), "i is even\n");

    Ok(())
}
