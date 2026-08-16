use alloc::{boxed::Box, collections::BTreeMap, vec, vec::Vec};
use core::time::Duration;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use java_runtime::{
    RuntimeClassProto, RuntimeContext,
    classes::java::util::{Date, Timer, TimerTask, TimerTaskQueue, TimerThread},
};
use jvm::{ClassInstanceRef, JavaError, Jvm, MonitorWait, Result};
use jvm_rust::ClassDefinitionImpl;

use test_utils::{TestRuntime, create_test_jvm};

const TEST_BARRIER_TIMEOUT: Duration = Duration::from_secs(2);

struct TestTimerTask;

impl TestTimerTask {
    fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "TestTimerTask",
            parent_class: Some("java/util/TimerTask"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("run", "()V", Self::run, MethodAccessFlags::PUBLIC),
            ],
            fields: vec![
                JavaFieldProto::new("runCount", "I", FieldAccessFlags::PUBLIC),
                JavaFieldProto::new("firstScheduledTime", "J", FieldAccessFlags::PUBLIC),
                JavaFieldProto::new("lastScheduledTime", "J", FieldAccessFlags::PUBLIC),
                JavaFieldProto::new("firstActualTime", "J", FieldAccessFlags::PUBLIC),
                JavaFieldProto::new("lastActualTime", "J", FieldAccessFlags::PUBLIC),
                JavaFieldProto::new("throwOnRun", "Z", FieldAccessFlags::PUBLIC),
            ],
            access_flags: ClassAccessFlags::PUBLIC,
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/util/TimerTask", "<init>", "()V", ()).await?;
        Ok(())
    }

    async fn run(jvm: &Jvm, context: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.monitor_enter(&this).await?;
        let update_result = async {
            let run_count: i32 = jvm.get_field(&this, "runCount", "I").await?;
            let scheduled_time: i64 = jvm.invoke_virtual(&this, "TestTimerTask", "scheduledExecutionTime", "()J", ()).await?;
            let actual_time = context.now() as i64;
            if run_count == 0 {
                jvm.put_field(&mut this, "firstScheduledTime", "J", scheduled_time).await?;
                jvm.put_field(&mut this, "firstActualTime", "J", actual_time).await?;
            }
            jvm.put_field(&mut this, "lastScheduledTime", "J", scheduled_time).await?;
            jvm.put_field(&mut this, "lastActualTime", "J", actual_time).await?;
            jvm.put_field(&mut this, "runCount", "I", run_count + 1).await?;
            jvm.object_notify(&this, usize::MAX).await
        }
        .await;
        let exit_result = jvm.monitor_exit(&this).await;
        update_result.and(exit_result)?;

        if jvm.get_field::<bool>(&this, "throwOnRun", "Z").await? {
            return Err(jvm.exception("java/lang/RuntimeException", "timer task failed").await);
        }
        Ok(())
    }
}

async fn timer_test_jvm(now: u64) -> Result<(TestRuntime, Jvm)> {
    let runtime = TestRuntime::new_with_queued_spawns_and_manual_clock(BTreeMap::new(), now);
    let jvm = create_test_jvm(runtime.clone()).await?;
    jvm.register_class(
        Box::new(ClassDefinitionImpl::from_class_proto(
            TestTimerTask::as_proto(),
            Box::new(runtime.clone()) as Box<_>,
        )),
        None,
    )
    .await?;
    Ok((runtime, jvm))
}

async fn next_spawn(runtime: &TestRuntime) -> tokio::task::JoinHandle<Result<()>> {
    let callback = tokio::time::timeout(TEST_BARRIER_TIMEOUT, runtime.next_spawn_callback())
        .await
        .expect("timed out waiting for queued callback")
        .expect("queued callback");
    tokio::spawn(async move { callback.call().await })
}

async fn next_spawn_while_worker_runs(
    runtime: &TestRuntime,
    worker: &mut tokio::task::JoinHandle<Result<()>>,
) -> tokio::task::JoinHandle<Result<()>> {
    let callback = tokio::time::timeout(TEST_BARRIER_TIMEOUT, async {
        tokio::select! {
            callback = runtime.next_spawn_callback() => callback.expect("queued callback"),
            result = worker => panic!("timer worker exited before queuing its timeout: {result:?}"),
        }
    })
    .await
    .expect("timed out waiting for timer worker timeout callback");
    tokio::spawn(async move { callback.call().await })
}

async fn prepare_task_notification(jvm: &Jvm, task: &ClassInstanceRef<TestTimerTask>) -> Result<MonitorWait> {
    tokio::time::timeout(TEST_BARRIER_TIMEOUT, async {
        jvm.monitor_enter(task).await?;
        let (wait, _) = jvm.object_wait_prepare(task).await?;
        Ok(wait)
    })
    .await
    .expect("timed out preparing task notification")
}

