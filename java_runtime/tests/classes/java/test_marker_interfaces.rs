use java_runtime::classes::java::lang::Class;
use jvm::{Array, ClassInstanceRef, Result, runtime::JavaLangClass};
use test_utils::test_jvm;

#[tokio::test]
async fn marker_interfaces_match_public_class_declarations() -> Result<()> {
    let jvm = test_jvm().await?;

    let direct_markers: [(&str, &[&str]); 15] = [
        ("java/lang/Throwable", &["java/io/Serializable"]),
        ("java/lang/Class", &["java/io/Serializable"]),
        ("java/io/File", &["java/io/Serializable"]),
        ("java/net/URL", &["java/io/Serializable"]),
        ("java/util/Calendar", &["java/io/Serializable", "java/lang/Cloneable"]),
        ("java/util/ArrayList", &["java/lang/Cloneable", "java/io/Serializable"]),
        ("java/util/HashMap", &["java/lang/Cloneable", "java/io/Serializable"]),
        ("java/util/HashSet", &["java/lang/Cloneable", "java/io/Serializable"]),
        ("java/util/Hashtable", &["java/lang/Cloneable", "java/io/Serializable"]),
        ("java/util/Vector", &["java/lang/Cloneable", "java/io/Serializable"]),
        ("java/util/jar/Attributes", &["java/lang/Cloneable"]),
        ("java/util/jar/Manifest", &["java/lang/Cloneable"]),
        ("java/util/logging/Level", &["java/io/Serializable"]),
        ("java/util/logging/LogRecord", &["java/io/Serializable"]),
        ("java/util/zip/ZipEntry", &["java/lang/Cloneable"]),
    ];

    for (class_name, expected_markers) in direct_markers {
        let class = jvm.resolve_class(class_name).await?.java_class();
        let interfaces: ClassInstanceRef<Array<Class>> = jvm.invoke_virtual(&class, "getInterfaces", "()[Ljava/lang/Class;", ()).await?;
        let interfaces: Vec<ClassInstanceRef<Class>> = jvm.load_array(&interfaces, 0, jvm.array_length(&interfaces).await?).await?;
        let mut actual_markers = Vec::new();
        for interface in interfaces {
            let interface_name = JavaLangClass::name(&jvm, &interface).await?;
            if interface_name == "java/lang/Cloneable" || interface_name == "java/io/Serializable" {
                actual_markers.push(interface_name);
            }
        }
        assert_eq!(
            actual_markers.iter().map(String::as_str).collect::<Vec<_>>(),
            expected_markers,
            "incorrect direct marker interfaces for {class_name}"
        );

        for marker_name in expected_markers {
            let marker = jvm.resolve_class(marker_name).await?.java_class();
            assert!(
                jvm.invoke_virtual::<_, bool>(&marker, "isAssignableFrom", "(Ljava/lang/Class;)Z", (class.clone(),))
                    .await?,
                "{class_name} must implement {marker_name}"
            );
        }
    }

    let inherited_markers: [(&str, &[&str]); 6] = [
        ("java/lang/RuntimeException", &["java/io/Serializable"]),
        ("java/util/GregorianCalendar", &["java/io/Serializable", "java/lang/Cloneable"]),
        ("java/util/Properties", &["java/lang/Cloneable", "java/io/Serializable"]),
        ("java/util/Stack", &["java/lang/Cloneable", "java/io/Serializable"]),
        ("java/util/LinkedHashMap", &["java/lang/Cloneable", "java/io/Serializable"]),
        ("java/util/jar/JarEntry", &["java/lang/Cloneable"]),
    ];

    for (class_name, expected_markers) in inherited_markers {
        let class = jvm.resolve_class(class_name).await?.java_class();
        let interfaces: ClassInstanceRef<Array<Class>> = jvm.invoke_virtual(&class, "getInterfaces", "()[Ljava/lang/Class;", ()).await?;
        let interfaces: Vec<ClassInstanceRef<Class>> = jvm.load_array(&interfaces, 0, jvm.array_length(&interfaces).await?).await?;
        for interface in interfaces {
            let interface_name = JavaLangClass::name(&jvm, &interface).await?;
            assert!(
                interface_name != "java/lang/Cloneable" && interface_name != "java/io/Serializable",
                "{class_name} must inherit {interface_name}, not declare it directly"
            );
        }

        for marker_name in expected_markers {
            let marker = jvm.resolve_class(marker_name).await?.java_class();
            assert!(
                jvm.invoke_virtual::<_, bool>(&marker, "isAssignableFrom", "(Ljava/lang/Class;)Z", (class.clone(),))
                    .await?,
                "{class_name} must inherit {marker_name}"
            );
        }
    }

    for class_name in ["java/text/FieldPosition", "java/text/ParsePosition"] {
        let class = jvm.resolve_class(class_name).await?.java_class();
        let interfaces: ClassInstanceRef<Array<Class>> = jvm.invoke_virtual(&class, "getInterfaces", "()[Ljava/lang/Class;", ()).await?;
        let interfaces: Vec<ClassInstanceRef<Class>> = jvm.load_array(&interfaces, 0, jvm.array_length(&interfaces).await?).await?;
        for interface in interfaces {
            let interface_name = JavaLangClass::name(&jvm, &interface).await?;
            assert!(interface_name != "java/lang/Cloneable" && interface_name != "java/io/Serializable");
        }

        for marker_name in ["java/lang/Cloneable", "java/io/Serializable"] {
            let marker = jvm.resolve_class(marker_name).await?.java_class();
            assert!(
                !jvm.invoke_virtual::<_, bool>(&marker, "isAssignableFrom", "(Ljava/lang/Class;)Z", (class.clone(),))
                    .await?,
                "{class_name} must not implement {marker_name}"
            );
        }
    }

    Ok(())
}
