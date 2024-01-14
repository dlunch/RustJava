use alloc::vec;

use java_class_proto::{JavaMethodProto, JavaResult};
use jvm::{ClassInstanceRef, Jvm};

use crate::{
    classes::java::lang::{Class, ClassLoader, String},
    get_class_proto, RuntimeClassProto, RuntimeContext,
};

// class rustjava.RuntimeClassLoader
pub struct RuntimeClassLoader {}

impl RuntimeClassLoader {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            parent_class: Some("java/lang/ClassLoader"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/lang/ClassLoader;)V", Self::init, Default::default()),
                JavaMethodProto::new("findClass", "(Ljava/lang/String;)Ljava/lang/Class;", Self::find_class, Default::default()),
            ],
            fields: vec![],
        }
    }

    async fn init(jvm: &mut Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, parent: ClassInstanceRef<ClassLoader>) -> JavaResult<()> {
        tracing::debug!("rustjava.RuntimeClassLoader::<init>({:?}, {:?})", &this, &parent);

        jvm.invoke_special(&this, "java/lang/ClassLoader", "<init>", "(Ljava/lang/ClassLoader;)V", (parent,))
            .await?;

        Ok(())
    }

    async fn find_class(
        jvm: &mut Jvm,
        runtime: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        name: ClassInstanceRef<String>,
    ) -> JavaResult<ClassInstanceRef<Class>> {
        tracing::debug!("rustjava.RuntimeClassLoader::findClass({:?}, {:?})", &this, name);

        let rust_name = String::to_rust_string(jvm, &name)?;
        let proto = get_class_proto(&rust_name);
        if proto.is_none() {
            return Ok(None.into());
        }

        let proto = proto.unwrap();
        let _rust_class = runtime.define_class_proto(&rust_name, proto);

        // TODO proper java/lang/class creation

        let java_class = jvm.new_class("java/lang/Class", "()V", ()).await?;

        Ok(java_class.into())
    }
}
