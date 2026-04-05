use alloc::{boxed::Box, vec};
use core::time::Duration;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::MethodAccessFlags;
use jvm::{ClassInstanceRef, Jvm, Result, runtime::JavaLangString};

use crate::{RuntimeClassProto, RuntimeContext, SpawnCallback, classes::java::lang::Runnable};

// class java.lang.Thread
pub struct Thread;

impl Thread {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/lang/Thread",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("<init>", "(Ljava/lang/Runnable;)V", Self::init_with_runnable, Default::default()),
                JavaMethodProto::new("start", "()V", Self::start, Default::default()),
                JavaMethodProto::new("join", "()V", Self::join, Default::default()),
                JavaMethodProto::new("run", "()V", Self::run, Default::default()),
                JavaMethodProto::new("isAlive", "()Z", Self::is_alive, Default::default()),
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
                JavaFieldProto::new("id", "J", Default::default()),
                JavaFieldProto::new("target", "Ljava/lang/Runnable;", Default::default()),
                JavaFieldProto::new("alive", "Z", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.lang.Thread::<init>({:?})", &this);

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        Ok(())
    }

    async fn init_with_runnable(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        target: ClassInstanceRef<Runnable>,
    ) -> Result<()> {
        tracing::debug!("java.lang.Thread::<init>({:?}, {:?})", &this, &target);

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        jvm.put_field(&mut this, "target", "Ljava/lang/Runnable;", target).await?;

        Ok(())
    }

    async fn init_internal(jvm: &Jvm, context: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, internal: bool) -> Result<()> {
        tracing::debug!("java.lang.Thread::<init>({:?}, {:?})", &this, internal);

        let id = context.current_task_id();
        jvm.put_field(&mut this, "id", "J", id as i64).await?;

        Ok(())
    }

    async fn start(jvm: &Jvm, context: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.lang.Thread::start({:?})", &this);

        struct ThreadStartProxy {
            jvm: Jvm,
            thread_id: i32,
            this: ClassInstanceRef<Thread>,
        }

        #[async_trait::async_trait]
        impl SpawnCallback for ThreadStartProxy {
            #[tracing::instrument(name = "java thread", fields(id = self.thread_id), skip_all)]
            async fn call(&self) -> Result<()> {
                tracing::trace!("Thread start");

                self.jvm.attach_thread()?;

                let result: Result<()> = self.jvm.invoke_virtual(&self.this, "run", "()V", []).await;

                if let Err(jvm::JavaError::JavaException(x)) = result {
                    let string_writer = self.jvm.new_class("java/io/StringWriter", "()V", ()).await.unwrap();
                    let print_writer = self
                        .jvm
                        .new_class("java/io/PrintWriter", "(Ljava/io/Writer;)V", (string_writer.clone(),))
                        .await
                        .unwrap();

                    let _: () = self
                        .jvm
                        .invoke_virtual(&x, "printStackTrace", "(Ljava/io/PrintWriter;)V", (print_writer,))
                        .await
                        .unwrap();

                    let trace = self
                        .jvm
                        .invoke_virtual(&string_writer, "toString", "()Ljava/lang/String;", [])
                        .await
                        .unwrap();

                    tracing::error!(
                        "Uncaught exception in thread {}:\n{}",
                        self.thread_id,
                        JavaLangString::to_rust_string(&self.jvm, &trace).await.unwrap()
                    );
                } else {
                    result?;
                }

                self.jvm.detach_thread()?;

                let mut this = self.this.clone();
                self.jvm.put_field(&mut this, "alive", "Z", false).await.unwrap();
                self.jvm.object_notify(&self.this, usize::MAX);

                Ok(())
            }
        }

        jvm.put_field(&mut this, "alive", "Z", true).await?;

        let id: i32 = jvm.invoke_virtual(&this, "hashCode", "()I", ()).await?;

        context.spawn(
            jvm,
            Box::new(ThreadStartProxy {
                jvm: jvm.clone(),
                thread_id: id,
                this: this.clone(),
            }),
        );

        Ok(())
    }

    async fn run(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.lang.Thread::run({:?})", &this);

        let target: ClassInstanceRef<Runnable> = jvm.get_field(&this, "target", "Ljava/lang/Runnable;").await?;
        if !target.is_null() {
            let _: () = jvm.invoke_virtual(&target, "run", "()V", ()).await?;
        }

        Ok(())
    }

    async fn join(jvm: &Jvm, _context: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.lang.Thread::join({:?})", &this);

        loop {
            let listener = jvm.object_listen(&this);
            let alive: bool = jvm.get_field(&this, "alive", "Z").await?;
            if !alive {
                return Ok(());
            }
            listener.await;
        }
    }

    async fn is_alive(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::debug!("java.lang.Thread::isAlive({:?})", &this);
        let alive: bool = jvm.get_field(&this, "alive", "Z").await?;
        Ok(alive)
    }

    async fn sleep(_: &Jvm, context: &mut RuntimeContext, duration: i64) -> Result<()> {
        tracing::debug!("java.lang.Thread::sleep({:?})", duration);

        context.sleep(Duration::from_millis(duration as _)).await;

        Ok(())
    }

    async fn r#yield(_: &Jvm, context: &mut RuntimeContext) -> Result<()> {
        tracing::debug!("java.lang.Thread::yield()");
        context.r#yield().await;

        Ok(())
    }

    async fn set_priority(_: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Thread>, new_priority: i32) -> Result<()> {
        tracing::warn!("stub java.lang.Thread::setPriority({:?}, {:?})", &this, new_priority);

        Ok(())
    }

    async fn current_thread(jvm: &Jvm, _: &mut RuntimeContext) -> Result<ClassInstanceRef<Self>> {
        tracing::warn!("stub java.lang.Thread::currentThread()");

        let thread = jvm.new_class("java/lang/Thread", "(Z)V", (true,)).await?;

        Ok(thread.into())
    }
}
