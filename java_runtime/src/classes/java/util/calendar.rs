use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, MethodAccessFlags};
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::java::{
        lang::{Object, String},
        util::{Date, TimeZone},
    },
};

// abstract class java.util.Calendar
pub struct Calendar;

impl Calendar {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/Calendar",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("getInstance", "()Ljava/util/Calendar;", Self::get_instance, MethodAccessFlags::STATIC),
                JavaMethodProto::new(
                    "getInstance",
                    "(Ljava/util/TimeZone;)Ljava/util/Calendar;",
                    Self::get_instance_with_time_zone,
                    MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new("setTime", "(Ljava/util/Date;)V", Self::set_time, Default::default()),
                JavaMethodProto::new("getTime", "()Ljava/util/Date;", Self::get_time, Default::default()),
                JavaMethodProto::new("setTimeInMillis", "(J)V", Self::set_time_in_millis, Default::default()),
                JavaMethodProto::new("getTimeInMillis", "()J", Self::get_time_in_millis, Default::default()),
                JavaMethodProto::new("getTimeZone", "()Ljava/util/TimeZone;", Self::get_time_zone, Default::default()),
                JavaMethodProto::new("setTimeZone", "(Ljava/util/TimeZone;)V", Self::set_time_zone, Default::default()),
                JavaMethodProto::new("isLenient", "()Z", Self::is_lenient, Default::default()),
                JavaMethodProto::new("setLenient", "(Z)V", Self::set_lenient, Default::default()),
                JavaMethodProto::new("equals", "(Ljava/lang/Object;)Z", Self::equals, Default::default()),
                JavaMethodProto::new("hashCode", "()I", Self::hash_code, Default::default()),
                JavaMethodProto::new("before", "(Ljava/lang/Object;)Z", Self::before, Default::default()),
                JavaMethodProto::new("after", "(Ljava/lang/Object;)Z", Self::after, Default::default()),
                JavaMethodProto::new("set", "(II)V", Self::set, Default::default()),
                JavaMethodProto::new("get", "(I)I", Self::get, Default::default()),
                JavaMethodProto::new_abstract("computeTime", "()V", Default::default()),
                JavaMethodProto::new_abstract("computeFields", "()V", Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("time", "J", Default::default()),
                JavaFieldProto::new("fields", "[I", Default::default()),
                JavaFieldProto::new("timeZone", "Ljava/util/TimeZone;", Default::default()),
                JavaFieldProto::new("lenient", "Z", Default::default()),
            ],
            access_flags: ClassAccessFlags::ABSTRACT,
        }
    }

    async fn get_instance(jvm: &Jvm, _: &mut RuntimeContext) -> Result<ClassInstanceRef<Calendar>> {
        tracing::debug!("java.util.Calendar::getInstance()");

        let instance = jvm.new_class("java/util/GregorianCalendar", "()V", []).await?;

        Ok(instance.into())
    }

    async fn get_instance_with_time_zone(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        time_zone: ClassInstanceRef<TimeZone>,
    ) -> Result<ClassInstanceRef<Calendar>> {
        tracing::debug!("java.util.Calendar::getInstance({time_zone:?})");

        if time_zone.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "timeZone").await);
        }

        let instance = jvm
            .new_class("java/util/GregorianCalendar", "(Ljava/util/TimeZone;)V", (time_zone,))
            .await?;

        Ok(instance.into())
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.Calendar::<init>({this:?})");

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        // TODO constant
        let fields = jvm.instantiate_array("I", 17).await?;
        jvm.put_field(&mut this, "fields", "[I", fields).await?;
        let time_zone: ClassInstanceRef<TimeZone> = jvm
            .invoke_static("java/util/TimeZone", "getDefault", "()Ljava/util/TimeZone;", ())
            .await?;
        jvm.put_field(&mut this, "timeZone", "Ljava/util/TimeZone;", time_zone).await?;
        jvm.put_field(&mut this, "lenient", "Z", true).await?;

        Ok(())
    }

    async fn set_time(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, date: ClassInstanceRef<Date>) -> Result<()> {
        tracing::debug!("java.util.Calendar::setTime({this:?}, {date:?})");

        if date.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "date").await);
        }

        let time: i64 = jvm.invoke_virtual(&date, "getTime", "()J", ()).await?;
        jvm.put_field(&mut this, "time", "J", time).await?;

        let _: () = jvm.invoke_virtual(&this, "computeFields", "()V", ()).await?;

        Ok(())
    }

    async fn get_time(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Date>> {
        tracing::debug!("java.util.Calendar::getTime({this:?})");

        let time: i64 = jvm.get_field(&this, "time", "J").await?;
        let date = jvm.new_class("java/util/Date", "(J)V", (time,)).await?;

        Ok(date.into())
    }

    async fn set_time_in_millis(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, time: i64) -> Result<()> {
        tracing::debug!("java.util.Calendar::setTimeInMillis({this:?}, {time:?})");

        jvm.put_field(&mut this, "time", "J", time).await?;
        jvm.invoke_virtual(&this, "computeFields", "()V", ()).await
    }

    async fn get_time_in_millis(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i64> {
        tracing::debug!("java.util.Calendar::getTimeInMillis({this:?})");
        jvm.get_field(&this, "time", "J").await
    }

    async fn get_time_zone(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<TimeZone>> {
        tracing::debug!("java.util.Calendar::getTimeZone({this:?})");
        jvm.get_field(&this, "timeZone", "Ljava/util/TimeZone;").await
    }

    async fn set_time_zone(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, time_zone: ClassInstanceRef<TimeZone>) -> Result<()> {
        tracing::debug!("java.util.Calendar::setTimeZone({this:?}, {time_zone:?})");
        if time_zone.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "timeZone").await);
        }
        jvm.put_field(&mut this, "timeZone", "Ljava/util/TimeZone;", time_zone).await?;
        jvm.invoke_virtual(&this, "computeFields", "()V", ()).await
    }

    async fn is_lenient(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::debug!("java.util.Calendar::isLenient({this:?})");
        jvm.get_field(&this, "lenient", "Z").await
    }

    async fn set_lenient(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, lenient: bool) -> Result<()> {
        tracing::debug!("java.util.Calendar::setLenient({this:?}, {lenient:?})");
        jvm.put_field(&mut this, "lenient", "Z", lenient).await
    }

    async fn equals(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Object>) -> Result<bool> {
        tracing::debug!("java.util.Calendar::equals({this:?}, {other:?})");

        if other.is_null() || !jvm.is_instance(&**other, "java/util/Calendar") {
            return Ok(false);
        }

        let other: ClassInstanceRef<Calendar> = ClassInstanceRef::new(other.instance);
        let time: i64 = jvm.get_field(&this, "time", "J").await?;
        let other_time: i64 = jvm.get_field(&other, "time", "J").await?;
        if time != other_time {
            return Ok(false);
        }

        let lenient: bool = jvm.get_field(&this, "lenient", "Z").await?;
        let other_lenient: bool = jvm.get_field(&other, "lenient", "Z").await?;
        if lenient != other_lenient {
            return Ok(false);
        }

        let time_zone: ClassInstanceRef<TimeZone> = jvm.get_field(&this, "timeZone", "Ljava/util/TimeZone;").await?;
        let other_time_zone: ClassInstanceRef<TimeZone> = jvm.get_field(&other, "timeZone", "Ljava/util/TimeZone;").await?;
        let raw_offset: i32 = jvm.invoke_virtual(&time_zone, "getRawOffset", "()I", ()).await?;
        let other_raw_offset: i32 = jvm.invoke_virtual(&other_time_zone, "getRawOffset", "()I", ()).await?;
        if raw_offset != other_raw_offset {
            return Ok(false);
        }

        let id: ClassInstanceRef<String> = jvm.invoke_virtual(&time_zone, "getID", "()Ljava/lang/String;", ()).await?;
        let other_id: ClassInstanceRef<String> = jvm.invoke_virtual(&other_time_zone, "getID", "()Ljava/lang/String;", ()).await?;
        jvm.invoke_virtual(&id, "equals", "(Ljava/lang/Object;)Z", (other_id,)).await
    }

    async fn hash_code(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.util.Calendar::hashCode({this:?})");

        let time: i64 = jvm.get_field(&this, "time", "J").await?;
        let time_zone: ClassInstanceRef<TimeZone> = jvm.get_field(&this, "timeZone", "Ljava/util/TimeZone;").await?;
        let raw_offset: i32 = jvm.invoke_virtual(&time_zone, "getRawOffset", "()I", ()).await?;
        let id: ClassInstanceRef<String> = jvm.invoke_virtual(&time_zone, "getID", "()Ljava/lang/String;", ()).await?;
        let id_hash: i32 = jvm.invoke_virtual(&id, "hashCode", "()I", ()).await?;
        let lenient: bool = jvm.get_field(&this, "lenient", "Z").await?;
        Ok((time ^ ((time as u64 >> 32) as i64)) as i32 ^ raw_offset ^ id_hash ^ if lenient { 1 } else { 0 })
    }

    async fn before(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Object>) -> Result<bool> {
        tracing::debug!("java.util.Calendar::before({this:?}, {other:?})");

        if other.is_null() || !jvm.is_instance(&**other, "java/util/Calendar") {
            return Ok(false);
        }

        let other: ClassInstanceRef<Calendar> = ClassInstanceRef::new(other.instance);
        let time: i64 = jvm.get_field(&this, "time", "J").await?;
        let other_time: i64 = jvm.get_field(&other, "time", "J").await?;
        Ok(time < other_time)
    }

    async fn after(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Object>) -> Result<bool> {
        tracing::debug!("java.util.Calendar::after({this:?}, {other:?})");

        if other.is_null() || !jvm.is_instance(&**other, "java/util/Calendar") {
            return Ok(false);
        }

        let other: ClassInstanceRef<Calendar> = ClassInstanceRef::new(other.instance);
        let time: i64 = jvm.get_field(&this, "time", "J").await?;
        let other_time: i64 = jvm.get_field(&other, "time", "J").await?;
        Ok(time > other_time)
    }

    async fn set(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, field: i32, value: i32) -> Result<()> {
        tracing::debug!("java.util.Calendar::set({this:?}, {field:?}, {value:?})");

        if !(0..17).contains(&field) {
            return Err(jvm.exception("java/lang/ArrayIndexOutOfBoundsException", "calendar field").await);
        }

        let mut fields = jvm.get_field(&this, "fields", "[I").await?;
        jvm.store_array(&mut fields, field as usize, vec![value]).await?;

        let _: () = jvm.invoke_virtual(&this, "computeTime", "()V", ()).await?;
        let _: () = jvm.invoke_virtual(&this, "computeFields", "()V", ()).await?;

        Ok(())
    }

    async fn get(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, field: i32) -> Result<i32> {
        tracing::debug!("java.util.Calendar::get({this:?}, {field:?})");

        if !(0..17).contains(&field) {
            return Err(jvm.exception("java/lang/ArrayIndexOutOfBoundsException", "calendar field").await);
        }

        let fields = jvm.get_field(&this, "fields", "[I").await?;
        let value = jvm.load_array(&fields, field as usize, 1).await?[0];

        Ok(value)
    }
}
