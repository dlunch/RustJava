use jvm::Result;
use jvm_rust::ClassDefinitionImpl;

use test_utils::test_jvm;

#[tokio::test]
async fn test_putstatic_narrows_to_field_type() -> Result<()> {
    let jvm = test_jvm().await?;

    let class = ClassDefinitionImpl::from_classfile(include_bytes!("../test_data/unit/StaticFlag.class"))?;
    jvm.register_class(Box::new(class), None).await?;

    assert!(jvm.get_static_field::<bool>("StaticFlag", "FLAG", "Z").await?);
    assert_eq!(jvm.get_static_field::<i8>("StaticFlag", "SMALL", "B").await?, 3);
    assert_eq!(jvm.get_static_field::<u16>("StaticFlag", "LETTER", "C").await?, 'a' as u16);
    assert_eq!(jvm.get_static_field::<i16>("StaticFlag", "COUNT", "S").await?, 7);

    Ok(())
}
