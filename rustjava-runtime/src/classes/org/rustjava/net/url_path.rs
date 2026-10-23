use alloc::{string::ToString, vec};

use jvm::{ClassInstanceRef, Jvm, Result, runtime::JavaLangString};
use jvm_class_proto::JavaMethodProto;
use jvm_types::{ClassAccessFlags, MethodAccessFlags};
use percent_encoding::{AsciiSet, CONTROLS, percent_decode_str, utf8_percent_encode};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::String};

pub struct URLPath;

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

    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "org/rustjava/net/URLPath",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new(
                    "encode",
                    "(Ljava/lang/String;)Ljava/lang/String;",
                    Self::encode,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "decode",
                    "(Ljava/lang/String;)Ljava/lang/String;",
                    Self::decode,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
            ],
            fields: vec![],
            access_flags: ClassAccessFlags::PUBLIC | ClassAccessFlags::FINAL,
        }
    }

    async fn encode(jvm: &Jvm, _: &mut RuntimeContext, path: ClassInstanceRef<String>) -> Result<ClassInstanceRef<String>> {
        let path = JavaLangString::to_rust_string(jvm, &path).await?;
        Ok(
            JavaLangString::from_rust_string(jvm, &utf8_percent_encode(&path, Self::ESCAPED).to_string())
                .await?
                .into(),
        )
    }

    async fn decode(jvm: &Jvm, _: &mut RuntimeContext, path: ClassInstanceRef<String>) -> Result<ClassInstanceRef<String>> {
        let path = JavaLangString::to_rust_string(jvm, &path).await?;
        match percent_decode_str(&path).decode_utf8() {
            Ok(path) => Ok(JavaLangString::from_rust_string(jvm, &path).await?.into()),
            Err(_) => Err(jvm.exception("java/lang/IllegalArgumentException", "Invalid UTF-8 in URL path").await),
        }
    }
}
