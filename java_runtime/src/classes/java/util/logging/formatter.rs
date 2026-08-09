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

        let parameter_count = jvm.array_length(&parameters).await?;
        if parameter_count == 0 {
            return Ok(message);
        }

        let parameters: Vec<ClassInstanceRef<Object>> = jvm.load_array(&parameters, 0, parameter_count).await?;
        let mut replacements = Vec::with_capacity(parameter_count);
        for parameter in parameters {
            replacements.push(if parameter.is_null() {
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
            if characters[index] == '{' {
                let mut cursor = index + 1;
                let mut parameter_index = Some(0usize);
                let mut has_digit = false;
                while cursor < characters.len() && characters[cursor].is_ascii_digit() {
                    has_digit = true;
                    let digit = (characters[cursor] as u8 - b'0') as usize;
                    parameter_index = parameter_index.and_then(|value| value.checked_mul(10)?.checked_add(digit));
                    cursor += 1;
                }
                if has_digit && cursor < characters.len() && characters[cursor] == '}' {
                    if let Some(replacement) = parameter_index.and_then(|parameter_index| replacements.get(parameter_index)) {
                        formatted.push_str(replacement);
                    } else {
                        formatted.extend(&characters[index..=cursor]);
                    }
                    index = cursor + 1;
                    continue;
                }
            }
            formatted.push(characters[index]);
            index += 1;
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
