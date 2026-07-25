use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::java::{lang::String, util::Date},
};

// class java.util.SimpleTimeZone
pub struct SimpleTimeZone;

impl SimpleTimeZone {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/SimpleTimeZone",
            parent_class: Some("java/util/TimeZone"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/lang/String;)V", Self::init, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("<init>", "(ILjava/lang/String;)V", Self::init_with_raw_offset, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("getOffset", "(IIIIII)I", Self::get_offset, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("getRawOffset", "()I", Self::get_raw_offset, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("setRawOffset", "(I)V", Self::set_raw_offset, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("useDaylightTime", "()Z", Self::use_daylight_time, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("inDaylightTime", "(Ljava/util/Date;)Z", Self::in_daylight_time, MethodAccessFlags::PUBLIC),
            ],
            fields: vec![JavaFieldProto::new("rawOffset", "I", FieldAccessFlags::PRIVATE)],
            access_flags: ClassAccessFlags::PUBLIC,
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, id: ClassInstanceRef<String>) -> Result<()> {
        tracing::debug!("java.util.SimpleTimeZone::<init>({this:?}, {id:?})");

        jvm.invoke_special(&this, "java/util/SimpleTimeZone", "<init>", "(ILjava/lang/String;)V", (0i32, id))
            .await
    }

    async fn init_with_raw_offset(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        raw_offset: i32,
        id: ClassInstanceRef<String>,
    ) -> Result<()> {
        tracing::debug!("java.util.SimpleTimeZone::<init>({this:?}, {raw_offset:?}, {id:?})");

        if id.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "ID").await);
        }

        let _: () = jvm.invoke_special(&this, "java/util/TimeZone", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "rawOffset", "I", raw_offset).await?;
        jvm.put_field(&mut this, "ID", "Ljava/lang/String;", id).await
    }

    #[allow(clippy::too_many_arguments)]
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
    ) -> Result<i32> {
        let days_in_month = match month {
            1 if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) => 29,
            1 => 28,
            3 | 5 | 8 | 10 => 30,
            _ => 31,
        };
        if !(0..=1).contains(&era)
            || !(0..=11).contains(&month)
            || day < 1
            || day > days_in_month
            || !(1..=7).contains(&day_of_week)
            || !(0..86_400_000).contains(&millis)
        {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "invalid date fields").await);
        }

        jvm.get_field(&this, "rawOffset", "I").await
    }

    async fn get_raw_offset(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "rawOffset", "I").await
    }

    async fn set_raw_offset(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, offset: i32) -> Result<()> {
        tracing::debug!("java.util.SimpleTimeZone::setRawOffset({this:?}, {offset:?})");
        jvm.put_field(&mut this, "rawOffset", "I", offset).await
    }

    async fn use_daylight_time(_: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>) -> Result<bool> {
        Ok(false)
    }

    async fn in_daylight_time(jvm: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>, date: ClassInstanceRef<Date>) -> Result<bool> {
        if date.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "date").await);
        }
        Ok(false)
    }
}
