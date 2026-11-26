use alloc::{boxed::Box, format};
use core::time::Duration;

use jvm::{ClassInstance, ClassInstanceRef, GlobalRef, Jvm, Result, runtime::JavaLangString};
use jvm_class_proto::{java_class, java_methods};

use crate::{
    RuntimeContext, SpawnCallback,
    classes::java::lang::{Runnable, String},
};

// class java.lang.Thread
#[java_class(name = "java/lang/Thread", extends = "java/lang/Object", implements("java/lang/Runnable"), public)]
pub struct Thread {
    #[field(public, static, final)]
    MIN_PRIORITY: i32,
    #[field(public, static, final)]
    NORM_PRIORITY: i32,
    #[field(public, static, final)]
    MAX_PRIORITY: i32,
    #[field(private, static)]
    threadInitNumber: i32,
    #[field(private)]
    id: i64,
    #[field(private)]
    target: ClassInstanceRef<Runnable>,
    #[field(private)]
    name: ClassInstanceRef<String>,
    #[field(private)]
    priority: i32,
    #[field(private)]
    interrupted: bool,
    #[field(private)]
    started: bool,
    #[field(private)]
    alive: bool,
    #[field(private)]
    daemon: bool,
}

#[java_methods]
impl Thread {
    #[method(static_initializer)]
    async fn clinit(jvm: &Jvm, _: &mut RuntimeContext) -> Result<()> {
        jvm.put_static_field("java/lang/Thread", "MIN_PRIORITY", "I", 1i32).await?;
        jvm.put_static_field("java/lang/Thread", "NORM_PRIORITY", "I", 5i32).await?;
        jvm.put_static_field("java/lang/Thread", "MAX_PRIORITY", "I", 10i32).await?;
        jvm.put_static_field("java/lang/Thread", "threadInitNumber", "I", 0i32).await
    }

    #[method(constructor, public)]
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

    #[method(constructor, public)]
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

    #[method(constructor, public)]
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

    #[method(constructor, public)]
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
        jvm.put_field(&mut this, "java/lang/Thread", "target", "Ljava/lang/Runnable;", target)
            .await?;
        jvm.put_field(&mut this, "java/lang/Thread", "name", "Ljava/lang/String;", name).await?;
        jvm.put_field(&mut this, "java/lang/Thread", "priority", "I", 5i32).await?;
        jvm.put_field(&mut this, "java/lang/Thread", "interrupted", "Z", false).await?;
        jvm.put_field(&mut this, "java/lang/Thread", "started", "Z", false).await?;
        jvm.put_field(&mut this, "java/lang/Thread", "alive", "Z", false).await?;
        let current_thread = jvm.current_java_thread();
        let daemon: bool = jvm.get_field(&current_thread, "java/lang/Thread", "daemon", "Z").await?;
        jvm.put_field(&mut this, "java/lang/Thread", "daemon", "Z", daemon).await?;

