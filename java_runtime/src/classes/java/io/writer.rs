use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use jvm::{Array, ClassInstanceRef, JavaChar, Jvm, Result};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::java::lang::{Object, String},
};

// abstract class java.io.Writer
pub struct Writer;

impl Writer {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/io/Writer",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, MethodAccessFlags::PROTECTED),
                JavaMethodProto::new("<init>", "(Ljava/lang/Object;)V", Self::init_with_lock, MethodAccessFlags::PROTECTED),
                JavaMethodProto::new("write", "(I)V", Self::write_char, Default::default()),
                JavaMethodProto::new("write", "([C)V", Self::write_chars, Default::default()),
                JavaMethodProto::new_abstract("write", "([CII)V", Default::default()),
                JavaMethodProto::new("write", "(Ljava/lang/String;)V", Self::write_string, Default::default()),
                JavaMethodProto::new("write", "(Ljava/lang/String;II)V", Self::write_string_offset, Default::default()),
                JavaMethodProto::new_abstract("flush", "()V", Default::default()),
                JavaMethodProto::new_abstract("close", "()V", Default::default()),
            ],
            fields: vec![JavaFieldProto::new("lock", "Ljava/lang/Object;", FieldAccessFlags::PROTECTED)],
            access_flags: ClassAccessFlags::ABSTRACT,
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.io.Writer::<init>({this:?})");

        let _: () = jvm
            .invoke_special(&this, "java/io/Writer", "<init>", "(Ljava/lang/Object;)V", (this.clone(),))
            .await?;

        Ok(())
    }

    async fn init_with_lock(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, lock: ClassInstanceRef<Object>) -> Result<()> {
        tracing::debug!("java.io.Writer::<init>({this:?}, {lock:?})");

        if lock.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "lock is null").await);
        }

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "lock", "Ljava/lang/Object;", lock).await?;

        Ok(())
    }

    async fn write_char(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: i32) -> Result<()> {
        tracing::debug!("java.io.Writer::write({this:?}, {value})");

        let mut chars = jvm.instantiate_array("C", 1).await?;
        jvm.store_array(&mut chars, 0, [value as JavaChar]).await?;
        jvm.invoke_virtual(&this, "write", "([CII)V", (chars, 0, 1)).await
    }

    async fn write_chars(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, chars: ClassInstanceRef<Array<JavaChar>>) -> Result<()> {
        tracing::debug!("java.io.Writer::write({this:?}, {chars:?})");

        let length = jvm.array_length(&chars).await? as i32;
        jvm.invoke_virtual(&this, "write", "([CII)V", (chars, 0, length)).await
    }

    async fn write_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, string: ClassInstanceRef<String>) -> Result<()> {
        tracing::debug!("java.io.Writer::write_string({this:?}, {string:?})");

        let chars: ClassInstanceRef<Array<JavaChar>> = jvm.invoke_virtual(&string, "toCharArray", "()[C", ()).await?;
        let length = jvm.array_length(&chars).await?;

        let _: () = jvm.invoke_virtual(&this, "write", "([CII)V", (chars, 0, length as i32)).await?;

        Ok(())
    }

    async fn write_string_offset(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        string: ClassInstanceRef<String>,
        off: i32,
        len: i32,
    ) -> Result<()> {
        tracing::debug!("java.io.Writer::write({this:?}, {string:?}, {off}, {len})");

        let chars: ClassInstanceRef<Array<JavaChar>> = jvm.invoke_virtual(&string, "toCharArray", "()[C", ()).await?;
        jvm.invoke_virtual(&this, "write", "([CII)V", (chars, off, len)).await
    }
}
