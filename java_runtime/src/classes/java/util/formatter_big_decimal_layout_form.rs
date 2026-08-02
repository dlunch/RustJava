use alloc::{vec, vec::Vec};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use jvm::{Array, ClassInstanceRef, Jvm, Result, runtime::JavaLangString};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::java::lang::{Enum, String},
};

// public enum java.util.Formatter.BigDecimalLayoutForm
pub struct FormatterBigDecimalLayoutForm;

impl FormatterBigDecimalLayoutForm {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/Formatter$BigDecimalLayoutForm",
            parent_class: Some("java/lang/Enum"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<clinit>", "()V", Self::clinit, MethodAccessFlags::STATIC),
                JavaMethodProto::new("<init>", "(Ljava/lang/String;I)V", Self::init, MethodAccessFlags::PRIVATE),
                JavaMethodProto::new(
                    "values",
                    "()[Ljava/util/Formatter$BigDecimalLayoutForm;",
                    Self::values,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "valueOf",
                    "(Ljava/lang/String;)Ljava/util/Formatter$BigDecimalLayoutForm;",
                    Self::value_of,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
            ],
            fields: vec![
                JavaFieldProto::new(
                    "SCIENTIFIC",
                    "Ljava/util/Formatter$BigDecimalLayoutForm;",
                    FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL | FieldAccessFlags::ENUM,
                ),
                JavaFieldProto::new(
                    "DECIMAL_FLOAT",
                    "Ljava/util/Formatter$BigDecimalLayoutForm;",
                    FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL | FieldAccessFlags::ENUM,
                ),
                JavaFieldProto::new(
                    "$VALUES",
                    "[Ljava/util/Formatter$BigDecimalLayoutForm;",
                    FieldAccessFlags::PRIVATE | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL | FieldAccessFlags::SYNTHETIC,
                ),
            ],
            access_flags: ClassAccessFlags::PUBLIC | ClassAccessFlags::FINAL | ClassAccessFlags::ENUM,
        }
    }

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

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, name: ClassInstanceRef<String>, ordinal: i32) -> Result<()> {
        jvm.invoke_special(&this, "java/lang/Enum", "<init>", "(Ljava/lang/String;I)V", (name, ordinal))
            .await
    }

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
