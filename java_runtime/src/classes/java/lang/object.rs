use core::{
    hash::{Hash, Hasher},
    time::Duration,
};

use alloc::{boxed::Box, format, vec};

use dyn_clone::clone_box;
use java_class_proto::JavaMethodProto;
use java_constants::MethodAccessFlags;
use jvm::{ClassInstance, ClassInstanceRef, Jvm, MonitorWaitTimeout, Result, runtime::JavaLangString};

use crate::{Runtime, RuntimeClassProto, RuntimeContext, SpawnCallback, classes::java::lang::String};

// class java.lang.Object
pub struct Object;

const FNV_OFFSET_BASIS_64: u64 = 0xcbf29ce484222325;
const FNV_PRIME_64: u64 = 0x100000001b3;

#[derive(Default)]
struct IdentityHasher(u64);

impl Hasher for IdentityHasher {
    fn finish(&self) -> u64 {
        self.0
    }

    fn write(&mut self, bytes: &[u8]) {
        let mut hash = if self.0 == 0 { FNV_OFFSET_BASIS_64 } else { self.0 };
        for byte in bytes {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(FNV_PRIME_64);
        }
        self.0 = hash;
    }
}

impl Object {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/lang/Object",
            parent_class: None,
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("getClass", "()Ljava/lang/Class;", Self::get_class, Default::default()),
                JavaMethodProto::new("hashCode", "()I", Self::hash_code, Default::default()),
                JavaMethodProto::new("equals", "(Ljava/lang/Object;)Z", Self::equals, Default::default()),
                JavaMethodProto::new("clone", "()Ljava/lang/Object;", Self::clone, MethodAccessFlags::NATIVE),
                JavaMethodProto::new("toString", "()Ljava/lang/String;", Self::to_string, Default::default()),
                JavaMethodProto::new("notify", "()V", Self::notify, Default::default()),
                JavaMethodProto::new("notifyAll", "()V", Self::notify_all, Default::default()),
                JavaMethodProto::new("wait", "(J)V", Self::wait_long, Default::default()),
                JavaMethodProto::new("wait", "(JI)V", Self::wait_long_int, Default::default()),
                JavaMethodProto::new("wait", "()V", Self::wait, Default::default()),
                JavaMethodProto::new("finalize", "()V", Self::finalize, Default::default()),
            ],
            fields: vec![],
            access_flags: Default::default(),
        }
    }

    async fn init(_: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.lang.Object::<init>({this:?})");

        Ok(())
    }

    async fn get_class(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.Object::getClass({this:?})");

        // TODO can we get class directly?
        let this: Box<dyn ClassInstance> = this.into();
        let class_name = this.class_definition().name();

        let class = jvm.resolve_class(&class_name).await?.java_class();

        Ok(class.into())
    }

    async fn hash_code(_: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.lang.Object::hashCode({this:?})");

        let rust_this: Box<dyn ClassInstance> = this.into();

        let mut hasher = IdentityHasher::default();
        rust_this.hash(&mut hasher);
        let hash = hasher.finish();

        Ok(hash as _)
    }

    async fn equals(_: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::debug!("java.lang.Object::equals({this:?}, {other:?})");

        if other.is_null() {
            return Ok(false);
        }

        let rust_this: Box<dyn ClassInstance> = this.into();
        let rust_other: Box<dyn ClassInstance> = other.into();

        rust_this.equals(&*rust_other)
    }

    async fn clone(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.Object::clone({this:?})");

        if !jvm.is_instance(&**this, "java/lang/Cloneable") {
            return Err(jvm.exception("java/lang/CloneNotSupportedException", "Cannot clone this object").await);
        }

        Ok(jvm.shallow_clone(&this)?.into())
    }

    async fn to_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.lang.Object::toString({this:?})");

        let class = jvm.invoke_virtual(&this, "getClass", "()Ljava/lang/Class;", ()).await?;
        let class_name = jvm.invoke_virtual(&class, "getName", "()Ljava/lang/String;", ()).await?;
        let class_name_rust = JavaLangString::to_rust_string(jvm, &class_name).await?;

        let hash_code: i32 = jvm.invoke_virtual(&this, "hashCode", "()I", ()).await?;

        let result = format!("{class_name_rust}@{hash_code:x}");

        Ok(JavaLangString::from_rust_string(jvm, &result).await?.into())
    }

    async fn notify(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.lang.Object::notify({this:?})");

        jvm.object_notify(&this, 1).await?;

        Ok(())
    }

    async fn notify_all(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.lang.Object::notifyAll({this:?})");

        jvm.object_notify(&this, usize::MAX).await?;

        Ok(())
    }

    async fn wait_long(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, millis: i64) -> Result<()> {
        tracing::debug!("java.lang.Object::wait({this:?}, {millis:?})");

        let _: () = jvm.invoke_virtual(&this, "wait", "(JI)V", (millis, 0)).await?;

        Ok(())
    }

    async fn wait_long_int(jvm: &Jvm, context: &mut RuntimeContext, this: ClassInstanceRef<Self>, millis: i64, nanos: i32) -> Result<()> {
        tracing::debug!("java.lang.Object::wait({this:?}, {millis:?}, {nanos:?})");

        if millis < 0 || !(0..=999_999).contains(&nanos) {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "invalid wait timeout").await);
        }

        struct TimeoutNotifier {
            timeout: u64,
            waiter: MonitorWaitTimeout,
            context: Box<dyn Runtime>,
        }

        #[async_trait::async_trait]
        impl SpawnCallback for TimeoutNotifier {
            async fn call(&self) -> Result<()> {
                self.context.sleep(Duration::from_millis(self.timeout)).await;
                self.waiter.clone().notify();
                Ok(())
            }
        }

        let (waiter, timeout_notifier) = jvm.object_wait_prepare(&this).await?;
        let timeout = millis as u64 + u64::from(nanos > 0);
        if timeout != 0 {
            context.spawn(
                jvm,
                Box::new(TimeoutNotifier {
                    timeout,
                    waiter: timeout_notifier,
                    context: clone_box(context),
                }),
            );
        }

        jvm.object_wait(waiter).await?;

        Ok(())
    }

    async fn wait(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.lang.Object::wait({this:?})");

        let _: () = jvm.invoke_virtual(&this, "wait", "(JI)V", (0i64, 0)).await?;

        Ok(())
    }

    async fn finalize(_: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::warn!("stub java.lang.Object::finalize({this:?})");

        Ok(())
    }
}
