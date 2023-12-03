extern crate alloc;

use alloc::{format, rc::Rc, string::String, vec};
use core::cell::RefCell;

use jvm_tests::TestClassLoader;

use jvm::Jvm;
use jvm_impl::ThreadContextProviderImpl;

#[test]
fn test_hello() -> anyhow::Result<()> {
    let hello = include_bytes!("../../test_data/Hello.class");

    let printed = Rc::new(RefCell::new(String::new()));

    let printed1 = printed.clone();
    let println_handler = move |x: &str| printed1.borrow_mut().push_str(&format!("{}\n", x));

    let mut jvm = Jvm::new(
        TestClassLoader::new(
            vec![("Hello".to_string(), hello.to_vec())].into_iter().collect(),
            Box::new(println_handler),
        ),
        &ThreadContextProviderImpl {},
    );

    jvm.invoke_static_method("Hello", "main", "([Ljava/lang/String;)V", &[])?;

    assert_eq!(printed.borrow().as_str(), "Hello, world!\n");

    Ok(())
}
