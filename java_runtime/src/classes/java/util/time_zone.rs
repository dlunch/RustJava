use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, MethodAccessFlags};
use jvm::{Array, ClassInstanceRef, Jvm, Result, runtime::JavaLangString};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::String};

// abstract class java.util.TimeZone
pub struct TimeZone;

impl TimeZone {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/TimeZone",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new(
                    "getTimeZone",
                    "(Ljava/lang/String;)Ljava/util/TimeZone;",
                    Self::get_time_zone,
                    MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new("getDefault", "()Ljava/util/TimeZone;", Self::get_default, MethodAccessFlags::STATIC),
                JavaMethodProto::new(
                    "getAvailableIDs",
                    "()[Ljava/lang/String;",
                    Self::get_available_ids,
                    MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new("getID", "()Ljava/lang/String;", Self::get_id, Default::default()),
                JavaMethodProto::new_abstract("getOffset", "(IIIIII)I", Default::default()),
                JavaMethodProto::new_abstract("getRawOffset", "()I", Default::default()),
                JavaMethodProto::new_abstract("useDaylightTime", "()Z", Default::default()),
            ],
            fields: vec![JavaFieldProto::new("ID", "Ljava/lang/String;", Default::default())],
            access_flags: ClassAccessFlags::ABSTRACT,
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.TimeZone::<init>({this:?})");

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        let id = JavaLangString::from_rust_string(jvm, "GMT").await?;
        jvm.put_field(&mut this, "ID", "Ljava/lang/String;", id).await?;

        Ok(())
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
        let id = JavaLangString::from_rust_string(jvm, "GMT").await?;
        jvm.invoke_static("java/util/TimeZone", "getTimeZone", "(Ljava/lang/String;)Ljava/util/TimeZone;", (id,))
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
}
