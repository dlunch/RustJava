use java_constants::MethodAccessFlags;
use java_runtime::{
    Runtime,
    classes::java::{
        lang::{Class, ClassLoader, String},
        net::URL,
    },
    get_bootstrap_class_loader,
};
use jvm::{
    Array, ClassInstanceRef, JavaError, Jvm, Result,
    runtime::{JavaLangClass, JavaLangString},
};

use test_utils::{TestRuntime, test_jvm};

#[tokio::test]
async fn cls_01_to_04_descriptors_and_standard_class_metadata() -> Result<()> {
    let proto = Class::as_proto();
    for (name, descriptor) in [
        ("getSuperclass", "()Ljava/lang/Class;"),
        ("getClassLoader", "()Ljava/lang/ClassLoader;"),
        ("getComponentType", "()Ljava/lang/Class;"),
        ("getInterfaces", "()[Ljava/lang/Class;"),
    ] {
        let method = proto
            .methods
            .iter()
            .find(|method| method.name == name && method.descriptor == descriptor)
            .unwrap_or_else(|| panic!("missing java/lang/Class.{name}{descriptor}"));
        assert!(method.access_flags.contains(MethodAccessFlags::PUBLIC));
    }

    let jvm = test_jvm().await?;
    let object = jvm.resolve_class("java/lang/Object").await?.java_class();
    let string = jvm.resolve_class("java/lang/String").await?.java_class();
    let comparable = jvm.resolve_class("java/lang/Comparable").await?.java_class();
    let serializable = jvm.resolve_class("java/io/Serializable").await?.java_class();
    let primitive = jvm.get_static_field("java/lang/Integer", "TYPE", "Ljava/lang/Class;").await?;
    let primitive_array = jvm.resolve_class("[I").await?.java_class();
    let string_array = jvm.resolve_class("[Ljava/lang/String;").await?.java_class();
    let string_matrix = jvm.resolve_class("[[Ljava/lang/String;").await?.java_class();

    let superclass: ClassInstanceRef<Class> = jvm
        .invoke_virtual(&string, &string.class_definition().name(), "getSuperclass", "()Ljava/lang/Class;", ())
        .await?;
    assert_eq!(superclass.identity(), object.identity());
    for class in [&object, &primitive] {
        let superclass: ClassInstanceRef<Class> = jvm
            .invoke_virtual(class, &class.class_definition().name(), "getSuperclass", "()Ljava/lang/Class;", ())
            .await?;
        assert!(superclass.is_null());
    }
    let array_superclass: ClassInstanceRef<Class> = jvm
        .invoke_virtual(
            &string_array,
            &string_array.class_definition().name(),
            "getSuperclass",
            "()Ljava/lang/Class;",
            (),
        )
        .await?;
    assert_eq!(array_superclass.identity(), object.identity());

    let primitive_component: ClassInstanceRef<Class> = jvm
        .invoke_virtual(
            &primitive_array,
            &primitive_array.class_definition().name(),
            "getComponentType",
            "()Ljava/lang/Class;",
            (),
        )
        .await?;
    assert_eq!(primitive_component.identity(), primitive.identity());
    let string_component: ClassInstanceRef<Class> = jvm
        .invoke_virtual(
            &string_array,
            &string_array.class_definition().name(),
            "getComponentType",
            "()Ljava/lang/Class;",
            (),
        )
        .await?;
    assert_eq!(string_component.identity(), string.identity());
    let matrix_component: ClassInstanceRef<Class> = jvm
        .invoke_virtual(
            &string_matrix,
            &string_matrix.class_definition().name(),
            "getComponentType",
            "()Ljava/lang/Class;",
            (),
        )
        .await?;
    assert_eq!(matrix_component.identity(), string_array.identity());
    let non_array_component: ClassInstanceRef<Class> = jvm
        .invoke_virtual(&string, &string.class_definition().name(), "getComponentType", "()Ljava/lang/Class;", ())
        .await?;
    assert!(non_array_component.is_null());

    let string_interfaces: ClassInstanceRef<Array<Class>> = jvm
        .invoke_virtual(&string, &string.class_definition().name(), "getInterfaces", "()[Ljava/lang/Class;", ())
        .await?;
    let string_interfaces: Vec<ClassInstanceRef<Class>> = jvm.load_array(&string_interfaces, 0, 2).await?;
    assert_eq!(string_interfaces[0].identity(), serializable.identity());
    assert_eq!(string_interfaces[1].identity(), comparable.identity());

    let array_interfaces: ClassInstanceRef<Array<Class>> = jvm
        .invoke_virtual(
            &string_array,
            &string_array.class_definition().name(),
            "getInterfaces",
            "()[Ljava/lang/Class;",
            (),
        )
        .await?;
    let array_interfaces: Vec<ClassInstanceRef<Class>> = jvm.load_array(&array_interfaces, 0, 2).await?;
    assert_eq!(JavaLangClass::name(&jvm, &array_interfaces[0]).await?, "java/lang/Cloneable");
    assert_eq!(JavaLangClass::name(&jvm, &array_interfaces[1]).await?, "java/io/Serializable");

    for class in [object, string, primitive_array, string_array, string_matrix] {
        let loader: ClassInstanceRef<ClassLoader> = jvm
            .invoke_virtual(
                &class,
                &class.class_definition().name(),
                "getClassLoader",
                "()Ljava/lang/ClassLoader;",
                (),
            )
            .await?;
        assert!(loader.is_null());
    }

    Ok(())
}

