use alloc::{
    str,
    string::{String as RustString, ToString},
    vec,
    vec::Vec,
};

use bytemuck::{cast_slice, cast_vec};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::MethodAccessFlags;
use jvm::{runtime::JavaLangString, Array, ClassInstanceRef, JavaChar, Jvm, Result};

use crate::{
    classes::java::lang::{Object, System},
    RuntimeClassProto, RuntimeContext,
};

// class java.lang.String
pub struct String {}

impl String {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "([B)V", Self::init_with_byte_array, Default::default()),
                JavaMethodProto::new("<init>", "([C)V", Self::init_with_char_array, Default::default()),
                JavaMethodProto::new("<init>", "([CII)V", Self::init_with_partial_char_array, Default::default()),
                JavaMethodProto::new("<init>", "([BII)V", Self::init_with_partial_byte_array, Default::default()),
                JavaMethodProto::new("equals", "(Ljava/lang/Object;)Z", Self::equals, Default::default()),
                JavaMethodProto::new("hashCode", "()I", Self::hash_code, Default::default()),
                JavaMethodProto::new("charAt", "(I)C", Self::char_at, Default::default()),
                JavaMethodProto::new("getBytes", "()[B", Self::get_bytes, Default::default()),
                JavaMethodProto::new("length", "()I", Self::length, Default::default()),
                JavaMethodProto::new("concat", "(Ljava/lang/String;)Ljava/lang/String;", Self::concat, Default::default()),
                JavaMethodProto::new("substring", "(I)Ljava/lang/String;", Self::substring, Default::default()),
                JavaMethodProto::new("substring", "(II)Ljava/lang/String;", Self::substring_with_end, Default::default()),
                JavaMethodProto::new("valueOf", "(I)Ljava/lang/String;", Self::value_of_integer, MethodAccessFlags::STATIC),
                JavaMethodProto::new(
                    "valueOf",
                    "(Ljava/lang/Object;)Ljava/lang/String;",
                    Self::value_of_object,
                    MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new("indexOf", "(Ljava/lang/String;I)I", Self::index_of_with_from, Default::default()),
                JavaMethodProto::new("trim", "()Ljava/lang/String;", Self::trim, Default::default()),
            ],
            fields: vec![JavaFieldProto::new("value", "[C", Default::default())],
        }
    }

    async fn init_with_byte_array(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: ClassInstanceRef<Array<i8>>) -> Result<()> {
        tracing::debug!("java.lang.String::<init>({:?}, {:?})", &this, &value);

        let count = jvm.array_length(&value).await? as i32;

        jvm.invoke_special(&this, "java/lang/String", "<init>", "([BII)V", (value, 0, count))
            .await?;

        Ok(())
    }

    async fn init_with_char_array(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        value: ClassInstanceRef<Array<u16>>,
    ) -> Result<()> {
        tracing::debug!("java.lang.String::<init>({:?}, {:?})", &this, &value);

        let count = jvm.array_length(&value).await? as i32;

        jvm.invoke_special(&this, "java/lang/String", "<init>", "([CII)V", (value, 0, count))
            .await?;

        Ok(())
    }

    async fn init_with_partial_char_array(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        value: ClassInstanceRef<Array<u16>>,
        offset: i32,
        count: i32,
    ) -> Result<()> {
        tracing::debug!("java.lang.String::<init>({:?}, {:?}, {}, {})", &this, &value, offset, count);

        let mut array = jvm.instantiate_array("C", count as _).await?;
        jvm.put_field(&mut this, "value", "[C", array.clone()).await?;

        let data: Vec<JavaChar> = jvm.load_array(&value, offset as _, count as _).await?;
        jvm.store_array(&mut array, 0, data).await?; // TODO we should store value, offset, count like in java

        Ok(())
    }

    async fn init_with_partial_byte_array(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        value: ClassInstanceRef<Array<i8>>,
        offset: i32,
        count: i32,
    ) -> Result<()> {
        tracing::debug!("java.lang.String::<init>({:?}, {:?}, {}, {})", &this, &value, offset, count);

        let bytes: Vec<i8> = jvm.load_array(&value, offset as _, count as _).await?;

        let charset = System::get_charset(jvm).await?;
        let string = Self::decode_str(&charset, cast_slice(&bytes));

        let utf16 = string.encode_utf16().collect::<Vec<_>>();

        let mut array = jvm.instantiate_array("C", utf16.len()).await?;
        jvm.store_array(&mut array, 0, utf16).await?;

        jvm.invoke_special(&this, "java/lang/String", "<init>", "([C)V", [array.into()]).await?;

        Ok(())
    }

    async fn equals(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::debug!("java.lang.String::equals({:?}, {:?})", &this, &other);

        let other_string = JavaLangString::to_rust_string(jvm, &other.clone()).await?;
        let this_string = JavaLangString::to_rust_string(jvm, &this.clone()).await?;

        if this_string == other_string {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    async fn hash_code(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.lang.String::hashCode({:?})", &this);

        let chars = jvm.get_field(&this, "value", "[C").await?;
        let chars: Vec<JavaChar> = jvm.load_array(&chars, 0, jvm.array_length(&chars).await? as _).await?;

        let hash = chars.iter().fold(0i32, |acc, &c| acc.wrapping_mul(31).wrapping_add(c as i32));

        Ok(hash)
    }

    async fn char_at(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, index: i32) -> Result<u16> {
        tracing::debug!("java.lang.String::charAt({:?}, {})", &this, index);

        let value = jvm.get_field(&this, "value", "[C").await?;

        Ok(jvm.load_array(&value, index as _, 1).await?[0])
    }

    async fn concat(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        other: ClassInstanceRef<Self>,
    ) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.String::concat({:?}, {:?})", &this, &other);

        let this_string = JavaLangString::to_rust_string(jvm, &this.clone()).await?;
        let other_string = JavaLangString::to_rust_string(jvm, &other.clone()).await?;

        let concat = this_string + &other_string;

        Ok(JavaLangString::from_rust_string(jvm, &concat).await?.into())
    }

    async fn get_bytes(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Array<i8>>> {
        tracing::debug!("java.lang.String::getBytes({:?})", &this);

        let string = JavaLangString::to_rust_string(jvm, &this.clone()).await?;

        let charset = System::get_charset(jvm).await?;
        let bytes = cast_vec(Self::encode_str(&charset, &string));

        let mut byte_array = jvm.instantiate_array("B", bytes.len()).await?;
        jvm.store_byte_array(&mut byte_array, 0, bytes).await?;

        Ok(byte_array.into())
    }

    async fn length(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.lang.String::length({:?})", &this);

        let value = jvm.get_field(&this, "value", "[C").await?;

        Ok(jvm.array_length(&value).await? as _)
    }

    async fn substring(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, begin_index: i32) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.String::substring({:?}, {})", &this, begin_index);

        let string = JavaLangString::to_rust_string(jvm, &this.clone()).await?;

        let substr = string.chars().skip(begin_index as usize).collect::<RustString>(); // TODO buffer sharing

        Ok(JavaLangString::from_rust_string(jvm, &substr).await?.into())
    }

    async fn substring_with_end(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        begin_index: i32,
        end_index: i32,
    ) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.String::substring({:?}, {}, {})", &this, begin_index, end_index);

        let string = JavaLangString::to_rust_string(jvm, &this.clone()).await?;

        let substr = string
            .chars()
            .skip(begin_index as usize)
            .take(end_index as usize - begin_index as usize)
            .collect::<RustString>(); // TODO buffer sharing

        Ok(JavaLangString::from_rust_string(jvm, &substr).await?.into())
    }

    async fn value_of_integer(jvm: &Jvm, _: &mut RuntimeContext, value: i32) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.String::valueOf({})", value);

        let string = value.to_string();

        Ok(JavaLangString::from_rust_string(jvm, &string).await?.into())
    }

    async fn value_of_object(jvm: &Jvm, _: &mut RuntimeContext, value: ClassInstanceRef<Object>) -> Result<ClassInstanceRef<Self>> {
        tracing::warn!("stub java.lang.String::valueOf({:?})", &value);

        Ok(if value.is_null() {
            JavaLangString::from_rust_string(jvm, "null").await?.into()
        } else {
            jvm.invoke_virtual(&value, "toString", "()Ljava/lang/String;", ()).await?
        })
    }

    async fn index_of_with_from(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        str: ClassInstanceRef<Self>,
        from_index: i32,
    ) -> Result<i32> {
        tracing::debug!("java.lang.String::indexOf({:?}, {:?})", &this, &str);

        let this_string = JavaLangString::to_rust_string(jvm, &this.clone()).await?;
        let str_string = JavaLangString::to_rust_string(jvm, &str.clone()).await?;

        let index = this_string[from_index as usize..].find(&str_string).map(|x| x as i32 + from_index);

        Ok(index.unwrap_or(-1))
    }

    async fn trim(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.String::trim({:?})", &this);

        let string = JavaLangString::to_rust_string(jvm, &this.clone()).await?;

        let trimmed = string.trim().to_string();

        Ok(JavaLangString::from_rust_string(jvm, &trimmed).await?.into()) // TODO buffer sharing
    }

    fn decode_str(charset: &str, bytes: &[u8]) -> RustString {
        match charset {
            "UTF-8" => str::from_utf8(bytes).unwrap().to_string(),
            "EUC-KR" => encoding_rs::EUC_KR.decode(bytes).0.to_string(),
            _ => unimplemented!("unsupported charset: {}", charset),
        }
    }

    fn encode_str(charset: &str, string: &str) -> Vec<u8> {
        match charset {
            "UTF-8" => string.as_bytes().to_vec(),
            "EUC-KR" => encoding_rs::EUC_KR.encode(string).0.to_vec(),
            _ => unimplemented!("unsupported charset: {}", charset),
        }
    }
}

