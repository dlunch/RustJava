#![allow(clippy::type_complexity)]
extern crate alloc;

use alloc::{collections::BTreeMap, rc::Rc, string::String, vec};
use core::{cell::RefCell, future::Future, pin::Pin};

use jvm::{runtime::JavaLangString, Class, ClassInstance, JavaValue, Jvm, JvmResult};
use jvm_impl::{ClassImpl, FieldImpl, JvmDetailImpl, MethodBody, MethodImpl};

async fn system_clinit(jvm: &mut Jvm, _args: Box<[JavaValue]>) -> anyhow::Result<JavaValue> {
    let out = jvm.instantiate_class("java/io/PrintStream").await?;
    jvm.invoke_virtual(&out, "java/io/PrintStream", "<init>", "()V", []).await?;

    jvm.put_static_field("java/lang/System", "out", "Ljava/io/PrintStream;", JavaValue::Object(Some(out)))
        .await
        .unwrap();

    Ok(JavaValue::Void)
}

async fn string_init(jvm: &mut Jvm, args: Box<[JavaValue]>) -> anyhow::Result<JavaValue> {
    let mut args = args.to_vec();
    let mut string: Box<dyn ClassInstance> = args.remove(0).into();
    let chars: Box<dyn ClassInstance> = args.remove(0).into();

    jvm.put_field(&mut string, "value", "[C", JavaValue::Object(Some(chars.clone()))).unwrap();
    Ok(JavaValue::Void)
}

async fn integer_parse_int(jvm: &mut Jvm, args: Box<[JavaValue]>) -> anyhow::Result<JavaValue> {
    let mut args = args.to_vec();
    let string: Box<dyn ClassInstance> = args.remove(0).into();

    let str = JavaLangString::from_instance(string);
    let str = str.to_string(jvm).unwrap();

    let value = str.parse::<i32>().unwrap();

    Ok(JavaValue::Int(value))
}

fn get_println<T>(println_handler: Rc<T>) -> Box<dyn Fn(&mut Jvm, Box<[JavaValue]>) -> Pin<Box<dyn Future<Output = anyhow::Result<JavaValue>>>>>
where
    T: Fn(&str) + 'static,
{
    let println_body = move |jvm: &mut Jvm, args: Box<[JavaValue]>| -> Pin<Box<dyn Future<Output = anyhow::Result<JavaValue>>>> {
        let mut args = args.to_vec();
        let string: Box<dyn ClassInstance> = args.remove(1).into();

        let str = JavaLangString::from_instance(string.clone());
        println_handler(&str.to_string(jvm).unwrap());

        Box::pin(async { Ok(JavaValue::Void) })
    };

    Box::new(println_body)
}

fn get_class_loader<T>(class_files: BTreeMap<String, Vec<u8>>, println_handler: T) -> impl Fn(&str) -> JvmResult<Option<Box<dyn Class>>>
where
    T: Fn(&str) + 'static,
{
    let println_handler = Rc::new(println_handler);
    move |class_name| {
        if class_name == "java/lang/Object" {
            let class = ClassImpl::new(
                "java/lang/Object",
                None,
                vec![MethodImpl::new(
                    "<init>",
                    "()V",
                    MethodBody::from_rust(|_: &mut Jvm, _: Box<[JavaValue]>| async { Ok(JavaValue::Void) }),
                )],
                vec![],
            );

            Ok(Some(Box::new(class)))
        } else if class_name == "java/lang/String" {
            let class = ClassImpl::new(
                "java/lang/String",
                None, // TODO
                vec![MethodImpl::new("<init>", "([C)V", MethodBody::from_rust(string_init))],
                vec![FieldImpl::new("value", "[C", false, 0)],
            );

            Ok(Some(Box::new(class)))
        } else if class_name == "java/lang/System" {
            let class = ClassImpl::new(
                "java/lang/System",
                None, // TODO
                vec![MethodImpl::new("<clinit>", "()V", MethodBody::from_rust(system_clinit))],
                vec![FieldImpl::new("out", "Ljava/io/PrintStream;", true, 0)],
            );

            Ok(Some(Box::new(class)))
        } else if class_name == "java/io/PrintStream" {
            let class = ClassImpl::new(
                "java/io/PrintStream",
                None, // TODO
                vec![
                    MethodImpl::new(
                        "<init>",
                        "()V",
                        MethodBody::from_rust(|_: &mut Jvm, _: Box<[JavaValue]>| async { Ok(JavaValue::Void) }),
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
        } else if class_name == "java/lang/Integer" {
            let class = ClassImpl::new(
                "java/lang/Integer",
                None, // TODO
                vec![MethodImpl::new(
                    "parseInt",
                    "(Ljava/lang/String;)I",
                    MethodBody::from_rust(integer_parse_int),
                )],
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
    let mut array = jvm.instantiate_array("Ljava/lang/String;", args.len()).await?;
    jvm.store_array(&mut array, 0, java_args).unwrap();

    jvm.invoke_static(name, "main", "([Ljava/lang/String;)V", [JavaValue::Object(Some(array))])
        .await?;

    let result = printed.borrow().to_string();
    Ok(result)
}