#[tokio::test]
async fn test_class() -> Result<()> {
    let jvm = test_jvm().await?;

    let java_class = jvm.resolve_class("java/lang/String").await?.java_class();

    let rust_class = JavaLangClass::to_rust_class(&jvm, &java_class).await?;
    assert_eq!(rust_class.name(), "java/lang/String");

    // try call to_rust_class twice to test if box is not dropped
    let rust_class = JavaLangClass::to_rust_class(&jvm, &java_class).await?;
    assert_eq!(rust_class.name(), "java/lang/String");

    Ok(())
}

#[tokio::test]
async fn test_is_assignable_from() -> Result<()> {
    let jvm = test_jvm().await?;

    let string_class = jvm.resolve_class("java/lang/String").await?.java_class();
    let object_class = jvm.resolve_class("java/lang/Object").await?.java_class();

    let result: bool = jvm
        .invoke_virtual(
            &object_class,
            &object_class.class_definition().name(),
            "isAssignableFrom",
            "(Ljava/lang/Class;)Z",
            (string_class.clone(),),
        )
        .await?;
    assert!(result);

    let thread_class = jvm.resolve_class("java/lang/Thread").await?.java_class();

    let result: bool = jvm
        .invoke_virtual(
            &string_class,
            &string_class.class_definition().name(),
            "isAssignableFrom",
            "(Ljava/lang/Class;)Z",
            (thread_class,),
        )
        .await?;
    assert!(!result);

    let string_array_class = jvm.resolve_class("[Ljava/lang/String;").await?.java_class();
    let object_array_class = jvm.resolve_class("[Ljava/lang/Object;").await?.java_class();
    let cloneable_class = jvm.resolve_class("java/lang/Cloneable").await?.java_class();
    let serializable_class = jvm.resolve_class("java/io/Serializable").await?.java_class();

    assert!(
        jvm.invoke_virtual::<_, bool>(
            &object_array_class,
            &object_array_class.class_definition().name(),
            "isAssignableFrom",
            "(Ljava/lang/Class;)Z",
            (string_array_class.clone(),),
        )
        .await?
    );
    assert!(
        !jvm.invoke_virtual::<_, bool>(
            &string_array_class,
            &string_array_class.class_definition().name(),
            "isAssignableFrom",
            "(Ljava/lang/Class;)Z",
            (object_array_class,),
        )
        .await?
    );
    assert!(
        jvm.invoke_virtual::<_, bool>(
            &object_class,
            &object_class.class_definition().name(),
            "isAssignableFrom",
            "(Ljava/lang/Class;)Z",
            (string_array_class.clone(),),
        )
        .await?
    );
    assert!(
        jvm.invoke_virtual::<_, bool>(
            &cloneable_class,
            &cloneable_class.class_definition().name(),
            "isAssignableFrom",
            "(Ljava/lang/Class;)Z",
            (string_array_class.clone(),),
        )
        .await?
    );
    assert!(
        jvm.invoke_virtual::<_, bool>(
            &serializable_class,
            &serializable_class.class_definition().name(),
            "isAssignableFrom",
            "(Ljava/lang/Class;)Z",
            (string_array_class,),
        )
        .await?
    );

    Ok(())
}

