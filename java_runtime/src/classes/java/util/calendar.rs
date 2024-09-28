use alloc::vec;

use java_class_proto::JavaMethodProto;
use java_constants::MethodAccessFlags;
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{classes::java::util::Date, RuntimeClassProto, RuntimeContext};

// class java.util.Calendar
pub struct Calendar {}

impl Calendar {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/Calendar",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("getInstance", "()Ljava/util/Calendar;", Self::get_instance, MethodAccessFlags::STATIC),
                JavaMethodProto::new("setTime", "(Ljava/util/Date;)V", Self::set_time, Default::default()),
                JavaMethodProto::new("getTime", "()Ljava/util/Date;", Self::get_time, Default::default()),
                JavaMethodProto::new("set", "(II)V", Self::set, Default::default()),
            ],
            fields: vec![],
        }
    }

    async fn get_instance(jvm: &Jvm, _: &mut RuntimeContext) -> Result<ClassInstanceRef<Calendar>> {
        tracing::warn!("stub java.util.Calendar::getInstance()");

        let instance = jvm.new_class("java/util/GregorianCalendar", "()V", []).await?;

        Ok(instance.into())
    }

    async fn set_time(_: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, date: ClassInstanceRef<Date>) -> Result<()> {
        tracing::warn!("stub java.util.Calendar::setTime({:?}, {:?})", &this, &date);

        Ok(())
    }

    async fn get_time(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Date>> {
        tracing::warn!("stub java.util.Calendar::getTime({:?})", &this);

        let date = jvm.new_class("java/util/Date", "()V", ()).await?;

        Ok(date.into())
    }

    async fn set(_: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, field: i32, value: i32) -> Result<()> {
        tracing::warn!("stub java.util.Calendar::set({:?}, {:?}, {:?})", &this, field, value);

        Ok(())
    }
}
