use alloc::string::ToString;

use jvm::{ClassInstanceRef, Jvm, Result, runtime::JavaLangString};
use jvm_class_proto::{java_class, java_methods};
use percent_encoding::{AsciiSet, CONTROLS, percent_decode_str, utf8_percent_encode};

use crate::{RuntimeContext, classes::java::lang::String};

#[java_class(name = "org/rustjava/net/URLPath", public, final)]
pub struct URLPath;

#[java_methods]
impl URLPath {
    const ESCAPED: &AsciiSet = &CONTROLS
        .add(b' ')
        .add(b'"')
        .add(b'#')
        .add(b'%')
        .add(b'<')
        .add(b'>')
        .add(b'?')
        .add(b'`')
        .add(b'{')
        .add(b'}')
        .add(b'!');

    #[method(public, static)]
    async fn encode(jvm: &Jvm, _: &mut RuntimeContext, path: ClassInstanceRef<String>) -> Result<ClassInstanceRef<String>> {
        if path.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "path is null").await);
        }
        let path = JavaLangString::to_rust_string(jvm, &path).await?;
        Ok(
            JavaLangString::from_rust_string(jvm, &utf8_percent_encode(&path, Self::ESCAPED).to_string())
                .await?
                .into(),
        )
    }

    #[method(public, static)]
    async fn decode(jvm: &Jvm, _: &mut RuntimeContext, path: ClassInstanceRef<String>) -> Result<ClassInstanceRef<String>> {
        if path.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "path is null").await);
        }
        let path = JavaLangString::to_rust_string(jvm, &path).await?;
        match percent_decode_str(&path).decode_utf8() {
            Ok(path) => Ok(JavaLangString::from_rust_string(jvm, &path).await?.into()),
            Err(_) => Err(jvm.exception("java/lang/IllegalArgumentException", "Invalid UTF-8 in URL path").await),
        }
    }
}