#[tokio::test]
async fn test_for_name() -> Result<()> {
    let jvm = test_jvm().await?;

    let class_name = JavaLangString::from_rust_string(&jvm, "java.lang.String").await?;
    let class: ClassInstanceRef<Class> = jvm
        .invoke_static("java/lang/Class", "forName", "(Ljava/lang/String;)Ljava/lang/Class;", (class_name,))
        .await?;

    let rust_class = JavaLangClass::to_rust_class(&jvm, &class).await?;
    assert_eq!(rust_class.name(), "java/lang/String");

    let result: Result<ClassInstanceRef<Class>> = jvm
        .invoke_static("java/lang/Class", "forName", "(Ljava/lang/String;)Ljava/lang/Class;", (None,))
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("Class.forName(null) must throw NullPointerException");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));

    Ok(())
}

#[tokio::test]
async fn test_primitive_class_api() -> Result<()> {
    let jvm = test_jvm().await?;

    for name in ["boolean", "byte", "char", "short", "int", "long", "float", "double"] {
        let primitive = JavaLangClass::from_rust_primitive(&jvm, name).await?;
        let primitive_name = JavaLangClass::name(&jvm, &primitive).await?;
        assert_eq!(primitive_name, name);

        let virtual_name: ClassInstanceRef<String> = jvm
            .invoke_virtual(&primitive, &primitive.class_definition().name(), "getName", "()Ljava/lang/String;", ())
            .await?;
        assert_eq!(JavaLangString::to_rust_string(&jvm, &virtual_name).await?, name);

        let is_primitive: bool = jvm
            .invoke_virtual(&primitive, &primitive.class_definition().name(), "isPrimitive", "()Z", ())
            .await?;
        assert!(is_primitive);

        let class_name = JavaLangString::from_rust_string(&jvm, name).await?;
        let result: Result<ClassInstanceRef<Class>> = jvm
            .invoke_static("java/lang/Class", "forName", "(Ljava/lang/String;)Ljava/lang/Class;", (class_name,))
            .await;
        let Err(JavaError::JavaException(exception)) = result else {
            panic!("Class.forName must reject primitive source names");
        };
        assert!(jvm.is_instance(&*exception, "java/lang/ClassNotFoundException"));
    }

    let object_class = jvm.resolve_class("java/lang/Object").await?.java_class();
    let string_class = jvm.resolve_class("java/lang/String").await?.java_class();
    let primitive = JavaLangClass::from_rust_primitive(&jvm, "int").await?;
    let other_primitive = JavaLangClass::from_rust_primitive(&jvm, "long").await?;

    let result: bool = jvm
        .invoke_virtual(
            &primitive,
            &primitive.class_definition().name(),
            "isAssignableFrom",
            "(Ljava/lang/Class;)Z",
            (primitive.clone(),),
        )
        .await?;
    assert!(result);

    let result: bool = jvm
        .invoke_virtual(
            &primitive,
            &primitive.class_definition().name(),
            "isAssignableFrom",
            "(Ljava/lang/Class;)Z",
            (other_primitive,),
        )
        .await?;
    assert!(!result);

    let result: bool = jvm
        .invoke_virtual(
            &primitive,
            &primitive.class_definition().name(),
            "isAssignableFrom",
            "(Ljava/lang/Class;)Z",
            (string_class.clone(),),
        )
        .await?;
    assert!(!result);

    let result: bool = jvm
        .invoke_virtual(
            &object_class,
            &object_class.class_definition().name(),
            "isAssignableFrom",
            "(Ljava/lang/Class;)Z",
            (primitive,),
        )
        .await?;
    assert!(!result);

    let result: Result<bool> = jvm
        .invoke_virtual(
            &object_class,
            &object_class.class_definition().name(),
            "isAssignableFrom",
            "(Ljava/lang/Class;)Z",
            (None,),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("Class.isAssignableFrom(null) must throw NullPointerException");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));

    Ok(())
}

