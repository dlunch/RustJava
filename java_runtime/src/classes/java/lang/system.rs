use alloc::{vec, vec::Vec};

use jvm::JavaValue;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{FieldAccessFlags, MethodAccessFlags};
use jvm::{Array, ClassInstanceRef, Jvm, JvmResult};

use crate::{RuntimeClassProto, RuntimeContext};

// class java.lang.System
pub struct System {}

impl System {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<clinit>", "()V", Self::cl_init, MethodAccessFlags::STATIC),
                JavaMethodProto::new(
                    "currentTimeMillis",
                    "()J",
                    Self::current_time_millis,
                    MethodAccessFlags::NATIVE | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new("gc", "()V", Self::gc, MethodAccessFlags::STATIC),
                JavaMethodProto::new(
                    "arraycopy",
                    "(Ljava/lang/Object;ILjava/lang/Object;II)V",
                    Self::arraycopy,
                    MethodAccessFlags::NATIVE | MethodAccessFlags::STATIC,
                ),
            ],
            fields: vec![JavaFieldProto::new("out", "Ljava/io/PrintStream;", FieldAccessFlags::STATIC)],
        }
    }

    async fn cl_init(jvm: &Jvm, _: &mut RuntimeContext) -> JvmResult<()> {
        tracing::debug!("java.lang.System::<clinit>()");

        let out = jvm.new_class("java/io/PrintStream", "()V", []).await?;
        // TODO call constructor with dummy output stream?

        jvm.put_static_field("java/lang/System", "out", "Ljava/io/PrintStream;", out).await?;

        Ok(())
    }

    async fn current_time_millis(_: &Jvm, context: &mut RuntimeContext) -> JvmResult<i64> {
        tracing::debug!("java.lang.System::currentTimeMillis()");

        Ok(context.now() as _)
    }

    async fn gc(_: &Jvm, _: &mut RuntimeContext) -> JvmResult<i32> {
        tracing::warn!("stub java.lang.System::gc()");

        Ok(0)
    }

    async fn arraycopy(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        src: ClassInstanceRef<Array<()>>, // Any Array
        src_pos: i32,
        mut dest: ClassInstanceRef<Array<()>>,
        dest_pos: i32,
        length: i32,
    ) -> JvmResult<()> {
        tracing::debug!(
            "java.lang.System::arraycopy({:?}, {}, {:?}, {}, {})",
            &src,
            src_pos,
            &dest,
            dest_pos,
            length
        );

        // TODO i think we can make it faster
        let src: Vec<JavaValue> = jvm.load_array(&src, src_pos as _, length as _)?;
        jvm.store_array(&mut dest, dest_pos as _, src)?;

        Ok(())
    }
}
