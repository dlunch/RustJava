use alloc::{string::String as RustString, vec, vec::Vec};

use jvm::{runtime::JavaLangString, JavaValue};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{FieldAccessFlags, MethodAccessFlags};
use jvm::{Array, ClassInstanceRef, Jvm, Result};

use crate::{
    classes::java::{io::FileDescriptor, lang::String},
    RuntimeClassProto, RuntimeContext,
};

// class java.lang.System
pub struct System;

impl System {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/lang/System",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<clinit>", "()V", Self::cl_init, MethodAccessFlags::STATIC),
                JavaMethodProto::new(
                    "currentTimeMillis",
                    "()J",
                    Self::current_time_millis,
                    MethodAccessFlags::NATIVE | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new("gc", "()V", Self::gc, MethodAccessFlags::STATIC),
                JavaMethodProto::new(
                    "arraycopy",
                    "(Ljava/lang/Object;ILjava/lang/Object;II)V",
                    Self::arraycopy,
                    MethodAccessFlags::NATIVE | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "getProperty",
                    "(Ljava/lang/String;)Ljava/lang/String;",
                    Self::get_property,
                    MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "setProperty",
                    "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;",
                    Self::set_property,
                    MethodAccessFlags::STATIC,
                ),
            ],
            fields: vec![
                JavaFieldProto::new("out", "Ljava/io/PrintStream;", FieldAccessFlags::STATIC),
                JavaFieldProto::new("props", "Ljava/util/Properties;", FieldAccessFlags::STATIC),
            ],
        }
    }

    async fn cl_init(jvm: &Jvm, _: &mut RuntimeContext) -> Result<()> {
        tracing::debug!("java.lang.System::<clinit>()");

        let out_descriptor: ClassInstanceRef<FileDescriptor> =
            jvm.get_static_field("java/io/FileDescriptor", "out", "Ljava/io/FileDescriptor;").await?;
        let file_output_stream = jvm
            .new_class("java/io/FileOutputStream", "(Ljava/io/FileDescriptor;)V", (out_descriptor,))
            .await?;
        let out = jvm
            .new_class("java/io/PrintStream", "(Ljava/io/OutputStream;)V", (file_output_stream,))
            .await?;

        jvm.put_static_field("java/lang/System", "out", "Ljava/io/PrintStream;", out).await?;

        let props = jvm.new_class("java/util/Properties", "()V", ()).await?;
        jvm.put_static_field("java/lang/System", "props", "Ljava/util/Properties;", props).await?;

        Ok(())
    }

    async fn current_time_millis(_: &Jvm, context: &mut RuntimeContext) -> Result<i64> {
        tracing::debug!("java.lang.System::currentTimeMillis()");

        Ok(context.now() as _)
    }

    async fn gc(_: &Jvm, _: &mut RuntimeContext) -> Result<i32> {
        tracing::warn!("stub java.lang.System::gc()");

        Ok(0)
    }

    async fn arraycopy(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        src: ClassInstanceRef<Array<()>>, // Any Array
        src_pos: i32,
        mut dest: ClassInstanceRef<Array<()>>,
        dest_pos: i32,
        length: i32,
    ) -> Result<()> {
        tracing::debug!(
            "java.lang.System::arraycopy({:?}, {}, {:?}, {}, {})",
            &src,
            src_pos,
            &dest,
            dest_pos,
            length
        );

        // TODO i think we can make it faster
        let src: Vec<JavaValue> = jvm.load_array(&src, src_pos as _, length as _).await?;
        jvm.store_array(&mut dest, dest_pos as _, src).await?;

        Ok(())
    }

    async fn get_property(jvm: &Jvm, _: &mut RuntimeContext, key: ClassInstanceRef<String>) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.lang.System::getProperty({:?})", key);

        let props = jvm.get_static_field("java/lang/System", "props", "Ljava/util/Properties;").await?;
        let value = jvm
            .invoke_virtual(&props, "getProperty", "(Ljava/lang/String;)Ljava/lang/String;", (key,))
            .await?;

        Ok(value)
    }

    async fn set_property(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        key: ClassInstanceRef<String>,
        value: ClassInstanceRef<String>,
    ) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.lang.System::setProperty({:?}, {:?})", key, value);

        let props = jvm.get_static_field("java/lang/System", "props", "Ljava/util/Properties;").await?;
        let value = jvm
            .invoke_virtual(
                &props,
                "setProperty",
                "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;",
                (key, value),
            )
            .await?;

        Ok(value)
    }

    pub async fn get_charset(jvm: &Jvm) -> Result<RustString> {
        let charset: ClassInstanceRef<Self> = jvm
            .invoke_static(
                "java/lang/System",
                "getProperty",
                "(Ljava/lang/String;)Ljava/lang/String;",
                (JavaLangString::from_rust_string(jvm, "file.encoding").await?,),
            )
            .await?;

        Ok(if !charset.is_null() {
            JavaLangString::to_rust_string(jvm, &charset).await?
        } else {
            "UTF-8".into()
        })
    }
}
