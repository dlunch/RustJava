use alloc::{vec, vec::Vec};

use java_class_proto::{JavaFieldProto, JavaMethodProto, JavaResult};
use java_constants::FieldAccessFlags;
use jvm::{runtime::JavaLangClass, Array, ClassInstanceRef, Jvm};

use crate::{
    classes::java::lang::{Class, ClassLoader, String},
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
            fields: vec![JavaFieldProto::new("classes", "[Ljava/lang/Class;", FieldAccessFlags::STATIC)],
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, parent: ClassInstanceRef<ClassLoader>) -> JavaResult<()> {
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
    ) -> JavaResult<ClassInstanceRef<Class>> {
        tracing::debug!("rustjava.RuntimeClassLoader::findClass({:?}, {:?})", &this, name);

        let name = String::to_rust_string(jvm, &name)?;

        let java_classes_array: ClassInstanceRef<Array<Class>> = jvm
            .get_static_field("rustjava/RuntimeClassLoader", "classes", "[Ljava/lang/Class;")
            .await?;
        // can be null before initialization
        if !java_classes_array.is_null() {
            let java_classes: Vec<ClassInstanceRef<Class>> = jvm.load_array(&java_classes_array, 0, jvm.array_length(&java_classes_array)?)?;
            for java_class in java_classes {
                let rust_class = JavaLangClass::to_rust_class(jvm, java_class.clone())?; // TODO we can use class.name()
                if rust_class.name() == name {
                    return Ok(java_class);
                }
            }
        }

        Ok(None.into())
    }
}