async fn wait_for_task_notification(jvm: &Jvm, task: &ClassInstanceRef<TestTimerTask>, wait: MonitorWait) -> Result<()> {
    tokio::time::timeout(TEST_BARRIER_TIMEOUT, jvm.object_wait(wait))
        .await
        .expect("timed out waiting for timer task notification")?;
    jvm.monitor_exit(task).await
}

async fn await_spawn(callback: tokio::task::JoinHandle<Result<()>>, stage: &str) -> Result<()> {
    let joined = tokio::time::timeout(TEST_BARRIER_TIMEOUT, callback)
        .await
        .unwrap_or_else(|_| panic!("timed out waiting for {stage}"));
    joined.unwrap_or_else(|error| panic!("{stage} panicked: {error}"))
}

async fn assert_exception<T>(result: Result<T>, expected: &str) {
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("expected {expected}");
    };
    assert_eq!(exception.class_definition().name(), expected);
}

#[tokio::test]
async fn timer_01_to_05_registers_jdk_shaped_api_and_state() -> Result<()> {
    let timer = Timer::as_proto();
    assert_eq!(timer.access_flags, ClassAccessFlags::PUBLIC);
    assert_eq!(
        timer
            .fields
            .iter()
            .map(|field| (field.name.as_str(), field.descriptor.as_str(), field.access_flags))
            .collect::<Vec<_>>(),
        vec![(
            "thread",
            "Ljava/util/Timer$TimerThread;",
            FieldAccessFlags::PRIVATE | FieldAccessFlags::FINAL
        )]
    );
    for (name, descriptor) in [
        ("<init>", "()V"),
        ("schedule", "(Ljava/util/TimerTask;J)V"),
        ("schedule", "(Ljava/util/TimerTask;Ljava/util/Date;)V"),
        ("schedule", "(Ljava/util/TimerTask;JJ)V"),
        ("schedule", "(Ljava/util/TimerTask;Ljava/util/Date;J)V"),
        ("scheduleAtFixedRate", "(Ljava/util/TimerTask;JJ)V"),
        ("cancel", "()V"),
    ] {
        let method = timer
            .methods
            .iter()
            .find(|method| method.name == name && method.descriptor == descriptor)
            .unwrap();
        assert_eq!(method.access_flags, MethodAccessFlags::PUBLIC);
    }

    let task = TimerTask::as_proto();
    assert_eq!(task.access_flags, ClassAccessFlags::PUBLIC | ClassAccessFlags::ABSTRACT);
    assert_eq!(
        task.methods
            .iter()
            .map(|method| (method.name.as_str(), method.descriptor.as_str(), method.access_flags))
            .collect::<Vec<_>>(),
        vec![
            ("<init>", "()V", MethodAccessFlags::PROTECTED),
            ("run", "()V", MethodAccessFlags::PUBLIC | MethodAccessFlags::ABSTRACT),
            ("cancel", "()Z", MethodAccessFlags::PUBLIC),
            ("scheduledExecutionTime", "()J", MethodAccessFlags::PUBLIC),
        ]
    );
    assert_eq!(
        task.fields
            .iter()
            .map(|field| (field.name.as_str(), field.descriptor.as_str(), field.access_flags))
            .collect::<Vec<_>>(),
        vec![
            ("lock", "Ljava/lang/Object;", FieldAccessFlags::FINAL),
            ("state", "I", FieldAccessFlags::empty()),
            ("nextExecutionTime", "J", FieldAccessFlags::empty()),
            ("period", "J", FieldAccessFlags::empty()),
            ("lastScheduledExecutionTime", "J", FieldAccessFlags::empty()),
        ]
    );

    let thread = TimerThread::as_proto();
    assert_eq!(
        thread
            .fields
            .iter()
            .map(|field| (field.name.as_str(), field.descriptor.as_str(), field.access_flags))
            .collect::<Vec<_>>(),
        vec![
            ("queue", "Ljava/util/Timer$TaskQueue;", FieldAccessFlags::PRIVATE),
            ("newTasksMayBeScheduled", "Z", FieldAccessFlags::empty()),
        ]
    );
    let queue = TimerTaskQueue::as_proto();
    assert_eq!(
        queue
            .fields
            .iter()
            .map(|field| (field.name.as_str(), field.descriptor.as_str(), field.access_flags))
            .collect::<Vec<_>>(),
        vec![
            ("queue", "[Ljava/util/TimerTask;", FieldAccessFlags::PRIVATE),
            ("size", "I", FieldAccessFlags::PRIVATE),
        ]
    );

    let (runtime, jvm) = timer_test_jvm(1_000).await?;
    let instance: ClassInstanceRef<TestTimerTask> = jvm.new_class("TestTimerTask", "()V", ()).await?.into();
    assert_eq!(jvm.get_field::<i32>(&instance, "state", "I").await?, 0);
    assert!(!jvm.invoke_virtual::<_, bool>(&instance, "TestTimerTask", "cancel", "()Z", ()).await?);
    assert_eq!(jvm.get_field::<i32>(&instance, "state", "I").await?, 3);
    assert_eq!(
        jvm.invoke_virtual::<_, i64>(&instance, "TestTimerTask", "scheduledExecutionTime", "()J", ())
            .await?,
        0
    );
    let timer = jvm.new_class("java/util/Timer", "()V", ()).await?;
    let _: () = jvm.invoke_virtual(&timer, &timer.class_definition().name(), "cancel", "()V", ()).await?;
    await_spawn(next_spawn(&runtime).await, "cancelled timer worker").await?;
    Ok(())
}

