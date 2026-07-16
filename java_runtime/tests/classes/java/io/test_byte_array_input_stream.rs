use jvm::{JavaError, Result};

use test_utils::test_jvm;

#[tokio::test]
async fn test_mark_reset() -> Result<()> {
    let jvm = test_jvm().await?;

    let mut buffer = jvm.instantiate_array("B", 5).await?;
    jvm.array_raw_buffer_mut(&mut buffer).await?.write(0, &[10, 20, 30, 40, 50])?;

    let stream = jvm.new_class("java/io/ByteArrayInputStream", "([B)V", (buffer,)).await?;

    let first: i32 = jvm.invoke_virtual(&stream, "read", "()I", ()).await?;
    assert_eq!(first, 10);

    let _: () = jvm.invoke_virtual(&stream, "mark", "(I)V", (100,)).await?;

    let second: i32 = jvm.invoke_virtual(&stream, "read", "()I", ()).await?;
    assert_eq!(second, 20);

    let _: () = jvm.invoke_virtual(&stream, "reset", "()V", ()).await?;

    let again: i32 = jvm.invoke_virtual(&stream, "read", "()I", ()).await?;
    assert_eq!(again, 20);

    Ok(())
}

#[tokio::test]
async fn test_input_stream_default_contract() -> Result<()> {
    let jvm = test_jvm().await?;

    let mut data = jvm.instantiate_array("B", 3).await?;
    jvm.store_array(&mut data, 0, [10i8, 20, 30]).await?;
    let stream = jvm.new_class("java/io/ByteArrayInputStream", "([B)V", (data,)).await?;
    let target = jvm.instantiate_array("B", 5).await?;

    assert_eq!(
        jvm.invoke_special::<_, i32>(&stream, "java/io/InputStream", "read", "([BII)I", (target.clone(), 1, 3))
            .await?,
        3
    );
    assert_eq!(jvm.load_array::<i8>(&target, 0, 5).await?, [0, 10, 20, 30, 0]);
    assert_eq!(
        jvm.invoke_special::<_, i32>(&stream, "java/io/InputStream", "read", "([BII)I", (target.clone(), 0, 1))
            .await?,
        -1
    );
    assert_eq!(
        jvm.invoke_special::<_, i32>(&stream, "java/io/InputStream", "read", "([BII)I", (target.clone(), 0, 0))
            .await?,
        0
    );

    let invalid: Result<i32> = jvm
        .invoke_special(&stream, "java/io/InputStream", "read", "([BII)I", (target, -1, 1))
        .await;
    let Err(JavaError::JavaException(exception)) = invalid else {
        panic!("invalid range must throw IndexOutOfBoundsException");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/IndexOutOfBoundsException"));

    let mut data = jvm.instantiate_array("B", 3).await?;
    jvm.store_array(&mut data, 0, [1i8, 2, 3]).await?;
    let stream = jvm.new_class("java/io/ByteArrayInputStream", "([B)V", (data,)).await?;
    assert_eq!(
        jvm.invoke_special::<_, i64>(&stream, "java/io/InputStream", "skip", "(J)J", (2i64,))
            .await?,
        2
    );
    assert_eq!(
        jvm.invoke_special::<_, i64>(&stream, "java/io/InputStream", "skip", "(J)J", (5i64,))
            .await?,
        1
    );
    assert_eq!(
        jvm.invoke_special::<_, i64>(&stream, "java/io/InputStream", "skip", "(J)J", (-1i64,))
            .await?,
        0
    );
    assert_eq!(
        jvm.invoke_special::<_, i32>(&stream, "java/io/InputStream", "available", "()I", ())
            .await?,
        0
    );
    assert!(
        !jvm.invoke_special::<_, bool>(&stream, "java/io/InputStream", "markSupported", "()Z", ())
            .await?
    );
    let _: () = jvm.invoke_special(&stream, "java/io/InputStream", "mark", "(I)V", (10,)).await?;

    let reset: Result<()> = jvm.invoke_special(&stream, "java/io/InputStream", "reset", "()V", ()).await;
    let Err(JavaError::JavaException(exception)) = reset else {
        panic!("default reset must throw IOException");
    };
    assert!(jvm.is_instance(&*exception, "java/io/IOException"));
    let _: () = jvm.invoke_special(&stream, "java/io/InputStream", "close", "()V", ()).await?;

    Ok(())
}
