#![allow(clippy::type_complexity)]
extern crate alloc;

use alloc::{collections::BTreeMap, rc::Rc, string::String, vec};
use core::{cell::RefCell, future::Future, pin::Pin};

use jvm::{runtime::JavaLangString, ArrayClass, Class, ClassLoader, JavaValue, Jvm, JvmResult};
use jvm_impl::{ArrayClassImpl, ClassImpl, FieldImpl, MethodBody, MethodImpl, ThreadContextProviderImpl};

struct TestClassLoader {
    class_files: BTreeMap<String, Vec<u8>>,
    println_handler: Rc<Box<dyn Fn(&str)>>,
}

impl TestClassLoader {
    pub fn new(class_files: BTreeMap<String, Vec<u8>>, println_handler: Box<dyn Fn(&str)>) -> Self {
        Self {
            class_files,
            println_handler: Rc::new(println_handler),
        }
    }

    async fn system_clinit(jvm: &mut Jvm, _args: &[JavaValue]) -> anyhow::Result<JavaValue> {
        let out = jvm.instantiate_class("java/io/PrintStream", "()V", &[]).await.unwrap();

        jvm.put_static_field("java/lang/System", "out", "Ljava/io/PrintStream;", JavaValue::Object(Some(out)))
            .await
            .unwrap();

        Ok(JavaValue::Void)
    }

    async fn string_init(jvm: &mut Jvm, args: &[JavaValue]) -> anyhow::Result<JavaValue> {
        let string = args[0].as_object().unwrap();
        let chars = args[1].as_object().unwrap();

        jvm.put_field(string, "value", "[C", JavaValue::Object(Some(chars.clone()))).unwrap();

        Ok(JavaValue::Void)
    }

    fn println(&self) -> Box<dyn Fn(&mut Jvm, &[JavaValue]) -> Pin<Box<dyn Future<Output = anyhow::Result<JavaValue>>>>> {
        let println_handler = self.println_handler.clone();

        let println_body = move |jvm: &mut Jvm, args: &[JavaValue]| -> Pin<Box<dyn Future<Output = anyhow::Result<JavaValue>>>> {
            let string = args[1].as_object().unwrap();

            let str = JavaLangString::from_instance(string.clone());
            println_handler(&str.to_string(jvm).unwrap());

            Box::pin(async { anyhow::Ok(JavaValue::Void) })
        };

        Box::new(println_body)
    }
}

impl ClassLoader for TestClassLoader {
    fn load(&mut self, class_name: &str) -> JvmResult<Option<Box<dyn Class>>> {
        if class_name == "java/lang/String" {
            let class = ClassImpl::new(
                "java/lang/String",
                vec![MethodImpl::new("<init>", "([C)V", MethodBody::from_rust(Self::string_init))],
                vec![FieldImpl::new("value", "[C", false, 0)],
            );

            Ok(Some(Box::new(class)))
        } else if class_name == "java/lang/System" {
            let class = ClassImpl::new(
                "java/lang/System",
                vec![MethodImpl::new("<clinit>", "()V", MethodBody::from_rust(Self::system_clinit))],
                vec![FieldImpl::new("out", "Ljava/io/PrintStream;", true, 0)],
            );

            Ok(Some(Box::new(class)))
        } else if class_name == "java/io/PrintStream" {
            let class = ClassImpl::new(
                "java/io/PrintStream",
                vec![
                    MethodImpl::new(
                        "<init>",
                        "()V",
                        MethodBody::from_rust(|_: &mut Jvm, _: &[JavaValue]| async { Ok(JavaValue::Void) }),
                    ),
                    MethodImpl::new("println", "(Ljava/lang/String;)V", MethodBody::from_rust(self.println())),
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

pub async fn run_class(name: &str, class: &[u8], args: &[&str]) -> JvmResult<String> {
    let printed = Rc::new(RefCell::new(String::new()));

    let printed1 = printed.clone();
    let println_handler = move |x: &str| printed1.borrow_mut().push_str(&format!("{}\n", x));

    let mut jvm = Jvm::new(
        TestClassLoader::new(vec![(name.to_string(), class.to_vec())].into_iter().collect(), Box::new(println_handler)),
        &ThreadContextProviderImpl {},
    );

    let mut java_args = Vec::with_capacity(args.len());
    for arg in args {
        java_args.push(JavaValue::Object(Some(JavaLangString::new(&mut jvm, arg).await?.instance)));
    }

    jvm.invoke_static_method(name, "main", "([Ljava/lang/String;)V", &java_args).await?;

    let result = printed.borrow().to_string();
    Ok(result)
}
