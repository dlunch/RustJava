use jvm::{
    ClassInstanceRef, Jvm, Result,
    runtime::{JavaLangClass, JavaLangString},
};
use jvm_class_proto::{java_class, java_methods};

use crate::{
    RuntimeContext,
    classes::java::lang::{Class, Object, String},
};

// public final class java.lang.Boolean
#[java_class(
    name = "java/lang/Boolean",
    extends = "java/lang/Object",
    implements("java/io/Serializable"),
    public,
    final
)]
pub struct Boolean {
    #[field(public, static, final)]
    TRUE: ClassInstanceRef<Self>,
    #[field(public, static, final)]
    FALSE: ClassInstanceRef<Self>,
    #[field(public, static, final)]
    TYPE: ClassInstanceRef<Class>,
    #[field(private, final)]
    value: bool,
}

#[java_methods]
impl Boolean {
    #[method(static_initializer)]
    async fn clinit(jvm: &Jvm, _: &mut RuntimeContext) -> Result<()> {
        let value = jvm.new_class("java/lang/Boolean", "(Z)V", (true,)).await?;
        jvm.put_static_field("java/lang/Boolean", "TRUE", "Ljava/lang/Boolean;", value).await?;

        let value = jvm.new_class("java/lang/Boolean", "(Z)V", (false,)).await?;
        jvm.put_static_field("java/lang/Boolean", "FALSE", "Ljava/lang/Boolean;", value).await?;

        jvm.put_static_field(
            "java/lang/Boolean",
            "TYPE",
            "Ljava/lang/Class;",
            JavaLangClass::from_rust_primitive(jvm, "boolean").await?,
        )
        .await
    }

    #[method(constructor, public)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, value: bool) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "java/lang/Boolean", "value", "Z", value).await
    }

    #[method(constructor, public)]
    async fn init_string(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, value: ClassInstanceRef<String>) -> Result<()> {
        let parsed = if value.is_null() {
            false
        } else {
            JavaLangString::to_rust_string(jvm, &value).await?.eq_ignore_ascii_case("true")
        };

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "java/lang/Boolean", "value", "Z", parsed).await
    }

    #[method(public)]
    async fn boolean_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        jvm.get_field(&this, "java/lang/Boolean", "value", "Z").await
    }

    #[method(name = "valueOf", public, static)]
    async fn value_of_string(jvm: &Jvm, _: &mut RuntimeContext, value: ClassInstanceRef<String>) -> Result<ClassInstanceRef<Self>> {
        let parsed = if value.is_null() {
            false
        } else {
            JavaLangString::to_rust_string(jvm, &value).await?.eq_ignore_ascii_case("true")
        };

        if parsed {
            jvm.get_static_field("java/lang/Boolean", "TRUE", "Ljava/lang/Boolean;").await
        } else {
            jvm.get_static_field("java/lang/Boolean", "FALSE", "Ljava/lang/Boolean;").await
        }
    }

    #[method(public, static)]
    async fn value_of(jvm: &Jvm, _: &mut RuntimeContext, value: bool) -> Result<ClassInstanceRef<Self>> {
        jvm.get_static_field("java/lang/Boolean", if value { "TRUE" } else { "FALSE" }, "Ljava/lang/Boolean;")
            .await
    }

    #[method(public)]
    async fn to_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        let value: bool = jvm.invoke_virtual(&this, "java/lang/Boolean", "booleanValue", "()Z", ()).await?;
        Ok(JavaLangString::from_rust_string(jvm, if value { "true" } else { "false" }).await?.into())
    }

    #[method(public)]
    async fn hash_code(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        let value: bool = jvm.invoke_virtual(&this, "java/lang/Boolean", "booleanValue", "()Z", ()).await?;
        Ok(if value { 1231 } else { 1237 })
    }

    #[method(public)]
    async fn equals(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Object>) -> Result<bool> {
        if other.is_null() || !jvm.is_instance(&**other, "java/lang/Boolean") {
            return Ok(false);
        }

        let this_value: bool = jvm.invoke_virtual(&this, "java/lang/Boolean", "booleanValue", "()Z", ()).await?;
        let other_value: bool = jvm.invoke_virtual(&other, "java/lang/Boolean", "booleanValue", "()Z", ()).await?;
        Ok(this_value == other_value)
    }

    #[method(public, static)]
    async fn get_boolean(jvm: &Jvm, _: &mut RuntimeContext, name: ClassInstanceRef<String>) -> Result<bool> {
        if name.is_null() {
            return Ok(false);
        }

        let value: ClassInstanceRef<String> = jvm
            .invoke_static("java/lang/System", "getProperty", "(Ljava/lang/String;)Ljava/lang/String;", (name,))
            .await?;
        if value.is_null() {
            return Ok(false);
        }

        Ok(JavaLangString::to_rust_string(jvm, &value).await?.eq_ignore_ascii_case("true"))
    }
}
