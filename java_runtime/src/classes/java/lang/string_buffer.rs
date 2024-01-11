use alloc::{
    string::{String as RustString, ToString},
    vec,
    vec::Vec,
};

use java_class_proto::{JavaFieldAccessFlag, JavaFieldProto, JavaMethodFlag, JavaMethodProto, JavaResult};
use jvm::{Array, ClassInstanceRef, JavaChar, Jvm};

use crate::{classes::java::lang::String, RuntimeClassProto, RuntimeContext};

// class java.lang.StringBuffer
pub struct StringBuffer {}

impl StringBuffer {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, JavaMethodFlag::NONE),
                JavaMethodProto::new("<init>", "(Ljava/lang/String;)V", Self::init_with_string, JavaMethodFlag::NONE),
                JavaMethodProto::new(
                    "append",
                    "(Ljava/lang/String;)Ljava/lang/StringBuffer;",
                    Self::append_string,
                    JavaMethodFlag::NONE,
                ),
                JavaMethodProto::new("append", "(I)Ljava/lang/StringBuffer;", Self::append_integer, JavaMethodFlag::NONE),
                JavaMethodProto::new("append", "(J)Ljava/lang/StringBuffer;", Self::append_long, JavaMethodFlag::NONE),
                JavaMethodProto::new("append", "(C)Ljava/lang/StringBuffer;", Self::append_character, JavaMethodFlag::NONE),
                JavaMethodProto::new("toString", "()Ljava/lang/String;", Self::to_string, JavaMethodFlag::NONE),
            ],
            fields: vec![
                JavaFieldProto::new("value", "[C", JavaFieldAccessFlag::NONE),
                JavaFieldProto::new("count", "I", JavaFieldAccessFlag::NONE),
            ],
        }
    }

    async fn init(jvm: &mut Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> JavaResult<()> {
        tracing::debug!("java.lang.StringBuffer::<init>({:?})", &this);

        let array = jvm.instantiate_array("C", 16).await?;
        jvm.put_field(&mut this, "value", "[C", array)?;
        jvm.put_field(&mut this, "count", "I", 0)?;

        Ok(())
    }

    async fn init_with_string(
        jvm: &mut Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        string: ClassInstanceRef<String>,
    ) -> JavaResult<()> {
        tracing::debug!("java.lang.StringBuffer::<init>({:?}, {:?})", &this, &string,);

        let value_array = jvm.get_field(&string, "value", "[C")?;
        let length = jvm.array_length(&value_array)? as i32;

        jvm.put_field(&mut this, "value", "[C", value_array)?;
        jvm.put_field(&mut this, "count", "I", length)?;

        Ok(())
    }

    async fn append_string(
        jvm: &mut Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        string: ClassInstanceRef<String>,
    ) -> JavaResult<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.StringBuffer::append({:?}, {:?})", &this, &string,);

        let string = String::to_rust_string(jvm, &string)?;

        Self::append(jvm, &mut this, &string).await?;

        Ok(this)
    }

    async fn append_integer(
        jvm: &mut Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        value: i32,
    ) -> JavaResult<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.StringBuffer::append({:?}, {:?})", &this, value);

        let digits = value.to_string();

        Self::append(jvm, &mut this, &digits).await?;

        Ok(this)
    }

    async fn append_long(jvm: &mut Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, value: i64) -> JavaResult<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.StringBuffer::append({:?}, {:?})", &this, value);

        let digits = value.to_string();

        Self::append(jvm, &mut this, &digits).await?;

        Ok(this)
    }

    async fn append_character(
        jvm: &mut Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        value: u16,
    ) -> JavaResult<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.StringBuffer::append({:?}, {:?})", &this, value);

        let value = RustString::from_utf16(&[value]).unwrap();

        Self::append(jvm, &mut this, &value).await?;

        Ok(this)
    }

    async fn to_string(jvm: &mut Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> JavaResult<ClassInstanceRef<String>> {
        tracing::debug!("java.lang.StringBuffer::toString({:?})", &this);

        let java_value: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(&this, "value", "[C")?;
        let count: i32 = jvm.get_field(&this, "count", "I")?;

        let string = jvm.new_class("java/lang/String", "([CII)V", (java_value, 0, count)).await?;

        Ok(string.into())
    }

    async fn ensure_capacity(jvm: &mut Jvm, this: &mut ClassInstanceRef<Self>, capacity: usize) -> JavaResult<()> {
        let java_value_array = jvm.get_field(this, "value", "[C")?;
        let current_capacity = jvm.array_length(&java_value_array)?;

        if current_capacity < capacity {
            let old_values: Vec<JavaChar> = jvm.load_array(&java_value_array, 0, current_capacity)?;
            let new_capacity = capacity * 2;

            let mut java_new_value_array = jvm.instantiate_array("C", new_capacity).await?;
            jvm.put_field(this, "value", "[C", java_new_value_array.clone())?;
            jvm.store_array(&mut java_new_value_array, 0, old_values)?;
        }

        Ok(())
    }

    async fn append(jvm: &mut Jvm, this: &mut ClassInstanceRef<Self>, string: &str) -> JavaResult<()> {
        let current_count: i32 = jvm.get_field(this, "count", "I")?;

        let value_to_add = string.encode_utf16().collect::<Vec<_>>();
        let count_to_add = value_to_add.len() as i32;

        StringBuffer::ensure_capacity(jvm, this, (current_count + count_to_add) as _).await?;

        let mut java_value_array = jvm.get_field(this, "value", "[C")?;
        jvm.store_array(&mut java_value_array, current_count as _, value_to_add)?;
        jvm.put_field(this, "count", "I", current_count + count_to_add)?;

        Ok(())
    }
}