#[tokio::test]
async fn timer_validation_happens_before_task_mutation() -> Result<()> {
    let (runtime, jvm) = timer_test_jvm(i64::MAX as u64 - 5).await?;
    let timer = jvm.new_class("java/util/Timer", "()V", ()).await?;
    let task: ClassInstanceRef<TestTimerTask> = jvm.new_class("TestTimerTask", "()V", ()).await?.into();
    let null_task: ClassInstanceRef<TimerTask> = None.into();

    assert_exception(
        jvm.invoke_virtual::<_, ()>(
            &timer,
            &timer.class_definition().name(),
            "schedule",
            "(Ljava/util/TimerTask;J)V",
            (null_task, -1i64),
        )
        .await,
        "java/lang/NullPointerException",
    )
    .await;
    assert_exception(
        jvm.invoke_virtual::<_, ()>(
            &timer,
            &timer.class_definition().name(),
            "schedule",
            "(Ljava/util/TimerTask;J)V",
            (task.clone(), -1i64),
        )
        .await,
        "java/lang/IllegalArgumentException",
    )
    .await;
    assert_exception(
        jvm.invoke_virtual::<_, ()>(
            &timer,
            &timer.class_definition().name(),
            "schedule",
            "(Ljava/util/TimerTask;J)V",
            (task.clone(), 10i64),
        )
        .await,
        "java/lang/IllegalArgumentException",
    )
    .await;
    for method in ["schedule", "scheduleAtFixedRate"] {
        assert_exception(
            jvm.invoke_virtual::<_, ()>(
                &timer,
                &timer.class_definition().name(),
                method,
                "(Ljava/util/TimerTask;JJ)V",
                (task.clone(), 10i64, 1i64),
            )
            .await,
            "java/lang/IllegalArgumentException",
        )
        .await;
    }
    for method in ["schedule", "scheduleAtFixedRate"] {
        assert_exception(
            jvm.invoke_virtual::<_, ()>(
                &timer,
                &timer.class_definition().name(),
                method,
                "(Ljava/util/TimerTask;JJ)V",
                (task.clone(), 0i64, 0i64),
            )
            .await,
            "java/lang/IllegalArgumentException",
        )
        .await;
    }

    let negative_date = jvm.new_class("java/util/Date", "(J)V", (-1i64,)).await?;
    assert_exception(
        jvm.invoke_virtual::<_, ()>(
            &timer,
            &timer.class_definition().name(),
            "schedule",
            "(Ljava/util/TimerTask;Ljava/util/Date;)V",
            (task.clone(), negative_date),
        )
        .await,
        "java/lang/IllegalArgumentException",
    )
    .await;
    let null_date: ClassInstanceRef<Date> = None.into();
    assert_exception(
        jvm.invoke_virtual::<_, ()>(
            &timer,
            &timer.class_definition().name(),
            "schedule",
            "(Ljava/util/TimerTask;Ljava/util/Date;)V",
            (task.clone(), null_date),
        )
        .await,
        "java/lang/NullPointerException",
    )
    .await;
    assert_eq!(jvm.get_field::<i32>(&task, "state", "I").await?, 0);
    assert_eq!(jvm.get_field::<i64>(&task, "nextExecutionTime", "J").await?, 0);
    assert_eq!(jvm.get_field::<i64>(&task, "period", "J").await?, 0);

    let _: () = jvm.invoke_virtual(&timer, &timer.class_definition().name(), "cancel", "()V", ()).await?;
    assert_exception(
        jvm.invoke_virtual::<_, ()>(
            &timer,
            &timer.class_definition().name(),
            "schedule",
            "(Ljava/util/TimerTask;J)V",
            (task.clone(), 0i64),
        )
        .await,
        "java/lang/IllegalStateException",
    )
    .await;
    assert_eq!(jvm.get_field::<i32>(&task, "state", "I").await?, 0);
    await_spawn(next_spawn(&runtime).await, "validation timer worker").await?;
    Ok(())
}

