use jvm::{ClassInstance, JavaChar, Result, runtime::JavaLangString};
use jvm_rust::ClassDefinitionImpl;

use test_utils::test_jvm;

#[tokio::test]
async fn test_constant_value_fields() -> Result<()> {
    let jvm = test_jvm().await?;

    let class = ClassDefinitionImpl::from_classfile(include_bytes!("../test_data/unit/Constants.class"))?;
    jvm.register_class(Box::new(class), None).await?;

    assert!(jvm.get_static_field::<bool>("Constants", "FLAG", "Z").await?);
    assert_eq!(jvm.get_static_field::<i8>("Constants", "B", "B").await?, 3);
    assert_eq!(jvm.get_static_field::<JavaChar>("Constants", "C", "C").await?, 'a' as JavaChar);
    assert_eq!(jvm.get_static_field::<i16>("Constants", "S", "S").await?, 7);
    assert_eq!(jvm.get_static_field::<i32>("Constants", "I", "I").await?, 42);
    assert_eq!(jvm.get_static_field::<i64>("Constants", "L", "J").await?, 1234567890123);
    assert_eq!(jvm.get_static_field::<f32>("Constants", "F", "F").await?, 1.5);
    assert_eq!(jvm.get_static_field::<f64>("Constants", "D", "D").await?, 2.5);

    let string: Box<dyn ClassInstance> = jvm.get_static_field("Constants", "STR", "Ljava/lang/String;").await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &string).await?, "hello");

    Ok(())
}
