mod test_helper;

use std::{fs, path::Path};

use jvm::{JavaError, JavaValue, Result};

use test_helper::{run_class, run_jar};

// TODO parameterized tests..
#[tokio::test]
async fn test_class() -> Result<()> {
    let base_path = Path::new("test-data");

    let paths = fs::read_dir(base_path).unwrap();

    for path in paths {
        let path = path.unwrap().path();
        let extension = path.extension();
        if let Some(x) = extension {
            if x != "class" && x != "jar" {
                continue;
            }
        } else {
            continue;
        }

        let name = path.file_stem().unwrap().to_str().unwrap();
        if name.contains('$') {
            continue;
        }

        let expected_path = base_path.join(format!("{name}.txt"));
        let expected = fs::read_to_string(expected_path).unwrap();

        let result = if extension.unwrap().to_str().unwrap() == "jar" {
            run_jar(&path, &[]).await
        } else {
            run_class(&path, &[Path::new("./test-data/")], &[]).await
        };

        if let Err(err) = result {
            panic!("Test {name} failed with error: {err}");
        } else {
            assert_eq!(result.as_ref().unwrap().clone(), expected, "Test {} failed: {}", name, result.unwrap());
        }
    }

    Ok(())
}

#[tokio::test]
async fn test_special_resolution_errors() -> Result<()> {
    let jvm = test_utils::test_jvm().await?;
    for name in ["Abstract$Base", "Abstract$Derived", "StaticInherit$Base", "StaticInherit$Sub"] {
        let class = jvm_bytecode::ClassDefinitionImpl::from_classfile(&fs::read(format!("test-data/{name}.class")).unwrap()).unwrap();
        jvm.register_class(Box::new(class), None).await?;
    }
    let derived = jvm.new_class("Abstract$Derived", "()V", ()).await?;
    let static_sub = jvm.new_class("StaticInherit$Sub", "()V", ()).await?;

    for (instance, owner, name, descriptor, args, expected) in [
        (
            &derived,
            "Abstract$Derived",
            "<init>",
            "(I)V",
            vec![JavaValue::Int(1)],
            "java/lang/NoSuchMethodError",
        ),
        (&derived, "Abstract$Base", "test", "()V", vec![], "java/lang/AbstractMethodError"),
        (
            &static_sub,
            "StaticInherit$Sub",
            "method",
            "()I",
            vec![],
            "java/lang/IncompatibleClassChangeError",
        ),
    ] {
        let result: Result<JavaValue> = jvm.invoke_special(instance, owner, name, descriptor, args).await;
        match result {
            Err(JavaError::JavaException(exception)) => assert_eq!(exception.class_definition().name(), expected),
            other => panic!("Expected {expected} for {owner}.{name}{descriptor}, got {other:?}"),
        }
    }

    Ok(())
}
