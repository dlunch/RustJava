use alloc::{vec, vec::Vec};

use bytemuck::{cast_slice, cast_vec};
use zip::ZipArchive;

use java_class_proto::{JavaFieldProto, JavaMethodProto, JavaResult};
use jvm::{Array, ClassInstanceRef, Jvm};

use crate::{
    classes::{
        java::{
            lang::{Class, ClassLoader, String},
            net::URL,
        },
        rustjava::ClassPathEntry,
    },
    RuntimeClassProto, RuntimeContext,
};

// class rustjava.ClassPathClassLoader
pub struct ClassPathClassLoader {}

impl ClassPathClassLoader {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            parent_class: Some("java/lang/ClassLoader"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/lang/ClassLoader;)V", Self::init, Default::default()),
                JavaMethodProto::new("findClass", "(Ljava/lang/String;)Ljava/lang/Class;", Self::find_class, Default::default()),
                JavaMethodProto::new(
                    "findResource",
                    "(Ljava/lang/String;)Ljava/net/URL;",
                    Self::find_resource,
                    Default::default(),
                ),
                JavaMethodProto::new("addClassFile", "(Ljava/lang/String;[B)V", Self::add_class_file, Default::default()),
                JavaMethodProto::new("addJarFile", "([B)V", Self::add_jar_file, Default::default()),
            ],
            fields: vec![JavaFieldProto::new("entries", "[Lrustjava/ClassPathEntry;", Default::default())],
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, parent: ClassInstanceRef<ClassLoader>) -> JavaResult<()> {
        tracing::debug!("rustjava.ClassPathClassLoader::<init>({:?}, {:?})", &this, &parent);

        jvm.invoke_special(&this, "java/lang/ClassLoader", "<init>", "(Ljava/lang/ClassLoader;)V", (parent,))
            .await?;

        let entries = jvm.instantiate_array("Lrustjava/ClassPathEntry;", 0).await?;
        jvm.put_field(&mut this, "entries", "[Lrustjava/ClassPathEntry;", entries)?;

        Ok(())
    }

    async fn find_class(
        jvm: &Jvm,
        _runtime: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        name: ClassInstanceRef<String>,
    ) -> JavaResult<ClassInstanceRef<Class>> {
        tracing::debug!("rustjava.ClassPathClassLoader::findClass({:?}, {:?})", &this, name);

        let resource: ClassInstanceRef<URL> = jvm
            .invoke_virtual(&this, "getResource", "(Ljava/lang/String;)Ljava/net/URL;", (name.clone(),))
            .await?;

        if resource.is_null() {
            return Ok(None.into());
        }

        // TODO use ClassLoader.defineClass

        let stream = jvm.invoke_virtual(&resource, "openStream", "()Ljava/io/InputStream;", ()).await?;
        let length: i32 = jvm.invoke_virtual(&stream, "available", "()I", ()).await?;
        let array = jvm.instantiate_array("B", length as _).await?;

        let _: i32 = jvm.invoke_virtual(&stream, "read", "([B)I", (array.clone(),)).await?;

        let data: Vec<i8> = jvm.load_byte_array(&array, 0, length as _)?;

        let name = String::to_rust_string(jvm, &name)?;
        let rust_class = jvm.define_class(&name, cast_slice(&data)).await?;

        let java_class = Class::from_rust_class(jvm, rust_class).await?;

        Ok(java_class)
    }

    async fn find_resource(
        jvm: &Jvm,
        _runtime: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        name: ClassInstanceRef<String>,
    ) -> JavaResult<ClassInstanceRef<URL>> {
        tracing::debug!("rustjava.ClassPathClassLoader::findResource({:?}, {:?})", &this, name);

        let name = String::to_rust_string(jvm, &name)?;

        let entries: ClassInstanceRef<Array<ClassPathEntry>> = jvm.get_field(&this, "entries", "[Lrustjava/ClassPathEntry;")?;

        let entries = jvm.load_array(&entries, 0, jvm.array_length(&entries)?)?;
        for entry in entries {
            let entry_name = ClassPathEntry::name(jvm, &entry)?;

            if name == entry_name {
                let data = ClassPathEntry::data(jvm, &entry)?;

                let protocol = String::from_rust_string(jvm, "bytes").await?;
                let host = String::from_rust_string(jvm, "").await?;
                let port = 0;
                let file = String::from_rust_string(jvm, &name).await?;
                let handler = jvm.new_class("rustjava/ByteArrayURLHandler", "([B)V", (data,)).await?;

                let url = jvm
                    .new_class(
                        "java/net/URL",
                        "(Ljava/lang/String;Ljava/lang/String;ILjava/lang/String;Ljava/net/URLStreamHandler;)V",
                        (protocol, host, port, file, handler),
                    )
                    .await?;

                return Ok(url.into());
            }
        }

        Ok(None.into())
    }

    // we don't have classpath (yet), so we need backdoor to add classes to loader
    async fn add_class_file(
        jvm: &Jvm,
        _runtime: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        file_name: ClassInstanceRef<String>,
        data: ClassInstanceRef<Array<i8>>,
    ) -> JavaResult<()> {
        tracing::debug!("rustjava.ClassPathClassLoader::addClassFile({:?})", &this);

        let entry = jvm
            .new_class("rustjava/ClassPathEntry", "(Ljava/lang/String;[B)V", (file_name, data))
            .await?;

        let entries = jvm.get_field(&this, "entries", "[Lrustjava/ClassPathEntry;")?;

        let length = jvm.array_length(&entries)?;
        let mut entries: Vec<ClassInstanceRef<ClassPathEntry>> = jvm.load_array(&entries, 0, length)?;

        entries.push(entry.into());

        let mut new_entries = jvm.instantiate_array("Ljava/lang/String;", length + 1).await?;
        jvm.store_array(&mut new_entries, 0, entries)?;
        jvm.put_field(&mut this, "entries", "[Lrustjava/ClassPathEntry;", new_entries)?;

        Ok(())
    }

    async fn add_jar_file(
        jvm: &Jvm,
        _runtime: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        data: ClassInstanceRef<Array<i8>>,
    ) -> JavaResult<()> {
        tracing::debug!("rustjava.ClassPathClassLoader::addJarFile({:?})", &this);
        // TODO we need to implement java/util/jar/JarFile

        let data = jvm.load_byte_array(&data, 0, jvm.array_length(&data)?)?;

        let entries = jvm.get_field(&this, "entries", "[Lrustjava/ClassPathEntry;")?;
        let mut entries: Vec<ClassInstanceRef<ClassPathEntry>> = jvm.load_array(&entries, 0, jvm.array_length(&entries)?)?;

        // XXX is there no_std zip library?..
        extern crate std;
        use std::io::{Cursor, Read};

        let mut archive = ZipArchive::new(Cursor::new(cast_vec(data)))?;
        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;

            if file.is_file() {
                let name = String::from_rust_string(jvm, file.name()).await?;
                let mut data = Vec::new();
                file.read_to_end(&mut data)?;

                let mut data_array = jvm.instantiate_array("B", data.len()).await?;
                jvm.store_byte_array(&mut data_array, 0, cast_vec(data))?;

                let entry = jvm
                    .new_class("rustjava/ClassPathEntry", "(Ljava/lang/String;[B)V", (name, data_array))
                    .await?;

                entries.push(entry.into())
            }
        }

        let mut new_entries = jvm.instantiate_array("Ljava/lang/String;", entries.len()).await?;
        jvm.store_array(&mut new_entries, 0, entries)?;
        jvm.put_field(&mut this, "entries", "[Lrustjava/ClassPathEntry;", new_entries)?;

        Ok(())
    }
}