#[tokio::test]
async fn test_wrapper_type_fields_survive_gc() -> Result<()> {
    let jvm = test_jvm().await?;

    for (wrapper, primitive_name) in [
        ("java/lang/Boolean", "boolean"),
        ("java/lang/Byte", "byte"),
        ("java/lang/Character", "char"),
        ("java/lang/Short", "short"),
        ("java/lang/Integer", "int"),
        ("java/lang/Long", "long"),
        ("java/lang/Float", "float"),
        ("java/lang/Double", "double"),
    ] {
        let typ = jvm.get_static_field(wrapper, "TYPE", "Ljava/lang/Class;").await?;
        let name: ClassInstanceRef<String> = jvm
            .invoke_virtual(&typ, &typ.class_definition().name(), "getName", "()Ljava/lang/String;", ())
            .await?;
        assert_eq!(JavaLangString::to_rust_string(&jvm, &name).await?, primitive_name);
        assert!(
            jvm.invoke_virtual::<_, bool>(&typ, &typ.class_definition().name(), "isPrimitive", "()Z", ())
                .await?
        );
    }

    let _: () = jvm.invoke_static("java/lang/System", "gc", "()V", ()).await?;

    for wrapper in [
        "java/lang/Boolean",
        "java/lang/Byte",
        "java/lang/Character",
        "java/lang/Short",
        "java/lang/Integer",
        "java/lang/Long",
        "java/lang/Float",
        "java/lang/Double",
    ] {
        let typ = jvm.get_static_field(wrapper, "TYPE", "Ljava/lang/Class;").await?;
        assert!(
            jvm.invoke_virtual::<_, bool>(&typ, &typ.class_definition().name(), "isPrimitive", "()Z", ())
                .await?
        );
    }

    Ok(())
}

#[tokio::test]
async fn test_cldc_class_queries_and_new_instance() -> Result<()> {
    let jvm = test_jvm().await?;

    let string_class = jvm.resolve_class("java/lang/String").await?.java_class();
    let runnable_class = jvm.resolve_class("java/lang/Runnable").await?.java_class();
    let array_class = jvm.resolve_class("[Ljava/lang/String;").await?.java_class();

    assert!(
        !jvm.invoke_virtual::<_, bool>(&string_class, &string_class.class_definition().name(), "isArray", "()Z", ())
            .await?
    );
    assert!(
        jvm.invoke_virtual::<_, bool>(&array_class, &array_class.class_definition().name(), "isArray", "()Z", ())
            .await?
    );
    assert!(
        jvm.invoke_virtual::<_, bool>(&runnable_class, &runnable_class.class_definition().name(), "isInterface", "()Z", ())
            .await?
    );

    let value = JavaLangString::from_rust_string(&jvm, "value").await?;
    assert!(
        jvm.invoke_virtual::<_, bool>(
            &string_class,
            &string_class.class_definition().name(),
            "isInstance",
            "(Ljava/lang/Object;)Z",
            (value,)
        )
        .await?
    );
    assert!(
        !jvm.invoke_virtual::<_, bool>(
            &string_class,
            &string_class.class_definition().name(),
            "isInstance",
            "(Ljava/lang/Object;)Z",
            (None,)
        )
        .await?
    );

    let object_class = jvm.resolve_class("java/lang/Object").await?.java_class();
    let instance: ClassInstanceRef<java_runtime::classes::java::lang::Object> = jvm
        .invoke_virtual(
            &object_class,
            &object_class.class_definition().name(),
            "newInstance",
            "()Ljava/lang/Object;",
            (),
        )
        .await?;
    assert!(jvm.is_instance(&**instance, "java/lang/Object"));

    let result: Result<ClassInstanceRef<java_runtime::classes::java::lang::Object>> = jvm
        .invoke_virtual(
            &runnable_class,
            &runnable_class.class_definition().name(),
            "newInstance",
            "()Ljava/lang/Object;",
            (),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("interface instantiation must throw InstantiationException");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/InstantiationException"));

    let text: ClassInstanceRef<String> = jvm
        .invoke_virtual(
            &string_class,
            &string_class.class_definition().name(),
            "toString",
            "()Ljava/lang/String;",
            (),
        )
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &text).await?, "class java.lang.String");
    let text: ClassInstanceRef<String> = jvm
        .invoke_virtual(
            &runnable_class,
            &runnable_class.class_definition().name(),
            "toString",
            "()Ljava/lang/String;",
            (),
        )
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &text).await?, "interface java.lang.Runnable");

    Ok(())
}

