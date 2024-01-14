use alloc::vec;

use bytemuck::cast_slice;

use java_class_proto::{JavaFieldProto, JavaMethodProto, JavaResult};
use jvm::{Array, ClassInstanceRef, Jvm};

use crate::{
    classes::java::lang::{Class, ClassLoader, String},
    RuntimeClassProto, RuntimeContext,
};

// class rustjava.ClassPathClassLoader
pub struct ClassPathClassLoader {}

impl ClassPathClassLoader {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            parent_class: Some("java/lang/ClassLoader"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/lang/ClassLoader;)V", Self::init, Default::default()),
                JavaMethodProto::new("findClass", "(Ljava/lang/String;)Ljava/lang/Class;", Self::find_class, Default::default()),
                JavaMethodProto::new("addClassFile", "(Ljava/lang/String;[B)V", Self::add_class_file, Default::default()),
                JavaMethodProto::new("addJarFile", "([B)V", Self::add_jar_file, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("class_file_name", "Ljava/lang/String;", Default::default()),
                JavaFieldProto::new("class_file", "[B", Default::default()),
                JavaFieldProto::new("jar_file", "[B", Default::default()),
            ],
        }
    }

    async fn init(jvm: &mut Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, parent: ClassInstanceRef<ClassLoader>) -> JavaResult<()> {
        tracing::debug!("rustjava.ClassPathClassLoader::<init>({:?}, {:?})", &this, &parent);

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
        tracing::debug!("rustjava.ClassPathClassLoader::findClass({:?}, {:?})", &this, name);

        let name = String::to_rust_string(jvm, &name)?;

        let class_file_name: ClassInstanceRef<String> = jvm.get_field(&this, "class_file_name", "Ljava/lang/String;")?;
        if !class_file_name.is_null() {
            let class_file_name = String::to_rust_string(jvm, &class_file_name)?;

            if name == class_file_name {
                let class_file = jvm.get_field(&this, "class_file", "[B")?;
                let length = jvm.array_length(&class_file)?;

                let class_file_data = jvm.load_byte_array(&class_file, 0, length)?;

                let rust_class = jvm.define_class(&name, cast_slice(&class_file_data))?;

                let java_class = Class::from_rust_class(jvm, rust_class).await?;

                return Ok(java_class);
            }
        }

        let jar_file: ClassInstanceRef<Array<i8>> = jvm.get_field(&this, "jar_file", "[B")?;
        if !jar_file.is_null() {
            // TODO
        }

        Ok(None.into())
    }

    // we don't have classpath (yet), so we need backdoor to add classes to loader
    async fn add_class_file(
        jvm: &mut Jvm,
        _runtime: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        file_name: ClassInstanceRef<String>,
        data: ClassInstanceRef<Array<i8>>,
    ) -> JavaResult<()> {
        tracing::debug!("rustjava.ClassPathClassLoader::addClassFile({:?})", &this);

        jvm.put_field(&mut this, "class_file_name", "Ljava/lang/String;", file_name)?;
        jvm.put_field(&mut this, "class_file", "[B", data)?; // TODO multiple classes

        Ok(())
    }

    async fn add_jar_file(
        jvm: &mut Jvm,
        _runtime: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        data: ClassInstanceRef<Array<i8>>,
    ) -> JavaResult<()> {
        tracing::debug!("rustjava.ClassPathClassLoader::addJarFile({:?})", &this);

        jvm.put_field(&mut this, "jar_file", "[B", data)?; // TODO multiple jars

        Ok(())
    }
}
