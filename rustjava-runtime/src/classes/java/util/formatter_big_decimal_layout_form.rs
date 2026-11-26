use alloc::vec::Vec;

use jvm::{Array, ClassInstanceRef, Jvm, Result, runtime::JavaLangString};
use jvm_class_proto::{java_class, java_methods};

use crate::{
    RuntimeContext,
    classes::java::lang::{Enum, String},
};

// public enum java.util.Formatter.BigDecimalLayoutForm
#[java_class(name = "java/util/Formatter$BigDecimalLayoutForm", extends = "java/lang/Enum", public, final, enum)]
pub struct FormatterBigDecimalLayoutForm {
    #[field(public, static, final, enum)]
    SCIENTIFIC: ClassInstanceRef<Self>,
    #[field(public, static, final, enum)]
    DECIMAL_FLOAT: ClassInstanceRef<Self>,
    #[field(name = "$VALUES", private, static, final, synthetic)]
    VALUES: ClassInstanceRef<Array<Self>>,
}

#[java_methods]
impl FormatterBigDecimalLayoutForm {
    #[method(static_initializer)]
    async fn clinit(jvm: &Jvm, _: &mut RuntimeContext) -> Result<()> {
        let scientific_name = JavaLangString::from_rust_string(jvm, "SCIENTIFIC").await?;
        let scientific = jvm
            .new_class("java/util/Formatter$BigDecimalLayoutForm", "(Ljava/lang/String;I)V", (scientific_name, 0))
            .await?;
        let decimal_float_name = JavaLangString::from_rust_string(jvm, "DECIMAL_FLOAT").await?;
        let decimal_float = jvm
            .new_class(
                "java/util/Formatter$BigDecimalLayoutForm",
                "(Ljava/lang/String;I)V",
                (decimal_float_name, 1),
            )
            .await?;
        jvm.put_static_field(
            "java/util/Formatter$BigDecimalLayoutForm",
            "SCIENTIFIC",
            "Ljava/util/Formatter$BigDecimalLayoutForm;",
            scientific.clone(),
        )
        .await?;
        jvm.put_static_field(
            "java/util/Formatter$BigDecimalLayoutForm",
            "DECIMAL_FLOAT",
            "Ljava/util/Formatter$BigDecimalLayoutForm;",
            decimal_float.clone(),
        )
        .await?;
        let mut values = jvm.instantiate_array("Ljava/util/Formatter$BigDecimalLayoutForm;", 2).await?;
        jvm.store_array(&mut values, 0, [scientific, decimal_float]).await?;
        jvm.put_static_field(
            "java/util/Formatter$BigDecimalLayoutForm",
            "$VALUES",
            "[Ljava/util/Formatter$BigDecimalLayoutForm;",
            values,
        )
        .await
    }

    #[method(constructor, private)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, name: ClassInstanceRef<String>, ordinal: i32) -> Result<()> {
        jvm.invoke_special(&this, "java/lang/Enum", "<init>", "(Ljava/lang/String;I)V", (name, ordinal))
            .await
    }

    #[method(public, static)]
    async fn values(jvm: &Jvm, _: &mut RuntimeContext) -> Result<ClassInstanceRef<Array<Self>>> {
        let values: ClassInstanceRef<Array<Self>> = jvm
            .get_static_field(
                "java/util/Formatter$BigDecimalLayoutForm",
                "$VALUES",
                "[Ljava/util/Formatter$BigDecimalLayoutForm;",
            )
            .await?;
        let length = jvm.array_length(&values).await?;
        let contents: Vec<ClassInstanceRef<Self>> = jvm.load_array(&values, 0, length).await?;
        let mut copy = jvm.instantiate_array("Ljava/util/Formatter$BigDecimalLayoutForm;", length).await?;
        jvm.store_array(&mut copy, 0, contents).await?;
        Ok(copy.into())
    }

    #[method(public, static)]
    async fn value_of(jvm: &Jvm, _: &mut RuntimeContext, name: ClassInstanceRef<String>) -> Result<ClassInstanceRef<Self>> {
        let class = jvm.resolve_class("java/util/Formatter$BigDecimalLayoutForm").await?.java_class();
        let value: ClassInstanceRef<Enum> = jvm
            .invoke_static(
                "java/lang/Enum",
                "valueOf",
                "(Ljava/lang/Class;Ljava/lang/String;)Ljava/lang/Enum;",
                (class, name),
            )
            .await?;
        Ok(value.instance.into())
    }
}
