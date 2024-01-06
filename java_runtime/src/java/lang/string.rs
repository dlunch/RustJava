use alloc::{
    str,
    string::{String as RustString, ToString},
    vec,
    vec::Vec,
};

use bytemuck::{cast_slice, cast_vec};

use java_runtime_base::{Array, JavaFieldAccessFlag, JavaFieldProto, JavaMethodFlag, JavaMethodProto, JavaResult, JvmClassInstanceHandle};
use jvm::{JavaChar, Jvm};

use crate::{java::lang::Object, JavaClassProto, JavaContext};

// class java.lang.String
pub struct String {}

impl String {
    pub fn as_proto() -> JavaClassProto {
        JavaClassProto {
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "([B)V", Self::init_with_byte_array, JavaMethodFlag::NONE),
                JavaMethodProto::new("<init>", "([C)V", Self::init_with_char_array, JavaMethodFlag::NONE),
                JavaMethodProto::new("<init>", "([CII)V", Self::init_with_partial_char_array, JavaMethodFlag::NONE),
                JavaMethodProto::new("<init>", "([BII)V", Self::init_with_partial_byte_array, JavaMethodFlag::NONE),
                JavaMethodProto::new("equals", "(Ljava/lang/Object;)Z", Self::equals, JavaMethodFlag::NONE),
                JavaMethodProto::new("charAt", "(I)C", Self::char_at, JavaMethodFlag::NONE),
                JavaMethodProto::new("getBytes", "()[B", Self::get_bytes, JavaMethodFlag::NONE),
                JavaMethodProto::new("length", "()I", Self::length, JavaMethodFlag::NONE),
                JavaMethodProto::new("concat", "(Ljava/lang/String;)Ljava/lang/String;", Self::concat, JavaMethodFlag::NONE),
                JavaMethodProto::new("substring", "(I)Ljava/lang/String;", Self::substring, JavaMethodFlag::NONE),
                JavaMethodProto::new("substring", "(II)Ljava/lang/String;", Self::substring_with_end, JavaMethodFlag::NONE),
                JavaMethodProto::new("valueOf", "(I)Ljava/lang/String;", Self::value_of_integer, JavaMethodFlag::STATIC),
                JavaMethodProto::new(
                    "valueOf",
                    "(Ljava/lang/Object;)Ljava/lang/String;",
                    Self::value_of_object,
                    JavaMethodFlag::STATIC,
                ),
                JavaMethodProto::new("indexOf", "(Ljava/lang/String;I)I", Self::index_of_with_from, JavaMethodFlag::NONE),
                JavaMethodProto::new("trim", "()Ljava/lang/String;", Self::trim, JavaMethodFlag::NONE),
            ],
            fields: vec![JavaFieldProto::new("value", "[C", JavaFieldAccessFlag::NONE)],
        }
    }

    async fn init_with_byte_array(
        jvm: &mut Jvm,
        _: &JavaContext,
        this: JvmClassInstanceHandle<Self>,
        value: JvmClassInstanceHandle<Array<i8>>,
    ) -> JavaResult<()> {
        tracing::debug!("java.lang.String::<init>({:?}, {:?})", &this, &value);

        let count = jvm.array_length(&value)? as i32;

        jvm.invoke_special(&this, "java/lang/String", "<init>", "([BII)V", (value, 0, count))
            .await?;

        Ok(())
    }

    async fn init_with_char_array(
        jvm: &mut Jvm,
        _: &JavaContext,
        this: JvmClassInstanceHandle<Self>,
        value: JvmClassInstanceHandle<Array<u16>>,
    ) -> JavaResult<()> {
        tracing::debug!("java.lang.String::<init>({:?}, {:?})", &this, &value);

        let count = jvm.array_length(&value)? as i32;

        jvm.invoke_special(&this, "java/lang/String", "<init>", "([CII)V", (value, 0, count))
            .await?;

        Ok(())
    }

    async fn init_with_partial_char_array(
        jvm: &mut Jvm,
        _: &JavaContext,
        mut this: JvmClassInstanceHandle<Self>,
        value: JvmClassInstanceHandle<Array<u16>>,
        offset: i32,
        count: i32,
    ) -> JavaResult<()> {
        tracing::debug!("java.lang.String::<init>({:?}, {:?}, {}, {})", &this, &value, offset, count);

        let mut array = jvm.instantiate_array("C", count as _).await?;
        jvm.put_field(&mut this, "value", "[C", array.clone())?;

        let data: Vec<JavaChar> = jvm.load_array(&value, offset as _, count as _)?;
        jvm.store_array(&mut array, 0, data)?; // TODO we should store value, offset, count like in java

        Ok(())
    }

    async fn init_with_partial_byte_array(
        jvm: &mut Jvm,
        context: &JavaContext,
        this: JvmClassInstanceHandle<Self>,
        value: JvmClassInstanceHandle<Array<i8>>,
        offset: i32,
        count: i32,
    ) -> JavaResult<()> {
        tracing::debug!("java.lang.String::<init>({:?}, {:?}, {}, {})", &this, &value, offset, count);

        let bytes: Vec<i8> = jvm.load_array(&value, offset as _, count as _)?;
        let string = context.decode_str(cast_slice(&bytes));

        let utf16 = string.encode_utf16().collect::<Vec<_>>();

        let mut array = jvm.instantiate_array("C", utf16.len()).await?;
        jvm.store_array(&mut array, 0, utf16)?;

        jvm.invoke_special(&this, "java/lang/String", "<init>", "([C)V", [array.into()]).await?;

        Ok(())
    }

    async fn equals(jvm: &mut Jvm, _: &JavaContext, this: JvmClassInstanceHandle<Self>, other: JvmClassInstanceHandle<Self>) -> JavaResult<bool> {
        tracing::debug!("java.lang.String::equals({:?}, {:?})", &this, &other);

        // TODO Object.equals()

        let other_string = Self::to_rust_string(jvm, &other)?;
        let this_string = Self::to_rust_string(jvm, &this)?;

        if this_string == other_string {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    async fn char_at(jvm: &mut Jvm, _: &JavaContext, this: JvmClassInstanceHandle<Self>, index: i32) -> JavaResult<u16> {
        tracing::debug!("java.lang.String::charAt({:?}, {})", &this, index);

        let value = jvm.get_field(&this, "value", "[C")?;

        Ok(jvm.load_array(&value, index as _, 1)?[0])
    }

    async fn concat(
        jvm: &mut Jvm,
        _: &JavaContext,
        this: JvmClassInstanceHandle<Self>,
        other: JvmClassInstanceHandle<Self>,
    ) -> JavaResult<JvmClassInstanceHandle<Self>> {
        tracing::debug!("java.lang.String::concat({:?}, {:?})", &this, &other);

        let this_string = Self::to_rust_string(jvm, &this)?;
        let other_string = Self::to_rust_string(jvm, &other)?;

        let concat = this_string + &other_string;

        Self::from_rust_string(jvm, &concat).await
    }

    async fn get_bytes(jvm: &mut Jvm, context: &JavaContext, this: JvmClassInstanceHandle<Self>) -> JavaResult<JvmClassInstanceHandle<Array<i8>>> {
        tracing::debug!("java.lang.String::getBytes({:?})", &this);

        let string = Self::to_rust_string(jvm, &this)?;

        let bytes = context.encode_str(&string);
        let bytes: Vec<i8> = cast_vec(bytes);

        let mut byte_array = jvm.instantiate_array("B", bytes.len()).await?;
        jvm.store_array(&mut byte_array, 0, bytes)?;

        Ok(byte_array.into())
    }

    async fn length(jvm: &mut Jvm, _: &JavaContext, this: JvmClassInstanceHandle<Self>) -> JavaResult<i32> {
        tracing::debug!("java.lang.String::length({:?})", &this);

        let value = jvm.get_field(&this, "value", "[C")?;

        Ok(jvm.array_length(&value)? as _)
    }

    async fn substring(
        jvm: &mut Jvm,
        _: &JavaContext,
        this: JvmClassInstanceHandle<Self>,
        begin_index: i32,
    ) -> JavaResult<JvmClassInstanceHandle<Self>> {
        tracing::debug!("java.lang.String::substring({:?}, {})", &this, begin_index);

        let string = Self::to_rust_string(jvm, &this)?;

        let substr = string.chars().skip(begin_index as usize).collect::<RustString>(); // TODO buffer sharing

        Self::from_rust_string(jvm, &substr).await
    }

    async fn substring_with_end(
        jvm: &mut Jvm,
        _: &JavaContext,
        this: JvmClassInstanceHandle<Self>,
        begin_index: i32,
        end_index: i32,
    ) -> JavaResult<JvmClassInstanceHandle<Self>> {
        tracing::debug!("java.lang.String::substring({:?}, {}, {})", &this, begin_index, end_index);

        let string = Self::to_rust_string(jvm, &this)?;

        let substr = string
            .chars()
            .skip(begin_index as usize)
            .take(end_index as usize - begin_index as usize)
            .collect::<RustString>(); // TODO buffer sharing

        Self::from_rust_string(jvm, &substr).await
    }

    async fn value_of_integer(jvm: &mut Jvm, _: &JavaContext, value: i32) -> JavaResult<JvmClassInstanceHandle<Self>> {
        tracing::debug!("java.lang.String::valueOf({})", value);

        let string = value.to_string();

        Self::from_rust_string(jvm, &string).await
    }

    async fn value_of_object(jvm: &mut Jvm, _: &JavaContext, value: JvmClassInstanceHandle<Object>) -> JavaResult<JvmClassInstanceHandle<Self>> {
        tracing::warn!("stub java.lang.String::valueOf({:?})", &value);

        // TODO Object.toString()

        Self::from_rust_string(jvm, "").await
    }

    async fn index_of_with_from(
        jvm: &mut Jvm,
        _: &JavaContext,
        this: JvmClassInstanceHandle<Self>,
        str: JvmClassInstanceHandle<Self>,
        from_index: i32,
    ) -> JavaResult<i32> {
        tracing::debug!("java.lang.String::indexOf({:?}, {:?})", &this, &str);

        let this_string = Self::to_rust_string(jvm, &this)?;
        let str_string = Self::to_rust_string(jvm, &str)?;

        let index = this_string[from_index as usize..].find(&str_string).map(|x| x as i32 + from_index);

        Ok(index.unwrap_or(-1))
    }

    async fn trim(jvm: &mut Jvm, _: &JavaContext, this: JvmClassInstanceHandle<Self>) -> JavaResult<JvmClassInstanceHandle<Self>> {
        tracing::debug!("java.lang.String::trim({:?})", &this);

        let string = Self::to_rust_string(jvm, &this)?;

        let trimmed = string.trim().to_string();

        Self::from_rust_string(jvm, &trimmed).await // TODO buffer sharing
    }

    pub fn to_rust_string(jvm: &mut Jvm, instance: &JvmClassInstanceHandle<String>) -> JavaResult<RustString> {
        let value = jvm.get_field(instance, "value", "[C")?;

        let length = jvm.array_length(&value)?;
        let string: Vec<JavaChar> = jvm.load_array(&value, 0, length)?;

        Ok(RustString::from_utf16(&string).unwrap())
    }

    pub async fn from_rust_string(jvm: &mut Jvm, string: &str) -> JavaResult<JvmClassInstanceHandle<Self>> {
        let utf16 = string.encode_utf16().collect::<Vec<_>>();

        Self::from_utf16(jvm, utf16).await
    }

    pub async fn from_utf16(jvm: &mut Jvm, data: Vec<u16>) -> JavaResult<JvmClassInstanceHandle<Self>> {
        let mut java_value = jvm.instantiate_array("C", data.len()).await?;

        jvm.store_array(&mut java_value, 0, data.to_vec())?;

        let instance = jvm.new_class("java/lang/String", "([C)V", (java_value,)).await?;

        Ok(instance.into())
    }
}
