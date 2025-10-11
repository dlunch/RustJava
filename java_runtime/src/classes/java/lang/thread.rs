use alloc::{boxed::Box, sync::Arc, vec};
use core::time::Duration;

use event_listener::Event;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::MethodAccessFlags;
use jvm::{Array, ClassInstanceRef, Jvm, Result};

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
                JavaFieldProto::new("joinEvent", "[B", Default::default()),
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
            join_event: Arc<Event>,
            this: ClassInstanceRef<Thread>,
        }

        #[async_trait::async_trait]
        impl SpawnCallback for ThreadStartProxy {
            #[tracing::instrument(name = "java thread", fields(id = self.thread_id), skip_all)]
            async fn call(&self) -> Result<()> {
                tracing::trace!("Thread start");

                self.jvm.attach_thread()?;

                let _: () = self.jvm.invoke_virtual(&self.this, "run", "()V", []).await?;

                self.jvm.detach_thread()?;

                self.join_event.notify(usize::MAX);

                Ok(())
            }
        }

        let join_event = Arc::new(Event::new());
        jvm.put_rust_object_field(&mut this, "joinEvent", join_event.clone()).await?;

        let id: i32 = jvm.invoke_virtual(&this, "hashCode", "()I", ()).await?;

        context.spawn(
            jvm,
            Box::new(ThreadStartProxy {
                jvm: jvm.clone(),
                thread_id: id,
                join_event,
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

    async fn join(jvm: &Jvm, _context: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.lang.Thread::join({:?})", &this);

        // TODO we don't have get same field twice
        let raw_join_event: ClassInstanceRef<Array<i8>> = jvm.get_field(&this, "joinEvent", "[B").await?;
        if raw_join_event.is_null() {
            return Ok(()); // already joined or not started
        }

        let join_event: Arc<Event> = jvm.get_rust_object_field(&this, "joinEvent").await?;
        join_event.listen().await;

        jvm.put_field(&mut this, "joinEvent", "[B", None).await?;

        Ok(())
    }

    async fn is_alive(_jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::warn!("stub java.lang.Thread::isAlive({:?})", &this);

        Ok(true)
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
