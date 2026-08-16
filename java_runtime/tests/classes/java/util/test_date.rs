use java_runtime::classes::java::lang::Object;
use jvm::{ClassInstanceRef, JavaError, Result, runtime::JavaLangString};

use test_utils::test_jvm;

#[tokio::test]
async fn test_date_cldc11_value_contract() -> Result<()> {
    let jvm = test_jvm().await?;
    let epoch = jvm.new_class("java/util/Date", "(J)V", (0i64,)).await?;
    let same = jvm.new_class("java/util/Date", "(J)V", (0i64,)).await?;
    let later = jvm.new_class("java/util/Date", "(J)V", (1i64,)).await?;

    assert!(
        jvm.invoke_virtual::<_, bool>(&epoch, &epoch.class_definition().name(), "equals", "(Ljava/lang/Object;)Z", (same,))
            .await?
    );
    assert!(
        !jvm.invoke_virtual::<_, bool>(&epoch, &epoch.class_definition().name(), "equals", "(Ljava/lang/Object;)Z", (later,))
            .await?
    );
    let null: ClassInstanceRef<Object> = None.into();
    assert!(
        !jvm.invoke_virtual::<_, bool>(&epoch, &epoch.class_definition().name(), "equals", "(Ljava/lang/Object;)Z", (null,))
            .await?
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&epoch, &epoch.class_definition().name(), "hashCode", "()I", ())
            .await?,
        0
    );

    let text: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&epoch, &epoch.class_definition().name(), "toString", "()Ljava/lang/String;", ())
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &text).await?, "Thu Jan 01 00:00:00 GMT 1970");

    Ok(())
}

#[tokio::test]
async fn test_date_01_compare_to_typed_and_object_contract() -> Result<()> {
    let jvm = test_jvm().await?;
    let earlier = jvm.new_class("java/util/Date", "(J)V", (-10i64,)).await?;
    let same = jvm.new_class("java/util/Date", "(J)V", (-10i64,)).await?;
    let later = jvm.new_class("java/util/Date", "(J)V", (20i64,)).await?;

    assert_eq!(
        jvm.invoke_virtual::<_, i32>(
            &earlier,
            &earlier.class_definition().name(),
            "compareTo",
            "(Ljava/util/Date;)I",
            (later.clone(),)
        )
        .await?,
        -1
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(
            &later,
            &later.class_definition().name(),
            "compareTo",
            "(Ljava/util/Date;)I",
            (earlier.clone(),)
        )
        .await?,
        1
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(
            &earlier,
            &earlier.class_definition().name(),
            "compareTo",
            "(Ljava/lang/Object;)I",
            (same,)
        )
        .await?,
        0
    );

    let null: ClassInstanceRef<Object> = None.into();
    let result: Result<i32> = jvm
        .invoke_virtual(
            &earlier,
            &earlier.class_definition().name(),
            "compareTo",
            "(Ljava/util/Date;)I",
            (null.clone(),),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("Date.compareTo(Date) must reject null");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));

    let result: Result<i32> = jvm
        .invoke_virtual(
            &earlier,
            &earlier.class_definition().name(),
            "compareTo",
            "(Ljava/lang/Object;)I",
            (null,),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("Date.compareTo(Object) must reject null");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));

    let object = jvm.new_class("java/lang/Object", "()V", ()).await?;
    let result: Result<i32> = jvm
        .invoke_virtual(
            &earlier,
            &earlier.class_definition().name(),
            "compareTo",
            "(Ljava/lang/Object;)I",
            (object,),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("Date.compareTo(Object) must reject non-Date values");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/ClassCastException"));

    Ok(())
}

#[tokio::test]
async fn test_date_02_before_after_and_null_contract() -> Result<()> {
    let jvm = test_jvm().await?;
    let earlier = jvm.new_class("java/util/Date", "(J)V", (10i64,)).await?;
    let later = jvm.new_class("java/util/Date", "(J)V", (20i64,)).await?;

    assert!(
        jvm.invoke_virtual::<_, bool>(
            &earlier,
            &earlier.class_definition().name(),
            "before",
            "(Ljava/util/Date;)Z",
            (later.clone(),)
        )
        .await?
    );
    assert!(
        !jvm.invoke_virtual::<_, bool>(&earlier, &earlier.class_definition().name(), "after", "(Ljava/util/Date;)Z", (later,))
            .await?
    );

    let null: ClassInstanceRef<Object> = None.into();
    let result: Result<bool> = jvm
        .invoke_virtual(
            &earlier,
            &earlier.class_definition().name(),
            "before",
            "(Ljava/util/Date;)Z",
            (null.clone(),),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("Date.before must reject null");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));

    let result: Result<bool> = jvm
        .invoke_virtual(&earlier, &earlier.class_definition().name(), "after", "(Ljava/util/Date;)Z", (null,))
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("Date.after must reject null");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));

    Ok(())
}
