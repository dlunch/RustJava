use alloc::{format, vec, vec::Vec};

use bytemuck::cast_slice;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{FieldAccessFlags, MethodAccessFlags};
use jvm::{runtime::JavaLangString, Array, ClassInstanceRef, Jvm, Result};

use crate::{
    classes::java::{
        lang::{Class, String},
        net::URL,
    },
    RuntimeClassProto, RuntimeContext,
};

// class java.lang.ClassLoader
pub struct ClassLoader;

impl ClassLoader {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/lang/ClassLoader",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/lang/ClassLoader;)V", Self::init, Default::default()),
                JavaMethodProto::new("loadClass", "(Ljava/lang/String;)Ljava/lang/Class;", Self::load_class, Default::default()),
                JavaMethodProto::new("findClass", "(Ljava/lang/String;)Ljava/lang/Class;", Self::find_class, Default::default()),
                JavaMethodProto::new(
                    "findLoadedClass",
                    "(Ljava/lang/String;)Ljava/lang/Class;",
                    Self::find_loaded_class,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "getSystemClassLoader",
                    "()Ljava/lang/ClassLoader;",
                    Self::get_system_class_loader,
                    MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "getResource",
                    "(Ljava/lang/String;)Ljava/net/URL;",
                    Self::get_resource,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "getResourceAsStream",
                    "(Ljava/lang/String;)Ljava/io/InputStream;",
                    Self::get_resource_as_stream,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "findResource",
                    "(Ljava/lang/String;)Ljava/net/URL;",
                    Self::find_resource,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "defineClass",
                    "(Ljava/lang/String;[BII)Ljava/lang/Class;",
                    Self::define_class,
                    Default::default(),
                ),
            ],
            fields: vec![
                JavaFieldProto::new("systemClassLoader", "Ljava/lang/ClassLoader;", FieldAccessFlags::STATIC),
                JavaFieldProto::new("parent", "Ljava/lang/ClassLoader;", Default::default()),
            ],
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, parent: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.lang.ClassLoader::<init>({:?}, {:?})", &this, parent);

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        jvm.put_field(&mut this, "parent", "Ljava/lang/ClassLoader;", parent).await?;

        Ok(())
    }

    async fn get_system_class_loader(jvm: &Jvm, _: &mut RuntimeContext) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.ClassLoader::getSystemClassLoader()");

        let system_class_loader: ClassInstanceRef<Self> = jvm
            .get_static_field("java/lang/ClassLoader", "systemClassLoader", "Ljava/lang/ClassLoader;")
            .await?;

        if system_class_loader.is_null() {
            let class_path: ClassInstanceRef<String> = jvm
                .invoke_static(
                    "java/lang/System",
                    "getProperty",
                    "(Ljava/lang/String;)Ljava/lang/String;",
                    (JavaLangString::from_rust_string(jvm, "java.class.path").await?,),
                )
                .await?;

            let url_array = if !class_path.is_null() {
                let class_path = JavaLangString::to_rust_string(jvm, &class_path).await?;

                let mut urls = Vec::new();
                for path in class_path.split(':') {
                    // TODO File.pathSeparator
                    let path = JavaLangString::from_rust_string(jvm, &format!("file:{}", path)).await?;
                    let url = jvm.new_class("java/net/URL", "(Ljava/lang/String;)V", (path,)).await?;

                    urls.push(url);
                }

                let mut url_array = jvm.instantiate_array("Ljava/net/URL;", urls.len()).await?;
                jvm.store_array(&mut url_array, 0, urls).await?;

                url_array
            } else {
                jvm.instantiate_array("Ljava/net/URL;", 0).await?
            };

            let url_class_loader = jvm
                .new_class("java/net/URLClassLoader", "([Ljava/net/URL;Ljava/lang/ClassLoader;)V", (url_array, None))
                .await?;

            jvm.put_static_field(
                "java/lang/ClassLoader",
                "systemClassLoader",
                "Ljava/lang/ClassLoader;",
                url_class_loader.clone(),
            )
            .await?;

            return Ok(url_class_loader.into());
        }

        Ok(system_class_loader)
    }

    async fn load_class(
        jvm: &Jvm,
        runtime: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        name: ClassInstanceRef<String>,
    ) -> Result<ClassInstanceRef<Class>> {
        tracing::debug!("java.lang.ClassLoader::loadClass({:?}, {:?})", &this, name);

        let class: ClassInstanceRef<Class> = jvm
            .invoke_virtual(&this, "findLoadedClass", "(Ljava/lang/String;)Ljava/lang/Class;", (name.clone(),))
            .await?;

        if !class.is_null() {
            return Ok(class);
        }

        let name_str = JavaLangString::to_rust_string(jvm, &name).await?;

        if let Some(element_type_name) = name_str.strip_prefix('[') {
            // TODO do we need another class loader for array?
            let class = runtime.define_array_class(jvm, element_type_name).await?;
            let java_class = jvm.register_class(class, Some(this.into())).await?;

            return Ok(java_class.into());
        }

        let parent: ClassInstanceRef<Self> = jvm.get_field(&this, "parent", "Ljava/lang/ClassLoader;").await?;
        let class: ClassInstanceRef<Class> = if !parent.is_null() {
            jvm.invoke_virtual(&parent, "loadClass", "(Ljava/lang/String;)Ljava/lang/Class;", (name.clone(),))
                .await?
        } else {
            None.into()
        };

        if !class.is_null() {
            return Ok(class);
        }

        let class = jvm
            .invoke_virtual(&this, "findClass", "(Ljava/lang/String;)Ljava/lang/Class;", (name,))
            .await?;

        Ok(class)
    }

    async fn find_class(
        _: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        name: ClassInstanceRef<String>,
    ) -> Result<ClassInstanceRef<Class>> {
        tracing::debug!("java.lang.ClassLoader::findClass({:?}, {:?})", &this, name);

        // TODO raise ClassNotFoundException

        Ok(None.into())
    }

    async fn find_loaded_class(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        name: ClassInstanceRef<String>,
    ) -> Result<ClassInstanceRef<Class>> {
        tracing::debug!("java.lang.ClassLoader::findLoadedClass({:?}, {:?})", &this, name);

        let rust_name = JavaLangString::to_rust_string(jvm, &name).await?;
        if !jvm.has_class(&rust_name).await {
            return Ok(None.into());
        }

        let class = jvm.resolve_class(&rust_name).await?;

        Ok(class.java_class(jvm).await?.into())
    }

    async fn get_resource(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        name: ClassInstanceRef<String>,
    ) -> Result<ClassInstanceRef<URL>> {
        tracing::debug!("java.lang.ClassLoader::getResource({:?})", &this);

        let parent: ClassInstanceRef<Self> = jvm.get_field(&this, "parent", "Ljava/lang/ClassLoader;").await?;

        let result: ClassInstanceRef<URL> = if !parent.is_null() {
            jvm.invoke_virtual(&parent, "getResource", "(Ljava/lang/String;)Ljava/net/URL;", (name.clone(),))
                .await?
        } else {
            None.into()
        };

        if !result.is_null() {
            return Ok(result);
        }

        let result = jvm
            .invoke_virtual(&this, "findResource", "(Ljava/lang/String;)Ljava/net/URL;", (name,))
            .await?;

        Ok(result)
    }

    async fn get_resource_as_stream(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        name: ClassInstanceRef<String>,
    ) -> Result<ClassInstanceRef<URL>> {
        tracing::debug!("java.lang.ClassLoader::getResourceAsStream({:?})", &this);

        let resource_url: ClassInstanceRef<URL> = jvm
            .invoke_virtual(&this, "getResource", "(Ljava/lang/String;)Ljava/net/URL;", (name.clone(),))
            .await?;

        if resource_url.is_null() {
            return Ok(None.into());
        }

        let stream = jvm.invoke_virtual(&resource_url, "openStream", "()Ljava/io/InputStream;", ()).await?;

        Ok(stream)
    }

    async fn find_resource(
        _: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        _: ClassInstanceRef<String>,
    ) -> Result<ClassInstanceRef<URL>> {
        tracing::debug!("java.lang.ClassLoader::findResource({:?})", &this);

        Ok(None.into())
    }

    async fn define_class(
        jvm: &Jvm,
        runtime: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        name: ClassInstanceRef<String>,
        bytes: ClassInstanceRef<Array<i8>>,
        offset: i32,
        length: i32,
    ) -> Result<ClassInstanceRef<Class>> {
        tracing::debug!(
            "java.lang.ClassLoader::defineClass({:?}, {:?}, {:?}, {:?}, {:?})",
            &this,
            name,
            bytes,
            offset,
            length
        );

        let data: Vec<i8> = jvm.load_byte_array(&bytes, 0, length as _).await?;

        let class = runtime.define_class(jvm, cast_slice(&data)).await?;
        let java_class = jvm.register_class(class, Some(this.into())).await?;

        Ok(java_class.into())
    }
}