#[tokio::test]
async fn timer_normalizes_long_max_period_and_rejects_first_recurrence_overflow() -> Result<()> {
    let (runtime, jvm) = timer_test_jvm(1_000).await?;
    let timer = jvm.new_class("java/util/Timer", "()V", ()).await?;
    let fixed_delay: ClassInstanceRef<TestTimerTask> = jvm.new_class("TestTimerTask", "()V", ()).await?.into();
    let fixed_rate: ClassInstanceRef<TestTimerTask> = jvm.new_class("TestTimerTask", "()V", ()).await?.into();
    let _: () = jvm
        .invoke_virtual(
            &timer,
            &timer.class_definition().name(),
            "schedule",
            "(Ljava/util/TimerTask;JJ)V",
            (fixed_delay.clone(), 0i64, i64::MAX),
        )
        .await?;
    let _: () = jvm
        .invoke_virtual(
            &timer,
            &timer.class_definition().name(),
            "scheduleAtFixedRate",
            "(Ljava/util/TimerTask;JJ)V",
            (fixed_rate.clone(), 0i64, i64::MAX),
        )
        .await?;
    assert_eq!(jvm.get_field::<i64>(&fixed_delay, "period", "J").await?, (-i64::MAX) >> 1);
    assert_eq!(jvm.get_field::<i64>(&fixed_rate, "period", "J").await?, i64::MAX >> 1);
    let _: () = jvm.invoke_virtual(&timer, &timer.class_definition().name(), "cancel", "()V", ()).await?;
    await_spawn(next_spawn(&runtime).await, "large-period timer worker").await?;

    let (runtime, jvm) = timer_test_jvm(i64::MAX as u64 - 100).await?;
    let timer = jvm.new_class("java/util/Timer", "()V", ()).await?;
    for method in ["schedule", "scheduleAtFixedRate"] {
        let task: ClassInstanceRef<TestTimerTask> = jvm.new_class("TestTimerTask", "()V", ()).await?.into();
        assert_exception(
            jvm.invoke_virtual::<_, ()>(
                &timer,
                &timer.class_definition().name(),
                method,
                "(Ljava/util/TimerTask;JJ)V",
                (task.clone(), 0i64, 101i64),
            )
            .await,
            "java/lang/IllegalArgumentException",
        )
        .await;
        assert_eq!(jvm.get_field::<i32>(&task, "state", "I").await?, 0);
        assert_eq!(jvm.get_field::<i64>(&task, "period", "J").await?, 0);
    }
    let date = jvm.new_class("java/util/Date", "(J)V", (i64::MAX - 50,)).await?;
    let dated: ClassInstanceRef<TestTimerTask> = jvm.new_class("TestTimerTask", "()V", ()).await?.into();
    assert_exception(
        jvm.invoke_virtual::<_, ()>(
            &timer,
            &timer.class_definition().name(),
            "schedule",
            "(Ljava/util/TimerTask;Ljava/util/Date;J)V",
            (dated.clone(), date, 60i64),
        )
        .await,
        "java/lang/IllegalArgumentException",
    )
    .await;
    assert_eq!(jvm.get_field::<i32>(&dated, "state", "I").await?, 0);
    let _: () = jvm.invoke_virtual(&timer, &timer.class_definition().name(), "cancel", "()V", ()).await?;
    await_spawn(next_spawn(&runtime).await, "near-boundary validation timer worker").await?;
    Ok(())
}

