use alloc::{string::String as RustString, vec, vec::Vec};

use java_class_proto::JavaMethodProto;
use java_constants::{ClassAccessFlags, MethodAccessFlags};
use jvm::{Array, ClassInstanceRef, Jvm, Result, runtime::JavaLangString};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::java::lang::{Object, String},
};

use super::LogRecord;

// public abstract class java.util.logging.Formatter
pub struct Formatter;

impl Formatter {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/logging/Formatter",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, MethodAccessFlags::PROTECTED),
                JavaMethodProto::new_abstract(
                    "format",
                    "(Ljava/util/logging/LogRecord;)Ljava/lang/String;",
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::ABSTRACT,
                ),
                JavaMethodProto::new(
                    "formatMessage",
                    "(Ljava/util/logging/LogRecord;)Ljava/lang/String;",
                    Self::format_message,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "getHead",
                    "(Ljava/util/logging/Handler;)Ljava/lang/String;",
                    Self::get_head,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "getTail",
                    "(Ljava/util/logging/Handler;)Ljava/lang/String;",
                    Self::get_tail,
                    MethodAccessFlags::PUBLIC,
                ),
            ],
            fields: vec![],
            access_flags: ClassAccessFlags::PUBLIC | ClassAccessFlags::ABSTRACT,
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.logging.Formatter::<init>({this:?})");

        jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await
    }

    async fn format_message(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        record: ClassInstanceRef<LogRecord>,
    ) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.util.logging.Formatter::formatMessage({this:?}, {record:?})");

        if record.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "record").await);
        }

        let message: ClassInstanceRef<String> = jvm.invoke_virtual(&record, "getMessage", "()Ljava/lang/String;", ()).await?;
        if message.is_null() {
            return Ok(message);
        }

        let parameters: ClassInstanceRef<Array<Object>> = jvm.invoke_virtual(&record, "getParameters", "()[Ljava/lang/Object;", ()).await?;
        if parameters.is_null() {
            return Ok(message);
        }

        let parameter_count = jvm.array_length(&parameters).await?.min(4);
        if parameter_count == 0 {
            return Ok(message);
        }

        let parameters: Vec<ClassInstanceRef<Object>> = jvm.load_array(&parameters, 0, parameter_count).await?;
        let mut replacements: Vec<Option<RustString>> = vec![None; 4];
        for (index, parameter) in parameters.into_iter().enumerate() {
            replacements[index] = Some(if parameter.is_null() {
                RustString::from("null")
            } else {
                let value: ClassInstanceRef<String> = jvm.invoke_virtual(&parameter, "toString", "()Ljava/lang/String;", ()).await?;
                if value.is_null() {
                    RustString::from("null")
                } else {
                    JavaLangString::to_rust_string(jvm, &value).await?
                }
            });
        }

        let message = JavaLangString::to_rust_string(jvm, &message).await?;
        let characters: Vec<char> = message.chars().collect();
        let mut formatted = RustString::new();
        let mut index = 0;
        while index < characters.len() {
            if index + 2 < characters.len()
                && characters[index] == '{'
                && ('0'..='3').contains(&characters[index + 1])
                && characters[index + 2] == '}'
            {
                let parameter_index = characters[index + 1] as usize - '0' as usize;
                if let Some(replacement) = &replacements[parameter_index] {
                    formatted.push_str(replacement);
                } else {
                    formatted.extend(&characters[index..index + 3]);
                }
                index += 3;
            } else {
                formatted.push(characters[index]);
                index += 1;
            }
        }

        Ok(JavaLangString::from_rust_string(jvm, &formatted).await?.into())
    }

    async fn get_head(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        _: ClassInstanceRef<super::Handler>,
    ) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.util.logging.Formatter::getHead({this:?})");

        Ok(JavaLangString::from_rust_string(jvm, "").await?.into())
    }

    async fn get_tail(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        _: ClassInstanceRef<super::Handler>,
    ) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.util.logging.Formatter::getTail({this:?})");

        Ok(JavaLangString::from_rust_string(jvm, "").await?.into())
    }
}
