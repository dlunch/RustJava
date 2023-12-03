extern crate alloc;

use alloc::collections::BTreeMap;

use jvm::{ArrayClass, Class, ClassLoader, JavaValue, Jvm, JvmResult};
use jvm_impl::{ArrayClassImpl, ClassImpl, FieldImpl, MethodBody, MethodImpl, ThreadContextProviderImpl};

struct TestClassLoader {
    class_files: BTreeMap<String, Vec<u8>>,
}

impl TestClassLoader {
    fn new(class_files: BTreeMap<String, Vec<u8>>) -> Self {
        Self { class_files }
    }

    fn system_clinit(jvm: &mut Jvm) -> JavaValue {
        let out = jvm.instantiate_class("java/io/PrintStream", "()V", &[]).unwrap();

        jvm.put_static_field("java/lang/System", "out", "Ljava/io/PrintStream;", JavaValue::Object(Some(out)))
            .unwrap();

        JavaValue::Void
    }
}

impl ClassLoader for TestClassLoader {
    fn load(&mut self, class_name: &str) -> JvmResult<Option<Box<dyn Class>>> {
        if class_name == "java/lang/String" {
            let class = ClassImpl::new(
                "java/lang/String",
                vec![MethodImpl::new("<init>", "([C)V", MethodBody::Rust(Box::new(|_| JavaValue::Void)))],
                vec![],
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
                    MethodImpl::new("<init>", "()V", MethodBody::Rust(Box::new(|_| JavaValue::Void))),
                    MethodImpl::new("println", "(Ljava/lang/String;)V", MethodBody::Rust(Box::new(|_| JavaValue::Void))),
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

    let mut jvm = Jvm::new(
        TestClassLoader::new(vec![("Hello".to_string(), hello.to_vec())].into_iter().collect()),
        &ThreadContextProviderImpl {},
    );

    jvm.invoke_static_method("Hello", "main", "([Ljava/lang/String;)V", &[])?;

    Ok(())
}
