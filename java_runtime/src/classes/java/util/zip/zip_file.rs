use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{classes::java::io::File, RuntimeClassProto, RuntimeContext};

// class java.util.zip.ZipFile
pub struct ZipFile {}

impl ZipFile {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![JavaMethodProto::new("<init>", "(Ljava/io/File;)V", Self::init, Default::default())],
            fields: vec![JavaFieldProto::new("buf", "[B", Default::default())],
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, file: ClassInstanceRef<File>) -> Result<()> {
        tracing::debug!("java.util.zip.ZipFile::<init>({:?}, {:?})", &this, &file,);

        jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        let length: i64 = jvm.invoke_virtual(&file, "length", "()J", ()).await?;
        let is = jvm.new_class("java/io/FileInputStream", "(Ljava/io/File;)V", (file,)).await?;

        let buf = jvm.instantiate_array("B", length as _).await?;
        let _: i32 = jvm.invoke_virtual(&is, "read", "([B)I", (buf.clone(),)).await?;

        jvm.put_field(&mut this, "buf", "[B", buf).await?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use jvm::{runtime::JavaLangString, Result};

    use crate::test::test_jvm_filesystem;

    #[futures_test::test]
    async fn test_zip() -> Result<()> {
        let jar = include_bytes!("../../../../../../test_data/test.jar");
        let filesystem = [("test.jar".into(), jar.to_vec())].into_iter().collect();
        let jvm = test_jvm_filesystem(filesystem).await?;

        let name = JavaLangString::from_rust_string(&jvm, "test.jar").await?;
        let file = jvm.new_class("java/io/File", "(Ljava/lang/String;)V", (name,)).await?;
        let _ = jvm.new_class("java/util/zip/ZipFile", "(Ljava/io/File;)V", (file,)).await?;

        Ok(())
    }
}
