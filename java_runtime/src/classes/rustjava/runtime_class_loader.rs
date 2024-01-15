use alloc::{boxed::Box, vec, vec::Vec};

use java_class_proto::{JavaFieldProto, JavaMethodProto, JavaResult};
use java_constants::FieldAccessFlags;
use jvm::{Array, Class as JvmClass, ClassInstanceRef, JavaValue, Jvm};

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

    async fn init(jvm: &mut Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, parent: ClassInstanceRef<ClassLoader>) -> JavaResult<()> {
        tracing::debug!("rustjava.RuntimeClassLoader::<init>({:?}, {:?})", &this, &parent);

        jvm.invoke_special(&this, "java/lang/ClassLoader", "<init>", "(Ljava/lang/ClassLoader;)V", (parent,))
            .await?;

        Ok(())
    }

    async fn find_class(
        jvm: &mut Jvm,
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
                let rust_class = Class::to_rust_class(jvm, java_class.clone().into())?; // TODO we can use class.name()
                if rust_class.name() == name {
                    return Ok(java_class);
                }
            }
        }

        Ok(None.into())
    }

    // TODO load class on demand
    pub async fn initialize(jvm: &mut Jvm, classes: Vec<Box<dyn JvmClass>>) -> JavaResult<()> {
        let mut java_classes: Vec<JavaValue> = Vec::with_capacity(classes.len());

        for class in classes {
            java_classes.push(Class::from_rust_class(jvm, class).await?.into());
        }

        let mut java_classes_array = jvm.instantiate_array("Ljava/lang/Class;", java_classes.len() as _).await?;
        jvm.store_array(&mut java_classes_array, 0, java_classes)?;

        jvm.put_static_field("rustjava/RuntimeClassLoader", "classes", "[Ljava/lang/Class;", java_classes_array)
            .await?;

        Ok(())
    }
}
