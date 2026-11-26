use alloc::{string::String as RustString, vec::Vec};

use jvm::{Array, ClassInstanceRef, Jvm, Result, runtime::JavaLangString};
use jvm_class_proto::{java_class, java_methods};

use crate::{
    RuntimeContext,
    classes::java::lang::{Object, String},
};

use super::LogRecord;

// public abstract class java.util.logging.Formatter
#[java_class(name = "java/util/logging/Formatter", public, abstract)]
pub struct Formatter;

#[java_methods]
impl Formatter {
    #[method(constructor, protected)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.logging.Formatter::<init>({this:?})");

        jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await
    }

    #[method(public)]
    async fn format(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        record: ClassInstanceRef<LogRecord>,
    ) -> Result<ClassInstanceRef<String>>;

    #[method(public)]
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

        let message: ClassInstanceRef<String> = jvm
            .invoke_virtual(&record, "java/util/logging/LogRecord", "getMessage", "()Ljava/lang/String;", ())
            .await?;
        if message.is_null() {
            return Ok(message);
        }

        let parameters: ClassInstanceRef<Array<Object>> = jvm
            .invoke_virtual(&record, "java/util/logging/LogRecord", "getParameters", "()[Ljava/lang/Object;", ())
            .await?;
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
                let value: ClassInstanceRef<String> = jvm
                    .invoke_virtual(&parameter, "java/lang/Object", "toString", "()Ljava/lang/String;", ())
                    .await?;
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

    #[method(public)]
    async fn get_head(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        _: ClassInstanceRef<super::Handler>,
    ) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.util.logging.Formatter::getHead({this:?})");

        Ok(JavaLangString::from_rust_string(jvm, "").await?.into())
    }

    #[method(public)]
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
