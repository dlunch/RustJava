use java_constants::{ClassAccessFlags, MethodAccessFlags};
use java_runtime::classes::java::lang::{Comparable, Number};
use jvm::{JavaError, Result};

use test_utils::test_jvm;

#[tokio::test]
async fn test_number_is_abstract() -> Result<()> {
    let jvm = test_jvm().await?;

    let number_proto = Number::as_proto();
    assert!(number_proto.access_flags.contains(ClassAccessFlags::PUBLIC | ClassAccessFlags::ABSTRACT));
    for method in &number_proto.methods {
        assert!(method.access_flags.contains(MethodAccessFlags::PUBLIC));
        if method.name != "<init>" && method.name != "byteValue" && method.name != "shortValue" {
            assert!(method.access_flags.contains(MethodAccessFlags::ABSTRACT));
        }
    }

    let comparable_proto = Comparable::as_proto();
    assert!(
        comparable_proto
            .access_flags
            .contains(ClassAccessFlags::PUBLIC | ClassAccessFlags::INTERFACE | ClassAccessFlags::ABSTRACT)
    );
    assert!(
        comparable_proto.methods[0]
            .access_flags
            .contains(MethodAccessFlags::PUBLIC | MethodAccessFlags::ABSTRACT)
    );

    let number_class = jvm.resolve_class("java/lang/Number").await?.java_class();
    let serializable_class = jvm.resolve_class("java/io/Serializable").await?.java_class();
    let is_serializable: bool = jvm
        .invoke_virtual(&serializable_class, "isAssignableFrom", "(Ljava/lang/Class;)Z", (number_class.clone(),))
        .await?;
    assert!(is_serializable);
    let comparable_class = jvm.resolve_class("java/lang/Comparable").await?.java_class();

    let integer = jvm.new_class("java/lang/Integer", "(I)V", (257,)).await?;
    assert_eq!(jvm.invoke_virtual::<_, i8>(&integer, "byteValue", "()B", ()).await?, 1);
    assert_eq!(jvm.invoke_virtual::<_, i16>(&integer, "shortValue", "()S", ()).await?, 257);
    let negative = jvm.new_class("java/lang/Integer", "(I)V", (-129,)).await?;
    assert_eq!(jvm.invoke_virtual::<_, i8>(&negative, "byteValue", "()B", ()).await?, 127);

    for class_name in [
        "java/lang/Byte",
        "java/lang/Short",
        "java/lang/Integer",
        "java/lang/Long",
        "java/lang/Float",
        "java/lang/Double",
    ] {
        let wrapper_class = jvm.resolve_class(class_name).await?.java_class();
        let is_number: bool = jvm
            .invoke_virtual(&number_class, "isAssignableFrom", "(Ljava/lang/Class;)Z", (wrapper_class.clone(),))
            .await?;
        assert!(is_number);
        let is_serializable: bool = jvm
            .invoke_virtual(&serializable_class, "isAssignableFrom", "(Ljava/lang/Class;)Z", (wrapper_class.clone(),))
            .await?;
        assert!(is_serializable);
        let is_comparable: bool = jvm
            .invoke_virtual(&comparable_class, "isAssignableFrom", "(Ljava/lang/Class;)Z", (wrapper_class,))
            .await?;
        assert!(is_comparable);
    }

    let result = jvm.new_class("java/lang/Number", "()V", ()).await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("Number must not be instantiable");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/InstantiationError"));

    Ok(())
}
