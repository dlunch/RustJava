use alloc::format;

use jvm::{ClassInstanceRef, Jvm, Result, runtime::JavaLangClass};
use jvm_class_proto::{java_class, java_methods};
use jvm_types::FieldAccessFlags;

use crate::{
    RuntimeContext,
    classes::java::lang::{Class, Object, String},
};

// public abstract class java.lang.Enum
#[java_class(
    name = "java/lang/Enum",
    extends = "java/lang/Object",
    implements("java/lang/Comparable", "java/io/Serializable"),
    public,
    abstract
)]
pub struct Enum {
    #[field(private, final)]
    name: ClassInstanceRef<String>,
    #[field(private, final)]
    ordinal: i32,
}

#[java_methods]
impl Enum {
    #[method(constructor, protected)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, name: ClassInstanceRef<String>, ordinal: i32) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "java/lang/Enum", "name", "Ljava/lang/String;", name).await?;
        jvm.put_field(&mut this, "java/lang/Enum", "ordinal", "I", ordinal).await
    }

    #[method(public, final)]
    async fn name(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        jvm.get_field(&this, "java/lang/Enum", "name", "Ljava/lang/String;").await
    }

    #[method(public, final)]
    async fn ordinal(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "java/lang/Enum", "ordinal", "I").await
    }

    #[method(public)]
    async fn to_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        jvm.get_field(&this, "java/lang/Enum", "name", "Ljava/lang/String;").await
    }

    #[method(public, final)]
    async fn equals(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Object>) -> Result<bool> {
        jvm.invoke_special(&this, "java/lang/Object", "equals", "(Ljava/lang/Object;)Z", (other,))
            .await
    }

    #[method(public, final)]
    async fn hash_code(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.invoke_special(&this, "java/lang/Object", "hashCode", "()I", ()).await
    }

    #[method(protected, final)]
    async fn clone(jvm: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        Err(jvm
            .exception("java/lang/CloneNotSupportedException", "enum types may not be cloned")
            .await)
    }

    #[method(public, final)]
    async fn compare_to(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Self>) -> Result<i32> {
        if other.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "other").await);
        }
        let this_class = this.class_definition();
        let this_declaring_class = match this_class.super_class_name() {
            Some(parent) if parent != "java/lang/Enum" => parent.into(),
            _ => this_class.name(),
        };
        let other_class = other.class_definition();
        let other_declaring_class = match other_class.super_class_name() {
            Some(parent) if parent != "java/lang/Enum" => parent.into(),
            _ => other_class.name(),
        };
        if this_declaring_class != other_declaring_class {
            return Err(jvm.exception("java/lang/ClassCastException", "enum types differ").await);
        }
        let this_ordinal: i32 = jvm.get_field(&this, "java/lang/Enum", "ordinal", "I").await?;
        let other_ordinal: i32 = jvm.get_field(&other, "java/lang/Enum", "ordinal", "I").await?;
        Ok(this_ordinal - other_ordinal)
    }

    #[method(name = "compareTo", public, bridge, synthetic)]
    async fn compare_to_object(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Object>) -> Result<i32> {
        jvm.invoke_virtual(&this, "java/lang/Enum", "compareTo", "(Ljava/lang/Enum;)I", (other,))
            .await
    }

    #[method(public, final)]
    async fn get_declaring_class(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Class>> {
        let class = this.class_definition();
        let declaring_class = match class.super_class_name() {
            Some(parent) if parent != "java/lang/Enum" => parent.into(),
            _ => class.name(),
        };
        Ok(jvm.resolve_class(&declaring_class).await?.java_class().into())
    }

    #[method(public, static)]
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

    #[method(protected, final)]
    async fn finalize(_: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>) -> Result<()> {
        Ok(())
    }
}
