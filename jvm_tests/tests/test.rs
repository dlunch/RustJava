#![allow(clippy::type_complexity)]
extern crate alloc;

use alloc::{collections::BTreeMap, format, rc::Rc, string::String, vec};
use core::cell::RefCell;

use jvm::{runtime::JavaLangString, ArrayClass, Class, ClassLoader, JavaValue, Jvm, JvmResult};
use jvm_impl::{ArrayClassImpl, ClassImpl, FieldImpl, MethodBody, MethodImpl, ThreadContextProviderImpl};

struct TestClassLoader {
    class_files: BTreeMap<String, Vec<u8>>,
    println_handler: Rc<Box<dyn Fn(&str)>>,
}

impl TestClassLoader {
    fn new(class_files: BTreeMap<String, Vec<u8>>, println_handler: Box<dyn Fn(&str)>) -> Self {
        Self {
            class_files,
            println_handler: Rc::new(println_handler),
        }
    }

    fn system_clinit(jvm: &mut Jvm, _args: &[JavaValue]) -> JavaValue {
        let out = jvm.instantiate_class("java/io/PrintStream", "()V", &[]).unwrap();

        jvm.put_static_field("java/lang/System", "out", "Ljava/io/PrintStream;", JavaValue::Object(Some(out)))
            .unwrap();

        JavaValue::Void
    }

    fn string_init(jvm: &mut Jvm, args: &[JavaValue]) -> JavaValue {
        let string = args[0].as_object().unwrap();
        let chars = args[1].as_object().unwrap();

        jvm.put_field(string, "value", "[C", JavaValue::Object(Some(chars.clone()))).unwrap();

        JavaValue::Void
    }

    fn println(&self) -> Box<dyn Fn(&mut Jvm, &[JavaValue]) -> JavaValue> {
        let println_handler = self.println_handler.clone();

        let println_body = move |jvm: &mut Jvm, args: &[JavaValue]| -> JavaValue {
            let string = args[1].as_object().unwrap();

            let str = JavaLangString::from_instance(string.clone());
            println_handler(&str.to_string(jvm).unwrap());

            JavaValue::Void
        };

        Box::new(println_body)
    }
}

impl ClassLoader for TestClassLoader {
    fn load(&mut self, class_name: &str) -> JvmResult<Option<Box<dyn Class>>> {
        if class_name == "java/lang/String" {
            let class = ClassImpl::new(
                "java/lang/String",
                vec![MethodImpl::new("<init>", "([C)V", MethodBody::Rust(Box::new(Self::string_init)))],
                vec![FieldImpl::new("value", "[C", false, 0)],
            );

            Ok(Some(Box::new(class)))
        } else if class_name == "java/lang/System" {
            let class = ClassImpl::new(
                "java/lang/System",
                vec![MethodImpl::new("<clinit>", "()V", MethodBody::Rust(Box::new(Self::system_clinit)))],
                vec![FieldImpl::new("out", "Ljava/io/PrintStream;", true, 0)],
            );

            Ok(Some(Box::new(class)))
        } else if class_name == "java/io/PrintStream" {
            let class = ClassImpl::new(
                "java/io/PrintStream",
                vec![
                    MethodImpl::new("<init>", "()V", MethodBody::Rust(Box::new(|_, _| JavaValue::Void))),
                    MethodImpl::new("println", "(Ljava/lang/String;)V", MethodBody::Rust(self.println())),
                ],
                vec![],
            );

            Ok(Some(Box::new(class)))
        } else if self.class_files.contains_key(class_name) {
            Ok(Some(Box::new(ClassImpl::from_classfile(self.class_files.get(class_name).unwrap())?)))
        } else {
            Ok(None)
        }
    }

    fn load_array_class(&mut self, element_type_name: &str) -> JvmResult<Option<Box<dyn ArrayClass>>> {
        Ok(Some(Box::new(ArrayClassImpl::new(element_type_name))))
    }
}

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
