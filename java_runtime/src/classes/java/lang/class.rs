use alloc::vec;

use bytemuck::cast_vec;

use java_class_proto::{JavaMethodProto, JavaResult};
use jvm::{ClassInstanceRef, Jvm};

use crate::{
    classes::java::{io::InputStream, lang::String},
    RuntimeClassProto, RuntimeContext,
};

// class java.lang.Class
pub struct Class {}

impl Class {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new(
                    "getResourceAsStream",
                    "(Ljava/lang/String;)Ljava/io/InputStream;",
                    Self::get_resource_as_stream,
                    Default::default(),
                ),
            ],
            fields: vec![],
        }
    }

    async fn init(_: &mut Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> JavaResult<()> {
        tracing::warn!("stub java.lang.Class::<init>({:?})", &this);

        Ok(())
    }

    #[allow(clippy::await_holding_refcell_ref)] // We manually drop Ref https://github.com/rust-lang/rust-clippy/issues/6353
    async fn get_resource_as_stream(
        jvm: &mut Jvm,
        context: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        name: ClassInstanceRef<String>,
    ) -> JavaResult<ClassInstanceRef<InputStream>> {
        let name = String::to_rust_string(jvm, &name)?;
        tracing::debug!("java.lang.Class::getResourceAsStream({:?}, {})", &this, name);

        let normalized_name = if let Some(x) = name.strip_prefix('/') { x } else { &name };

        let resource = context.load_resource(normalized_name);
        if let Some(resource) = resource {
            let mut array = jvm.instantiate_array("B", resource.len() as _).await?;

            jvm.store_byte_array(&mut array, 0, cast_vec(resource))?;

            let result = jvm.new_class("java/io/ByteArrayInputStream", "([B)V", (array,)).await?;

            Ok(result.into())
        } else {
            Ok(None.into())
        }
    }
}