#[tokio::test]
async fn test_base_class_loader_delegates_to_bootstrap_and_find_class_throws() -> Result<()> {
    let jvm = test_jvm().await?;
    let urls = jvm.instantiate_array("Ljava/net/URL;", 0).await?;
    let loader: ClassInstanceRef<ClassLoader> = jvm
        .new_class("java/net/URLClassLoader", "([Ljava/net/URL;Ljava/lang/ClassLoader;)V", (urls, None))
        .await?
        .into();

    let name = JavaLangString::from_rust_string(&jvm, "java/util/Random").await?;
    let class: ClassInstanceRef<Class> = jvm
        .invoke_virtual(
            &loader,
            "java/lang/ClassLoader",
            "loadClass",
            "(Ljava/lang/String;)Ljava/lang/Class;",
            (name,),
        )
        .await?;
    assert!(!class.is_null());

    let name = JavaLangString::from_rust_string(&jvm, "missing.Type").await?;

    let result: Result<ClassInstanceRef<Class>> = jvm
        .invoke_special(
            &loader,
            "java/lang/ClassLoader",
            "findClass",
            "(Ljava/lang/String;)Ljava/lang/Class;",
            (name,),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("ClassLoader.findClass must throw ClassNotFoundException");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/ClassNotFoundException"));

    Ok(())
}

#[tokio::test]
async fn test_system_class_loader_uses_rustjar_parent() -> Result<()> {
    let runtime = TestRuntime::new(Default::default());
    let bootstrap_class_loader = get_bootstrap_class_loader(Box::new(runtime.clone()));
    let class_path = std::env::join_paths(["external.rustjar", "classes"]).unwrap().into_string().unwrap();
    let properties = [("java.class.path", class_path.as_str())].into_iter().collect();
    let jvm = Jvm::new(bootstrap_class_loader, move || runtime.current_task_id(), properties).await?;

    let system_class_loader: ClassInstanceRef<ClassLoader> = jvm
        .invoke_static("java/lang/ClassLoader", "getSystemClassLoader", "()Ljava/lang/ClassLoader;", ())
        .await?;
    let rustjar_class_loader: ClassInstanceRef<ClassLoader> = jvm.get_field(&system_class_loader, "parent", "Ljava/lang/ClassLoader;").await?;

    assert!(jvm.is_instance(&**rustjar_class_loader, "org/rustjava/lang/RustJarClassLoader"));

    let class_paths: ClassInstanceRef<Array<String>> = jvm.get_field(&rustjar_class_loader, "classPaths", "[Ljava/lang/String;").await?;
    assert_eq!(jvm.array_length(&class_paths).await?, 2);
    let class_paths: Vec<ClassInstanceRef<String>> = jvm.load_array(&class_paths, 0, 2).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &class_paths[0]).await?, "external.rustjar");
    assert_eq!(JavaLangString::to_rust_string(&jvm, &class_paths[1]).await?, "classes");

    let urls: ClassInstanceRef<Array<URL>> = jvm.get_field(&system_class_loader, "urls", "[Ljava/net/URL;").await?;
    assert_eq!(jvm.array_length(&urls).await?, 2);
    let urls: Vec<ClassInstanceRef<URL>> = jvm.load_array(&urls, 0, 2).await?;
    let rustjar_file: ClassInstanceRef<String> = jvm
        .invoke_virtual(&urls[0], &urls[0].class_definition().name(), "getFile", "()Ljava/lang/String;", ())
        .await?;
    let classes_file: ClassInstanceRef<String> = jvm
        .invoke_virtual(&urls[1], &urls[1].class_definition().name(), "getFile", "()Ljava/lang/String;", ())
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &rustjar_file).await?, "external.rustjar");
    assert_eq!(JavaLangString::to_rust_string(&jvm, &classes_file).await?, "classes");

    Ok(())
}