#[tokio::test]
async fn timer_worker_stops_periodic_tasks_when_next_deadline_overflows() -> Result<()> {
    let (runtime, jvm) = timer_test_jvm(i64::MAX as u64 - 100).await?;
    let timer = jvm.new_class("java/util/Timer", "()V", ()).await?;
    let fixed_rate: ClassInstanceRef<TestTimerTask> = jvm.new_class("TestTimerTask", "()V", ()).await?.into();
    let _: () = jvm
        .invoke_virtual(
            &timer,
            &timer.class_definition().name(),
            "scheduleAtFixedRate",
            "(Ljava/util/TimerTask;JJ)V",
            (fixed_rate.clone(), 0i64, 60i64),
        )
        .await?;

    let first_notification = prepare_task_notification(&jvm, &fixed_rate).await?;
    let mut worker = next_spawn(&runtime).await;
    wait_for_task_notification(&jvm, &fixed_rate, first_notification).await?;
    assert_eq!(jvm.get_field::<i32>(&fixed_rate, "runCount", "I").await?, 1);
    assert_eq!(jvm.get_field::<i64>(&fixed_rate, "nextExecutionTime", "J").await?, i64::MAX - 40);

    let timeout = next_spawn_while_worker_runs(&runtime, &mut worker).await;
    assert_eq!(
        tokio::time::timeout(TEST_BARRIER_TIMEOUT, runtime.next_sleep_deadline())
            .await
            .expect("timed out waiting for near-boundary fixed-rate sleep"),
        i64::MAX as u64 - 40
    );
    let second_notification = prepare_task_notification(&jvm, &fixed_rate).await?;
    runtime.advance_time(Duration::from_millis(60));
    await_spawn(timeout, "near-boundary fixed-rate timeout callback").await?;
    wait_for_task_notification(&jvm, &fixed_rate, second_notification).await?;
    assert_eq!(jvm.get_field::<i32>(&fixed_rate, "runCount", "I").await?, 2);
    assert_eq!(jvm.get_field::<i32>(&fixed_rate, "state", "I").await?, 2);
    let thread: ClassInstanceRef<TimerThread> = jvm.get_field(&timer, "thread", "Ljava/util/Timer$TimerThread;").await?;
    let queue: ClassInstanceRef<TimerTaskQueue> = jvm.get_field(&thread, "queue", "Ljava/util/Timer$TaskQueue;").await?;
    assert_eq!(jvm.get_field::<i32>(&queue, "size", "I").await?, 0);
    let _: () = jvm.invoke_virtual(&timer, &timer.class_definition().name(), "cancel", "()V", ()).await?;
    await_spawn(worker, "near-boundary fixed-rate timer worker").await?;

    let (runtime, jvm) = timer_test_jvm(i64::MAX as u64 - 100).await?;
    let timer = jvm.new_class("java/util/Timer", "()V", ()).await?;
    let fixed_delay: ClassInstanceRef<TestTimerTask> = jvm.new_class("TestTimerTask", "()V", ()).await?.into();
    let _: () = jvm
        .invoke_virtual(
            &timer,
            &timer.class_definition().name(),
            "schedule",
            "(Ljava/util/TimerTask;JJ)V",
            (fixed_delay.clone(), 0i64, 60i64),
        )
        .await?;
    runtime.advance_time(Duration::from_millis(50));
    let notification = prepare_task_notification(&jvm, &fixed_delay).await?;
    let worker = next_spawn(&runtime).await;
    wait_for_task_notification(&jvm, &fixed_delay, notification).await?;
    assert_eq!(jvm.get_field::<i32>(&fixed_delay, "runCount", "I").await?, 1);
    assert_eq!(jvm.get_field::<i32>(&fixed_delay, "state", "I").await?, 2);
    let _: () = jvm.invoke_virtual(&timer, &timer.class_definition().name(), "cancel", "()V", ()).await?;
    await_spawn(worker, "near-boundary fixed-delay timer worker").await?;
    Ok(())
}

#[tokio::test]
async fn timer_uses_min_heap_and_rejects_task_reuse() -> Result<()> {
    let (runtime, jvm) = timer_test_jvm(1_000).await?;
    let timer = jvm.new_class("java/util/Timer", "()V", ()).await?;
    let later: ClassInstanceRef<TestTimerTask> = jvm.new_class("TestTimerTask", "()V", ()).await?.into();
    let earlier: ClassInstanceRef<TestTimerTask> = jvm.new_class("TestTimerTask", "()V", ()).await?.into();
    let _: () = jvm
        .invoke_virtual(
            &timer,
            &timer.class_definition().name(),
            "schedule",
            "(Ljava/util/TimerTask;J)V",
            (later.clone(), 200i64),
        )
        .await?;
    let _: () = jvm
        .invoke_virtual(
            &timer,
            &timer.class_definition().name(),
            "schedule",
            "(Ljava/util/TimerTask;J)V",
            (earlier.clone(), 100i64),
        )
        .await?;

    let thread: ClassInstanceRef<TimerThread> = jvm.get_field(&timer, "thread", "Ljava/util/Timer$TimerThread;").await?;
    let queue: ClassInstanceRef<TimerTaskQueue> = jvm.get_field(&thread, "queue", "Ljava/util/Timer$TaskQueue;").await?;
    assert_eq!(jvm.get_field::<i32>(&queue, "size", "I").await?, 2);
    let heap: ClassInstanceRef<jvm::Array<TimerTask>> = jvm.get_field(&queue, "queue", "[Ljava/util/TimerTask;").await?;
    let first: ClassInstanceRef<TimerTask> = jvm.load_array(&heap, 1, 1).await?.remove(0);
    assert_eq!(first.identity(), earlier.identity());

    assert_exception(
        jvm.invoke_virtual::<_, ()>(
            &timer,
            &timer.class_definition().name(),
            "schedule",
            "(Ljava/util/TimerTask;J)V",
            (earlier.clone(), 0i64),
        )
        .await,
        "java/lang/IllegalStateException",
    )
    .await;
    let _: () = jvm.invoke_virtual(&timer, &timer.class_definition().name(), "cancel", "()V", ()).await?;
    assert_eq!(jvm.get_field::<i32>(&queue, "size", "I").await?, 0);
    let _: () = jvm.invoke_virtual(&timer, &timer.class_definition().name(), "cancel", "()V", ()).await?;
    await_spawn(next_spawn(&runtime).await, "heap timer worker").await?;
    Ok(())
}

