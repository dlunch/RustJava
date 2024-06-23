use alloc::{sync::Arc, vec};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::FieldAccessFlags;
use jvm::{
    runtime::{JavaLangClass, JavaLangString},
    ClassInstanceRef, Jvm, Result,
};

use crate::{
    classes::java::lang::{Class, ClassLoader, String},
    loader::{get_proto, ClassCreator},
    RuntimeClassProto, RuntimeContext,
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
            fields: vec![JavaFieldProto::new("classCreator", "[B", FieldAccessFlags::STATIC)],
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, parent: ClassInstanceRef<ClassLoader>) -> Result<()> {
        tracing::debug!("rustjava.RuntimeClassLoader::<init>({:?}, {:?})", &this, &parent);

        jvm.invoke_special(&this, "java/lang/ClassLoader", "<init>", "(Ljava/lang/ClassLoader;)V", (parent,))
            .await?;

        Ok(())
    }

    async fn find_class(
        jvm: &Jvm,
        _runtime: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        name: ClassInstanceRef<String>,
    ) -> Result<ClassInstanceRef<Class>> {
        tracing::debug!("rustjava.RuntimeClassLoader::findClass({:?}, {:?})", &this, name);

        let name = JavaLangString::to_rust_string(jvm, &name).await?;

        let class_creator: Arc<dyn ClassCreator> = jvm.get_rust_object_static_field("rustjava/RuntimeClassLoader", "classCreator").await?;

        let proto = get_proto(&name);
        if proto.is_none() {
            return Ok(None.into());
        }

        let class = class_creator.create_class(&name, proto.unwrap()).await;
        let java_class = JavaLangClass::from_rust_class(jvm, class, Some(this.into())).await?;

        Ok(java_class.into())
    }
}
