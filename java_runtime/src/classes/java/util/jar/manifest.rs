use alloc::{vec, vec::Vec};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm::{ClassInstanceRef, Jvm, Result, runtime::JavaLangString};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::java::{io::InputStream, lang::String, util::jar::Attributes},
};

// class java.util.jar.Manifest
pub struct Manifest;

impl Manifest {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/jar/Manifest",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/io/InputStream;)V", Self::init, Default::default()),
                JavaMethodProto::new("read", "(Ljava/io/InputStream;)V", Self::read, Default::default()),
                JavaMethodProto::new(
                    "getMainAttributes",
                    "()Ljava/util/jar/Attributes;",
                    Self::get_main_attributes,
                    Default::default(),
                ),
            ],
            fields: vec![JavaFieldProto::new("attrs", "Ljava/util/jar/Attributes;", Default::default())],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, is: ClassInstanceRef<InputStream>) -> Result<()> {
        tracing::debug!("java.util.jar.Manifest::<init>({:?}, {:?})", &this, &is);

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        let _: () = jvm.invoke_virtual(&this, "read", "(Ljava/io/InputStream;)V", (is,)).await?;

        Ok(())
    }

    async fn read(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, is: ClassInstanceRef<InputStream>) -> Result<()> {
        tracing::debug!("java.util.jar.Manifest::read({:?}, {:?})", &this, &is);

        // TODO we currently support only main attribute

        let main_attributes = jvm.new_class("java/util/jar/Attributes", "()V", ()).await?;

        let reader = jvm.new_class("java/io/InputStreamReader", "(Ljava/io/InputStream;)V", (is,)).await?;
        let buffered_reader = jvm.new_class("java/io/BufferedReader", "(Ljava/io/Reader;)V", (reader,)).await?;

        loop {
            let line: ClassInstanceRef<String> = jvm.invoke_virtual(&buffered_reader, "readLine", "()Ljava/lang/String;", ()).await?;
            if line.is_null() {
                break;
            }

            let line = JavaLangString::to_rust_string(jvm, &line).await?;

            if line.trim().is_empty() {
                continue;
            }

            let parts = line.splitn(2, ':').collect::<Vec<_>>();

            let key = JavaLangString::from_rust_string(jvm, parts[0].trim()).await?;
            let value = JavaLangString::from_rust_string(jvm, parts[1].trim()).await?;

            let _: ClassInstanceRef<String> = jvm
                .invoke_virtual(
                    &main_attributes,
                    "putValue",
                    "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;",
                    (key, value),
                )
                .await?;
        }

        jvm.put_field(&mut this, "attrs", "Ljava/util/jar/Attributes;", main_attributes).await?;

        Ok(())
    }

    async fn get_main_attributes(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Attributes>> {
        tracing::debug!("java.util.jar.Manifest::getMainAttributes({:?})", &this);

        jvm.get_field(&this, "attrs", "Ljava/util/jar/Attributes;").await
    }
}