#[tokio::test]
async fn timer_one_shot_executes_once_and_records_scheduled_time() -> Result<()> {
    let (runtime, jvm) = timer_test_jvm(1_000).await?;
    let timer = jvm.new_class("java/util/Timer", "()V", ()).await?;
    let task: ClassInstanceRef<TestTimerTask> = jvm.new_class("TestTimerTask", "()V", ()).await?.into();
    let _: () = jvm
        .invoke_virtual(
            &timer,
            &timer.class_definition().name(),
            "schedule",
            "(Ljava/util/TimerTask;J)V",
            (task.clone(), 100i64),
        )
        .await?;

    let notification = prepare_task_notification(&jvm, &task).await?;
    let mut worker = next_spawn(&runtime).await;
    let timeout = next_spawn_while_worker_runs(&runtime, &mut worker).await;
    assert_eq!(
        tokio::time::timeout(TEST_BARRIER_TIMEOUT, runtime.next_sleep_deadline())
            .await
            .expect("timed out waiting for one-shot sleep registration"),
        1_100
    );
    runtime.advance_time(Duration::from_millis(100));
    await_spawn(timeout, "one-shot timeout callback").await?;
    wait_for_task_notification(&jvm, &task, notification).await?;

    assert_eq!(jvm.get_field::<i32>(&task, "runCount", "I").await?, 1);
    assert_eq!(jvm.get_field::<i32>(&task, "state", "I").await?, 2);
    assert_eq!(jvm.get_field::<i64>(&task, "firstScheduledTime", "J").await?, 1_100);
    assert_eq!(jvm.get_field::<i64>(&task, "firstActualTime", "J").await?, 1_100);
    assert_eq!(
        jvm.invoke_virtual::<_, i64>(&task, "TestTimerTask", "scheduledExecutionTime", "()J", ())
            .await?,
        1_100
    );
    assert!(!jvm.invoke_virtual::<_, bool>(&task, "TestTimerTask", "cancel", "()Z", ()).await?);
    assert_eq!(jvm.get_field::<i32>(&task, "state", "I").await?, 3);

    let _: () = jvm.invoke_virtual(&timer, &timer.class_definition().name(), "cancel", "()V", ()).await?;
    await_spawn(worker, "one-shot timer worker").await?;
    Ok(())
}

#[tokio::test]
async fn timer_new_first_task_wakes_worker_and_recomputes_deadline() -> Result<()> {
    let (runtime, jvm) = timer_test_jvm(1_000).await?;
    let timer = jvm.new_class("java/util/Timer", "()V", ()).await?;
    let later: ClassInstanceRef<TestTimerTask> = jvm.new_class("TestTimerTask", "()V", ()).await?.into();
    let earlier: ClassInstanceRef<TestTimerTask> = jvm.new_class("TestTimerTask", "()V", ()).await?.into();
    let _: () = jvm
        .invoke_virtual(
            &timer,
            &timer.class_definition().name(),
            "schedule",
            "(Ljava/util/TimerTask;J)V",
            (later.clone(), 200i64),
        )
        .await?;

    let mut worker = next_spawn(&runtime).await;
    let old_timeout = next_spawn_while_worker_runs(&runtime, &mut worker).await;
    assert_eq!(
        tokio::time::timeout(TEST_BARRIER_TIMEOUT, runtime.next_sleep_deadline())
            .await
            .expect("timed out waiting for initial later sleep registration"),
        1_200
    );
    let notification = prepare_task_notification(&jvm, &earlier).await?;
    let _: () = jvm
        .invoke_virtual(
            &timer,
            &timer.class_definition().name(),
            "schedule",
            "(Ljava/util/TimerTask;J)V",
            (earlier.clone(), 100i64),
        )
        .await?;
    let new_timeout = next_spawn_while_worker_runs(&runtime, &mut worker).await;
    tokio::time::timeout(TEST_BARRIER_TIMEOUT, runtime.wait_for_sleep_deadline(1_100))
        .await
        .expect("timed out waiting for earlier sleep registration");

    runtime.advance_time(Duration::from_millis(100));
    await_spawn(new_timeout, "earlier timeout callback").await?;
    wait_for_task_notification(&jvm, &earlier, notification).await?;
    assert_eq!(jvm.get_field::<i32>(&earlier, "runCount", "I").await?, 1);
    assert_eq!(jvm.get_field::<i32>(&later, "runCount", "I").await?, 0);

    let stale_later_timeout = tokio::time::timeout(TEST_BARRIER_TIMEOUT, runtime.next_spawn_callback())
        .await
        .expect("timed out waiting for later timeout callback")
        .expect("later timeout");
    drop(stale_later_timeout);
    let _: () = jvm.invoke_virtual(&timer, &timer.class_definition().name(), "cancel", "()V", ()).await?;
    await_spawn(worker, "new-first timer worker").await?;
    runtime.advance_time(Duration::from_millis(100));
    await_spawn(old_timeout, "stale timeout callback").await?;
    Ok(())
}

