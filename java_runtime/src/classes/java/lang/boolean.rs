use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use jvm::{
    ClassInstanceRef, Jvm, Result,
    runtime::{JavaLangClass, JavaLangString},
};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::java::lang::{Object, String},
};

// public final class java.lang.Boolean
pub struct Boolean;

impl Boolean {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/lang/Boolean",
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["java/io/Serializable"],
            methods: vec![
                JavaMethodProto::new("<clinit>", "()V", Self::clinit, MethodAccessFlags::STATIC),
                JavaMethodProto::new("<init>", "(Z)V", Self::init, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("<init>", "(Ljava/lang/String;)V", Self::init_string, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("booleanValue", "()Z", Self::boolean_value, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "valueOf",
                    "(Ljava/lang/String;)Ljava/lang/Boolean;",
                    Self::value_of_string,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new("toString", "()Ljava/lang/String;", Self::to_string, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("hashCode", "()I", Self::hash_code, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("equals", "(Ljava/lang/Object;)Z", Self::equals, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "getBoolean",
                    "(Ljava/lang/String;)Z",
                    Self::get_boolean,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
            ],
            fields: vec![
                JavaFieldProto::new(
                    "TRUE",
                    "Ljava/lang/Boolean;",
                    FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new(
                    "FALSE",
                    "Ljava/lang/Boolean;",
                    FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new(
                    "TYPE",
                    "Ljava/lang/Class;",
                    FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new("value", "Z", FieldAccessFlags::PRIVATE | FieldAccessFlags::FINAL),
            ],
            access_flags: ClassAccessFlags::PUBLIC | ClassAccessFlags::FINAL,
        }
    }

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

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, value: bool) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "value", "Z", value).await
    }

    async fn init_string(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, value: ClassInstanceRef<String>) -> Result<()> {
        let parsed = if value.is_null() {
            false
        } else {
            JavaLangString::to_rust_string(jvm, &value).await?.eq_ignore_ascii_case("true")
        };

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "value", "Z", parsed).await
    }

    async fn boolean_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        jvm.get_field(&this, "value", "Z").await
    }

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

    async fn to_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        let value: bool = jvm.invoke_virtual(&this, "booleanValue", "()Z", ()).await?;
        Ok(JavaLangString::from_rust_string(jvm, if value { "true" } else { "false" }).await?.into())
    }

    async fn hash_code(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        let value: bool = jvm.invoke_virtual(&this, "booleanValue", "()Z", ()).await?;
        Ok(if value { 1231 } else { 1237 })
    }

    async fn equals(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Object>) -> Result<bool> {
        if other.is_null() || !jvm.is_instance(&**other, "java/lang/Boolean") {
            return Ok(false);
        }

        let this_value: bool = jvm.invoke_virtual(&this, "booleanValue", "()Z", ()).await?;
        let other_value: bool = jvm.invoke_virtual(&other, "booleanValue", "()Z", ()).await?;
        Ok(this_value == other_value)
    }

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
