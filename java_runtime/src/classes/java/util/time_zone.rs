use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use jvm::{Array, ClassInstanceRef, Jvm, Result, runtime::JavaLangString};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::java::lang::{Object, String},
};

// abstract class java.util.TimeZone
pub struct TimeZone;

impl TimeZone {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/TimeZone",
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["java/io/Serializable", "java/lang/Cloneable"],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, MethodAccessFlags::PROTECTED),
                JavaMethodProto::new(
                    "getTimeZone",
                    "(Ljava/lang/String;)Ljava/util/TimeZone;",
                    Self::get_time_zone,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "getDefault",
                    "()Ljava/util/TimeZone;",
                    Self::get_default,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC | MethodAccessFlags::SYNCHRONIZED,
                ),
                JavaMethodProto::new(
                    "setDefault",
                    "(Ljava/util/TimeZone;)V",
                    Self::set_default,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC | MethodAccessFlags::SYNCHRONIZED,
                ),
                JavaMethodProto::new(
                    "getAvailableIDs",
                    "()[Ljava/lang/String;",
                    Self::get_available_ids,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new("getID", "()Ljava/lang/String;", Self::get_id, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("setID", "(Ljava/lang/String;)V", Self::set_id, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("clone", "()Ljava/lang/Object;", Self::clone, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new_abstract("getOffset", "(IIIIII)I", MethodAccessFlags::PUBLIC | MethodAccessFlags::ABSTRACT),
                JavaMethodProto::new_abstract("getRawOffset", "()I", MethodAccessFlags::PUBLIC | MethodAccessFlags::ABSTRACT),
                JavaMethodProto::new_abstract("setRawOffset", "(I)V", MethodAccessFlags::PUBLIC | MethodAccessFlags::ABSTRACT),
                JavaMethodProto::new_abstract("useDaylightTime", "()Z", MethodAccessFlags::PUBLIC | MethodAccessFlags::ABSTRACT),
                JavaMethodProto::new_abstract(
                    "inDaylightTime",
                    "(Ljava/util/Date;)Z",
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::ABSTRACT,
                ),
            ],
            fields: vec![
                JavaFieldProto::new("ID", "Ljava/lang/String;", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new(
                    "defaultTimeZone",
                    "Ljava/util/TimeZone;",
                    FieldAccessFlags::PRIVATE | FieldAccessFlags::STATIC,
                ),
            ],
            access_flags: ClassAccessFlags::PUBLIC | ClassAccessFlags::ABSTRACT,
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.TimeZone::<init>({this:?})");

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        let id = JavaLangString::from_rust_string(jvm, "GMT").await?;
        jvm.put_field(&mut this, "ID", "Ljava/lang/String;", id).await
    }

    async fn get_time_zone(jvm: &Jvm, _: &mut RuntimeContext, id: ClassInstanceRef<String>) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.util.TimeZone::getTimeZone({id:?})");

        if id.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "ID").await);
        }

        let requested_id = JavaLangString::to_rust_string(jvm, &id).await?;
        let id = if requested_id == "GMT" || requested_id == "UTC" {
            id
        } else {
            JavaLangString::from_rust_string(jvm, "GMT").await?.into()
        };
        let result = jvm.new_class("java/util/SimpleTimeZone", "(ILjava/lang/String;)V", (0i32, id)).await?;
        Ok(result.into())
    }

    async fn get_default(jvm: &Jvm, _: &mut RuntimeContext) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.util.TimeZone::getDefault()");

        let mut default: ClassInstanceRef<Self> = jvm
            .get_static_field("java/util/TimeZone", "defaultTimeZone", "Ljava/util/TimeZone;")
            .await?;
        if default.is_null() {
            let id = JavaLangString::from_rust_string(jvm, "GMT").await?;
            default = jvm
                .new_class("java/util/SimpleTimeZone", "(ILjava/lang/String;)V", (0i32, id))
                .await?
                .into();
            jvm.put_static_field("java/util/TimeZone", "defaultTimeZone", "Ljava/util/TimeZone;", default.clone())
                .await?;
        }

        let cloned: ClassInstanceRef<Object> = jvm
            .invoke_virtual(&default, "java/lang/Object", "clone", "()Ljava/lang/Object;", ())
            .await?;
        Ok(ClassInstanceRef::new(cloned.instance))
    }

    async fn set_default(jvm: &Jvm, _: &mut RuntimeContext, timezone: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.TimeZone::setDefault({timezone:?})");

        let stored: ClassInstanceRef<Self> = if timezone.is_null() {
            let id = JavaLangString::from_rust_string(jvm, "GMT").await?;
            jvm.new_class("java/util/SimpleTimeZone", "(ILjava/lang/String;)V", (0i32, id))
                .await?
                .into()
        } else {
            let cloned: ClassInstanceRef<Object> = jvm
                .invoke_virtual(&timezone, "java/lang/Object", "clone", "()Ljava/lang/Object;", ())
                .await?;
            ClassInstanceRef::new(cloned.instance)
        };
        jvm.put_static_field("java/util/TimeZone", "defaultTimeZone", "Ljava/util/TimeZone;", stored)
            .await
    }

    async fn get_available_ids(jvm: &Jvm, _: &mut RuntimeContext) -> Result<ClassInstanceRef<Array<String>>> {
        let mut ids: ClassInstanceRef<Array<String>> = jvm.instantiate_array("Ljava/lang/String;", 2).await?.into();
        let gmt = JavaLangString::from_rust_string(jvm, "GMT").await?;
        let utc = JavaLangString::from_rust_string(jvm, "UTC").await?;
        jvm.store_array(&mut ids, 0, [gmt, utc]).await?;
        Ok(ids)
    }

    async fn get_id(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        jvm.get_field(&this, "ID", "Ljava/lang/String;").await
    }

    async fn set_id(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, id: ClassInstanceRef<String>) -> Result<()> {
        tracing::debug!("java.util.TimeZone::setID({this:?}, {id:?})");

        if id.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "ID").await);
        }
        jvm.put_field(&mut this, "ID", "Ljava/lang/String;", id).await
    }

    async fn clone(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        jvm.invoke_special(&this, "java/lang/Object", "clone", "()Ljava/lang/Object;", ()).await
    }
}
