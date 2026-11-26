use jvm::{Array, ClassInstanceRef, JavaChar, Jvm, Result, runtime::JavaLangString};
use jvm_class_proto::{java_class, java_methods};

use crate::{
    RuntimeContext,
    classes::java::lang::{CharSequence, Object, String},
};

// abstract class java.io.Writer
#[java_class(
    name = "java/io/Writer",
    implements("java/lang/Appendable", "java/io/Closeable", "java/io/Flushable"),
    public,
    abstract
)]
pub struct Writer {
    #[field(protected)]
    lock: ClassInstanceRef<Object>,
}

#[java_methods]
impl Writer {
    #[method(constructor, protected)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.io.Writer::<init>({this:?})");

        let _: () = jvm
            .invoke_special(&this, "java/io/Writer", "<init>", "(Ljava/lang/Object;)V", (this.clone(),))
            .await?;

        Ok(())
    }

    #[method(constructor, protected)]
    async fn init_with_lock(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, lock: ClassInstanceRef<Object>) -> Result<()> {
        tracing::debug!("java.io.Writer::<init>({this:?}, {lock:?})");

        if lock.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "lock is null").await);
        }

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "java/io/Writer", "lock", "Ljava/lang/Object;", lock).await?;

        Ok(())
    }

    #[method(name = "write", public)]
    async fn write_char(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: i32) -> Result<()> {
        tracing::debug!("java.io.Writer::write({this:?}, {value})");

        let mut chars = jvm.instantiate_array("C", 1).await?;
        jvm.store_array(&mut chars, 0, [value as JavaChar]).await?;
        jvm.invoke_virtual(&this, "java/io/Writer", "write", "([CII)V", (chars, 0, 1)).await
    }

    #[method(name = "write", public)]
    async fn write_chars(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, chars: ClassInstanceRef<Array<JavaChar>>) -> Result<()> {
        tracing::debug!("java.io.Writer::write({this:?}, {chars:?})");
        if chars.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "argument is null").await);
        }

        let length = jvm.array_length(&chars).await? as i32;
        jvm.invoke_virtual(&this, "java/io/Writer", "write", "([CII)V", (chars, 0, length)).await
    }

    #[method(public)]
    async fn write(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        chars: ClassInstanceRef<Array<JavaChar>>,
        offset: i32,
        length: i32,
    ) -> Result<()>;

    #[method(name = "write", public)]
    async fn write_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, string: ClassInstanceRef<String>) -> Result<()> {
        tracing::debug!("java.io.Writer::write_string({this:?}, {string:?})");
        if string.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "argument is null").await);
        }

        let chars: ClassInstanceRef<Array<JavaChar>> = jvm.invoke_virtual(&string, "java/lang/String", "toCharArray", "()[C", ()).await?;
        let length = jvm.array_length(&chars).await?;

        let _: () = jvm
            .invoke_virtual(&this, "java/io/Writer", "write", "([CII)V", (chars, 0, length as i32))
            .await?;

        Ok(())
    }

    #[method(name = "write", public)]
    async fn write_string_offset(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        string: ClassInstanceRef<String>,
        off: i32,
        len: i32,
    ) -> Result<()> {
        tracing::debug!("java.io.Writer::write({this:?}, {string:?}, {off}, {len})");
        if string.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "argument is null").await);
        }

        let chars: ClassInstanceRef<Array<JavaChar>> = jvm.invoke_virtual(&string, "java/lang/String", "toCharArray", "()[C", ()).await?;
        jvm.invoke_virtual(&this, "java/io/Writer", "write", "([CII)V", (chars, off, len)).await
    }

    #[method(name = "append", public)]
    #[method(
        name = "append",
        descriptor = "(Ljava/lang/CharSequence;)Ljava/lang/Appendable;",
        public,
        bridge,
        synthetic
    )]
    async fn append_char_sequence(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        sequence: ClassInstanceRef<CharSequence>,
    ) -> Result<ClassInstanceRef<Self>> {
        let string: ClassInstanceRef<String> = if sequence.is_null() {
            JavaLangString::from_rust_string(jvm, "null").await?.into()
        } else {
            jvm.invoke_virtual(&sequence, &sequence.class_definition().name(), "toString", "()Ljava/lang/String;", ())
                .await?
        };
        let _: () = jvm
            .invoke_virtual(&this, "java/io/Writer", "write", "(Ljava/lang/String;)V", (string,))
            .await?;
        Ok(this)
    }

    #[method(name = "append", public)]
    #[method(
        name = "append",
        descriptor = "(Ljava/lang/CharSequence;II)Ljava/lang/Appendable;",
        public,
        bridge,
        synthetic
    )]
    async fn append_char_sequence_range(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        sequence: ClassInstanceRef<CharSequence>,
        start: i32,
        end: i32,
    ) -> Result<ClassInstanceRef<Self>> {
        let sequence: ClassInstanceRef<CharSequence> = if sequence.is_null() {
            JavaLangString::from_rust_string(jvm, "null").await?.into()
        } else {
            sequence
        };
        let subsequence: ClassInstanceRef<CharSequence> = jvm
            .invoke_virtual(
                &sequence,
                &sequence.class_definition().name(),
                "subSequence",
                "(II)Ljava/lang/CharSequence;",
                (start, end),
            )
            .await?;
        let string: ClassInstanceRef<String> = jvm
            .invoke_virtual(
                &subsequence,
                &subsequence.class_definition().name(),
                "toString",
                "()Ljava/lang/String;",
                (),
            )
            .await?;
        let _: () = jvm
            .invoke_virtual(&this, "java/io/Writer", "write", "(Ljava/lang/String;)V", (string,))
            .await?;
        Ok(this)
    }

    #[method(name = "append", public)]
    #[method(name = "append", descriptor = "(C)Ljava/lang/Appendable;", public, bridge, synthetic)]
    async fn append_char(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, character: JavaChar) -> Result<ClassInstanceRef<Self>> {
        let _: () = jvm.invoke_virtual(&this, "java/io/Writer", "write", "(I)V", (character as i32,)).await?;
        Ok(this)
    }

    #[method(public)]
    async fn flush(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()>;

    #[method(public)]
    async fn close(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()>;
}
