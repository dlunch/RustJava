use alloc::{boxed::Box, vec};
use core::mem::{forget, size_of_val};

use bytemuck::{cast_slice, cast_vec};

use java_class_proto::{JavaFieldProto, JavaMethodProto, JavaResult};
use jvm::{Class as JvmClass, ClassInstanceRef, Jvm};

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
            fields: vec![
                JavaFieldProto::new("raw", "[B", Default::default()), // raw rust pointer of Box<dyn Class>
            ],
        }
    }

    async fn init(_: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> JavaResult<()> {
        tracing::debug!("java.lang.Class::<init>({:?})", &this);

        Ok(())
    }

    #[allow(clippy::await_holding_refcell_ref)] // We manually drop Ref https://github.com/rust-lang/rust-clippy/issues/6353
    async fn get_resource_as_stream(
        jvm: &Jvm,
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

    pub async fn from_rust_class(jvm: &Jvm, rust_class: Box<dyn JvmClass>) -> JavaResult<ClassInstanceRef<Self>> {
        let mut java_class = jvm.new_class("java/lang/Class", "()V", ()).await?;

        let rust_class_raw = Box::into_raw(Box::new(rust_class)) as *const u8 as usize;

        let mut raw_storage = jvm.instantiate_array("B", size_of_val(&rust_class_raw)).await?;
        jvm.store_byte_array(&mut raw_storage, 0, cast_slice(&rust_class_raw.to_le_bytes()).to_vec())?;

        jvm.put_field(&mut java_class, "raw", "[B", raw_storage)?;

        Ok(java_class.into())
    }

    pub fn to_rust_class(jvm: &Jvm, java_class: ClassInstanceRef<Self>) -> JavaResult<Box<dyn JvmClass>> {
        let raw_storage = jvm.get_field(&java_class, "raw", "[B")?;
        let raw = jvm.load_byte_array(&raw_storage, 0, jvm.array_length(&raw_storage)?)?;

        let rust_class_raw = usize::from_le_bytes(cast_slice(&raw).try_into().unwrap());

        let rust_class = unsafe { Box::from_raw(rust_class_raw as *mut Box<dyn JvmClass>) };
        let result = (*rust_class).clone();

        forget(rust_class); // do not drop box as we still have it in java memory

        Ok(result)
    }
}

#[cfg(test)]
mod test {
    use crate::test::test_jvm;

    use super::Class;

    #[futures_test::test]
    async fn test_class() -> anyhow::Result<()> {
        let jvm = test_jvm().await?;

        let class = jvm.resolve_class("java/lang/String").await?.unwrap();

        let java_class = Class::from_rust_class(&jvm, class).await?;

        let rust_class = Class::to_rust_class(&jvm, java_class.clone().into())?;
        assert_eq!(rust_class.name(), "java/lang/String");

        // try call to_rust_class twice to test if box is not dropped
        let rust_class = Class::to_rust_class(&jvm, java_class)?;
        assert_eq!(rust_class.name(), "java/lang/String");

        Ok(())
    }
}
