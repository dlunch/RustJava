use jvm::{ArrayClass, Class, ClassLoader, JavaValue, Jvm, JvmResult};
use jvm_impl::{ClassFileLoader, ClassImpl, FieldImpl, MethodBody, MethodImpl, ThreadContextProviderImpl};

struct TestLocalClassLoader {}

impl ClassLoader for TestLocalClassLoader {
    fn load(&mut self, class_name: &str) -> JvmResult<Option<Box<dyn Class>>> {
        if class_name == "java/lang/String" {
            let class = ClassImpl::new(
                "java/lang/String",
                vec![MethodImpl::new("<init>", "([C)V", MethodBody::Rust(Box::new(|| JavaValue::Void)))],
                vec![],
            );

            Ok(Some(Box::new(class)))
        } else if class_name == "java/lang/System" {
            let class = ClassImpl::new("java/lang/System", vec![], vec![FieldImpl::new("out", "Ljava/io/PrintStream;", true, 0)]);

            Ok(Some(Box::new(class)))
        } else {
            Ok(None)
        }
    }

    fn load_array_class(&mut self, _element_type_name: &str) -> JvmResult<Option<Box<dyn ArrayClass>>> {
        Ok(None)
    }
}

#[test]
fn test_hello() -> anyhow::Result<()> {
    let hello = include_bytes!("../../test_data/Hello.class");

    let mut jvm = Jvm::new(&ThreadContextProviderImpl {});
    jvm.add_class_loader(ClassFileLoader::new(vec![("Hello".to_string(), hello.to_vec())].into_iter().collect()));
    jvm.add_class_loader(TestLocalClassLoader {});

    jvm.invoke_static_method("Hello", "main", "([Ljava/lang/String;)V", &[])?;

    Ok(())
}
