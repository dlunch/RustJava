use alloc::{format, vec};

use chrono::{DateTime, Datelike, Timelike, Utc};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm::{ClassInstanceRef, Jvm, Result, runtime::JavaLangString};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::java::lang::{Object, String},
};

// class java.util.Date
pub struct Date;

impl Date {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/Date",
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["java/io/Serializable", "java/lang/Cloneable", "java/lang/Comparable"],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("<init>", "(J)V", Self::init_with_time, Default::default()),
                JavaMethodProto::new("getTime", "()J", Self::get_time, Default::default()),
                JavaMethodProto::new("setTime", "(J)V", Self::set_time, Default::default()),
                JavaMethodProto::new("equals", "(Ljava/lang/Object;)Z", Self::equals, Default::default()),
                JavaMethodProto::new("hashCode", "()I", Self::hash_code, Default::default()),
                JavaMethodProto::new("toString", "()Ljava/lang/String;", Self::to_string, Default::default()),
            ],
            fields: vec![JavaFieldProto::new("value", "J", Default::default())],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.Date::<init>({this:?})");

        let now: u64 = context.now();

        let _: () = jvm.invoke_special(&this, "java/util/Date", "<init>", "(J)V", (now as i64,)).await?;

        Ok(())
    }

    async fn init_with_time(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, time: i64) -> Result<()> {
        tracing::debug!("java.util.Date::<init>({this:?}, {time:?})");

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        jvm.put_field(&mut this, "value", "J", time).await?;

        Ok(())
    }

    async fn get_time(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i64> {
        tracing::debug!("java.util.Date::getTime({this:?})");

        let time = jvm.get_field(&this, "value", "J").await?;

        Ok(time)
    }

    async fn set_time(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, time: i64) -> Result<()> {
        tracing::debug!("java.util.Date::setTime({this:?}, {time:?})");

        jvm.put_field(&mut this, "value", "J", time).await?;

        Ok(())
    }

    async fn equals(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Object>) -> Result<bool> {
        tracing::debug!("java.util.Date::equals({this:?}, {other:?})");

        if other.is_null() || !jvm.is_instance(&**other, "java/util/Date") {
            return Ok(false);
        }

        let other: ClassInstanceRef<Date> = ClassInstanceRef::new(other.instance);
        let time: i64 = jvm.get_field(&this, "value", "J").await?;
        let other_time: i64 = jvm.get_field(&other, "value", "J").await?;
        Ok(time == other_time)
    }

    async fn hash_code(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.util.Date::hashCode({this:?})");

        let time: i64 = jvm.get_field(&this, "value", "J").await?;
        Ok((time ^ ((time as u64 >> 32) as i64)) as i32)
    }

    async fn to_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.util.Date::toString({this:?})");

        let time: i64 = jvm.get_field(&this, "value", "J").await?;
        let text = if let Some(date_time) = DateTime::<Utc>::from_timestamp_millis(time) {
            let weekdays = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
            let months = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
            format!(
                "{} {} {:02} {:02}:{:02}:{:02} GMT {:04}",
                weekdays[date_time.weekday().num_days_from_sunday() as usize],
                months[date_time.month0() as usize],
                date_time.day(),
                date_time.hour(),
                date_time.minute(),
                date_time.second(),
                date_time.year()
            )
        } else {
            format!("Date({time})")
        };

        Ok(JavaLangString::from_rust_string(jvm, &text).await?.into())
    }
}
