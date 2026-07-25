use alloc::{string::String as RustString, vec, vec::Vec};

use jvm::{JavaValue, runtime::JavaLangString};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{FieldAccessFlags, MethodAccessFlags};
use jvm::{Array, ClassInstanceRef, Jvm, Result};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::java::{
        io::{FileDescriptor, InputStream, PrintStream},
        lang::{Object, String},
        util::Properties,
    },
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
                JavaMethodProto::new("exit", "(I)V", Self::exit, MethodAccessFlags::STATIC),
                JavaMethodProto::new(
                    "identityHashCode",
                    "(Ljava/lang/Object;)I",
                    Self::identity_hash_code,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "setIn",
                    "(Ljava/io/InputStream;)V",
                    Self::set_in,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "setOut",
                    "(Ljava/io/PrintStream;)V",
                    Self::set_out,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "setErr",
                    "(Ljava/io/PrintStream;)V",
                    Self::set_err,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "getProperties",
                    "()Ljava/util/Properties;",
                    Self::get_properties,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "getProperty",
                    "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;",
                    Self::get_property_with_default,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
            ],
            fields: vec![
                JavaFieldProto::new(
                    "in",
                    "Ljava/io/InputStream;",
                    FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new(
                    "out",
                    "Ljava/io/PrintStream;",
                    FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new(
                    "err",
                    "Ljava/io/PrintStream;",
                    FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new("props", "Ljava/util/Properties;", FieldAccessFlags::PRIVATE | FieldAccessFlags::STATIC),
            ],
            access_flags: Default::default(),
        }
    }

    async fn cl_init(jvm: &Jvm, _: &mut RuntimeContext) -> Result<()> {
        tracing::debug!("java.lang.System::<clinit>()");

        let in_descriptor: ClassInstanceRef<FileDescriptor> =
            jvm.get_static_field("java/io/FileDescriptor", "in", "Ljava/io/FileDescriptor;").await?;
        if !in_descriptor.is_null() {
            let input = jvm
                .new_class("java/io/FileInputStream", "(Ljava/io/FileDescriptor;)V", (in_descriptor,))
                .await?;
            jvm.put_static_field("java/lang/System", "in", "Ljava/io/InputStream;", input).await?;
        }

        let out_descriptor: ClassInstanceRef<FileDescriptor> =
            jvm.get_static_field("java/io/FileDescriptor", "out", "Ljava/io/FileDescriptor;").await?;
        let out_file_output_stream = jvm
            .new_class("java/io/FileOutputStream", "(Ljava/io/FileDescriptor;)V", (out_descriptor,))
            .await?;
        let out = jvm
            .new_class("java/io/PrintStream", "(Ljava/io/OutputStream;)V", (out_file_output_stream,))
            .await?;

        let err_descriptor: ClassInstanceRef<FileDescriptor> =
            jvm.get_static_field("java/io/FileDescriptor", "err", "Ljava/io/FileDescriptor;").await?;
        let err_file_output_stream = jvm
            .new_class("java/io/FileOutputStream", "(Ljava/io/FileDescriptor;)V", (err_descriptor,))
            .await?;
        let err = jvm
            .new_class("java/io/PrintStream", "(Ljava/io/OutputStream;)V", (err_file_output_stream,))
            .await?;

        jvm.put_static_field("java/lang/System", "out", "Ljava/io/PrintStream;", out).await?;
        jvm.put_static_field("java/lang/System", "err", "Ljava/io/PrintStream;", err).await?;

        let props = jvm.new_class("java/util/Properties", "()V", ()).await?;
        jvm.put_static_field("java/lang/System", "props", "Ljava/util/Properties;", props).await?;

        Ok(())
    }

    async fn current_time_millis(_: &Jvm, context: &mut RuntimeContext) -> Result<i64> {
        tracing::debug!("java.lang.System::currentTimeMillis()");

        Ok(context.now() as _)
    }

    async fn gc(jvm: &Jvm, _: &mut RuntimeContext) -> Result<()> {
        tracing::debug!("java.lang.System::gc()");

        jvm.collect_garbage()?;

        Ok(())
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
        tracing::debug!("java.lang.System::arraycopy({src:?}, {src_pos}, {dest:?}, {dest_pos}, {length})");

        // TODO i think we can make it faster
        let src: Vec<JavaValue> = jvm.load_array(&src, src_pos as _, length as _).await?;
        jvm.store_array(&mut dest, dest_pos as _, src).await?;

        Ok(())
    }

    async fn get_property(jvm: &Jvm, _: &mut RuntimeContext, key: ClassInstanceRef<String>) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.lang.System::getProperty({key:?})");

        if key.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "key").await);
        }

        let props = jvm.get_static_field("java/lang/System", "props", "Ljava/util/Properties;").await?;
        let value = jvm
            .invoke_virtual(&props, "getProperty", "(Ljava/lang/String;)Ljava/lang/String;", (key,))
            .await?;

        Ok(value)
    }

    async fn get_property_with_default(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        key: ClassInstanceRef<String>,
        default_value: ClassInstanceRef<String>,
    ) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.lang.System::getProperty({key:?}, {default_value:?})");

        if key.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "key").await);
        }

        let props = jvm.get_static_field("java/lang/System", "props", "Ljava/util/Properties;").await?;
        jvm.invoke_virtual(
            &props,
            "getProperty",
            "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;",
            (key, default_value),
        )
        .await
    }

    async fn get_properties(jvm: &Jvm, _: &mut RuntimeContext) -> Result<ClassInstanceRef<Properties>> {
        tracing::debug!("java.lang.System::getProperties()");

        jvm.get_static_field("java/lang/System", "props", "Ljava/util/Properties;").await
    }

    async fn set_in(jvm: &Jvm, _: &mut RuntimeContext, input: ClassInstanceRef<InputStream>) -> Result<()> {
        tracing::debug!("java.lang.System::setIn({input:?})");

        jvm.put_static_field("java/lang/System", "in", "Ljava/io/InputStream;", input).await
    }

    async fn set_out(jvm: &Jvm, _: &mut RuntimeContext, output: ClassInstanceRef<PrintStream>) -> Result<()> {
        tracing::debug!("java.lang.System::setOut({output:?})");

        jvm.put_static_field("java/lang/System", "out", "Ljava/io/PrintStream;", output).await
    }

    async fn set_err(jvm: &Jvm, _: &mut RuntimeContext, error: ClassInstanceRef<PrintStream>) -> Result<()> {
        tracing::debug!("java.lang.System::setErr({error:?})");

        jvm.put_static_field("java/lang/System", "err", "Ljava/io/PrintStream;", error).await
    }

    async fn identity_hash_code(_: &Jvm, _: &mut RuntimeContext, object: ClassInstanceRef<Object>) -> Result<i32> {
        tracing::debug!("java.lang.System::identityHashCode({object:?})");

        if object.is_null() {
            return Ok(0);
        }

        Ok(Object::identity_hash_code(&object))
    }

    async fn set_property(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        key: ClassInstanceRef<String>,
        value: ClassInstanceRef<String>,
    ) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.lang.System::setProperty({key:?}, {value:?})");

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

    async fn exit(_jvm: &Jvm, context: &mut RuntimeContext, status: i32) -> Result<()> {
        tracing::debug!("java.lang.System::exit({status})");

        context.exit(status);
        Ok(())
    }

    pub async fn get_charset(jvm: &Jvm) -> Result<RustString> {
        let charset: ClassInstanceRef<String> = jvm
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
