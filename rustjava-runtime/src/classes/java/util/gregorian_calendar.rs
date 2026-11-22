use alloc::{vec, vec::Vec};

use jiff::{Timestamp, civil, tz::Offset};

use jvm::{ClassInstanceRef, Jvm, Result};
use jvm_class_proto::JavaMethodProto;
use jvm_types::{ClassAccessFlags, MethodAccessFlags};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::util::TimeZone};

// class java.util.GregorianCalendar
pub struct GregorianCalendar;

impl GregorianCalendar {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/GregorianCalendar",
            parent_class: Some("java/util/Calendar"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("<init>", "(Ljava/util/TimeZone;)V", Self::init_with_time_zone, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("computeTime", "()V", Self::compute_time, MethodAccessFlags::PROTECTED),
                JavaMethodProto::new("computeFields", "()V", Self::compute_fields, MethodAccessFlags::PROTECTED),
            ],
            fields: vec![],
            access_flags: ClassAccessFlags::PUBLIC,
        }
    }

    async fn init(jvm: &Jvm, context: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.GregorianCalendar::<init>({this:?})");

        let _: () = jvm.invoke_special(&this, "java/util/Calendar", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "java/util/GregorianCalendar", "time", "J", context.now() as i64)
            .await?;
        jvm.invoke_virtual(&this, "java/util/GregorianCalendar", "computeFields", "()V", ()).await
    }

    async fn init_with_time_zone(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        time_zone: ClassInstanceRef<TimeZone>,
    ) -> Result<()> {
        tracing::debug!("java.util.GregorianCalendar::<init>({this:?}, {time_zone:?})");

        if time_zone.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "timeZone").await);
        }

        let _: () = jvm.invoke_special(&this, "java/util/Calendar", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "java/util/GregorianCalendar", "timeZone", "Ljava/util/TimeZone;", time_zone)
            .await?;
        jvm.put_field(&mut this, "java/util/GregorianCalendar", "time", "J", context.now() as i64)
            .await?;
        jvm.invoke_virtual(&this, "java/util/GregorianCalendar", "computeFields", "()V", ()).await
    }

    async fn compute_time(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.GregorianCalendar::computeTime({this:?})");

        // fields -> time

        let fields = jvm.get_field(&this, "java/util/GregorianCalendar", "fields", "[I").await?;

        // TODO constant
        let fields: Vec<i32> = jvm.load_array(&fields, 0, 17).await?;
        let _era = fields[0];
        let year = fields[1];
        let month = fields[2];
        let _week_of_year = fields[3];
        let _week_of_month = fields[4];
        let date = fields[5];
        let _day_of_year = fields[6];
        let _day_of_week = fields[7];
        let _day_of_week_in_month = fields[8];
        let _am_pm = fields[9];
        let _hour = fields[10];
        let hour_of_day = fields[11];
        let minute = fields[12];
        let second = fields[13];
        let millisecond = fields[14];
        let zone_offset = fields[15]; // raw offset from GMT in milliseconds
        let _dst_offset = fields[16];

        // TODO handle more complex cases
        let [Ok(month), Ok(date), Ok(hour_of_day), Ok(minute), Ok(second)] = [month + 1, date, hour_of_day, minute, second].map(i8::try_from) else {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "invalid calendar fields").await);
        };
        let Some(timestamp) = i16::try_from(year)
            .ok()
            .and_then(|year| civil::DateTime::new(year, month, date, hour_of_day, minute, second, 0).ok())
            .and_then(|date_time| Offset::UTC.to_timestamp(date_time).ok())
        else {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "invalid calendar fields").await);
        };
        let Some(calculated_time) = timestamp
            .as_millisecond()
            .checked_sub(zone_offset as i64)
            .and_then(|timestamp| timestamp.checked_add(millisecond as i64))
        else {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "calendar time out of range").await);
        };

        jvm.put_field(&mut this, "java/util/GregorianCalendar", "time", "J", calculated_time)
            .await?;

        Ok(())
    }

    async fn compute_fields(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.GregorianCalendar::computeFields({this:?})");

        // time -> fields

        let time: i64 = jvm.get_field(&this, "java/util/GregorianCalendar", "time", "J").await?;
        let time_zone: ClassInstanceRef<TimeZone> = jvm
            .get_field(&this, "java/util/GregorianCalendar", "timeZone", "Ljava/util/TimeZone;")
            .await?;
        if time_zone.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "timeZone").await);
        }
        let zone_offset: i32 = jvm.invoke_virtual(&time_zone, "java/util/TimeZone", "getRawOffset", "()I", ()).await?;
        let Some(adjusted_time) = time.checked_add(zone_offset as i64) else {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "calendar time out of range").await);
        };
        let Ok(timestamp) = Timestamp::from_millisecond(adjusted_time) else {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "calendar time out of range").await);
        };
        let date_time = Offset::UTC.to_datetime(timestamp);

        let calculated_fields = vec![
            1, // CE
            date_time.year() as _,
            date_time.month() as i32 - 1,
            date_time.date().iso_week_date().week() as _,
            (date_time.day() / 7) as _, // TODO correctly get
            date_time.day() as _,
            date_time.day_of_year() as _,
            date_time.weekday().to_sunday_one_offset() as _,
            (date_time.day() % 7) as _, // TODO correctly get
            (date_time.hour() / 12) as _,
            (date_time.hour() % 12) as _,
            date_time.hour() as _,
            date_time.minute() as _,
            date_time.second() as _,
            date_time.millisecond() as _,
            zone_offset,
            0,
        ];

        let mut fields = jvm.get_field(&this, "java/util/GregorianCalendar", "fields", "[I").await?;
        jvm.store_array(&mut fields, 0, calculated_fields).await?;

        Ok(())
    }
}
