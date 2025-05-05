use alloc::vec;

use jvm::{Result, runtime::JavaLangString};

use test_utils::test_jvm_filesystem;

#[tokio::test]
async fn test_zip_entry() -> Result<()> {
    let jar = include_bytes!("../../../../../../test_data/test.jar");
    let filesystem = [("test.jar".into(), jar.to_vec())].into_iter().collect();
    let jvm = test_jvm_filesystem(filesystem).await?;

    let name = JavaLangString::from_rust_string(&jvm, "test.jar").await?;
    let file = jvm.new_class("java/io/File", "(Ljava/lang/String;)V", (name,)).await?;
    let zip = jvm.new_class("java/util/zip/ZipFile", "(Ljava/io/File;)V", (file,)).await?;

    let entry_name = JavaLangString::from_rust_string(&jvm, "test.txt").await?;
    let entry = jvm
        .invoke_virtual(&zip, "getEntry", "(Ljava/lang/String;)Ljava/util/zip/ZipEntry;", (entry_name,))
        .await?;

    let size: i64 = jvm.invoke_virtual(&entry, "getSize", "()J", ()).await?;

    let is = jvm
        .invoke_virtual(&zip, "getInputStream", "(Ljava/util/zip/ZipEntry;)Ljava/io/InputStream;", (entry,))
        .await?;

    let buf = jvm.instantiate_array("B", size as _).await?;
    let _: i32 = jvm.invoke_virtual(&is, "read", "([B)I", (buf.clone(),)).await?;

    let mut data = vec![0; size as _];
    jvm.array_raw_buffer(&buf).await?.read(0, &mut data).unwrap();
    assert_eq!(data, b"test content\n".to_vec());

    Ok(())
}