#[tokio::test]
async fn test_define_class_translates_parser_errors_to_java_errors() -> Result<()> {
    let jvm = test_jvm().await?;
    let urls = jvm.instantiate_array("Ljava/net/URL;", 0).await?;
    let loader: ClassInstanceRef<ClassLoader> = jvm
        .new_class("java/net/URLClassLoader", "([Ljava/net/URL;Ljava/lang/ClassLoader;)V", (urls, None))
        .await?
        .into();
    let name: ClassInstanceRef<String> = None.into();

    let mut unsupported_version = include_bytes!("../../../../../test_data/Hello.class").to_vec();
    unsupported_version[6..8].copy_from_slice(&71u16.to_be_bytes());

    let mut verification_error = include_bytes!("../../../../../test_data/MultiArray.class").to_vec();
    let multianewarray = [0x10, 0x0a, 0x10, 0x0a, 0x10, 0x0a, 0x10, 0x0a, 0x10, 0x0a, 0xc5, 0x00, 0x07, 0x05];
    let multianewarray_offset = verification_error
        .windows(multianewarray.len())
        .position(|window| window == multianewarray)
        .expect("MultiArray fixture must contain the expected multianewarray instruction");
    verification_error[multianewarray_offset + multianewarray.len() - 1] = 6;

    for (data, expected_exception) in [
        (vec![0, 1, 2, 3], "java/lang/ClassFormatError"),
        (unsupported_version, "java/lang/UnsupportedClassVersionError"),
        (verification_error, "java/lang/VerifyError"),
    ] {
        let length = data.len() as i32;
        let mut bytes = jvm.instantiate_array("B", data.len()).await?;
        jvm.store_array(&mut bytes, 0, data.into_iter().map(|byte| byte as i8).collect::<Vec<_>>())
            .await?;

        let result: Result<ClassInstanceRef<Class>> = jvm
            .invoke_virtual(
                &loader,
                "java/lang/ClassLoader",
                "defineClass",
                "(Ljava/lang/String;[BII)Ljava/lang/Class;",
                (name.clone(), bytes, 0, length),
            )
            .await;
        let Err(JavaError::JavaException(exception)) = result else {
            panic!("ClassLoader.defineClass must translate malformed class files to Java errors");
        };
        assert!(jvm.is_instance(&*exception, expected_exception));
    }

    Ok(())
}

#[tokio::test]
async fn test_define_class_validates_the_byte_range() -> Result<()> {
    let jvm = test_jvm().await?;
    let urls = jvm.instantiate_array("Ljava/net/URL;", 0).await?;
    let loader: ClassInstanceRef<ClassLoader> = jvm
        .new_class("java/net/URLClassLoader", "([Ljava/net/URL;Ljava/lang/ClassLoader;)V", (urls, None))
        .await?
        .into();
    let name: ClassInstanceRef<String> = None.into();
    let bytes: ClassInstanceRef<Array<i8>> = jvm.instantiate_array("B", 4).await?.into();

    for (bytes, offset, length, expected_exception) in [
        (bytes.clone(), -1, 1, "java/lang/IndexOutOfBoundsException"),
        (bytes, 2, 3, "java/lang/IndexOutOfBoundsException"),
        (ClassInstanceRef::new(None), 0, 0, "java/lang/NullPointerException"),
    ] {
        let result: Result<ClassInstanceRef<Class>> = jvm
            .invoke_virtual(
                &loader,
                "java/lang/ClassLoader",
                "defineClass",
                "(Ljava/lang/String;[BII)Ljava/lang/Class;",
                (name.clone(), bytes, offset, length),
            )
            .await;
        let Err(JavaError::JavaException(exception)) = result else {
            panic!("ClassLoader.defineClass must validate its byte range");
        };
        assert!(jvm.is_instance(&*exception, expected_exception));
    }

    Ok(())
}
