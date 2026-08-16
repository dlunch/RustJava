use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use jvm::{
    ClassInstanceRef, JavaType, Jvm, Result,
    runtime::{JavaLangClass, JavaLangClassLoader, JavaLangString},
};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::java::{
        io::InputStream,
        lang::{ClassLoader, Object, String},
    },
};

// class java.lang.Class
pub struct Class;

impl Class {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/lang/Class",
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["java/io/Serializable"],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, MethodAccessFlags::PRIVATE),
                JavaMethodProto::new("getName", "()Ljava/lang/String;", Self::get_name, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("isPrimitive", "()Z", Self::is_primitive, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("isArray", "()Z", Self::is_array, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("isInterface", "()Z", Self::is_interface, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("isInstance", "(Ljava/lang/Object;)Z", Self::is_instance, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "isAssignableFrom",
                    "(Ljava/lang/Class;)Z",
                    Self::is_assignable_from,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::NATIVE,
                ),
                JavaMethodProto::new("newInstance", "()Ljava/lang/Object;", Self::new_instance, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("toString", "()Ljava/lang/String;", Self::to_string, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("getSuperclass", "()Ljava/lang/Class;", Self::get_superclass, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "getClassLoader",
                    "()Ljava/lang/ClassLoader;",
                    Self::get_class_loader,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "getComponentType",
                    "()Ljava/lang/Class;",
                    Self::get_component_type,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new("getInterfaces", "()[Ljava/lang/Class;", Self::get_interfaces, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "getResourceAsStream",
                    "(Ljava/lang/String;)Ljava/io/InputStream;",
                    Self::get_resource_as_stream,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "forName",
                    "(Ljava/lang/String;)Ljava/lang/Class;",
                    Self::for_name,
                    MethodAccessFlags::STATIC,
                ),
            ],
            fields: vec![
                // Stored as raw bytes instead of java/lang/String to avoid circular dependency:
                // from_rust_class -> JavaLangString::from_rust_string -> new_class("java/lang/String") -> from_rust_class -> stack overflow
                JavaFieldProto::new("nameBytes", "[B", FieldAccessFlags::PRIVATE | FieldAccessFlags::FINAL),
                JavaFieldProto::new(
                    "classLoader",
                    "Ljava/lang/ClassLoader;",
                    FieldAccessFlags::PRIVATE | FieldAccessFlags::FINAL,
                ),
            ],
            access_flags: ClassAccessFlags::PUBLIC | ClassAccessFlags::FINAL,
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.lang.Class::<init>({this:?})");

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        Ok(())
    }

    async fn get_name(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.lang.Class::getName({this:?})");

        let class_name = JavaLangClass::name(jvm, &this).await?;
        let result = JavaLangString::from_rust_string(jvm, &class_name.replace('/', ".")).await?;

        Ok(result.into())
    }

    async fn is_primitive(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        let name = JavaLangClass::name(jvm, &this).await?;
        Ok(matches!(
            name.as_str(),
            "boolean" | "byte" | "char" | "short" | "int" | "long" | "float" | "double"
        ))
    }

    async fn is_array(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        let name = JavaLangClass::name(jvm, &this).await?;
        Ok(name.starts_with('['))
    }

    async fn is_interface(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        let name = JavaLangClass::name(jvm, &this).await?;
        if name.starts_with('[') || matches!(name.as_str(), "boolean" | "byte" | "char" | "short" | "int" | "long" | "float" | "double") {
            return Ok(false);
        }

        let class = JavaLangClass::to_rust_class(jvm, &this).await?;
        Ok(class.access_flags().contains(ClassAccessFlags::INTERFACE))
    }

    async fn is_instance(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, object: ClassInstanceRef<Object>) -> Result<bool> {
        if object.is_null() {
            return Ok(false);
        }

        let name = JavaLangClass::name(jvm, &this).await?;
        if matches!(name.as_str(), "boolean" | "byte" | "char" | "short" | "int" | "long" | "float" | "double") {
            return Ok(false);
        }

        Ok(jvm.is_instance(&**object, &name))
    }

    async fn new_instance(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let name = JavaLangClass::name(jvm, &this).await?;
        if name.starts_with('[') || matches!(name.as_str(), "boolean" | "byte" | "char" | "short" | "int" | "long" | "float" | "double") {
            return Err(jvm.exception("java/lang/InstantiationException", &name).await);
        }

        let class = JavaLangClass::to_rust_class(jvm, &this).await?;
        let access_flags = class.access_flags();
        if access_flags.contains(ClassAccessFlags::INTERFACE)
            || access_flags.contains(ClassAccessFlags::ABSTRACT)
            || class.method("<init>", "()V", false).is_none()
        {
            return Err(jvm.exception("java/lang/InstantiationException", &name).await);
        }

        let instance = jvm.instantiate_class(&name).await?;
        let _: () = jvm.invoke_special(&instance, &name, "<init>", "()V", ()).await?;

        Ok(instance.into())
    }

    async fn to_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        let name = JavaLangClass::name(jvm, &this).await?;
        let text = if matches!(name.as_str(), "boolean" | "byte" | "char" | "short" | "int" | "long" | "float" | "double") {
            name
        } else {
            let class = JavaLangClass::to_rust_class(jvm, &this).await?;
            let prefix = if class.access_flags().contains(ClassAccessFlags::INTERFACE) {
                "interface "
            } else {
                "class "
            };
            alloc::format!("{prefix}{}", name.replace('/', "."))
        };

        Ok(JavaLangString::from_rust_string(jvm, &text).await?.into())
    }

    async fn is_assignable_from(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::debug!("java.lang.Class::isAssignableFrom({this:?}, {other:?})");

        if other.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "other").await);
        }

        let class_name = JavaLangClass::name(jvm, &this).await?;
        let other_name = JavaLangClass::name(jvm, &other).await?;
        let class_is_primitive = matches!(
            class_name.as_str(),
            "boolean" | "byte" | "char" | "short" | "int" | "long" | "float" | "double"
        );
        let other_is_primitive = matches!(
            other_name.as_str(),
            "boolean" | "byte" | "char" | "short" | "int" | "long" | "float" | "double"
        );

        if class_is_primitive || other_is_primitive {
            return Ok(class_name == other_name);
        }

        Ok(jvm.is_type_assignable(&JavaType::from_class_name(&other_name), &JavaType::from_class_name(&class_name)))
    }

    async fn get_superclass(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.Class::getSuperclass({this:?})");

        let name = JavaLangClass::name(jvm, &this).await?;
        if matches!(
            name.as_str(),
            "boolean" | "byte" | "char" | "short" | "int" | "long" | "float" | "double" | "void" | "java/lang/Object"
        ) {
            return Ok(None.into());
        }
        if name.starts_with('[') {
            return Ok(jvm.resolve_class("java/lang/Object").await?.java_class().into());
        }

        let class = JavaLangClass::to_rust_class(jvm, &this).await?;
        if class.access_flags().contains(ClassAccessFlags::INTERFACE) {
            return Ok(None.into());
        }

        match class.super_class_name() {
            Some(super_class_name) => Ok(jvm.resolve_class(&super_class_name).await?.java_class().into()),
            None => Ok(None.into()),
        }
    }

    async fn get_class_loader(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<ClassLoader>> {
        tracing::debug!("java.lang.Class::getClassLoader({this:?})");

        jvm.get_field(&this, "classLoader", "Ljava/lang/ClassLoader;").await
    }

    async fn get_component_type(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.Class::getComponentType({this:?})");

        let name = JavaLangClass::name(jvm, &this).await?;
        let Some(component_descriptor) = name.strip_prefix('[') else {
            return Ok(None.into());
        };

        let primitive_wrapper = match component_descriptor {
            "Z" => Some("java/lang/Boolean"),
            "B" => Some("java/lang/Byte"),
            "C" => Some("java/lang/Character"),
            "S" => Some("java/lang/Short"),
            "I" => Some("java/lang/Integer"),
            "J" => Some("java/lang/Long"),
            "F" => Some("java/lang/Float"),
            "D" => Some("java/lang/Double"),
            _ => None,
        };
        if let Some(wrapper) = primitive_wrapper {
            return jvm.get_static_field(wrapper, "TYPE", "Ljava/lang/Class;").await;
        }

        let component_name = if let Some(reference_name) = component_descriptor.strip_prefix('L').and_then(|name| name.strip_suffix(';')) {
            reference_name
        } else {
            component_descriptor
        };
        let defining_loader: ClassInstanceRef<ClassLoader> = jvm.get_field(&this, "classLoader", "Ljava/lang/ClassLoader;").await?;
        if defining_loader.is_null() {
            return Ok(jvm.resolve_class(component_name).await?.java_class().into());
        }

        let component_name = JavaLangString::from_rust_string(jvm, component_name).await?;
        jvm.invoke_virtual(
            &defining_loader,
            "java/lang/ClassLoader",
            "loadClass",
            "(Ljava/lang/String;)Ljava/lang/Class;",
            (component_name,),
        )
        .await
    }

    async fn get_interfaces(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<jvm::Array<Self>>> {
        tracing::debug!("java.lang.Class::getInterfaces({this:?})");

        let name = JavaLangClass::name(jvm, &this).await?;
        let interface_names = if matches!(
            name.as_str(),
            "boolean" | "byte" | "char" | "short" | "int" | "long" | "float" | "double" | "void"
        ) {
            vec![]
        } else {
            JavaLangClass::to_rust_class(jvm, &this).await?.interface_names()
        };

        let mut interfaces = jvm.instantiate_array("Ljava/lang/Class;", interface_names.len()).await?;
        for (index, interface_name) in interface_names.iter().enumerate() {
            let interface = jvm.resolve_class(interface_name).await?.java_class();
            jvm.store_array(&mut interfaces, index, [interface]).await?;
        }

        Ok(interfaces.into())
    }

    async fn get_resource_as_stream(
        jvm: &Jvm,
        _context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        name: ClassInstanceRef<String>,
    ) -> Result<ClassInstanceRef<InputStream>> {
        tracing::debug!("java.lang.Class::getResourceAsStream({this:?}, {name:?})");

        let class_loader: ClassInstanceRef<ClassLoader> = jvm.get_field(&this, "classLoader", "Ljava/lang/ClassLoader;").await?;

        let class_loader = if class_loader.is_null() {
            // TODO ClassLoader.getSystemResourceAsStream?
            JavaLangClassLoader::get_system_class_loader(jvm).await?
        } else {
            class_loader.into()
        };

        jvm.invoke_virtual(
            &class_loader,
            "java/lang/ClassLoader",
            "getResourceAsStream",
            "(Ljava/lang/String;)Ljava/io/InputStream;",
            (name,),
        )
        .await
    }

    async fn for_name(jvm: &Jvm, _context: &mut RuntimeContext, name: ClassInstanceRef<String>) -> Result<ClassInstanceRef<Class>> {
        tracing::debug!("java.lang.Class::forName({name:?})");

        if name.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "name").await);
        }

        let rust_name = JavaLangString::to_rust_string(jvm, &name).await?;
        let qualified_name = rust_name.replace('.', "/");

        match jvm.resolve_class(&qualified_name).await {
            Ok(class) => Ok(class.java_class().into()),
            Err(_) => Err(jvm.exception("java/lang/ClassNotFoundException", &rust_name).await),
        }
    }
}
