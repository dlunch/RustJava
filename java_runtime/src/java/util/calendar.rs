use alloc::vec;

use java_runtime_base::{JavaMethodFlag, JavaMethodProto, JavaResult, JvmClassInstanceHandle};
use jvm::Jvm;

use crate::{RuntimeClassProto, RuntimeContext};

// class java.util.Calendar
pub struct Calendar {}

impl Calendar {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![JavaMethodProto::new(
                "getInstance",
                "()Ljava/util/Calendar;",
                Self::get_instance,
                JavaMethodFlag::STATIC,
            )],
            fields: vec![],
        }
    }

    async fn get_instance(jvm: &mut Jvm, _: &mut RuntimeContext) -> JavaResult<JvmClassInstanceHandle<Calendar>> {
        tracing::warn!("stub java.util.Calendar::getInstance()");

        let instance = jvm.new_class("java/util/GregorianCalendar", "()V", []).await?;

        Ok(instance.into())
    }
}
