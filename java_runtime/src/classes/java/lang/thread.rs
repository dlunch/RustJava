use alloc::{boxed::Box, format, vec};
use core::time::Duration;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{FieldAccessFlags, MethodAccessFlags};
use jvm::{ClassInstanceRef, GlobalRef, Jvm, Result, runtime::JavaLangString};

use crate::{
    RuntimeClassProto, RuntimeContext, SpawnCallback,
    classes::java::lang::{Runnable, String},
};

// class java.lang.Thread
pub struct Thread;

impl Thread {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/lang/Thread",
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["java/lang/Runnable"],
            methods: vec![
                JavaMethodProto::new("<clinit>", "()V", Self::clinit, MethodAccessFlags::STATIC),
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("<init>", "(Ljava/lang/Runnable;)V", Self::init_with_runnable, Default::default()),
                JavaMethodProto::new("<init>", "(Ljava/lang/String;)V", Self::init_with_name, Default::default()),
                JavaMethodProto::new(
                    "<init>",
                    "(Ljava/lang/Runnable;Ljava/lang/String;)V",
                    Self::init_with_runnable_and_name,
                    Default::default(),
                ),
                JavaMethodProto::new("start", "()V", Self::start, MethodAccessFlags::SYNCHRONIZED),
                JavaMethodProto::new("join", "()V", Self::join, MethodAccessFlags::SYNCHRONIZED),
                JavaMethodProto::new("run", "()V", Self::run, Default::default()),
                JavaMethodProto::new("isAlive", "()Z", Self::is_alive, Default::default()),
                JavaMethodProto::new("getName", "()Ljava/lang/String;", Self::get_name, Default::default()),
                JavaMethodProto::new("getPriority", "()I", Self::get_priority, Default::default()),
                JavaMethodProto::new("interrupt", "()V", Self::interrupt, Default::default()),
                JavaMethodProto::new("activeCount", "()I", Self::active_count, MethodAccessFlags::STATIC),
                JavaMethodProto::new("toString", "()Ljava/lang/String;", Self::to_string, Default::default()),
                JavaMethodProto::new("sleep", "(J)V", Self::sleep, MethodAccessFlags::NATIVE | MethodAccessFlags::STATIC),
                JavaMethodProto::new("yield", "()V", Self::r#yield, MethodAccessFlags::NATIVE | MethodAccessFlags::STATIC),
                JavaMethodProto::new("setPriority", "(I)V", Self::set_priority, Default::default()),
                JavaMethodProto::new(
                    "currentThread",
                    "()Ljava/lang/Thread;",
                    Self::current_thread,
                    MethodAccessFlags::NATIVE | MethodAccessFlags::STATIC,
                ),
                // rustjava internal
                JavaMethodProto::new("<init>", "(Z)V", Self::init_internal, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new(
                    "MIN_PRIORITY",
                    "I",
                    FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new(
                    "NORM_PRIORITY",
                    "I",
                    FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new(
                    "MAX_PRIORITY",
                    "I",
                    FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
                ),
                JavaFieldProto::new("threadInitNumber", "I", FieldAccessFlags::STATIC),
                JavaFieldProto::new("id", "J", Default::default()),
                JavaFieldProto::new("target", "Ljava/lang/Runnable;", Default::default()),
                JavaFieldProto::new("name", "Ljava/lang/String;", Default::default()),
                JavaFieldProto::new("priority", "I", Default::default()),
                JavaFieldProto::new("interrupted", "Z", Default::default()),
                JavaFieldProto::new("started", "Z", Default::default()),
                JavaFieldProto::new("alive", "Z", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    async fn clinit(jvm: &Jvm, _: &mut RuntimeContext) -> Result<()> {
        jvm.put_static_field("java/lang/Thread", "MIN_PRIORITY", "I", 1i32).await?;
        jvm.put_static_field("java/lang/Thread", "NORM_PRIORITY", "I", 5i32).await?;
        jvm.put_static_field("java/lang/Thread", "MAX_PRIORITY", "I", 10i32).await?;
        jvm.put_static_field("java/lang/Thread", "threadInitNumber", "I", 0i32).await
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.lang.Thread::<init>({this:?})");

        let number: i32 = jvm.get_static_field("java/lang/Thread", "threadInitNumber", "I").await?;
        jvm.put_static_field("java/lang/Thread", "threadInitNumber", "I", number + 1).await?;
        let name = JavaLangString::from_rust_string(jvm, &format!("Thread-{number}")).await?;
        let target = ClassInstanceRef::<Runnable>::new(None);
        let _: () = jvm
            .invoke_special(
                &this,
                "java/lang/Thread",
                "<init>",
                "(Ljava/lang/Runnable;Ljava/lang/String;)V",
                (target, name),
            )
            .await?;

        Ok(())
    }

    async fn init_with_runnable(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, target: ClassInstanceRef<Runnable>) -> Result<()> {
        tracing::debug!("java.lang.Thread::<init>({this:?}, {target:?})");

        let number: i32 = jvm.get_static_field("java/lang/Thread", "threadInitNumber", "I").await?;
        jvm.put_static_field("java/lang/Thread", "threadInitNumber", "I", number + 1).await?;
        let name = JavaLangString::from_rust_string(jvm, &format!("Thread-{number}")).await?;
        let _: () = jvm
            .invoke_special(
                &this,
                "java/lang/Thread",
                "<init>",
                "(Ljava/lang/Runnable;Ljava/lang/String;)V",
                (target, name),
            )
            .await?;

        Ok(())
    }

    async fn init_with_name(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, name: ClassInstanceRef<String>) -> Result<()> {
        tracing::debug!("java.lang.Thread::<init>({this:?}, {name:?})");

        let target = ClassInstanceRef::<Runnable>::new(None);
        let _: () = jvm
            .invoke_special(
                &this,
                "java/lang/Thread",
                "<init>",
                "(Ljava/lang/Runnable;Ljava/lang/String;)V",
                (target, name),
            )
            .await?;

        Ok(())
    }

    async fn init_with_runnable_and_name(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        target: ClassInstanceRef<Runnable>,
        name: ClassInstanceRef<String>,
    ) -> Result<()> {
        tracing::debug!("java.lang.Thread::<init>({this:?}, {target:?}, {name:?})");

        if name.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "name").await);
        }

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "target", "Ljava/lang/Runnable;", target).await?;
        jvm.put_field(&mut this, "name", "Ljava/lang/String;", name).await?;
        jvm.put_field(&mut this, "priority", "I", 5i32).await?;
        jvm.put_field(&mut this, "interrupted", "Z", false).await?;
        jvm.put_field(&mut this, "started", "Z", false).await?;
        jvm.put_field(&mut this, "alive", "Z", false).await?;

        Ok(())
    }

    async fn init_internal(jvm: &Jvm, context: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, internal: bool) -> Result<()> {
        tracing::debug!("java.lang.Thread::<init>({this:?}, {internal:?})");

        let id = context.current_task_id();
        jvm.put_field(&mut this, "id", "J", id as i64).await?;
        jvm.put_field(&mut this, "priority", "I", 5i32).await?;
        jvm.put_field(&mut this, "interrupted", "Z", false).await?;
        jvm.put_field(&mut this, "started", "Z", true).await?;
        jvm.put_field(&mut this, "alive", "Z", internal).await?;

        Ok(())
    }

    async fn start(jvm: &Jvm, context: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.lang.Thread::start({this:?})");

        let started: bool = jvm.get_field(&this, "started", "Z").await?;
        if started {
            return Err(jvm.exception("java/lang/IllegalThreadStateException", "thread already started").await);
        }

        struct ThreadStartProxy {
            jvm: Jvm,
            thread_id: i32,
            this: GlobalRef<Thread>,
        }

        #[async_trait::async_trait]
        impl SpawnCallback for ThreadStartProxy {
            #[tracing::instrument(name = "java thread", fields(id = self.thread_id), skip_all)]
            async fn call(&self) -> Result<()> {
                tracing::trace!("Thread start");

                self.jvm.attach_thread(self.this.instance.clone()).await?;

                let result: Result<()> = self.jvm.invoke_virtual(&self.this, "run", "()V", []).await;

                if let Err(jvm::JavaError::JavaException(exception)) = &result {
                    let trace = async {
                        let string_writer = self.jvm.new_class("java/io/StringWriter", "()V", ()).await?;
                        let print_writer = self
                            .jvm
                            .new_class("java/io/PrintWriter", "(Ljava/io/Writer;)V", (string_writer.clone(),))
                            .await?;
                        let _: () = self
                            .jvm
                            .invoke_virtual(exception, "printStackTrace", "(Ljava/io/PrintWriter;)V", (print_writer,))
                            .await?;
                        let trace = self.jvm.invoke_virtual(&string_writer, "toString", "()Ljava/lang/String;", []).await?;
                        JavaLangString::to_rust_string(&self.jvm, &trace).await
                    }
                    .await;

                    match trace {
                        Ok(trace) => tracing::error!("Uncaught exception in thread {}:\n{}", self.thread_id, trace),
                        Err(error) => tracing::error!(?error, "failed to format uncaught exception in thread {}", self.thread_id),
                    }
                }

                let mut this = (*self.this).clone();
                let cleanup = if let Err(error) = self.jvm.monitor_enter(&self.this).await {
                    Err(error)
                } else {
                    let alive_result = self.jvm.put_field(&mut this, "alive", "Z", false).await;
                    let notify_result = if alive_result.is_ok() {
                        self.jvm.object_notify(&self.this, usize::MAX).await
                    } else {
                        Ok(())
                    };
                    let exit_result = self.jvm.monitor_exit(&self.this).await;
                    alive_result.and(notify_result).and(exit_result)
                };
                let detach_result = self.jvm.detach_thread();

                cleanup?;
                detach_result?;

                Ok(())
            }
        }

        jvm.put_field(&mut this, "started", "Z", true).await?;
        jvm.put_field(&mut this, "alive", "Z", true).await?;

        let id: i32 = jvm.invoke_virtual(&this, "hashCode", "()I", ()).await?;

        let this = match jvm.new_global_ref(&this) {
            Some(this) => this,
            None => return Err(jvm.exception("java/lang/NullPointerException", "thread is null").await),
        };
        context.spawn(
            jvm,
            Box::new(ThreadStartProxy {
                jvm: jvm.clone(),
                thread_id: id,
                this,
            }),
        );

        Ok(())
    }

    async fn run(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.lang.Thread::run({this:?})");

        let target: ClassInstanceRef<Runnable> = jvm.get_field(&this, "target", "Ljava/lang/Runnable;").await?;
        if !target.is_null() {
            let _: () = jvm.invoke_virtual(&target, "run", "()V", ()).await?;
        }

        Ok(())
    }

    async fn join(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.lang.Thread::join({this:?})");

        loop {
            let alive: bool = jvm.get_field(&this, "alive", "Z").await?;
            if !alive {
                return Ok(());
            }
            let _: () = jvm.invoke_virtual(&this, "wait", "()V", ()).await?;
        }
    }

    async fn is_alive(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::debug!("java.lang.Thread::isAlive({this:?})");
        let alive: bool = jvm.get_field(&this, "alive", "Z").await?;
        Ok(alive)
    }

    async fn get_name(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.lang.Thread::getName({this:?})");

        let name: ClassInstanceRef<String> = jvm.get_field(&this, "name", "Ljava/lang/String;").await?;
        if name.is_null() {
            return Ok(JavaLangString::from_rust_string(jvm, "main").await?.into());
        }

        Ok(name)
    }

    async fn get_priority(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.lang.Thread::getPriority({this:?})");
        jvm.get_field(&this, "priority", "I").await
    }

    async fn interrupt(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.lang.Thread::interrupt({this:?})");
        jvm.put_field(&mut this, "interrupted", "Z", true).await
    }

    async fn active_count(jvm: &Jvm, _: &mut RuntimeContext) -> Result<i32> {
        Ok(jvm.active_thread_count() as i32)
    }

    async fn to_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.lang.Thread::toString({this:?})");

        let name: ClassInstanceRef<String> = jvm.invoke_virtual(&this, "getName", "()Ljava/lang/String;", ()).await?;
        let name = JavaLangString::to_rust_string(jvm, &name).await?;
        let priority: i32 = jvm.get_field(&this, "priority", "I").await?;
        Ok(JavaLangString::from_rust_string(jvm, &format!("Thread[{name},{priority}]")).await?.into())
    }

    async fn sleep(jvm: &Jvm, context: &mut RuntimeContext, duration: i64) -> Result<()> {
        tracing::debug!("java.lang.Thread::sleep({duration:?})");

        if duration < 0 {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "timeout value is negative").await);
        }

        context.sleep(Duration::from_millis(duration as _)).await;

        Ok(())
    }

    async fn r#yield(_: &Jvm, context: &mut RuntimeContext) -> Result<()> {
        tracing::debug!("java.lang.Thread::yield()");
        context.r#yield().await;

        Ok(())
    }

    async fn set_priority(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Thread>, new_priority: i32) -> Result<()> {
        tracing::debug!("java.lang.Thread::setPriority({this:?}, {new_priority:?})");

        if !(1..=10).contains(&new_priority) {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "priority out of range").await);
        }

        jvm.put_field(&mut this, "priority", "I", new_priority).await?;

        Ok(())
    }

    async fn current_thread(jvm: &Jvm, _: &mut RuntimeContext) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.Thread::currentThread()");

        Ok(jvm.current_java_thread().into())
    }
}
