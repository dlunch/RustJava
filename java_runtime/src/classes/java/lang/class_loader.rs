use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{FieldAccessFlags, MethodAccessFlags};
use jvm::{runtime::JavaLangString, ClassInstanceRef, Jvm, JvmResult};

use crate::{
    classes::java::{
        lang::{Class, String},
        net::URL,
    },
    RuntimeClassProto, RuntimeContext,
};

// class java.lang.ClassLoader
pub struct ClassLoader {}

impl ClassLoader {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
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
            ],
            fields: vec![
                JavaFieldProto::new("systemClassLoader", "Ljava/lang/ClassLoader;", FieldAccessFlags::STATIC),
                JavaFieldProto::new("parent", "Ljava/lang/ClassLoader;", Default::default()),
            ],
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, parent: ClassInstanceRef<Self>) -> JvmResult<()> {
        tracing::debug!("java.lang.ClassLoader::<init>({:?}, {:?})", &this, parent);

        jvm.put_field(&mut this, "parent", "Ljava/lang/ClassLoader;", parent)?;

        Ok(())
    }

    async fn get_system_class_loader(jvm: &Jvm, _: &mut RuntimeContext) -> JvmResult<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.ClassLoader::getSystemClassLoader()");

        let system_class_loader: ClassInstanceRef<Self> = jvm
            .get_static_field("java/lang/ClassLoader", "systemClassLoader", "Ljava/lang/ClassLoader;")
            .await?;

        if system_class_loader.is_null() {
            let runtime_class_loader = jvm
                .new_class("rustjava/RuntimeClassLoader", "(Ljava/lang/ClassLoader;)V", (None,))
                .await?;

            let array_class_loader = jvm
                .new_class("rustjava/ArrayClassLoader", "(Ljava/lang/ClassLoader;)V", (runtime_class_loader,))
                .await?;

            let classpath_class_loader = jvm
                .new_class("rustjava/ClassPathClassLoader", "(Ljava/lang/ClassLoader;)V", (array_class_loader,))
                .await?;

            jvm.put_static_field(
                "java/lang/ClassLoader",
                "systemClassLoader",
                "Ljava/lang/ClassLoader;",
                classpath_class_loader.clone(),
            )
            .await?;

            return Ok(classpath_class_loader.into());
        }

        Ok(system_class_loader)
    }

    async fn load_class(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        name: ClassInstanceRef<String>,
    ) -> JvmResult<ClassInstanceRef<Class>> {
        tracing::debug!("java.lang.ClassLoader::loadClass({:?}, {:?})", &this, name);

        let class: ClassInstanceRef<Class> = jvm
            .invoke_virtual(&this, "findLoadedClass", "(Ljava/lang/String;)Ljava/lang/Class;", (name.clone(),))
            .await?;

        if !class.is_null() {
            return Ok(class);
        }

        let parent: ClassInstanceRef<Self> = jvm.get_field(&this, "parent", "Ljava/lang/ClassLoader;")?;
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
    ) -> JvmResult<ClassInstanceRef<Class>> {
        tracing::debug!("java.lang.ClassLoader::findClass({:?}, {:?})", &this, name);

        // TODO raise ClassNotFoundException

        Ok(None.into())
    }

    async fn find_loaded_class(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        name: ClassInstanceRef<String>,
    ) -> JvmResult<ClassInstanceRef<Class>> {
        tracing::debug!("java.lang.ClassLoader::findLoadedClass({:?}, {:?})", &this, name);

        let rust_name = JavaLangString::to_rust_string(jvm, name.into())?;
        if !jvm.has_class(&rust_name) {
            return Ok(None.into());
        }

        let mut class = jvm.resolve_class(&rust_name).await?;

        Ok(class.java_class(jvm).await?.into())
    }

    async fn get_resource(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        name: ClassInstanceRef<String>,
    ) -> JvmResult<ClassInstanceRef<URL>> {
        tracing::debug!("java.lang.ClassLoader::getResource({:?})", &this);

        let parent: ClassInstanceRef<Self> = jvm.get_field(&this, "parent", "Ljava/lang/ClassLoader;")?;

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
    ) -> JvmResult<ClassInstanceRef<URL>> {
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
    ) -> JvmResult<ClassInstanceRef<String>> {
        tracing::debug!("java.lang.ClassLoader::findResource({:?})", &this);

        Ok(None.into())
    }
}
