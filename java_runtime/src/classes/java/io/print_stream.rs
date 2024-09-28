use alloc::{format, string::ToString, vec};

use bytemuck::cast_vec;

use java_class_proto::JavaMethodProto;
use jvm::{runtime::JavaLangString, ClassInstanceRef, JavaChar, Jvm, Result};

use crate::{
    classes::java::{
        io::OutputStream,
        lang::{Object, String},
    },
    RuntimeClassProto, RuntimeContext,
};

// class java.io.PrintStream
pub struct PrintStream;

impl PrintStream {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/io/PrintStream",
            parent_class: Some("java/io/FilterOutputStream"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/io/OutputStream;)V", Self::init, Default::default()),
                JavaMethodProto::new("println", "(Ljava/lang/Object;)V", Self::println_object, Default::default()),
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

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, out: ClassInstanceRef<OutputStream>) -> Result<()> {
        tracing::debug!("java.io.PrintStream::<init>({:?}, {:?})", &this, &out);

        let _: () = jvm
            .invoke_special(&this, "java/io/FilterOutputStream", "<init>", "(Ljava/io/OutputStream;)V", (out,))
            .await?;

        Ok(())
    }

    async fn println_object(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, obj: ClassInstanceRef<Object>) -> Result<()> {
        tracing::debug!("java.io.PrintStream::println({:?}, {:?})", &this, &obj);

        let result = if obj.is_null() {
            "null\n".into()
        } else {
            let string = jvm.invoke_virtual(&obj, "toString", "()Ljava/lang/String;", ()).await?;

            format!("{}\n", JavaLangString::to_rust_string(jvm, &string).await?)
        };

        let bytes = result.into_bytes();

        let mut string_bytes = jvm.instantiate_array("B", bytes.len()).await?;
        jvm.store_byte_array(&mut string_bytes, 0, cast_vec(bytes)).await?;

        let _: () = jvm.invoke_virtual(&this, "write", "([B)V", (string_bytes,)).await?;

        Ok(())
    }

    async fn println_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, str: ClassInstanceRef<String>) -> Result<()> {
        tracing::debug!("java.io.PrintStream::println({:?}, {:?})", &this, &str);

        let result = if str.is_null() {
            "null\n".into()
        } else {
            format!("{}\n", JavaLangString::to_rust_string(jvm, &str).await?)
        };

        let bytes = result.into_bytes();

        let mut string_bytes = jvm.instantiate_array("B", bytes.len()).await?;
        jvm.store_byte_array(&mut string_bytes, 0, cast_vec(bytes)).await?;

        let _: () = jvm.invoke_virtual(&this, "write", "([B)V", (string_bytes,)).await?;

        Ok(())
    }

    async fn println_int(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, int: i32) -> Result<()> {
        tracing::debug!("java.io.PrintStream::println({:?}, {:?})", &this, &int);

        let java_string = JavaLangString::from_rust_string(jvm, &int.to_string()).await?;

        let _: () = jvm.invoke_virtual(&this, "println", "(Ljava/lang/String;)V", (java_string,)).await?;

        Ok(())
    }

    async fn println_long(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, long: i64) -> Result<()> {
        tracing::debug!("java.io.PrintStream::println({:?}, {:?})", &this, &long);

        let java_string = JavaLangString::from_rust_string(jvm, &long.to_string()).await?;

        let _: () = jvm.invoke_virtual(&this, "println", "(Ljava/lang/String;)V", (java_string,)).await?;

        Ok(())
    }

    async fn println_char(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, char: JavaChar) -> Result<()> {
        tracing::debug!("java.io.PrintStream::println({:?}, {:?})", &this, &char);

        let char = char::from_u32(char as _).unwrap();

        let java_string = JavaLangString::from_rust_string(jvm, &char.to_string()).await?;

        let _: () = jvm.invoke_virtual(&this, "println", "(Ljava/lang/String;)V", (java_string,)).await?;

        Ok(())
    }

    async fn println_byte(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, byte: i8) -> Result<()> {
        tracing::debug!("java.io.PrintStream::println({:?}, {:?})", &this, &byte);

        let java_string = JavaLangString::from_rust_string(jvm, &byte.to_string()).await?;

        let _: () = jvm.invoke_virtual(&this, "println", "(Ljava/lang/String;)V", (java_string,)).await?;

        Ok(())
    }

    async fn println_short(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, short: i16) -> Result<()> {
        tracing::debug!("java.io.PrintStream::println({:?}, {:?})", &this, &short);

        let java_string = JavaLangString::from_rust_string(jvm, &short.to_string()).await?;

        let _: () = jvm.invoke_virtual(&this, "println", "(Ljava/lang/String;)V", (java_string,)).await?;

        Ok(())
    }

    async fn println_bool(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, bool: bool) -> Result<()> {
        tracing::debug!("java.io.PrintStream::println({:?}, {:?})", &this, &bool);

        let java_string = JavaLangString::from_rust_string(jvm, &bool.to_string()).await?;

        let _: () = jvm.invoke_virtual(&this, "println", "(Ljava/lang/String;)V", (java_string,)).await?;

        Ok(())
    }
}