#[cfg(test)]
mod test {
    use jvm::{runtime::JavaLangString, Result};

    use crate::test::test_jvm;

    #[futures_test::test]
    async fn test_string() -> Result<()> {
        let jvm = test_jvm().await?;

        let string = JavaLangString::from_rust_string(&jvm, "test").await?;

        let string = JavaLangString::to_rust_string(&jvm, &string).await?;

        assert_eq!(string, "test");

        Ok(())
    }

    #[futures_test::test]
    async fn test_string_concat() -> Result<()> {
        let jvm = test_jvm().await?;

        let string1 = JavaLangString::from_rust_string(&jvm, "test1").await?;
        let string2 = JavaLangString::from_rust_string(&jvm, "test2").await?;

        let result = jvm
            .invoke_virtual(&string1, "concat", "(Ljava/lang/String;)Ljava/lang/String;", (string2,))
            .await?;

        let string = JavaLangString::to_rust_string(&jvm, &result).await?;

        assert_eq!(string, "test1test2");

        Ok(())
    }

    #[futures_test::test]
    async fn test_hash_code() -> Result<()> {
        let jvm = test_jvm().await?;

        let string = JavaLangString::from_rust_string(&jvm, "Hi").await?;
        let hash_code: i32 = jvm.invoke_virtual(&string, "hashCode", "()I", ()).await?;
        assert_eq!(hash_code, 2337);

        let string1 = JavaLangString::from_rust_string(&jvm, "test").await?;
        let hash_code1: i32 = jvm.invoke_virtual(&string1, "hashCode", "()I", ()).await?;
        assert_eq!(hash_code1, 3556498);

        let string2 = JavaLangString::from_rust_string(&jvm, "Hi").await?;
        let hash_code: i32 = jvm.invoke_virtual(&string2, "hashCode", "()I", ()).await?;
        assert_eq!(hash_code, 2337);

        Ok(())
    }
}
