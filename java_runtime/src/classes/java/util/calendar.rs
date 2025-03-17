use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::MethodAccessFlags;
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::util::Date};

// class java.util.Calendar
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
                JavaMethodProto::new("setTime", "(Ljava/util/Date;)V", Self::set_time, Default::default()),
                JavaMethodProto::new("getTime", "()Ljava/util/Date;", Self::get_time, Default::default()),
                JavaMethodProto::new("set", "(II)V", Self::set, Default::default()),
                JavaMethodProto::new("get", "(I)I", Self::get, Default::default()),
                JavaMethodProto::new_abstract("computeTime", "()V", Default::default()),
                JavaMethodProto::new_abstract("computeFields", "()V", Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("time", "J", Default::default()),
                JavaFieldProto::new("fields", "[I", Default::default()),
            ],
        }
    }

    async fn get_instance(jvm: &Jvm, _: &mut RuntimeContext) -> Result<ClassInstanceRef<Calendar>> {
        tracing::debug!("java.util.Calendar::getInstance()");

        let instance = jvm.new_class("java/util/GregorianCalendar", "()V", []).await?;

        Ok(instance.into())
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.Calendar::<init>({:?})", &this);

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        // TODO constant
        let fields = jvm.instantiate_array("I", 17).await?;
        jvm.put_field(&mut this, "fields", "[I", fields).await?;

        Ok(())
    }

    async fn set_time(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, date: ClassInstanceRef<Date>) -> Result<()> {
        tracing::debug!("java.util.Calendar::setTime({:?}, {:?})", &this, &date);

        let time: i64 = jvm.invoke_virtual(&date, "getTime", "()J", ()).await?;
        jvm.put_field(&mut this, "time", "J", time).await?;

        let _: () = jvm.invoke_virtual(&this, "computeFields", "()V", ()).await?;

        Ok(())
    }

    async fn get_time(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Date>> {
        tracing::debug!("java.util.Calendar::getTime({:?})", &this);

        let time: i64 = jvm.get_field(&this, "time", "J").await?;
        let date = jvm.new_class("java/util/Date", "(J)V", (time,)).await?;

        Ok(date.into())
    }

    async fn set(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, field: i32, value: i32) -> Result<()> {
        tracing::debug!("java.util.Calendar::set({:?}, {:?}, {:?})", &this, field, value);

        let mut fields = jvm.get_field(&this, "fields", "[I").await?;
        jvm.store_array(&mut fields, field as usize, vec![value]).await?;

        let _: () = jvm.invoke_virtual(&this, "computeTime", "()V", ()).await?;
        let _: () = jvm.invoke_virtual(&this, "computeFields", "()V", ()).await?;

        Ok(())
    }

    async fn get(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, field: i32) -> Result<i32> {
        tracing::debug!("java.util.Calendar::get({:?}, {:?})", &this, field);

        let fields = jvm.get_field(&this, "fields", "[I").await?;
        let value = jvm.load_array(&fields, field as usize, 1).await?[0];

        Ok(value)
    }
}
