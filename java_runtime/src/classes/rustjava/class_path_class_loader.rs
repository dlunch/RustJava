use alloc::{vec, vec::Vec};

use bytemuck::cast_slice;

use java_class_proto::{JavaFieldProto, JavaMethodProto, JavaResult};
use jvm::{Array, ClassInstanceRef, Jvm};

use crate::{
    classes::{
        java::lang::{Class, ClassLoader, String},
        rustjava::ClassPathEntry,
    },
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
            fields: vec![JavaFieldProto::new("entries", "[Lrustjava/ClassPathEntry;", Default::default())],
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, parent: ClassInstanceRef<ClassLoader>) -> JavaResult<()> {
        tracing::debug!("rustjava.ClassPathClassLoader::<init>({:?}, {:?})", &this, &parent);

        jvm.invoke_special(&this, "java/lang/ClassLoader", "<init>", "(Ljava/lang/ClassLoader;)V", (parent,))
            .await?;

        let entries = jvm.instantiate_array("Lrustjava/ClassPathEntry;", 0).await?;
        jvm.put_field(&mut this, "entries", "[Lrustjava/ClassPathEntry;", entries)?;

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

        let entries: ClassInstanceRef<Array<ClassPathEntry>> = jvm.get_field(&this, "entries", "[Lrustjava/ClassPathEntry;")?;

        let entries = jvm.load_array(&entries, 0, jvm.array_length(&entries)?)?;
        for entry in entries {
            let entry_name = ClassPathEntry::name(jvm, &entry)?;

            if name == entry_name {
                let data = ClassPathEntry::data(jvm, &entry)?;
                let rust_class = jvm.define_class(&name, cast_slice(&data)).await?;

                let java_class = Class::from_rust_class(jvm, rust_class).await?;

                return Ok(java_class);
            }
        }

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

        let entry = jvm
            .new_class("rustjava/ClassPathEntry", "(Ljava/lang/String;[B)V", (file_name, data))
            .await?;

        let entries = jvm.get_field(&this, "entries", "[Lrustjava/ClassPathEntry;")?;

        let length = jvm.array_length(&entries)?;
        let mut entries: Vec<ClassInstanceRef<ClassPathEntry>> = jvm.load_array(&entries, 0, length)?;

        entries.push(entry.into());

        let mut new_entries = jvm.instantiate_array("Ljava/lang/String;", length + 1).await?;
        jvm.store_array(&mut new_entries, 0, entries)?;
        jvm.put_field(&mut this, "entries", "[Lrustjava/ClassPathEntry;", new_entries)?;

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