#[tokio::test]
async fn timer_task_cancel_only_reports_a_live_schedule() -> Result<()> {
    let (runtime, jvm) = timer_test_jvm(1_000).await?;
    let timer = jvm.new_class("java/util/Timer", "()V", ()).await?;
    let task: ClassInstanceRef<TestTimerTask> = jvm.new_class("TestTimerTask", "()V", ()).await?.into();
    let _: () = jvm
        .invoke_virtual(
            &timer,
            &timer.class_definition().name(),
            "schedule",
            "(Ljava/util/TimerTask;J)V",
            (task.clone(), 100i64),
        )
        .await?;
    assert!(jvm.invoke_virtual::<_, bool>(&task, "TestTimerTask", "cancel", "()Z", ()).await?);
    assert!(!jvm.invoke_virtual::<_, bool>(&task, "TestTimerTask", "cancel", "()Z", ()).await?);
    assert_eq!(jvm.get_field::<i32>(&task, "state", "I").await?, 3);

    let worker = next_spawn(&runtime).await;
    let _: () = jvm.invoke_virtual(&timer, &timer.class_definition().name(), "cancel", "()V", ()).await?;
    await_spawn(worker, "cancelled-task timer worker").await?;
    assert_eq!(jvm.get_field::<i32>(&task, "runCount", "I").await?, 0);
    assert_eq!(
        jvm.invoke_virtual::<_, i64>(&task, "TestTimerTask", "scheduledExecutionTime", "()J", ())
            .await?,
        0
    );
    Ok(())
}

#[tokio::test]
async fn timer_fixed_delay_uses_actual_execution_time() -> Result<()> {
    let (runtime, jvm) = timer_test_jvm(1_000).await?;
    let timer = jvm.new_class("java/util/Timer", "()V", ()).await?;
    let task: ClassInstanceRef<TestTimerTask> = jvm.new_class("TestTimerTask", "()V", ()).await?.into();
    let _: () = jvm
        .invoke_virtual(
            &timer,
            &timer.class_definition().name(),
            "schedule",
            "(Ljava/util/TimerTask;JJ)V",
            (task.clone(), 100i64, 50i64),
        )
        .await?;
    assert_eq!(jvm.get_field::<i64>(&task, "period", "J").await?, -50);

    let notification = prepare_task_notification(&jvm, &task).await?;
    let mut worker = next_spawn(&runtime).await;
    let first_timeout = next_spawn_while_worker_runs(&runtime, &mut worker).await;
    assert_eq!(
        tokio::time::timeout(TEST_BARRIER_TIMEOUT, runtime.next_sleep_deadline())
            .await
            .expect("timed out waiting for fixed-delay sleep registration"),
        1_100
    );
    runtime.advance_time(Duration::from_millis(200));
    await_spawn(first_timeout, "first fixed-delay timeout callback").await?;
    wait_for_task_notification(&jvm, &task, notification).await?;
    let next_timeout = tokio::time::timeout(TEST_BARRIER_TIMEOUT, async {
        tokio::select! {
            callback = runtime.next_spawn_callback() => callback.expect("next fixed-delay timeout"),
            result = &mut worker => panic!("timer worker exited before fixed-delay reschedule: {result:?}"),
        }
    })
    .await
    .expect("timed out waiting for fixed-delay reschedule");

    assert_eq!(jvm.get_field::<i32>(&task, "runCount", "I").await?, 1);
    assert_eq!(jvm.get_field::<i32>(&task, "state", "I").await?, 1);
    assert_eq!(jvm.get_field::<i64>(&task, "firstScheduledTime", "J").await?, 1_100);
    assert_eq!(jvm.get_field::<i64>(&task, "firstActualTime", "J").await?, 1_200);
    assert_eq!(jvm.get_field::<i64>(&task, "nextExecutionTime", "J").await?, 1_250);

    drop(next_timeout);
    let _: () = jvm.invoke_virtual(&timer, &timer.class_definition().name(), "cancel", "()V", ()).await?;
    await_spawn(worker, "fixed-delay timer worker").await?;
    Ok(())
}

