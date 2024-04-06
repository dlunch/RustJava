use alloc::{string::ToString, vec};

use java_class_proto::JavaMethodProto;
use jvm::{runtime::JavaLangString, ClassInstanceRef, JavaChar, Jvm, Result};

use crate::{
    classes::java::{io::OutputStream, lang::String},
    RuntimeClassProto, RuntimeContext,
};

// class java.io.PrintStream
pub struct PrintStream {}

impl PrintStream {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            parent_class: Some("java/io/FilterOutputStream"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/io/OutputStream;)V", Self::init, Default::default()),
                JavaMethodProto::new("println", "(Ljava/lang/String;)V", Self::println_string, Default::default()),
                JavaMethodProto::new("println", "(I)V", Self::println_int, Default::default()),
                JavaMethodProto::new("println", "(J)V", Self::println_long, Default::default()),
                JavaMethodProto::new("println", "(C)V", Self::println_char, Default::default()),
                JavaMethodProto::new("println", "(B)V", Self::println_byte, Default::default()),
                JavaMethodProto::new("println", "(S)V", Self::println_short, Default::default()),
                JavaMethodProto::new("println", "(Z)V", Self::println_bool, Default::default()),
            ],
            fields: vec![],
        }
    }

    async fn init(_: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, out: ClassInstanceRef<OutputStream>) -> Result<()> {
        tracing::warn!("stub java.io.PrintStream::<init>({:?}, {:?})", &this, &out);

        Ok(())
    }

    async fn println_string(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>, str: ClassInstanceRef<String>) -> Result<()> {
        tracing::warn!("stub java.io.PrintStream::println({:?}, {:?})", &this, &str);

        if str.is_null() {
            context.println("null");
        } else {
            let rust_str = JavaLangString::to_rust_string(jvm, &str).await?;
            context.println(&rust_str);
        }

        Ok(())
    }

    async fn println_int(_: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>, int: i32) -> Result<()> {
        tracing::warn!("stub java.io.PrintStream::println({:?}, {:?})", &this, &int);

        context.println(&int.to_string());

        Ok(())
    }

    async fn println_long(_: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>, long: i64) -> Result<()> {
        tracing::warn!("stub java.io.PrintStream::println({:?}, {:?})", &this, &long);

        context.println(&long.to_string());

        Ok(())
    }

    async fn println_char(_: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>, char: JavaChar) -> Result<()> {
        tracing::warn!("stub java.io.PrintStream::println({:?}, {:?})", &this, &char);

        let char = char::from_u32(char as _).unwrap();

        context.println(&char.to_string());

        Ok(())
    }

    async fn println_byte(_: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>, byte: i8) -> Result<()> {
        tracing::warn!("stub java.io.PrintStream::println({:?}, {:?})", &this, &byte);

        context.println(&byte.to_string());

        Ok(())
    }

    async fn println_short(_: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>, short: i16) -> Result<()> {
        tracing::warn!("stub java.io.PrintStream::println({:?}, {:?})", &this, &short);

        context.println(&short.to_string());

        Ok(())
    }

    async fn println_bool(_: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>, bool: bool) -> Result<()> {
        tracing::warn!("stub java.io.PrintStream::println({:?}, {:?})", &this, &bool);

        context.println(&bool.to_string());

        Ok(())
    }
}
