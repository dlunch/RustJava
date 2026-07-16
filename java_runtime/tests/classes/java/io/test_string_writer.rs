use alloc::vec;

use java_runtime::classes::java::lang::Object;
use jvm::{ClassInstanceRef, JavaChar, JavaError, Result, runtime::JavaLangString};

use test_utils::test_jvm;

#[tokio::test]
async fn test_string_writer() -> Result<()> {
    let jvm = test_jvm().await?;

    let string_writer = jvm.new_class("java/io/StringWriter", "()V", ()).await.unwrap();

    let mut buf = jvm.instantiate_array("C", 3).await?;

    jvm.store_array(&mut buf, 0, vec![b'a' as JavaChar, b'b' as JavaChar, b'c' as JavaChar])
        .await?;

    let _: () = jvm.invoke_virtual(&string_writer, "write", "([CII)V", (buf.clone(), 0, 3)).await?;

    let _: () = jvm.invoke_virtual(&string_writer, "write", "([CII)V", (buf.clone(), 1, 2)).await?;
    let _: () = jvm.invoke_virtual(&string_writer, "write", "(I)V", ('d' as i32,)).await?;
    let _: () = jvm.invoke_virtual(&string_writer, "write", "([C)V", (buf,)).await?;

    let value = JavaLangString::from_rust_string(&jvm, "XYZ").await?;
    let _: () = jvm
        .invoke_virtual(&string_writer, "write", "(Ljava/lang/String;)V", (value.clone(),))
        .await?;
    let _: () = jvm
        .invoke_virtual(&string_writer, "write", "(Ljava/lang/String;II)V", (value, 1, 1))
        .await?;
    let _: () = jvm.invoke_virtual(&string_writer, "flush", "()V", ()).await?;
    let _: () = jvm.invoke_virtual(&string_writer, "close", "()V", ()).await?;

    let string = jvm.invoke_virtual(&string_writer, "toString", "()Ljava/lang/String;", ()).await.unwrap();

    let string = JavaLangString::to_rust_string(&jvm, &string).await?;

    assert_eq!(string, "abcbcdabcXYZY"); // cspell: disable-line

    let null_lock: ClassInstanceRef<Object> = None.into();
    let result: Result<()> = jvm
        .invoke_special(&string_writer, "java/io/Writer", "<init>", "(Ljava/lang/Object;)V", (null_lock,))
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("null lock must throw NullPointerException");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));

    Ok(())
}
