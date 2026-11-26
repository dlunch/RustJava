use alloc::format;

use jiff::{Timestamp, tz::Offset};

use jvm::{ClassInstanceRef, Jvm, Result, runtime::JavaLangString};
use jvm_class_proto::{java_class, java_methods};

use crate::{
    RuntimeContext,
    classes::java::lang::{Object, String},
};

// class java.util.Date
#[java_class(
    name = "java/util/Date",
    extends = "java/lang/Object",
    implements("java/io/Serializable", "java/lang/Cloneable", "java/lang/Comparable"),
    public
)]
pub struct Date {
    #[field(private)]
    value: i64,
}

#[java_methods]
impl Date {
    #[method(constructor, public)]
    async fn init(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.Date::<init>({this:?})");

        let now: u64 = context.now();

        let _: () = jvm.invoke_special(&this, "java/util/Date", "<init>", "(J)V", (now as i64,)).await?;

        Ok(())
    }

    #[method(constructor, public)]
    async fn init_with_time(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, time: i64) -> Result<()> {
        tracing::debug!("java.util.Date::<init>({this:?}, {time:?})");

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        jvm.put_field(&mut this, "java/util/Date", "value", "J", time).await?;

        Ok(())
    }

    #[method(public)]
    async fn get_time(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i64> {
        tracing::debug!("java.util.Date::getTime({this:?})");

        let time = jvm.get_field(&this, "java/util/Date", "value", "J").await?;

        Ok(time)
    }

    #[method(public)]
    async fn set_time(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, time: i64) -> Result<()> {
        tracing::debug!("java.util.Date::setTime({this:?}, {time:?})");

        jvm.put_field(&mut this, "java/util/Date", "value", "J", time).await?;

        Ok(())
    }

    #[method(descriptor = "(Ljava/util/Date;)Z", public)]
    async fn before(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, when: ClassInstanceRef<Object>) -> Result<bool> {
        tracing::debug!("java.util.Date::before({this:?}, {when:?})");

        if when.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "when is null").await);
        }

        let when: ClassInstanceRef<Self> = ClassInstanceRef::new(when.instance);
        let time: i64 = jvm.get_field(&this, "java/util/Date", "value", "J").await?;
        let when_time: i64 = jvm.get_field(&when, "java/util/Date", "value", "J").await?;
        Ok(time < when_time)
    }

    #[method(descriptor = "(Ljava/util/Date;)Z", public)]
    async fn after(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, when: ClassInstanceRef<Object>) -> Result<bool> {
        tracing::debug!("java.util.Date::after({this:?}, {when:?})");

        if when.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "when is null").await);
        }

        let when: ClassInstanceRef<Self> = ClassInstanceRef::new(when.instance);
        let time: i64 = jvm.get_field(&this, "java/util/Date", "value", "J").await?;
        let when_time: i64 = jvm.get_field(&when, "java/util/Date", "value", "J").await?;
        Ok(time > when_time)
    }

    #[method(descriptor = "(Ljava/util/Date;)I", public)]
    #[method(public)]
    async fn compare_to(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Object>) -> Result<i32> {
        tracing::debug!("java.util.Date::compareTo({this:?}, {other:?})");

        if other.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "anotherDate is null").await);
        }
        if !jvm.is_instance(&**other, "java/util/Date") {
            return Err(jvm.exception("java/lang/ClassCastException", &other.class_definition().name()).await);
        }

        let other: ClassInstanceRef<Self> = ClassInstanceRef::new(other.instance);
        let time: i64 = jvm.get_field(&this, "java/util/Date", "value", "J").await?;
        let other_time: i64 = jvm.get_field(&other, "java/util/Date", "value", "J").await?;
        Ok(if time < other_time {
            -1
        } else if time == other_time {
            0
        } else {
            1
        })
    }

    #[method(public)]
    async fn equals(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Object>) -> Result<bool> {
        tracing::debug!("java.util.Date::equals({this:?}, {other:?})");

        if other.is_null() || !jvm.is_instance(&**other, "java/util/Date") {
            return Ok(false);
        }

        let other: ClassInstanceRef<Date> = ClassInstanceRef::new(other.instance);
        let time: i64 = jvm.get_field(&this, "java/util/Date", "value", "J").await?;
        let other_time: i64 = jvm.get_field(&other, "java/util/Date", "value", "J").await?;
        Ok(time == other_time)
    }

    #[method(public)]
    async fn hash_code(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.util.Date::hashCode({this:?})");

        let time: i64 = jvm.get_field(&this, "java/util/Date", "value", "J").await?;
        Ok((time ^ ((time as u64 >> 32) as i64)) as i32)
    }

    #[method(public)]
    async fn to_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.util.Date::toString({this:?})");

        let time: i64 = jvm.get_field(&this, "java/util/Date", "value", "J").await?;
        let text = if let Ok(timestamp) = Timestamp::from_millisecond(time) {
            let date_time = Offset::UTC.to_datetime(timestamp);
            let weekdays = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
            let months = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
            format!(
                "{} {} {:02} {:02}:{:02}:{:02} GMT {:04}",
                weekdays[date_time.weekday().to_sunday_zero_offset() as usize],
                months[(date_time.month() - 1) as usize],
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
