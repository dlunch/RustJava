use alloc::{format, vec};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use jvm::{ClassInstanceRef, Jvm, Result, runtime::JavaLangClass};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::java::lang::{Class, Object, String},
};

// public abstract class java.lang.Enum
pub struct Enum;

impl Enum {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/lang/Enum",
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["java/lang/Comparable", "java/io/Serializable"],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/lang/String;I)V", Self::init, MethodAccessFlags::PROTECTED),
                JavaMethodProto::new(
                    "name",
                    "()Ljava/lang/String;",
                    Self::name,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::FINAL,
                ),
                JavaMethodProto::new("ordinal", "()I", Self::ordinal, MethodAccessFlags::PUBLIC | MethodAccessFlags::FINAL),
                JavaMethodProto::new("toString", "()Ljava/lang/String;", Self::to_string, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "equals",
                    "(Ljava/lang/Object;)Z",
                    Self::equals,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::FINAL,
                ),
                JavaMethodProto::new("hashCode", "()I", Self::hash_code, MethodAccessFlags::PUBLIC | MethodAccessFlags::FINAL),
                JavaMethodProto::new(
                    "clone",
                    "()Ljava/lang/Object;",
                    Self::clone,
                    MethodAccessFlags::PROTECTED | MethodAccessFlags::FINAL,
                ),
                JavaMethodProto::new(
                    "compareTo",
                    "(Ljava/lang/Enum;)I",
                    Self::compare_to,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::FINAL,
                ),
                JavaMethodProto::new(
                    "compareTo",
                    "(Ljava/lang/Object;)I",
                    Self::compare_to_object,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::BRIDGE | MethodAccessFlags::SYNTHETIC,
                ),
                JavaMethodProto::new(
                    "getDeclaringClass",
                    "()Ljava/lang/Class;",
                    Self::get_declaring_class,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::FINAL,
                ),
                JavaMethodProto::new(
                    "valueOf",
                    "(Ljava/lang/Class;Ljava/lang/String;)Ljava/lang/Enum;",
                    Self::value_of,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new("finalize", "()V", Self::finalize, MethodAccessFlags::PROTECTED | MethodAccessFlags::FINAL),
            ],
            fields: vec![
                JavaFieldProto::new("name", "Ljava/lang/String;", FieldAccessFlags::PRIVATE | FieldAccessFlags::FINAL),
                JavaFieldProto::new("ordinal", "I", FieldAccessFlags::PRIVATE | FieldAccessFlags::FINAL),
            ],
            access_flags: ClassAccessFlags::PUBLIC | ClassAccessFlags::ABSTRACT,
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, name: ClassInstanceRef<String>, ordinal: i32) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "name", "Ljava/lang/String;", name).await?;
        jvm.put_field(&mut this, "ordinal", "I", ordinal).await
    }

    async fn name(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        jvm.get_field(&this, "name", "Ljava/lang/String;").await
    }

    async fn ordinal(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "ordinal", "I").await
    }

    async fn to_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        jvm.get_field(&this, "name", "Ljava/lang/String;").await
    }

    async fn equals(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Object>) -> Result<bool> {
        jvm.invoke_special(&this, "java/lang/Object", "equals", "(Ljava/lang/Object;)Z", (other,))
            .await
    }

    async fn hash_code(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.invoke_special(&this, "java/lang/Object", "hashCode", "()I", ()).await
    }

    async fn clone(jvm: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        Err(jvm
            .exception("java/lang/CloneNotSupportedException", "enum types may not be cloned")
            .await)
    }

    async fn compare_to(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Self>) -> Result<i32> {
        if other.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "other").await);
        }
        let this_class = this.class_definition();
        let this_declaring_class = match this_class.super_class_name() {
            Some(parent) if parent != "java/lang/Enum" => parent,
            _ => this_class.name(),
        };
        let other_class = other.class_definition();
        let other_declaring_class = match other_class.super_class_name() {
            Some(parent) if parent != "java/lang/Enum" => parent,
            _ => other_class.name(),
        };
        if this_declaring_class != other_declaring_class {
            return Err(jvm.exception("java/lang/ClassCastException", "enum types differ").await);
        }
        let this_ordinal: i32 = jvm.get_field(&this, "ordinal", "I").await?;
        let other_ordinal: i32 = jvm.get_field(&other, "ordinal", "I").await?;
        Ok(this_ordinal - other_ordinal)
    }

    async fn compare_to_object(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Object>) -> Result<i32> {
        jvm.invoke_virtual(&this, "java/lang/Enum", "compareTo", "(Ljava/lang/Enum;)I", (other,))
            .await
    }

    async fn get_declaring_class(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Class>> {
        let class = this.class_definition();
        let declaring_class = match class.super_class_name() {
            Some(parent) if parent != "java/lang/Enum" => parent,
            _ => class.name(),
        };
        Ok(jvm.resolve_class(&declaring_class).await?.java_class().into())
    }

    async fn value_of(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        enum_type: ClassInstanceRef<Class>,
        name: ClassInstanceRef<String>,
    ) -> Result<ClassInstanceRef<Self>> {
        if enum_type.is_null() || name.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "enumType or name is null").await);
        }

        let class_name = JavaLangClass::name(jvm, &enum_type).await?;
        let name = jvm::runtime::JavaLangString::to_rust_string(jvm, &name).await?;
        let class = JavaLangClass::to_rust_class(jvm, &enum_type).await?;
        for field in class.fields() {
            if field.name() == name
                && field
                    .access_flags()
                    .contains(FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::ENUM)
            {
                return jvm.get_static_field(&class_name, &field.name(), &field.descriptor()).await;
            }
        }

        Err(jvm
            .exception(
                "java/lang/IllegalArgumentException",
                &format!("No enum constant {}.{name}", class_name.replace('/', ".")),
            )
            .await)
    }

    async fn finalize(_: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>) -> Result<()> {
        Ok(())
    }
}
