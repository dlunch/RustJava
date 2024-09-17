use alloc::vec;

use java_class_proto::JavaMethodProto;
use jvm::{Array, ClassInstanceRef, JavaChar, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext};

// class java.io.Writer
pub struct Writer {}

impl Writer {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/io/Writer",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new_abstract("write", "([CII)I", Default::default()),
                JavaMethodProto::new("write", "(Ljava/lang/String;)V", Self::write_string, Default::default()),
            ],
            fields: vec![],
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.io.Writer::<init>({:?})", &this);

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        Ok(())
    }

    async fn write_string(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        string: ClassInstanceRef<crate::classes::java::lang::String>,
    ) -> Result<()> {
        tracing::debug!("java.io.Writer::write_string({:?}, {:?})", &this, &string);

        let chars: ClassInstanceRef<Array<JavaChar>> = jvm.invoke_virtual(&string, "toCharArray", "()[C", ()).await?;
        let length = jvm.array_length(&chars).await?;

        let _: i32 = jvm.invoke_virtual(&this, "write", "([CII)I", (chars, 0, length as i32)).await?;

        Ok(())
    }
}