#[tokio::test]
async fn timer_fixed_rate_catches_up_from_previous_deadline() -> Result<()> {
    let (runtime, jvm) = timer_test_jvm(1_000).await?;
    let timer = jvm.new_class("java/util/Timer", "()V", ()).await?;
    let task: ClassInstanceRef<TestTimerTask> = jvm.new_class("TestTimerTask", "()V", ()).await?.into();
    let _: () = jvm
        .invoke_virtual(
            &timer,
            &timer.class_definition().name(),
            "scheduleAtFixedRate",
            "(Ljava/util/TimerTask;JJ)V",
            (task.clone(), 100i64, 50i64),
        )
        .await?;
    assert_eq!(jvm.get_field::<i64>(&task, "period", "J").await?, 50);

    let notification = prepare_task_notification(&jvm, &task).await?;
    let mut worker = next_spawn(&runtime).await;
    let first_timeout = next_spawn_while_worker_runs(&runtime, &mut worker).await;
    assert_eq!(
        tokio::time::timeout(TEST_BARRIER_TIMEOUT, runtime.next_sleep_deadline())
            .await
            .expect("timed out waiting for fixed-rate sleep registration"),
        1_100
    );
    runtime.advance_time(Duration::from_millis(200));
    await_spawn(first_timeout, "first fixed-rate timeout callback").await?;
    wait_for_task_notification(&jvm, &task, notification).await?;
    let next_timeout = tokio::time::timeout(TEST_BARRIER_TIMEOUT, async {
        tokio::select! {
            callback = runtime.next_spawn_callback() => callback.expect("next fixed-rate timeout"),
            result = &mut worker => panic!("timer worker exited before fixed-rate reschedule: {result:?}"),
        }
    })
    .await
    .expect("timed out waiting for fixed-rate reschedule");

    assert_eq!(jvm.get_field::<i32>(&task, "runCount", "I").await?, 3);
    assert_eq!(jvm.get_field::<i64>(&task, "firstScheduledTime", "J").await?, 1_100);
    assert_eq!(jvm.get_field::<i64>(&task, "lastScheduledTime", "J").await?, 1_200);
    assert_eq!(jvm.get_field::<i64>(&task, "nextExecutionTime", "J").await?, 1_250);

    drop(next_timeout);
    let _: () = jvm.invoke_virtual(&timer, &timer.class_definition().name(), "cancel", "()V", ()).await?;
    await_spawn(worker, "fixed-rate timer worker").await?;
    Ok(())
}

#[tokio::test]
async fn timer_date_schedule_and_task_exception_close_worker() -> Result<()> {
    let (runtime, jvm) = timer_test_jvm(1_000).await?;
    let timer = jvm.new_class("java/util/Timer", "()V", ()).await?;
    let mut task: ClassInstanceRef<TestTimerTask> = jvm.new_class("TestTimerTask", "()V", ()).await?.into();
    let date = jvm.new_class("java/util/Date", "(J)V", (1_000i64,)).await?;
    let _: () = jvm
        .invoke_virtual(
            &timer,
            &timer.class_definition().name(),
            "schedule",
            "(Ljava/util/TimerTask;Ljava/util/Date;J)V",
            (task.clone(), date, 25i64),
        )
        .await?;
    assert_eq!(jvm.get_field::<i64>(&task, "nextExecutionTime", "J").await?, 1_000);
    assert_eq!(jvm.get_field::<i64>(&task, "period", "J").await?, -25);
    jvm.put_field(&mut task, "throwOnRun", "Z", true).await?;

    let notification = prepare_task_notification(&jvm, &task).await?;
    let worker = next_spawn(&runtime).await;
    wait_for_task_notification(&jvm, &task, notification).await?;
    await_spawn(worker, "exception timer worker").await?;

    let thread: ClassInstanceRef<TimerThread> = jvm.get_field(&timer, "thread", "Ljava/util/Timer$TimerThread;").await?;
    let queue: ClassInstanceRef<TimerTaskQueue> = jvm.get_field(&thread, "queue", "Ljava/util/Timer$TaskQueue;").await?;
    assert!(!jvm.get_field::<bool>(&thread, "newTasksMayBeScheduled", "Z").await?);
    assert!(!jvm.get_field::<bool>(&thread, "alive", "Z").await?);
    assert_eq!(jvm.get_field::<i32>(&queue, "size", "I").await?, 0);

    let fresh: ClassInstanceRef<TestTimerTask> = jvm.new_class("TestTimerTask", "()V", ()).await?.into();
    assert_exception(
        jvm.invoke_virtual::<_, ()>(
            &timer,
            &timer.class_definition().name(),
            "schedule",
            "(Ljava/util/TimerTask;J)V",
            (fresh.clone(), 0i64),
        )
        .await,
        "java/lang/IllegalStateException",
    )
    .await;
    assert_eq!(jvm.get_field::<i32>(&fresh, "state", "I").await?, 0);
    Ok(())
}
