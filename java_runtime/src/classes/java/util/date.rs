use alloc::{format, vec};

use chrono::{DateTime, Datelike, Timelike, Utc};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
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
                JavaMethodProto::new("<init>", "()V", Self::init, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("<init>", "(J)V", Self::init_with_time, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("getTime", "()J", Self::get_time, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("setTime", "(J)V", Self::set_time, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("before", "(Ljava/util/Date;)Z", Self::before, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("after", "(Ljava/util/Date;)Z", Self::after, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("compareTo", "(Ljava/util/Date;)I", Self::compare_to, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("compareTo", "(Ljava/lang/Object;)I", Self::compare_to, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("equals", "(Ljava/lang/Object;)Z", Self::equals, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("hashCode", "()I", Self::hash_code, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("toString", "()Ljava/lang/String;", Self::to_string, MethodAccessFlags::PUBLIC),
            ],
            fields: vec![JavaFieldProto::new("value", "J", FieldAccessFlags::PRIVATE)],
            access_flags: ClassAccessFlags::PUBLIC,
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

    async fn before(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, when: ClassInstanceRef<Object>) -> Result<bool> {
        tracing::debug!("java.util.Date::before({this:?}, {when:?})");

        if when.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "when is null").await);
        }

        let when: ClassInstanceRef<Self> = ClassInstanceRef::new(when.instance);
        let time: i64 = jvm.get_field(&this, "value", "J").await?;
        let when_time: i64 = jvm.get_field(&when, "value", "J").await?;
        Ok(time < when_time)
    }

    async fn after(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, when: ClassInstanceRef<Object>) -> Result<bool> {
        tracing::debug!("java.util.Date::after({this:?}, {when:?})");

        if when.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "when is null").await);
        }

        let when: ClassInstanceRef<Self> = ClassInstanceRef::new(when.instance);
        let time: i64 = jvm.get_field(&this, "value", "J").await?;
        let when_time: i64 = jvm.get_field(&when, "value", "J").await?;
        Ok(time > when_time)
    }

    async fn compare_to(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Object>) -> Result<i32> {
        tracing::debug!("java.util.Date::compareTo({this:?}, {other:?})");

        if other.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "anotherDate is null").await);
        }
        if !jvm.is_instance(&**other, "java/util/Date") {
            return Err(jvm.exception("java/lang/ClassCastException", &other.class_definition().name()).await);
        }

        let other: ClassInstanceRef<Self> = ClassInstanceRef::new(other.instance);
        let time: i64 = jvm.get_field(&this, "value", "J").await?;
        let other_time: i64 = jvm.get_field(&other, "value", "J").await?;
        Ok(if time < other_time {
            -1
        } else if time == other_time {
            0
        } else {
            1
        })
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
