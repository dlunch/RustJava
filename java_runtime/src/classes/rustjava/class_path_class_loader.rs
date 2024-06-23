use alloc::{
    collections::BTreeMap,
    string::{String as RustString, ToString},
    vec,
    vec::Vec,
};

use bytemuck::cast_vec;
use zip::ZipArchive;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm::{runtime::JavaLangString, Array, ClassInstanceRef, Jvm, Result};

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
                JavaMethodProto::new("addJarFile", "([B)Ljava/lang/String;", Self::add_jar_file, Default::default()),
            ],
            fields: vec![JavaFieldProto::new("entries", "[Lrustjava/ClassPathEntry;", Default::default())],
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, parent: ClassInstanceRef<ClassLoader>) -> Result<()> {
        tracing::debug!("rustjava.ClassPathClassLoader::<init>({:?}, {:?})", &this, &parent);

        jvm.invoke_special(&this, "java/lang/ClassLoader", "<init>", "(Ljava/lang/ClassLoader;)V", (parent,))
            .await?;

        let entries = jvm.instantiate_array("Lrustjava/ClassPathEntry;", 0).await?;
        jvm.put_field(&mut this, "entries", "[Lrustjava/ClassPathEntry;", entries).await?;

        Ok(())
    }

    async fn find_class(
        jvm: &Jvm,
        _runtime: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        name: ClassInstanceRef<String>,
    ) -> Result<ClassInstanceRef<Class>> {
        tracing::debug!("rustjava.ClassPathClassLoader::findClass({:?}, {:?})", &this, name);

        let class_file_name = JavaLangString::to_rust_string(jvm, &name).await?.replace('.', "/") + ".class";
        let class_file_name = JavaLangString::from_rust_string(jvm, &class_file_name).await?;

        let resource: ClassInstanceRef<URL> = jvm
            .invoke_virtual(&this, "getResource", "(Ljava/lang/String;)Ljava/net/URL;", (class_file_name,))
            .await?;

        if resource.is_null() {
            return Ok(None.into());
        }

        let stream = jvm.invoke_virtual(&resource, "openStream", "()Ljava/io/InputStream;", ()).await?;
        let length: i32 = jvm.invoke_virtual(&stream, "available", "()I", ()).await?;
        let array = jvm.instantiate_array("B", length as _).await?;

        let _: i32 = jvm.invoke_virtual(&stream, "read", "([B)I", (array.clone(),)).await?;

        let class = jvm
            .invoke_virtual(
                &this,
                "defineClass",
                "(Ljava/lang/String;[BII)Ljava/lang/Class;",
                (name, array, 0, length),
            )
            .await?;

        Ok(class)
    }

    async fn find_resource(
        jvm: &Jvm,
        _runtime: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        name: ClassInstanceRef<String>,
    ) -> Result<ClassInstanceRef<URL>> {
        tracing::debug!("rustjava.ClassPathClassLoader::findResource({:?}, {:?})", &this, name);

        let name = JavaLangString::to_rust_string(jvm, &name).await?;

        let entries: ClassInstanceRef<Array<ClassPathEntry>> = jvm.get_field(&this, "entries", "[Lrustjava/ClassPathEntry;").await?;

        let entries = jvm.load_array(&entries, 0, jvm.array_length(&entries).await?).await?;
        for entry in entries {
            let entry_name = ClassPathEntry::name(jvm, &entry).await?;

            if name == entry_name {
                let data = ClassPathEntry::data(jvm, &entry).await?;

                let protocol = JavaLangString::from_rust_string(jvm, "bytes").await?;
                let host = JavaLangString::from_rust_string(jvm, "").await?;
                let port = 0;
                let file = JavaLangString::from_rust_string(jvm, &name).await?;
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
    ) -> Result<()> {
        tracing::debug!("rustjava.ClassPathClassLoader::addClassFile({:?})", &this);

        let entry = jvm
            .new_class("rustjava/ClassPathEntry", "(Ljava/lang/String;[B)V", (file_name, data))
            .await?;

        let entries = jvm.get_field(&this, "entries", "[Lrustjava/ClassPathEntry;").await?;

        let length = jvm.array_length(&entries).await?;
        let mut entries: Vec<ClassInstanceRef<ClassPathEntry>> = jvm.load_array(&entries, 0, length).await?;

        entries.push(entry.into());

        let mut new_entries = jvm.instantiate_array("Ljava/lang/String;", length + 1).await?;
        jvm.store_array(&mut new_entries, 0, entries).await?;
        jvm.put_field(&mut this, "entries", "[Lrustjava/ClassPathEntry;", new_entries).await?;

        Ok(())
    }

    async fn add_jar_file(
        jvm: &Jvm,
        _runtime: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        data: ClassInstanceRef<Array<i8>>,
    ) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("rustjava.ClassPathClassLoader::addJarFile({:?})", &this);
        // TODO we need to implement java/util/jar/JarFile

        let data = jvm.load_byte_array(&data, 0, jvm.array_length(&data).await?).await?;

        let entries = jvm.get_field(&this, "entries", "[Lrustjava/ClassPathEntry;").await?;
        let mut entries: Vec<ClassInstanceRef<ClassPathEntry>> = jvm.load_array(&entries, 0, jvm.array_length(&entries).await?).await?;

        // XXX is there no_std zip library?..
        extern crate std;
        use std::io::{Cursor, Read};

        let mut manifest = None;

        let mut archive = ZipArchive::new(Cursor::new(cast_vec(data))).unwrap();
        let files = (0..archive.len())
            .map(|x| {
                let mut file = archive.by_index(x).unwrap();

                let mut data = Vec::new();
                file.read_to_end(&mut data).unwrap();

                (file.name().to_string(), data)
            })
            .collect::<BTreeMap<_, _>>();

        for (filename, data) in files {
            if !data.is_empty() {
                if filename == "META-INF/MANIFEST.MF" {
                    manifest = Some(data.clone())
                }

                let name = JavaLangString::from_rust_string(jvm, &filename).await?;

                let mut data_array = jvm.instantiate_array("B", data.len()).await?;
                jvm.store_byte_array(&mut data_array, 0, cast_vec(data)).await?;

                let entry = jvm
                    .new_class("rustjava/ClassPathEntry", "(Ljava/lang/String;[B)V", (name, data_array))
                    .await?;

                entries.push(entry.into())
            }
        }

        let mut new_entries = jvm.instantiate_array("Ljava/lang/String;", entries.len()).await?;
        jvm.store_array(&mut new_entries, 0, entries).await?;
        jvm.put_field(&mut this, "entries", "[Lrustjava/ClassPathEntry;", new_entries).await?;

        // TODO we need java/util/jar/Manifest
        if let Some(x) = manifest {
            let main_class_name = Self::get_main_class_name(&x);
            if let Some(x) = main_class_name {
                return Ok(JavaLangString::from_rust_string(jvm, &x).await?.into());
            }
        }
        Ok(None.into())
    }

    fn get_main_class_name(manifest: &[u8]) -> Option<RustString> {
        let manifest = RustString::from_utf8_lossy(manifest);
        for line in manifest.lines() {
            if let Some(x) = line.strip_prefix("Main-Class: ") {
                return Some(x.to_string());
            } else if let Some(x) = line.strip_prefix("MIDlet-1: ") {
                // XXX is it correct to put it here?
                let split = x.split(',').collect::<Vec<_>>();
                return Some(split[2].trim().to_string());
            }
        }

        None
    }
}
