use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::MethodAccessFlags;
use jvm::{
    ClassInstanceRef, Jvm, Result,
    runtime::{JavaLangClass, JavaLangClassLoader, JavaLangString},
};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::java::{
        io::InputStream,
        lang::{ClassLoader, String},
    },
};

// class java.lang.Class
pub struct Class;

impl Class {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/lang/Class",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("getName", "()Ljava/lang/String;", Self::get_name, Default::default()),
                JavaMethodProto::new("isAssignableFrom", "(Ljava/lang/Class;)Z", Self::is_assignable_from, Default::default()),
                JavaMethodProto::new(
                    "getResourceAsStream",
                    "(Ljava/lang/String;)Ljava/io/InputStream;",
                    Self::get_resource_as_stream,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "forName",
                    "(Ljava/lang/String;)Ljava/lang/Class;",
                    Self::for_name,
                    MethodAccessFlags::STATIC,
                ),
            ],
            fields: vec![
                JavaFieldProto::new("raw", "[B", Default::default()), // raw rust pointer of Box<dyn Class>
                JavaFieldProto::new("classLoader", "Ljava/lang/ClassLoader;", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.lang.Class::<init>({:?})", &this);

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        Ok(())
    }

    async fn get_name(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.lang.Class::getName({:?})", &this);

        let rust_class = JavaLangClass::to_rust_class(jvm, &this).await?;
        let result = JavaLangString::from_rust_string(jvm, &rust_class.name()).await?;

        Ok(result.into())
    }

    async fn is_assignable_from(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::debug!("java.lang.Class::isAssignableFrom({:?}, {:?})", &this, &other);

        let rust_class = JavaLangClass::to_rust_class(jvm, &this).await?;
        let other_rust_class = JavaLangClass::to_rust_class(jvm, &other).await?;

        Ok(jvm.is_inherited_from(&*other_rust_class, &rust_class.name()))
    }

    async fn get_resource_as_stream(
        jvm: &Jvm,
        _context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        name: ClassInstanceRef<String>,
    ) -> Result<ClassInstanceRef<InputStream>> {
        tracing::debug!("java.lang.Class::getResourceAsStream({:?}, {:?})", &this, &name);

        let class_loader: ClassInstanceRef<ClassLoader> = jvm.get_field(&this, "classLoader", "Ljava/lang/ClassLoader;").await?;

        let class_loader = if class_loader.is_null() {
            // TODO ClassLoader.getSystemResourceAsStream?
            JavaLangClassLoader::get_system_class_loader(jvm).await?
        } else {
            class_loader.into()
        };

        jvm.invoke_virtual(&class_loader, "getResourceAsStream", "(Ljava/lang/String;)Ljava/io/InputStream;", (name,))
            .await
    }

    async fn for_name(jvm: &Jvm, _context: &mut RuntimeContext, name: ClassInstanceRef<String>) -> Result<ClassInstanceRef<Class>> {
        tracing::debug!("java.lang.Class::forName({:?})", &name);

        let rust_name = JavaLangString::to_rust_string(jvm, &name).await?;
        let qualified_name = rust_name.replace('.', "/");
        let class = jvm.get_class(&qualified_name).unwrap().java_class();

        Ok(class.into())
    }
}
