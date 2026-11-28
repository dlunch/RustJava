use alloc::{boxed::Box, format, vec};

use jvm::{
    Array, ClassInstance, ClassInstanceRef, Jvm, Result,
    runtime::{JavaLangClass, JavaLangClassLoader, JavaLangString},
};
use jvm_class_proto::{java_class, java_methods};
use jvm_types::{ClassAccessFlags, JavaType};

use crate::{
    RuntimeContext,
    classes::java::{
        io::InputStream,
        lang::{ClassLoader, Object, String},
        net::URL,
    },
};

// class java.lang.Class
#[java_class(name = "java/lang/Class", implements("java/io/Serializable"), public, final)]
pub struct Class {
    // Stored as raw bytes instead of java/lang/String to avoid circular dependency:
    // from_rust_class -> JavaLangString::from_rust_string -> new_class("java/lang/String") -> from_rust_class -> stack overflow
    #[field(private, final)]
    nameBytes: ClassInstanceRef<Array<i8>>,
    #[field(private, final)]
    classLoader: ClassInstanceRef<ClassLoader>,
}

#[java_methods]
impl Class {
    #[method(constructor, private)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.lang.Class::<init>({this:?})");

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        Ok(())
    }

    #[method(public)]
    async fn get_name(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.lang.Class::getName({this:?})");

        let class_name = JavaLangClass::name(jvm, &this).await?;
        let result = JavaLangString::from_rust_string(jvm, &class_name.replace('/', ".")).await?;

        Ok(result.into())
    }

    #[method(public)]
    async fn is_primitive(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        let name = JavaLangClass::name(jvm, &this).await?;
        Ok(matches!(
            name.as_str(),
            "boolean" | "byte" | "char" | "short" | "int" | "long" | "float" | "double"
        ))
    }

    #[method(public)]
    async fn is_array(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        let name = JavaLangClass::name(jvm, &this).await?;
        Ok(name.starts_with('['))
    }

    #[method(public)]
    async fn is_interface(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        let name = JavaLangClass::name(jvm, &this).await?;
        if name.starts_with('[') || matches!(name.as_str(), "boolean" | "byte" | "char" | "short" | "int" | "long" | "float" | "double") {
            return Ok(false);
        }

        let class = JavaLangClass::to_rust_class(jvm, &this).await?;
        Ok(class.access_flags().contains(ClassAccessFlags::INTERFACE))
    }

    #[method(public)]
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

    #[method(public)]
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

    #[method(public)]
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

    #[method(public, native)]
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

    #[method(public)]
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

    #[method(public)]
    async fn get_class_loader(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<ClassLoader>> {
        tracing::debug!("java.lang.Class::getClassLoader({this:?})");

        jvm.get_field(&this, "java/lang/Class", "classLoader", "Ljava/lang/ClassLoader;").await
    }

    #[method(public)]
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
        let defining_loader: ClassInstanceRef<ClassLoader> =
            jvm.get_field(&this, "java/lang/Class", "classLoader", "Ljava/lang/ClassLoader;").await?;
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

    #[method(public)]
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

    // Applies the Class resource naming rule and picks the loader that searches for the resulting name.
    async fn resolve_resource(
        jvm: &Jvm,
        this: &ClassInstanceRef<Self>,
        name: &ClassInstanceRef<String>,
    ) -> Result<(Box<dyn ClassInstance>, ClassInstanceRef<String>)> {
        if name.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "name is null").await);
        }

        // An array class takes the package of its element type, and primitive types belong to java.lang.
        let class_name = JavaLangClass::name(jvm, this).await?;
        let package = if class_name.starts_with('[') {
            match class_name.trim_start_matches('[').strip_prefix('L') {
                Some(element_class_name) => element_class_name.rsplit_once('/').map(|(package, _)| package),
                None => Some("java/lang"),
            }
        } else if matches!(
            class_name.as_str(),
            "boolean" | "byte" | "char" | "short" | "int" | "long" | "float" | "double" | "void"
        ) {
            Some("java/lang")
        } else {
            class_name.rsplit_once('/').map(|(package, _)| package)
        };

        // A leading '/' makes the name absolute; otherwise it is relative to this class's package.
        let name = JavaLangString::to_rust_string(jvm, name).await?;
        let name = match (name.strip_prefix('/'), package) {
            (Some(absolute_name), _) => absolute_name.into(),
            (None, Some(package)) => format!("{package}/{name}"),
            (None, None) => name,
        };
        let name = JavaLangString::from_rust_string(jvm, &name).await?;

        let class_loader: ClassInstanceRef<ClassLoader> = jvm.get_field(this, "java/lang/Class", "classLoader", "Ljava/lang/ClassLoader;").await?;

        let class_loader = if class_loader.is_null() {
            // TODO ClassLoader.getSystemResourceAsStream?
            JavaLangClassLoader::get_system_class_loader(jvm).await?
        } else {
            class_loader.into()
        };

        Ok((class_loader, name.into()))
    }

    #[method(public)]
    async fn get_resource(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        name: ClassInstanceRef<String>,
    ) -> Result<ClassInstanceRef<URL>> {
        tracing::debug!("java.lang.Class::getResource({this:?}, {name:?})");

        let (class_loader, name) = Self::resolve_resource(jvm, &this, &name).await?;

        jvm.invoke_virtual(
            &class_loader,
            "java/lang/ClassLoader",
            "getResource",
            "(Ljava/lang/String;)Ljava/net/URL;",
            (name,),
        )
        .await
    }

    #[method(public)]
    async fn get_resource_as_stream(
        jvm: &Jvm,
        _context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        name: ClassInstanceRef<String>,
    ) -> Result<ClassInstanceRef<InputStream>> {
        tracing::debug!("java.lang.Class::getResourceAsStream({this:?}, {name:?})");

        let (class_loader, name) = Self::resolve_resource(jvm, &this, &name).await?;

        jvm.invoke_virtual(
            &class_loader,
            "java/lang/ClassLoader",
            "getResourceAsStream",
            "(Ljava/lang/String;)Ljava/io/InputStream;",
            (name,),
        )
        .await
    }

    #[method(static)]
    async fn for_name(jvm: &Jvm, _context: &mut RuntimeContext, name: ClassInstanceRef<String>) -> Result<ClassInstanceRef<Class>> {
        tracing::debug!("java.lang.Class::forName({name:?})");

        if name.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "name").await);
        }

        let rust_name = JavaLangString::to_rust_string(jvm, &name).await?;
        let qualified_name = rust_name.replace('.', "/");

        match jvm.resolve_class(&qualified_name).await {
            Ok(class) => {
                jvm.ensure_initialized(&class).await?;
                Ok(class.java_class().into())
            }
            Err(_) => Err(jvm.exception("java/lang/ClassNotFoundException", &rust_name).await),
        }
    }
}
