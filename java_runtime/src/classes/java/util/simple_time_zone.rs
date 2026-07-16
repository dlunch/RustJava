use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::String};

// class java.util.SimpleTimeZone
pub struct SimpleTimeZone;

impl SimpleTimeZone {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/SimpleTimeZone",
            parent_class: Some("java/util/TimeZone"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/lang/String;)V", Self::init, Default::default()),
                JavaMethodProto::new("<init>", "(ILjava/lang/String;)V", Self::init_with_raw_offset, Default::default()),
                JavaMethodProto::new("getOffset", "(IIIIII)I", Self::get_offset, Default::default()),
                JavaMethodProto::new("getRawOffset", "()I", Self::get_raw_offset, Default::default()),
                JavaMethodProto::new("useDaylightTime", "()Z", Self::use_daylight_time, Default::default()),
            ],
            fields: vec![JavaFieldProto::new("rawOffset", "I", Default::default())],
            access_flags: Default::default(),
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
        jvm.put_field(&mut this, "ID", "Ljava/lang/String;", id).await?;

        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    async fn get_offset(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        era: i32,
        _year: i32,
        month: i32,
        day: i32,
        day_of_week: i32,
        millis: i32,
    ) -> Result<i32> {
        if !(0..=1).contains(&era)
            || !(0..=11).contains(&month)
            || !(1..=31).contains(&day)
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

    async fn use_daylight_time(_: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>) -> Result<bool> {
        Ok(false)
    }
}
