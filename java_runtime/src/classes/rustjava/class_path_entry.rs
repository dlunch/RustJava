use alloc::{string::String as RustString, vec, vec::Vec};

use bytemuck::cast_vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto, JavaResult};
use jvm::{Array, ClassInstanceRef, Jvm};

use crate::{classes::java::lang::String, RuntimeClassProto, RuntimeContext};

// class rustjava.ClassPathEntry
pub struct ClassPathEntry {}

impl ClassPathEntry {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![JavaMethodProto::new("<init>", "(Ljava/lang/String;[B)V", Self::init, Default::default())],
            fields: vec![
                JavaFieldProto::new("name", "Ljava/lang/String;", Default::default()),
                JavaFieldProto::new("data", "[B", Default::default()),
            ],
        }
    }

    async fn init(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        name: ClassInstanceRef<String>,
        data: ClassInstanceRef<Array<i8>>,
    ) -> JavaResult<()> {
        tracing::debug!("rustjava.ClassPathEntry::<init>({:?}, {:?}, {:?})", &this, &name, &data);

        jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        jvm.put_field(&mut this, "name", "Ljava/lang/String;", name)?;
        jvm.put_field(&mut this, "data", "[B", data)?;

        Ok(())
    }

    pub fn name(jvm: &Jvm, this: &ClassInstanceRef<Self>) -> JavaResult<RustString> {
        let name = jvm.get_field(this, "name", "Ljava/lang/String;")?;

        String::to_rust_string(jvm, &name)
    }

    pub fn data(jvm: &Jvm, this: &ClassInstanceRef<Self>) -> JavaResult<Vec<u8>> {
        let data = jvm.get_field(this, "data", "[B")?;

        Ok(cast_vec(jvm.load_byte_array(&data, 0, jvm.array_length(&data)?)?))
    }

    pub fn data_array(jvm: &Jvm, this: &ClassInstanceRef<Self>) -> JavaResult<ClassInstanceRef<Array<i8>>> {
        jvm.get_field(this, "data", "[B")
    }
}
