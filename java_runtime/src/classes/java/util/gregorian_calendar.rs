use alloc::{vec, vec::Vec};

use chrono::{DateTime, Datelike, FixedOffset, TimeZone, Timelike};

use java_class_proto::JavaMethodProto;
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext};

// class java.util.GregorianCalendar
pub struct GregorianCalendar;

impl GregorianCalendar {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/GregorianCalendar",
            parent_class: Some("java/util/Calendar"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("computeTime", "()V", Self::compute_time, Default::default()),
                JavaMethodProto::new("computeFields", "()V", Self::compute_fields, Default::default()),
            ],
            fields: vec![],
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.GregorianCalendar::<init>({:?})", &this);

        let _: () = jvm.invoke_special(&this, "java/util/Calendar", "<init>", "()V", ()).await?;

        Ok(())
    }

    async fn compute_time(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.GregorianCalendar::computeTime({:?})", &this);

        // fields -> time

        let fields = jvm.get_field(&this, "fields", "[I").await?;

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
        let tz = FixedOffset::east_opt(zone_offset * 1000).unwrap();
        let timestamp = tz
            .with_ymd_and_hms(year, (month + 1) as _, date as _, hour_of_day as _, minute as _, second as _)
            .unwrap()
            .timestamp_millis();

        let calculated_time = timestamp + millisecond as i64;

        jvm.put_field(&mut this, "time", "J", calculated_time).await?;

        Ok(())
    }

    async fn compute_fields(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.GregorianCalendar::computeFields({:?})", &this);

        // time -> fields

        let time: i64 = jvm.get_field(&this, "time", "J").await?;
        let date_time = DateTime::from_timestamp_millis(time as _).unwrap();

        let calculated_fields = vec![
            1, // CE
            date_time.year(),
            date_time.month() as i32 - 1,
            date_time.iso_week().week() as _,
            (date_time.day() / 7) as _, // TODO correctly get
            date_time.day() as _,
            date_time.ordinal() as _,
            date_time.weekday().number_from_monday() as _,
            (date_time.day() % 7) as _, // TODO correctly get
            (date_time.hour() / 12) as _,
            (date_time.hour() % 12) as _,
            date_time.hour() as _,
            date_time.minute() as _,
            date_time.second() as _,
            (date_time.nanosecond() / 1_000_000) as _,
            0,
            0,
        ];

        let mut fields = jvm.get_field(&this, "fields", "[I").await?;
        jvm.store_array(&mut fields, 0, calculated_fields).await?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use jvm::Result;

    use test_utils::test_jvm;

    #[tokio::test]
    async fn test_gregorian_calendar() -> Result<()> {
        let jvm = test_jvm().await?;

        let timestamp = 0i64;
        let calendar = jvm
            .invoke_static("java/util/Calendar", "getInstance", "()Ljava/util/Calendar;", ())
            .await?;
        let date = jvm.new_class("java/util/Date", "(J)V", (timestamp,)).await?;

        let _: () = jvm.invoke_virtual(&calendar, "setTime", "(Ljava/util/Date;)V", (date,)).await?;
        let year: i32 = jvm.invoke_virtual(&calendar, "get", "(I)I", (1,)).await?;
        assert_eq!(1970, year);

        let month: i32 = jvm.invoke_virtual(&calendar, "get", "(I)I", (2,)).await?;
        assert_eq!(0, month);

        let day: i32 = jvm.invoke_virtual(&calendar, "get", "(I)I", (5,)).await?;
        assert_eq!(1, day);

        let timestamp = 737521516000i64;
        let date = jvm.new_class("java/util/Date", "(J)V", (timestamp,)).await?;

        let _: () = jvm.invoke_virtual(&calendar, "setTime", "(Ljava/util/Date;)V", (date,)).await?;

        let year: i32 = jvm.invoke_virtual(&calendar, "get", "(I)I", (1,)).await?;
        assert_eq!(1993, year);

        let month: i32 = jvm.invoke_virtual(&calendar, "get", "(I)I", (2,)).await?;
        assert_eq!(4, month);

        let day: i32 = jvm.invoke_virtual(&calendar, "get", "(I)I", (5,)).await?;
        assert_eq!(16, day);

        let hour: i32 = jvm.invoke_virtual(&calendar, "get", "(I)I", (11,)).await?;
        assert_eq!(3, hour);

        let minute: i32 = jvm.invoke_virtual(&calendar, "get", "(I)I", (12,)).await?;
        assert_eq!(5, minute);

        let second: i32 = jvm.invoke_virtual(&calendar, "get", "(I)I", (13,)).await?;
        assert_eq!(16, second);

        let _: () = jvm.invoke_virtual(&calendar, "set", "(II)V", (1, 1999)).await?;
        let date = jvm.invoke_virtual(&calendar, "getTime", "()Ljava/util/Date;", ()).await?;
        let timestamp: i64 = jvm.invoke_virtual(&date, "getTime", "()J", ()).await?;
        assert_eq!(926823916000, timestamp);

        Ok(())
    }
}
