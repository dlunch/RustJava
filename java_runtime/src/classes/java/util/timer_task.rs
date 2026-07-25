use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::Object};

// abstract class java.util.TimerTask
pub struct TimerTask;

impl TimerTask {
    pub(crate) const VIRGIN: i32 = 0;
    pub(crate) const SCHEDULED: i32 = 1;
    pub(crate) const EXECUTED: i32 = 2;
    pub(crate) const CANCELLED: i32 = 3;

    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/TimerTask",
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["java/lang/Runnable"],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, MethodAccessFlags::PROTECTED),
                JavaMethodProto::new_abstract("run", "()V", MethodAccessFlags::PUBLIC | MethodAccessFlags::ABSTRACT),
                JavaMethodProto::new("cancel", "()Z", Self::cancel, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("scheduledExecutionTime", "()J", Self::scheduled_execution_time, MethodAccessFlags::PUBLIC),
            ],
            fields: vec![
                JavaFieldProto::new("lock", "Ljava/lang/Object;", FieldAccessFlags::FINAL),
                JavaFieldProto::new("state", "I", Default::default()),
                JavaFieldProto::new("nextExecutionTime", "J", Default::default()),
                JavaFieldProto::new("period", "J", Default::default()),
                JavaFieldProto::new("lastScheduledExecutionTime", "J", Default::default()),
            ],
            access_flags: ClassAccessFlags::PUBLIC | ClassAccessFlags::ABSTRACT,
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.TimerTask::<init>({this:?})");

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        let lock = jvm.new_class("java/lang/Object", "()V", ()).await?;
        jvm.put_field(&mut this, "lock", "Ljava/lang/Object;", lock).await?;
        jvm.put_field(&mut this, "state", "I", Self::VIRGIN).await?;

        Ok(())
    }

    async fn cancel(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::debug!("java.util.TimerTask::cancel({this:?})");

        let lock: ClassInstanceRef<Object> = jvm.get_field(&this, "lock", "Ljava/lang/Object;").await?;
        jvm.monitor_enter(&lock).await?;
        let result = async {
            let state: i32 = jvm.get_field(&this, "state", "I").await?;
            jvm.put_field(&mut this, "state", "I", Self::CANCELLED).await?;
            Ok(state == Self::SCHEDULED)
        }
        .await;
        let exit_result = jvm.monitor_exit(&lock).await;
        match result {
            Ok(value) => {
                exit_result?;
                Ok(value)
            }
            Err(error) => {
                exit_result?;
                Err(error)
            }
        }
    }

    async fn scheduled_execution_time(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i64> {
        tracing::debug!("java.util.TimerTask::scheduledExecutionTime({this:?})");

        let lock: ClassInstanceRef<Object> = jvm.get_field(&this, "lock", "Ljava/lang/Object;").await?;
        jvm.monitor_enter(&lock).await?;
        let result = jvm.get_field(&this, "lastScheduledExecutionTime", "J").await;
        let exit_result = jvm.monitor_exit(&lock).await;
        match result {
            Ok(value) => {
                exit_result?;
                Ok(value)
            }
            Err(error) => {
                exit_result?;
                Err(error)
            }
        }
    }
}
