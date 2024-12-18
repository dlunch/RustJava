use alloc::{format, vec, vec::Vec};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm::{
    runtime::{JavaIoInputStream, JavaLangString},
    Array, ClassInstanceRef, Jvm, Result,
};

use crate::{
    classes::java::{
        lang::{Class, ClassLoader, String},
        net::{JarURLConnection, URL},
        util::jar::JarEntry,
    },
    RuntimeClassProto, RuntimeContext,
};

// class java.net.URLClassLoader
pub struct URLClassLoader;

impl URLClassLoader {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/net/URLClassLoader",
            parent_class: Some("java/lang/ClassLoader"), // TODO java.security.SecureClassLoader
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "([Ljava/net/URL;Ljava/lang/ClassLoader;)V", Self::init, Default::default()),
                JavaMethodProto::new("findClass", "(Ljava/lang/String;)Ljava/lang/Class;", Self::find_class, Default::default()),
                JavaMethodProto::new(
                    "findResource",
                    "(Ljava/lang/String;)Ljava/net/URL;",
                    Self::find_resource,
                    Default::default(),
                ),
            ],
            fields: vec![JavaFieldProto::new("urls", "[Ljava/net/URL;", Default::default())],
        }
    }

    async fn init(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        urls: ClassInstanceRef<Array<URL>>,
        parent: ClassInstanceRef<ClassLoader>,
    ) -> Result<()> {
        tracing::debug!("java.net.URLClassLoader::<init>({:?}, {:?}, {:?})", &this, &urls, &parent);

        let _: () = jvm
            .invoke_special(&this, "java/lang/ClassLoader", "<init>", "(Ljava/lang/ClassLoader;)V", (parent,))
            .await?;

        jvm.put_field(&mut this, "urls", "[Ljava/net/URL;", urls).await?;

        Ok(())
    }

    async fn find_class(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        name: ClassInstanceRef<String>,
    ) -> Result<ClassInstanceRef<Class>> {
        tracing::debug!("java.net.URLClassLoader::findClass({:?}, {:?})", &this, name);

        let name_str = JavaLangString::to_rust_string(jvm, &name).await?;

        // find rustjar first
        let urls = jvm.get_field(&this, "urls", "[Ljava/net/URL;").await?;
        let urls: Vec<ClassInstanceRef<URL>> = jvm.load_array(&urls, 0, jvm.array_length(&urls).await? as _).await?;

        for url in urls {
            let file = jvm.invoke_virtual(&url, "getFile", "()Ljava/lang/String;", ()).await?;
            let file = JavaLangString::to_rust_string(jvm, &file).await?;

            if file.ends_with(".rustjar") {
                let class = context.find_rustjar_class(jvm, &file, &name_str).await?;
                if let Some(class) = class {
                    let java_class = jvm.register_class(class, Some(this.into())).await?.unwrap();

                    return Ok(java_class.into());
                }
            }
        }

        let resource_name = format!("{}.class", name_str.replace('.', "/"));
        let resource_name = JavaLangString::from_rust_string(jvm, &resource_name).await?;

        let resource: ClassInstanceRef<URL> = jvm
            .invoke_virtual(&this, "findResource", "(Ljava/lang/String;)Ljava/net/URL;", (resource_name,))
            .await?;
        if resource.is_null() {
            return Ok(None.into());
        }

        let stream = jvm.invoke_virtual(&resource, "openStream", "()Ljava/io/InputStream;", ()).await?;
        let bytes = JavaIoInputStream::read_until_end(jvm, &stream).await?;

        let length = bytes.len() as i32;
        let mut bytes_java = jvm.instantiate_array("B", bytes.len()).await?;
        jvm.array_raw_buffer_mut(&mut bytes_java).await?.write(0, &bytes)?;

        let class = jvm
            .invoke_virtual(
                &this,
                "defineClass",
                "(Ljava/lang/String;[BII)Ljava/lang/Class;",
                (name, bytes_java.clone(), 0, length),
            )
            .await?;

        jvm.destroy(bytes_java)?; // TODO: this should be done by the GC

        Ok(class)
    }

    async fn find_resource(
        jvm: &Jvm,
        runtime: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        name: ClassInstanceRef<String>,
    ) -> Result<ClassInstanceRef<URL>> {
        tracing::debug!("java.net.URLClassLoader::findResource({:?}, {:?})", &this, name);

        let name_str = JavaLangString::to_rust_string(jvm, &name).await?;

        let urls = jvm.get_field(&this, "urls", "[Ljava/net/URL;").await?;
        let urls: Vec<ClassInstanceRef<URL>> = jvm.load_array(&urls, 0, jvm.array_length(&urls).await? as _).await?;

        for url in urls {
            let file = jvm.invoke_virtual(&url, "getFile", "()Ljava/lang/String;", ()).await?;
            let file = JavaLangString::to_rust_string(jvm, &file).await?;
            if file.ends_with('/') || file.is_empty() {
                // directory
                let final_path = if file.ends_with('/') {
                    format!("{}{}", file, name_str)
                } else {
                    name_str.clone()
                };

                if runtime.metadata(&final_path).await.is_ok() {
                    let new_url = jvm
                        .new_class(
                            "java/net/URL",
                            "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V",
                            (
                                JavaLangString::from_rust_string(jvm, "file").await?,
                                JavaLangString::from_rust_string(jvm, "").await?,
                                JavaLangString::from_rust_string(jvm, &final_path).await?,
                            ),
                        )
                        .await?;

                    return Ok(new_url.into());
                }
            } else if file.ends_with(".rustjar") {
                // TODO rustjar resource
            } else {
                // treat as jar
                let name_str = name_str.trim_start_matches('/');

                let jar_url_str = format!("jar:file:{}!/{}", file, name_str); // TODO url might not be file
                let jar_url = JavaLangString::from_rust_string(jvm, &jar_url_str).await?;
                let jar_url = jvm.new_class("java/net/URL", "(Ljava/lang/String;)V", (jar_url,)).await?;
                let connection: ClassInstanceRef<JarURLConnection> =
                    jvm.invoke_virtual(&jar_url, "openConnection", "()Ljava/net/URLConnection;", ()).await?;

                let entry: ClassInstanceRef<JarEntry> = jvm.invoke_virtual(&connection, "getJarEntry", "()Ljava/util/jar/JarEntry;", ()).await?;

                if !entry.is_null() {
                    return Ok(jar_url.into());
                }
            }
        }

        Ok(None.into())
    }
}