        Ok(())
    }

    // rustjava internal
    #[method(constructor, private)]
    async fn init_internal(jvm: &Jvm, context: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, internal: bool) -> Result<()> {
        tracing::debug!("java.lang.Thread::<init>({this:?}, {internal:?})");

        let id = context.current_task_id();
        jvm.put_field(&mut this, "java/lang/Thread", "id", "J", id as i64).await?;
        jvm.put_field(&mut this, "java/lang/Thread", "priority", "I", 5i32).await?;
        jvm.put_field(&mut this, "java/lang/Thread", "interrupted", "Z", false).await?;
        jvm.put_field(&mut this, "java/lang/Thread", "started", "Z", true).await?;
        jvm.put_field(&mut this, "java/lang/Thread", "alive", "Z", internal).await?;
        jvm.put_field(&mut this, "java/lang/Thread", "daemon", "Z", false).await?;

        Ok(())
    }

    #[method(public, synchronized)]
    async fn start(jvm: &Jvm, context: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.lang.Thread::start({this:?})");

        let started: bool = jvm.get_field(&this, "java/lang/Thread", "started", "Z").await?;
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

                let result: Result<()> = self.jvm.invoke_virtual(&self.this, "java/lang/Thread", "run", "()V", []).await;

                if let Err(jvm::JavaError::JavaException(exception)) = &result {
                    let trace = async {
                        let string_writer = self.jvm.new_class("java/io/StringWriter", "()V", ()).await?;
                        let print_writer = self
                            .jvm
                            .new_class("java/io/PrintWriter", "(Ljava/io/Writer;)V", (string_writer.clone(),))
                            .await?;
                        let _: () = self
                            .jvm
                            .invoke_virtual(
                                exception,
                                &exception.class_definition().name(),
                                "printStackTrace",
                                "(Ljava/io/PrintWriter;)V",
                                (print_writer,),
                            )
                            .await?;
                        let trace: Box<dyn ClassInstance> = self
                            .jvm
                            .invoke_virtual(&string_writer, "java/io/StringWriter", "toString", "()Ljava/lang/String;", [])
                            .await?;
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
                    let alive_result = self.jvm.put_field(&mut this, "java/lang/Thread", "alive", "Z", false).await;
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

        jvm.put_field(&mut this, "java/lang/Thread", "started", "Z", true).await?;
        jvm.put_field(&mut this, "java/lang/Thread", "alive", "Z", true).await?;

        let id: i32 = jvm.invoke_virtual(&this, "java/lang/Object", "hashCode", "()I", ()).await?;

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

    #[method(public)]
    async fn run(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.lang.Thread::run({this:?})");

        let target: ClassInstanceRef<Runnable> = jvm.get_field(&this, "java/lang/Thread", "target", "Ljava/lang/Runnable;").await?;
        if !target.is_null() {
            let _: () = jvm.invoke_virtual(&target, &target.class_definition().name(), "run", "()V", ()).await?;
        }

        Ok(())
    }

    #[method(public, final, synchronized)]
    async fn join(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.lang.Thread::join({this:?})");

        let _: () = jvm.invoke_virtual(&this, "java/lang/Thread", "join", "(J)V", (0i64,)).await?;

        Ok(())
    }

    #[method(name = "join", public, final, synchronized)]
    async fn join_long(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>, millis: i64) -> Result<()> {
        tracing::debug!("java.lang.Thread::join({this:?}, {millis:?})");

        if millis < 0 {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "timeout value is negative").await);
        }

        let start = context.now();
        loop {
            let alive: bool = jvm.get_field(&this, "java/lang/Thread", "alive", "Z").await?;
            if !alive {
                return Ok(());
            }

            if millis == 0 {
                let _: () = jvm.invoke_virtual(&this, "java/lang/Object", "wait", "(J)V", (0i64,)).await?;
            } else {
                let elapsed = context.now().saturating_sub(start);
                if elapsed >= millis as u64 {
                    return Ok(());
                }
                let _: () = jvm
                    .invoke_virtual(&this, "java/lang/Object", "wait", "(J)V", (millis - elapsed as i64,))
                    .await?;
            }
        }
    }

    #[method(public, final)]
    async fn is_alive(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::debug!("java.lang.Thread::isAlive({this:?})");
        let alive: bool = jvm.get_field(&this, "java/lang/Thread", "alive", "Z").await?;
        Ok(alive)
    }

    #[method(public, final)]
    async fn get_name(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.lang.Thread::getName({this:?})");

        let name: ClassInstanceRef<String> = jvm.get_field(&this, "java/lang/Thread", "name", "Ljava/lang/String;").await?;
        if name.is_null() {
            return Ok(JavaLangString::from_rust_string(jvm, "main").await?.into());
        }

        Ok(name)
    }

    #[method(public, final)]
    async fn set_name(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, name: ClassInstanceRef<String>) -> Result<()> {
        tracing::debug!("java.lang.Thread::setName({this:?}, {name:?})");

        if name.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "name").await);
        }

        jvm.put_field(&mut this, "java/lang/Thread", "name", "Ljava/lang/String;", name).await
    }

    #[method(public, final)]
    async fn get_priority(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.lang.Thread::getPriority({this:?})");
        jvm.get_field(&this, "java/lang/Thread", "priority", "I").await
    }

    #[method(public)]
    async fn interrupt(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.lang.Thread::interrupt({this:?})");
        jvm.interrupt_java_thread(&mut this).await
    }

    #[method(public)]
    async fn is_interrupted(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::debug!("java.lang.Thread::isInterrupted({this:?})");
        jvm.is_java_thread_interrupted(&this).await
    }

    #[method(public, static)]
    async fn interrupted(jvm: &Jvm, _: &mut RuntimeContext) -> Result<bool> {
        tracing::debug!("java.lang.Thread::interrupted()");
        jvm.current_java_thread_interrupted().await
    }

    #[method(public, static)]
    async fn active_count(jvm: &Jvm, _: &mut RuntimeContext) -> Result<i32> {
        Ok(jvm.active_thread_count() as i32)
    }

    #[method(public)]
    async fn to_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.lang.Thread::toString({this:?})");

        let name: ClassInstanceRef<String> = jvm
            .invoke_virtual(&this, "java/lang/Thread", "getName", "()Ljava/lang/String;", ())
            .await?;
        let name = JavaLangString::to_rust_string(jvm, &name).await?;
        let priority: i32 = jvm.get_field(&this, "java/lang/Thread", "priority", "I").await?;
        Ok(JavaLangString::from_rust_string(jvm, &format!("Thread[{name},{priority}]")).await?.into())
    }

    #[method(public, native, static)]
    async fn sleep(jvm: &Jvm, context: &mut RuntimeContext, duration: i64) -> Result<()> {
        tracing::debug!("java.lang.Thread::sleep({duration:?})");

        if duration < 0 {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "timeout value is negative").await);
        }

        jvm.sleep_interruptibly(context.sleep(Duration::from_millis(duration as _))).await
    }

    #[method(public, native, static)]
    async fn r#yield(_: &Jvm, context: &mut RuntimeContext) -> Result<()> {
        tracing::debug!("java.lang.Thread::yield()");
        context.r#yield().await;

        Ok(())
    }

    #[method(public, final)]
    async fn set_priority(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Thread>, new_priority: i32) -> Result<()> {
        tracing::debug!("java.lang.Thread::setPriority({this:?}, {new_priority:?})");

        if !(1..=10).contains(&new_priority) {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "priority out of range").await);
        }

        jvm.put_field(&mut this, "java/lang/Thread", "priority", "I", new_priority).await?;

        Ok(())
    }

    #[method(public, final)]
    async fn set_daemon(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Thread>, daemon: bool) -> Result<()> {
        tracing::debug!("java.lang.Thread::setDaemon({this:?}, {daemon:?})");

        let alive: bool = jvm.get_field(&this, "java/lang/Thread", "alive", "Z").await?;
        if alive {
            return Err(jvm.exception("java/lang/IllegalThreadStateException", "thread is active").await);
        }

        jvm.put_field(&mut this, "java/lang/Thread", "daemon", "Z", daemon).await
    }

    #[method(public, final)]
    async fn is_daemon(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Thread>) -> Result<bool> {
        tracing::debug!("java.lang.Thread::isDaemon({this:?})");
        jvm.get_field(&this, "java/lang/Thread", "daemon", "Z").await
    }

    #[method(public, native, static)]
    async fn current_thread(jvm: &Jvm, _: &mut RuntimeContext) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.Thread::currentThread()");

        Ok(jvm.current_java_thread().into())
    }
}
