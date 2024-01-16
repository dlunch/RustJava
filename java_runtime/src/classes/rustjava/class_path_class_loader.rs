use alloc::{vec, vec::Vec};

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
                JavaFieldProto::new("class_file_names", "[Ljava/lang/String;", Default::default()),
                JavaFieldProto::new("class_files", "[[B", Default::default()),
                JavaFieldProto::new("jar_files", "[[B", Default::default()),
            ],
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, parent: ClassInstanceRef<ClassLoader>) -> JavaResult<()> {
        tracing::debug!("rustjava.ClassPathClassLoader::<init>({:?}, {:?})", &this, &parent);

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
        tracing::debug!("rustjava.ClassPathClassLoader::findClass({:?}, {:?})", &this, name);

        let name = String::to_rust_string(jvm, &name)?;

        let class_file_names: ClassInstanceRef<Array<String>> = jvm.get_field(&this, "class_file_names", "[Ljava/lang/String;")?;
        if !class_file_names.is_null() {
            let class_file_names = jvm.load_array(&class_file_names, 0, jvm.array_length(&class_file_names)?)?;
            for (i, class_file_name) in class_file_names.iter().enumerate() {
                let class_file_name = String::to_rust_string(jvm, class_file_name)?;

                if name == class_file_name {
                    let class_files = jvm.get_field(&this, "class_files", "[[B")?;
                    let class_file = &jvm.load_array(&class_files, i, 1)?[0];
                    let length = jvm.array_length(class_file)?;

                    let class_file_data = jvm.load_byte_array(class_file, 0, length)?;

                    let rust_class = jvm.define_class(&name, cast_slice(&class_file_data)).await?;

                    let java_class = Class::from_rust_class(jvm, rust_class).await?;

                    return Ok(java_class);
                }
            }
        }

        // TODO jar

        Ok(None.into())
    }

    // we don't have classpath (yet), so we need backdoor to add classes to loader
    async fn add_class_file(
        jvm: &Jvm,
        _runtime: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        file_name: ClassInstanceRef<String>,
        data: ClassInstanceRef<Array<i8>>,
    ) -> JavaResult<()> {
        tracing::debug!("rustjava.ClassPathClassLoader::addClassFile({:?})", &this);

        let class_file_names: ClassInstanceRef<Array<String>> = jvm.get_field(&this, "class_file_names", "[Ljava/lang/String;")?;
        let class_file_names = if class_file_names.is_null() {
            let class_file_names = jvm.instantiate_array("Ljava/lang/String;", 0).await?;
            let class_files = jvm.instantiate_array("[B", 0).await?;
            let jar_files = jvm.instantiate_array("[B", 0).await?;

            jvm.put_field(&mut this, "class_file_names", "[Ljava/lang/String;", class_file_names.clone())?;
            jvm.put_field(&mut this, "class_files", "[[B", class_files)?;
            jvm.put_field(&mut this, "jar_files", "[[B", jar_files)?;

            class_file_names.into()
        } else {
            class_file_names
        };

        let length = jvm.array_length(&class_file_names)?;
        let mut class_file_names: Vec<ClassInstanceRef<String>> = jvm.load_array(&class_file_names, 0, length)?;

        class_file_names.push(file_name);

        let mut new_class_file_names = jvm.instantiate_array("Ljava/lang/String;", length + 1).await?;
        jvm.store_array(&mut new_class_file_names, 0, class_file_names)?;
        jvm.put_field(&mut this, "class_file_names", "[Ljava/lang/String;", new_class_file_names)?;

        let class_files = jvm.get_field(&this, "class_files", "[[B")?;
        let mut class_files: Vec<ClassInstanceRef<Array<i8>>> = jvm.load_array(&class_files, 0, length)?;

        class_files.push(data);

        let mut new_class_files = jvm.instantiate_array("[B", length + 1).await?;
        jvm.store_array(&mut new_class_files, 0, class_files)?;
        jvm.put_field(&mut this, "class_files", "[[B", new_class_files)?;

        Ok(())
    }

    async fn add_jar_file(
        jvm: &Jvm,
        _runtime: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        data: ClassInstanceRef<Array<i8>>,
    ) -> JavaResult<()> {
        tracing::debug!("rustjava.ClassPathClassLoader::addJarFile({:?})", &this);

        jvm.put_field(&mut this, "jar_file", "[B", data)?; // TODO multiple jars

        Ok(())
    }
}
