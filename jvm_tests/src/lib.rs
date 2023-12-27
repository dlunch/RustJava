#![allow(clippy::type_complexity)]
extern crate alloc;

use alloc::{collections::BTreeMap, rc::Rc, string::String, vec};
use core::{cell::RefCell, future::Future, pin::Pin};

use jvm::{runtime::JavaLangString, Class, JavaValue, Jvm, JvmResult};
use jvm_impl::{ClassImpl, FieldImpl, JvmDetailImpl, MethodBody, MethodImpl};

async fn system_clinit(jvm: &mut Jvm, _args: &[JavaValue]) -> anyhow::Result<JavaValue> {
    let out = jvm.instantiate_class("java/io/PrintStream").await.unwrap();
    jvm.invoke_method(&out, "java/io/PrintStream", "<init>", "()V", &[]).await.unwrap();

    jvm.put_static_field("java/lang/System", "out", "Ljava/io/PrintStream;", JavaValue::Object(Some(out)))
        .await
        .unwrap();

    Ok(JavaValue::Boolean(true)) // TODO
}

async fn string_init(jvm: &mut Jvm, args: &[JavaValue]) -> anyhow::Result<JavaValue> {
    let string = args[0].as_object_ref().unwrap();
    let chars = args[1].as_object_ref().unwrap();

    jvm.put_field(string, "value", "[C", JavaValue::Object(Some(chars.clone()))).unwrap();
    Ok(JavaValue::Boolean(true)) // TODO
}

fn get_println<T>(println_handler: Rc<T>) -> Box<dyn Fn(&mut Jvm, &[JavaValue]) -> Pin<Box<dyn Future<Output = anyhow::Result<JavaValue>>>>>
where
    T: Fn(&str) + 'static,
{
    let println_body = move |jvm: &mut Jvm, args: &[JavaValue]| -> Pin<Box<dyn Future<Output = anyhow::Result<JavaValue>>>> {
        let string = args[1].as_object_ref().unwrap();

        let str = JavaLangString::from_instance(string.clone());
        println_handler(&str.to_string(jvm).unwrap());

        Box::pin(async { Ok(JavaValue::Boolean(true)) }) // TODO
    };

    Box::new(println_body)
}

fn get_class_loader<T>(class_files: BTreeMap<String, Vec<u8>>, println_handler: T) -> impl Fn(&str) -> JvmResult<Option<Box<dyn Class>>>
where
    T: Fn(&str) + 'static,
{
    let println_handler = Rc::new(println_handler);
    move |class_name| {
        if class_name == "java/lang/String" {
            let class = ClassImpl::new(
                "java/lang/String",
                vec![MethodImpl::new("<init>", "([C)V", MethodBody::from_rust(string_init))],
                vec![FieldImpl::new("value", "[C", false, 0)],
            );

            Ok(Some(Box::new(class)))
        } else if class_name == "java/lang/System" {
            let class = ClassImpl::new(
                "java/lang/System",
                vec![MethodImpl::new("<clinit>", "()V", MethodBody::from_rust(system_clinit))],
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
                        MethodBody::from_rust(|_: &mut Jvm, _: &[JavaValue]| async { Ok(JavaValue::Boolean(true)) }), // TODO
                    ),
                    MethodImpl::new(
                        "println",
                        "(Ljava/lang/String;)V",
                        MethodBody::from_rust(get_println(println_handler.clone())),
                    ),
                ],
                vec![],
            );

            Ok(Some(Box::new(class)))
        } else if class_files.contains_key(class_name) {
            Ok(Some(Box::new(ClassImpl::from_classfile(class_files.get(class_name).unwrap())?)))
        } else {
            Ok(None)
        }
    }
}

pub async fn run_class(name: &str, class: &[u8], args: &[&str]) -> JvmResult<String> {
    let printed = Rc::new(RefCell::new(String::new()));

    let printed1 = printed.clone();
    let println_handler = move |x: &str| printed1.borrow_mut().push_str(&format!("{}\n", x));

    let mut jvm = Jvm::new(JvmDetailImpl::new(get_class_loader(
        vec![(name.to_string(), class.to_vec())].into_iter().collect(),
        Box::new(println_handler),
    )));

    let mut java_args = Vec::with_capacity(args.len());
    for arg in args {
        java_args.push(JavaValue::Object(Some(JavaLangString::new(&mut jvm, arg).await?.instance)));
    }

    jvm.invoke_static_method(name, "main", "([Ljava/lang/String;)V", &java_args).await?;

    let result = printed.borrow().to_string();
    Ok(result)
}
