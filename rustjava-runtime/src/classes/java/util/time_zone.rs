use jvm::{Array, ClassInstanceRef, Jvm, Result, runtime::JavaLangString};
use jvm_class_proto::{java_class, java_methods};

use crate::{
    RuntimeContext,
    classes::java::{
        lang::{Object, String},
        util::Date,
    },
};

// abstract class java.util.TimeZone
#[java_class(name = "java/util/TimeZone", implements("java/io/Serializable", "java/lang/Cloneable"), public, abstract)]
pub struct TimeZone {
    #[field(private)]
    ID: ClassInstanceRef<String>,
    #[field(private, static)]
    defaultTimeZone: ClassInstanceRef<Self>,
}

#[java_methods]
impl TimeZone {
    #[method(constructor, protected)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.TimeZone::<init>({this:?})");

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        let id = JavaLangString::from_rust_string(jvm, "GMT").await?;
        jvm.put_field(&mut this, "java/util/TimeZone", "ID", "Ljava/lang/String;", id).await
    }

    #[method(public, static)]
    async fn get_time_zone(jvm: &Jvm, _: &mut RuntimeContext, id: ClassInstanceRef<String>) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.util.TimeZone::getTimeZone({id:?})");

        if id.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "ID").await);
        }

        let requested_id = JavaLangString::to_rust_string(jvm, &id).await?;
        let id = if requested_id == "GMT" || requested_id == "UTC" {
            id
        } else {
            JavaLangString::from_rust_string(jvm, "GMT").await?.into()
        };
        let result = jvm.new_class("java/util/SimpleTimeZone", "(ILjava/lang/String;)V", (0i32, id)).await?;
        Ok(result.into())
    }

    #[method(public, static, synchronized)]
    async fn get_default(jvm: &Jvm, _: &mut RuntimeContext) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.util.TimeZone::getDefault()");

        let mut default: ClassInstanceRef<Self> = jvm
            .get_static_field("java/util/TimeZone", "defaultTimeZone", "Ljava/util/TimeZone;")
            .await?;
        if default.is_null() {
            let id = JavaLangString::from_rust_string(jvm, "GMT").await?;
            default = jvm
                .new_class("java/util/SimpleTimeZone", "(ILjava/lang/String;)V", (0i32, id))
                .await?
                .into();
            jvm.put_static_field("java/util/TimeZone", "defaultTimeZone", "Ljava/util/TimeZone;", default.clone())
                .await?;
        }

        let cloned: ClassInstanceRef<Object> = jvm
            .invoke_virtual(&default, "java/lang/Object", "clone", "()Ljava/lang/Object;", ())
            .await?;
        Ok(ClassInstanceRef::new(cloned.instance))
    }

    #[method(public, static, synchronized)]
    async fn set_default(jvm: &Jvm, _: &mut RuntimeContext, timezone: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.TimeZone::setDefault({timezone:?})");

        let stored: ClassInstanceRef<Self> = if timezone.is_null() {
            let id = JavaLangString::from_rust_string(jvm, "GMT").await?;
            jvm.new_class("java/util/SimpleTimeZone", "(ILjava/lang/String;)V", (0i32, id))
                .await?
                .into()
        } else {
            let cloned: ClassInstanceRef<Object> = jvm
                .invoke_virtual(&timezone, "java/lang/Object", "clone", "()Ljava/lang/Object;", ())
                .await?;
            ClassInstanceRef::new(cloned.instance)
        };
        jvm.put_static_field("java/util/TimeZone", "defaultTimeZone", "Ljava/util/TimeZone;", stored)
            .await
    }

    #[method(name = "getAvailableIDs", public, static)]
    async fn get_available_ids(jvm: &Jvm, _: &mut RuntimeContext) -> Result<ClassInstanceRef<Array<String>>> {
        let mut ids: ClassInstanceRef<Array<String>> = jvm.instantiate_array("Ljava/lang/String;", 2).await?.into();
        let gmt = JavaLangString::from_rust_string(jvm, "GMT").await?;
        let utc = JavaLangString::from_rust_string(jvm, "UTC").await?;
        jvm.store_array(&mut ids, 0, [gmt, utc]).await?;
        Ok(ids)
    }

    #[method(name = "getID", public)]
    async fn get_id(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        jvm.get_field(&this, "java/util/TimeZone", "ID", "Ljava/lang/String;").await
    }

    #[method(name = "setID", public)]
    async fn set_id(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, id: ClassInstanceRef<String>) -> Result<()> {
        tracing::debug!("java.util.TimeZone::setID({this:?}, {id:?})");

        if id.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "ID").await);
        }
        jvm.put_field(&mut this, "java/util/TimeZone", "ID", "Ljava/lang/String;", id).await
    }

    #[method(public)]
    async fn clone(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        jvm.invoke_special(&this, "java/lang/Object", "clone", "()Ljava/lang/Object;", ()).await
    }

    #[method(public)]
    async fn get_offset(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        era: i32,
        year: i32,
        month: i32,
        day: i32,
        day_of_week: i32,
        millis: i32,
    ) -> Result<i32>;

    #[method(public)]
    async fn get_raw_offset(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32>;

    #[method(public)]
    async fn set_raw_offset(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, offset: i32) -> Result<()>;

    #[method(public)]
    async fn use_daylight_time(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool>;

    #[method(public)]
    async fn in_daylight_time(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, date: ClassInstanceRef<Date>) -> Result<bool>;
}
