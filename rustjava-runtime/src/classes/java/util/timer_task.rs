use jvm::{ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::{RuntimeContext, classes::java::lang::Object};

// abstract class java.util.TimerTask
#[java_class(
    name = "java/util/TimerTask",
    extends = "java/lang/Object",
    implements("java/lang/Runnable"),
    public,
    abstract
)]
pub struct TimerTask {
    #[field(final)]
    lock: ClassInstanceRef<Object>,
    state: i32,
    nextExecutionTime: i64,
    period: i64,
    lastScheduledExecutionTime: i64,
}

#[java_methods]
impl TimerTask {
    pub(crate) const VIRGIN: i32 = 0;
    pub(crate) const SCHEDULED: i32 = 1;
    pub(crate) const EXECUTED: i32 = 2;
    pub(crate) const CANCELLED: i32 = 3;

    #[method(constructor, protected)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.TimerTask::<init>({this:?})");

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        let lock = jvm.new_class("java/lang/Object", "()V", ()).await?;
        jvm.put_field(&mut this, "java/util/TimerTask", "lock", "Ljava/lang/Object;", lock)
            .await?;
        jvm.put_field(&mut this, "java/util/TimerTask", "state", "I", Self::VIRGIN).await?;

        Ok(())
    }

    #[method(public)]
    async fn run(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()>;

    #[method(public)]
    async fn cancel(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::debug!("java.util.TimerTask::cancel({this:?})");

        let lock: ClassInstanceRef<Object> = jvm.get_field(&this, "java/util/TimerTask", "lock", "Ljava/lang/Object;").await?;
        jvm.monitor_enter(&lock).await?;
        let result = async {
            let state: i32 = jvm.get_field(&this, "java/util/TimerTask", "state", "I").await?;
            jvm.put_field(&mut this, "java/util/TimerTask", "state", "I", Self::CANCELLED).await?;
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

    #[method(public)]
    async fn scheduled_execution_time(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i64> {
        tracing::debug!("java.util.TimerTask::scheduledExecutionTime({this:?})");

        let lock: ClassInstanceRef<Object> = jvm.get_field(&this, "java/util/TimerTask", "lock", "Ljava/lang/Object;").await?;
        jvm.monitor_enter(&lock).await?;
        let result = jvm.get_field(&this, "java/util/TimerTask", "lastScheduledExecutionTime", "J").await;
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
