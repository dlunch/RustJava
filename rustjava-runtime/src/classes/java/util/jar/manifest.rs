use alloc::{borrow::Cow, string::String as RustString};
use core::iter;

use jvm::{
    ClassInstanceRef, JavaError, Jvm, Result,
    runtime::{JavaIoInputStream, JavaLangString},
};
use jvm_class_proto::{java_class, java_methods};

use crate::{
    RuntimeContext,
    classes::java::{io::InputStream, lang::String, util::jar::Attributes},
};

// class java.util.jar.Manifest
#[java_class(name = "java/util/jar/Manifest", implements("java/lang/Cloneable"), public)]
pub struct Manifest {
    #[field(private)]
    attrs: ClassInstanceRef<Attributes>,
}

#[java_methods]
impl Manifest {
    #[method(constructor, public)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, is: ClassInstanceRef<InputStream>) -> Result<()> {
        tracing::debug!("java.util.jar.Manifest::<init>({this:?}, {is:?})");

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        let _: () = jvm
            .invoke_virtual(&this, "java/util/jar/Manifest", "read", "(Ljava/io/InputStream;)V", (is,))
            .await?;

        Ok(())
    }

    #[method(public)]
    async fn read(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, is: ClassInstanceRef<InputStream>) -> Result<()> {
        tracing::debug!("java.util.jar.Manifest::read({this:?}, {is:?})");

        let main_attributes = jvm.new_class("java/util/jar/Attributes", "()V", ()).await?;
        if is.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "input stream is null").await);
        }
        let bytes = JavaIoInputStream::read_until_end(jvm, &is).await?;
        let mut lines = manifest_lines(&bytes).peekable();
        while let Some(line) = lines.next() {
            if line.is_empty() {
                break;
            }
            let Some(separator) = line.windows(2).position(|pair| pair == b": ") else {
                return Err(jvm.exception("java/io/IOException", "invalid manifest header").await);
            };
            let key = JavaLangString::from_rust_string(jvm, &RustString::from_utf8_lossy(&line[..separator])).await?;
            let mut value = Cow::Borrowed(&line[separator + 2..]);
            // A continuation can split a UTF-8 character across physical lines.
            while let Some(continuation) = lines.next_if(|line| line.starts_with(b" ")) {
                value.to_mut().extend_from_slice(&continuation[1..]);
            }
            let value = JavaLangString::from_rust_string(jvm, &RustString::from_utf8_lossy(&value)).await?;

            let result: Result<ClassInstanceRef<String>> = jvm
                .invoke_virtual(
                    &main_attributes,
                    "java/util/jar/Attributes",
                    "putValue",
                    "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;",
                    (key, value),
                )
                .await;
            match result {
                Err(JavaError::JavaException(exception)) if jvm.is_instance(&*exception, "java/lang/IllegalArgumentException") => {
                    return Err(jvm.exception("java/io/IOException", "invalid manifest attribute name").await);
                }
                result => {
                    result?;
                }
            }
        }

        jvm.put_field(
            &mut this,
            "java/util/jar/Manifest",
            "attrs",
            "Ljava/util/jar/Attributes;",
            main_attributes,
        )
        .await?;

        Ok(())
    }

    #[method(public)]
    async fn get_main_attributes(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Attributes>> {
        tracing::debug!("java.util.jar.Manifest::getMainAttributes({this:?})");

        jvm.get_field(&this, "java/util/jar/Manifest", "attrs", "Ljava/util/jar/Attributes;")
            .await
    }
}

fn manifest_lines(mut bytes: &[u8]) -> impl Iterator<Item = &[u8]> {
    iter::from_fn(move || {
        if bytes.is_empty() {
            return None;
        }
        let end = bytes.iter().position(|b| matches!(b, b'\r' | b'\n')).unwrap_or(bytes.len());
        let (line, rest) = bytes.split_at(end);
        bytes = match rest {
            [b'\r', b'\n', rest @ ..] | [_, rest @ ..] => rest,
            [] => &[],
        };
        Some(line)
    })
}
