use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto, JavaResult};
use java_constants::{FieldAccessFlags, MethodAccessFlags};
use jvm::{ClassInstanceRef, Jvm};

use crate::{
    classes::java::lang::{Class, String},
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
            ],
            fields: vec![
                JavaFieldProto::new("systemClassLoader", "Ljava/lang/ClassLoader;", FieldAccessFlags::STATIC),
                JavaFieldProto::new("parent", "Ljava/lang/ClassLoader;", Default::default()),
            ],
        }
    }

    async fn init(jvm: &mut Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, parent: ClassInstanceRef<Self>) -> JavaResult<()> {
        tracing::debug!("java.lang.ClassLoader::<init>({:?}, {:?})", &this, parent);

        jvm.put_field(&mut this, "parent", "Ljava/lang/ClassLoader;", parent)?;

        Ok(())
    }

    async fn get_system_class_loader(jvm: &mut Jvm, _: &mut RuntimeContext) -> JavaResult<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.ClassLoader::getSystemClassLoader()");

        let system_class_loader: ClassInstanceRef<Self> = jvm
            .get_static_field("java/lang/ClassLoader", "systemClassLoader", "Ljava/lang/ClassLoader;")
            .await?;

        if system_class_loader.is_null() {
            let array_class_loader = jvm.new_class("rustjava/ArrayClassLoader", "(Ljava/lang/ClassLoader;)V", (None,)).await?;

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
        jvm: &mut Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        name: ClassInstanceRef<String>,
    ) -> JavaResult<ClassInstanceRef<Class>> {
        tracing::debug!("java.lang.ClassLoader::loadClass({:?}, {:?})", &this, name);

        let class: ClassInstanceRef<Class> = jvm
            .invoke_virtual(
                &this,
                "java/lang/ClassLoader",
                "findLoadedClass",
                "(Ljava/lang/String;)Ljava/lang/Class;",
                (name.clone(),),
            )
            .await?;

        if !class.is_null() {
            return Ok(class);
        }

        let parent: ClassInstanceRef<Self> = jvm.get_field(&this, "parent", "Ljava/lang/ClassLoader;")?;
        let class: ClassInstanceRef<Class> = if !parent.is_null() {
            jvm.invoke_virtual(
                &parent,
                "java/lang/ClassLoader",
                "loadClass",
                "(Ljava/lang/String;)Ljava/lang/Class;",
                (name.clone(),),
            )
            .await?
        } else {
            None.into()
        };

        if !class.is_null() {
            return Ok(class);
        }

        let class = jvm
            .invoke_virtual(
                &this,
                "java/lang/ClassLoader",
                "findClass",
                "(Ljava/lang/String;)Ljava/lang/Class;",
                (name,),
            )
            .await?;

        Ok(class)
    }

    async fn find_class(
        _: &mut Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        name: ClassInstanceRef<String>,
    ) -> JavaResult<ClassInstanceRef<Class>> {
        tracing::debug!("java.lang.ClassLoader::findClass({:?}, {:?})", &this, name);

        // TODO raise ClassNotFoundException

        Ok(None.into())
    }

    async fn find_loaded_class(
        jvm: &mut Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        name: ClassInstanceRef<String>,
    ) -> JavaResult<ClassInstanceRef<Class>> {
        tracing::debug!("java.lang.ClassLoader::findLoadedClass({:?}, {:?})", &this, name);

        let rust_name = String::to_rust_string(jvm, &name)?;
        let class = jvm.get_class(&rust_name);

        if class.is_none() {
            return Ok(None.into());
        }

        // TODO create proper java/lang/Class instance
        let java_class = jvm.new_class("java/lang/Class", "()V", ()).await?;

        Ok(java_class.into())
    }
}
