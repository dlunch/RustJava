use alloc::{string::ToString, vec, vec::Vec};

use jvm::{Array, ClassInstanceRef, JavaChar, Jvm, Result, runtime::JavaLangString};
use jvm_class_proto::{java_class, java_methods};

use crate::{
    RuntimeContext,
    classes::java::lang::{AbstractStringBuilder, CharSequence, Object, String},
};

// public final class java.lang.StringBuffer
#[java_class(
    name = "java/lang/StringBuffer",
    extends = "java/lang/AbstractStringBuilder",
    implements("java/io/Serializable", "java/lang/CharSequence"),
    public,
    final
)]
pub struct StringBuffer;

#[java_methods]
impl StringBuffer {
    #[method(constructor, public)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.lang.StringBuffer::<init>({this:?})");

        jvm.invoke_special(&this, "java/lang/StringBuffer", "<init>", "(I)V", (16,)).await
    }

    #[method(constructor, public)]
    async fn init_with_capacity(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, capacity: i32) -> Result<()> {
        tracing::debug!("java.lang.StringBuffer::<init>({this:?}, {capacity})");

        jvm.invoke_special(&this, "java/lang/AbstractStringBuilder", "<init>", "(I)V", (capacity,))
            .await
    }

    #[method(constructor, public)]
    async fn init_with_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, string: ClassInstanceRef<String>) -> Result<()> {
        tracing::debug!("java.lang.StringBuffer::<init>({this:?}, {string:?})");

        if string.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "str is null").await);
        }

        let chars = JavaLangString::to_utf16(jvm, &string).await?;
        let _: () = jvm
            .invoke_special(&this, "java/lang/AbstractStringBuilder", "<init>", "(I)V", ((chars.len() + 16) as i32,))
            .await?;
        let _: ClassInstanceRef<Self> = jvm
            .invoke_virtual(
                &this,
                "java/lang/StringBuffer",
                "append",
                "(Ljava/lang/String;)Ljava/lang/StringBuffer;",
                (string,),
            )
            .await?;
        Ok(())
    }

    #[method(constructor, public)]
    async fn init_with_char_sequence(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        sequence: ClassInstanceRef<CharSequence>,
    ) -> Result<()> {
        if sequence.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "seq is null").await);
        }
        let length: i32 = jvm
            .invoke_virtual(&sequence, &sequence.class_definition().name(), "length", "()I", ())
            .await?;
        let _: () = jvm
            .invoke_special(&this, "java/lang/AbstractStringBuilder", "<init>", "(I)V", (length + 16,))
            .await?;
        let _: ClassInstanceRef<Self> = jvm
            .invoke_virtual(
                &this,
                "java/lang/StringBuffer",
                "append",
                "(Ljava/lang/CharSequence;)Ljava/lang/StringBuffer;",
                (sequence,),
            )
            .await?;
        Ok(())
    }

    #[method(name = "append", public, synchronized)]
    #[method(
        name = "append",
        descriptor = "(Ljava/lang/String;)Ljava/lang/AbstractStringBuilder;",
        public,
        bridge,
        synthetic
    )]
    async fn append_string(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        string: ClassInstanceRef<String>,
    ) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.StringBuffer::append({this:?}, {string:?})");

        let chars = if string.is_null() {
            "null".encode_utf16().collect()
        } else {
            JavaLangString::to_utf16(jvm, &string).await?
        };
        Self::append_utf16(jvm, &mut this, chars).await?;
        Ok(this)
    }

    #[method(name = "append", public, synchronized)]
    #[method(
        name = "append",
        descriptor = "(Ljava/lang/Object;)Ljava/lang/AbstractStringBuilder;",
        public,
        bridge,
        synthetic
    )]
    async fn append_object(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        object: ClassInstanceRef<Object>,
    ) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.StringBuffer::append({this:?}, {object:?})");

        let string: ClassInstanceRef<String> = if object.is_null() {
            JavaLangString::from_rust_string(jvm, "null").await?.into()
        } else {
            jvm.invoke_virtual(&object, "java/lang/Object", "toString", "()Ljava/lang/String;", ())
                .await?
        };
        jvm.invoke_virtual(
            &this,
            "java/lang/StringBuffer",
            "append",
            "(Ljava/lang/String;)Ljava/lang/StringBuffer;",
            (string,),
        )
        .await
    }

    #[method(name = "append", public, synchronized)]
    #[method(
        name = "append",
        descriptor = "(Ljava/lang/StringBuffer;)Ljava/lang/AbstractStringBuilder;",
        public,
        bridge,
        synthetic
    )]
    async fn append_string_buffer(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        buffer: ClassInstanceRef<Self>,
    ) -> Result<ClassInstanceRef<Self>> {
        let _: ClassInstanceRef<AbstractStringBuilder> = jvm
            .invoke_special(
                &this,
                "java/lang/AbstractStringBuilder",
                "append",
                "(Ljava/lang/StringBuffer;)Ljava/lang/AbstractStringBuilder;",
                (buffer,),
            )
            .await?;
        Ok(this)
    }

    #[method(name = "append", public, synchronized)]
    #[method(
        name = "append",
        descriptor = "(Ljava/lang/CharSequence;)Ljava/lang/AbstractStringBuilder;",
        public,
        bridge,
        synthetic
    )]
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
        let _: ClassInstanceRef<AbstractStringBuilder> = jvm
            .invoke_special(
                &this,
                "java/lang/AbstractStringBuilder",
                "append",
                "(Ljava/lang/CharSequence;)Ljava/lang/AbstractStringBuilder;",
                (sequence,),
            )
            .await?;
        Ok(this)
    }

    #[method(name = "append", public, synchronized)]
    #[method(
        name = "append",
        descriptor = "(Ljava/lang/CharSequence;II)Ljava/lang/AbstractStringBuilder;",
        public,
        bridge,
        synthetic
    )]
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
        let _: ClassInstanceRef<AbstractStringBuilder> = jvm
            .invoke_special(
                &this,
                "java/lang/AbstractStringBuilder",
                "append",
                "(Ljava/lang/CharSequence;II)Ljava/lang/AbstractStringBuilder;",
                (sequence, start, end),
            )
            .await?;
        Ok(this)
    }

    #[method(name = "append", public, synchronized)]
    #[method(
        name = "append",
        descriptor = "(Z)Ljava/lang/AbstractStringBuilder;",
        public,
        synchronized,
        bridge,
        synthetic
    )]
    async fn append_boolean(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, value: bool) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.StringBuffer::append({this:?}, {value})");

        Self::append_utf16(jvm, &mut this, (if value { "true" } else { "false" }).encode_utf16().collect()).await?;
        Ok(this)
    }

    #[method(name = "append", public, synchronized)]
    #[method(name = "append", descriptor = "(C)Ljava/lang/AbstractStringBuilder;", public, bridge, synthetic)]
    #[method(name = "append", descriptor = "(C)Ljava/lang/Appendable;", public, bridge, synthetic)]
    async fn append_character(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        value: JavaChar,
    ) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.StringBuffer::append({this:?}, {value})");

        Self::append_utf16(jvm, &mut this, vec![value]).await?;
        Ok(this)
    }

    #[method(name = "append", public, synchronized)]
    #[method(
        name = "append",
        descriptor = "(I)Ljava/lang/AbstractStringBuilder;",
        public,
        synchronized,
        bridge,
        synthetic
    )]
    async fn append_integer(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, value: i32) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.StringBuffer::append({this:?}, {value})");

        Self::append_utf16(jvm, &mut this, value.to_string().encode_utf16().collect()).await?;
        Ok(this)
    }

    #[method(public, synchronized)]
    #[method(descriptor = "(I)Ljava/lang/AbstractStringBuilder;", public, bridge, synthetic)]
    async fn append_code_point(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, code_point: i32) -> Result<ClassInstanceRef<Self>> {
        let _: ClassInstanceRef<AbstractStringBuilder> = jvm
            .invoke_special(
                &this,
                "java/lang/AbstractStringBuilder",
                "appendCodePoint",
                "(I)Ljava/lang/AbstractStringBuilder;",
                (code_point,),
            )
            .await?;
        Ok(this)
    }

    #[method(name = "append", public, synchronized)]
    #[method(
        name = "append",
        descriptor = "(J)Ljava/lang/AbstractStringBuilder;",
        public,
        synchronized,
        bridge,
        synthetic
    )]
    async fn append_long(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, value: i64) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.StringBuffer::append({this:?}, {value})");

        Self::append_utf16(jvm, &mut this, value.to_string().encode_utf16().collect()).await?;
        Ok(this)
    }

    #[method(name = "append", public, synchronized)]
    #[method(name = "append", descriptor = "(F)Ljava/lang/AbstractStringBuilder;", public, bridge, synthetic)]
    async fn append_float(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: f32) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.StringBuffer::append({this:?}, {value})");

        let string: ClassInstanceRef<String> = jvm
            .invoke_static("java/lang/Float", "toString", "(F)Ljava/lang/String;", (value,))
            .await?;
        jvm.invoke_virtual(
            &this,
            "java/lang/StringBuffer",
            "append",
            "(Ljava/lang/String;)Ljava/lang/StringBuffer;",
            (string,),
        )
        .await
    }

    #[method(name = "append", public, synchronized)]
    #[method(name = "append", descriptor = "(D)Ljava/lang/AbstractStringBuilder;", public, bridge, synthetic)]
    async fn append_double(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: f64) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.StringBuffer::append({this:?}, {value})");

        let string: ClassInstanceRef<String> = jvm
            .invoke_static("java/lang/Double", "toString", "(D)Ljava/lang/String;", (value,))
            .await?;
        jvm.invoke_virtual(
            &this,
            "java/lang/StringBuffer",
            "append",
            "(Ljava/lang/String;)Ljava/lang/StringBuffer;",
            (string,),
        )
        .await
    }

    #[method(name = "append", public, synchronized)]
    #[method(name = "append", descriptor = "([C)Ljava/lang/AbstractStringBuilder;", public, bridge, synthetic)]
    async fn append_char_array_all(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        array: ClassInstanceRef<Array<JavaChar>>,
    ) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.StringBuffer::append({this:?}, {array:?})");

        if array.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "str is null").await);
        }
        let length = jvm.array_length(&array).await?;
        let chars = jvm.load_array(&array, 0, length).await?;
        Self::append_utf16(jvm, &mut this, chars).await?;
        Ok(this)
    }

    #[method(name = "append", public, synchronized)]
    #[method(name = "append", descriptor = "([CII)Ljava/lang/AbstractStringBuilder;", public, bridge, synthetic)]
    async fn append_char_array(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        array: ClassInstanceRef<Array<JavaChar>>,
        offset: i32,
        length: i32,
    ) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.StringBuffer::append({this:?}, {array:?}, {offset}, {length})");

        if array.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "str is null").await);
        }
        let array_length = jvm.array_length(&array).await? as i32;
        if offset < 0 || length < 0 || offset > array_length - length {
            return Err(jvm
                .exception(
                    "java/lang/IndexOutOfBoundsException",
                    &alloc::format!("offset {offset}, length {length}, array length {array_length}"),
                )
                .await);
        }
        let chars = jvm.load_array(&array, offset as usize, length as usize).await?;
        Self::append_utf16(jvm, &mut this, chars).await?;
        Ok(this)
    }

    #[method(name = "insert", public, synchronized)]
    #[method(
        name = "insert",
        descriptor = "(ILjava/lang/String;)Ljava/lang/AbstractStringBuilder;",
        public,
        bridge,
        synthetic
    )]
    async fn insert_string(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        offset: i32,
        string: ClassInstanceRef<String>,
    ) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.StringBuffer::insert({this:?}, {offset}, {string:?})");

        let chars = if string.is_null() {
            "null".encode_utf16().collect()
        } else {
            JavaLangString::to_utf16(jvm, &string).await?
        };
        Self::insert_utf16(jvm, &mut this, offset, chars).await?;
        Ok(this)
    }

    #[method(name = "insert", public, synchronized)]
    #[method(
        name = "insert",
        descriptor = "(ILjava/lang/Object;)Ljava/lang/AbstractStringBuilder;",
        public,
        bridge,
        synthetic
    )]
    async fn insert_object(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        offset: i32,
        object: ClassInstanceRef<Object>,
    ) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.StringBuffer::insert({this:?}, {offset}, {object:?})");

        let string: ClassInstanceRef<String> = if object.is_null() {
            JavaLangString::from_rust_string(jvm, "null").await?.into()
        } else {
            jvm.invoke_virtual(&object, "java/lang/Object", "toString", "()Ljava/lang/String;", ())
                .await?
        };
        jvm.invoke_virtual(
            &this,
            "java/lang/StringBuffer",
            "insert",
            "(ILjava/lang/String;)Ljava/lang/StringBuffer;",
            (offset, string),
        )
        .await
    }

    #[method(name = "insert", public)]
    #[method(name = "insert", descriptor = "(IZ)Ljava/lang/AbstractStringBuilder;", public, bridge, synthetic)]
    async fn insert_boolean(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        offset: i32,
        value: bool,
    ) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.StringBuffer::insert({this:?}, {offset}, {value})");

        let string = JavaLangString::from_rust_string(jvm, if value { "true" } else { "false" }).await?;
        jvm.invoke_virtual(
            &this,
            "java/lang/StringBuffer",
            "insert",
            "(ILjava/lang/String;)Ljava/lang/StringBuffer;",
            (offset, string),
        )
        .await
    }

    #[method(name = "insert", public, synchronized)]
    #[method(name = "insert", descriptor = "(IC)Ljava/lang/AbstractStringBuilder;", public, bridge, synthetic)]
    async fn insert_character(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        offset: i32,
        value: JavaChar,
    ) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.StringBuffer::insert({this:?}, {offset}, {value})");

        Self::insert_utf16(jvm, &mut this, offset, vec![value]).await?;
        Ok(this)
    }

    #[method(name = "insert", public)]
    #[method(name = "insert", descriptor = "(II)Ljava/lang/AbstractStringBuilder;", public, bridge, synthetic)]
    async fn insert_integer(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        offset: i32,
        value: i32,
    ) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.StringBuffer::insert({this:?}, {offset}, {value})");

        let string = JavaLangString::from_rust_string(jvm, &value.to_string()).await?;
        jvm.invoke_virtual(
            &this,
            "java/lang/StringBuffer",
            "insert",
            "(ILjava/lang/String;)Ljava/lang/StringBuffer;",
            (offset, string),
        )
        .await
    }

    #[method(name = "insert", public)]
    #[method(name = "insert", descriptor = "(IJ)Ljava/lang/AbstractStringBuilder;", public, bridge, synthetic)]
    async fn insert_long(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, offset: i32, value: i64) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.StringBuffer::insert({this:?}, {offset}, {value})");

        let string = JavaLangString::from_rust_string(jvm, &value.to_string()).await?;
        jvm.invoke_virtual(
            &this,
            "java/lang/StringBuffer",
            "insert",
            "(ILjava/lang/String;)Ljava/lang/StringBuffer;",
            (offset, string),
        )
        .await
    }

    #[method(name = "insert", public)]
    #[method(name = "insert", descriptor = "(IF)Ljava/lang/AbstractStringBuilder;", public, bridge, synthetic)]
    async fn insert_float(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        offset: i32,
        value: f32,
    ) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.StringBuffer::insert({this:?}, {offset}, {value})");

        let string: ClassInstanceRef<String> = jvm
            .invoke_static("java/lang/Float", "toString", "(F)Ljava/lang/String;", (value,))
            .await?;
        jvm.invoke_virtual(
            &this,
            "java/lang/StringBuffer",
            "insert",
            "(ILjava/lang/String;)Ljava/lang/StringBuffer;",
            (offset, string),
        )
        .await
    }

    #[method(name = "insert", public)]
    #[method(name = "insert", descriptor = "(ID)Ljava/lang/AbstractStringBuilder;", public, bridge, synthetic)]
    async fn insert_double(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        offset: i32,
        value: f64,
    ) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.StringBuffer::insert({this:?}, {offset}, {value})");

        let string: ClassInstanceRef<String> = jvm
            .invoke_static("java/lang/Double", "toString", "(D)Ljava/lang/String;", (value,))
            .await?;
        jvm.invoke_virtual(
            &this,
            "java/lang/StringBuffer",
            "insert",
            "(ILjava/lang/String;)Ljava/lang/StringBuffer;",
            (offset, string),
        )
        .await
    }

    #[method(name = "insert", public, synchronized)]
    #[method(name = "insert", descriptor = "(I[C)Ljava/lang/AbstractStringBuilder;", public, bridge, synthetic)]
    async fn insert_char_array(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        offset: i32,
        array: ClassInstanceRef<Array<JavaChar>>,
    ) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.StringBuffer::insert({this:?}, {offset}, {array:?})");

        let count: i32 = jvm.get_field(&this, "java/lang/StringBuffer", "count", "I").await?;
        if offset < 0 || offset > count {
            return Err(jvm
                .exception(
                    "java/lang/StringIndexOutOfBoundsException",
                    &alloc::format!("offset {offset}, length {count}"),
                )
                .await);
        }
        if array.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "str is null").await);
        }
        let chars = jvm.load_array(&array, 0, jvm.array_length(&array).await?).await?;
        Self::insert_utf16(jvm, &mut this, offset, chars).await?;
        Ok(this)
    }

    #[method(name = "insert", public, synchronized)]
    #[method(name = "insert", descriptor = "(I[CII)Ljava/lang/AbstractStringBuilder;", public, bridge, synthetic)]
    async fn insert_char_array_range(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        index: i32,
        array: ClassInstanceRef<Array<JavaChar>>,
        offset: i32,
        length: i32,
    ) -> Result<ClassInstanceRef<Self>> {
        let _: ClassInstanceRef<AbstractStringBuilder> = jvm
            .invoke_special(
                &this,
                "java/lang/AbstractStringBuilder",
                "insert",
                "(I[CII)Ljava/lang/AbstractStringBuilder;",
                (index, array, offset, length),
            )
            .await?;
        Ok(this)
    }

    #[method(name = "insert", public)]
    #[method(
        name = "insert",
        descriptor = "(ILjava/lang/CharSequence;)Ljava/lang/AbstractStringBuilder;",
        public,
        bridge,
        synthetic
    )]
    async fn insert_char_sequence(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        offset: i32,
        sequence: ClassInstanceRef<CharSequence>,
    ) -> Result<ClassInstanceRef<Self>> {
        if sequence.is_null() {
            let sequence = JavaLangString::from_rust_string(jvm, "null").await?;
            let _: ClassInstanceRef<Self> = jvm
                .invoke_virtual(
                    &this,
                    "java/lang/StringBuffer",
                    "insert",
                    "(ILjava/lang/String;)Ljava/lang/StringBuffer;",
                    (offset, sequence),
                )
                .await?;
        } else if jvm.is_instance(&**sequence, "java/lang/String") {
            let sequence: ClassInstanceRef<String> = ClassInstanceRef::new(sequence.instance);
            let _: ClassInstanceRef<Self> = jvm
                .invoke_virtual(
                    &this,
                    "java/lang/StringBuffer",
                    "insert",
                    "(ILjava/lang/String;)Ljava/lang/StringBuffer;",
                    (offset, sequence),
                )
                .await?;
        } else {
            let length: i32 = jvm
                .invoke_virtual(&sequence, &sequence.class_definition().name(), "length", "()I", ())
                .await?;
            let _: ClassInstanceRef<Self> = jvm
                .invoke_virtual(
                    &this,
                    "java/lang/StringBuffer",
                    "insert",
                    "(ILjava/lang/CharSequence;II)Ljava/lang/StringBuffer;",
                    (offset, sequence, 0, length),
                )
                .await?;
        }
        Ok(this)
    }

    #[method(name = "insert", public, synchronized)]
    #[method(
        name = "insert",
        descriptor = "(ILjava/lang/CharSequence;II)Ljava/lang/AbstractStringBuilder;",
        public,
        bridge,
        synthetic
    )]
    async fn insert_char_sequence_range(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        offset: i32,
        sequence: ClassInstanceRef<CharSequence>,
        start: i32,
        end: i32,
    ) -> Result<ClassInstanceRef<Self>> {
        let _: ClassInstanceRef<AbstractStringBuilder> = jvm
            .invoke_special(
                &this,
                "java/lang/AbstractStringBuilder",
                "insert",
                "(ILjava/lang/CharSequence;II)Ljava/lang/AbstractStringBuilder;",
                (offset, sequence, start, end),
            )
            .await?;
        Ok(this)
    }

    #[method(public, synchronized)]
    #[method(descriptor = "(II)Ljava/lang/AbstractStringBuilder;", public, bridge, synthetic)]
    async fn delete(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, start: i32, end: i32) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.StringBuffer::delete({this:?}, {start}, {end})");

        let count: i32 = jvm.get_field(&this, "java/lang/StringBuffer", "count", "I").await?;
        let end = end.min(count);
        if start < 0 || start > end {
            return Err(jvm
                .exception(
                    "java/lang/StringIndexOutOfBoundsException",
                    &alloc::format!("start {start}, end {end}, length {count}"),
                )
                .await);
        }

        if start != end {
            let mut value: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(&this, "java/lang/StringBuffer", "value", "[C").await?;
            let tail: Vec<JavaChar> = jvm.load_array(&value, end as usize, (count - end) as usize).await?;
            jvm.store_array(&mut value, start as usize, tail).await?;
            jvm.put_field(&mut this, "java/lang/StringBuffer", "count", "I", count - (end - start))
                .await?;
        }
        Ok(this)
    }

    #[method(public, synchronized)]
    #[method(descriptor = "(I)Ljava/lang/AbstractStringBuilder;", public, bridge, synthetic)]
    async fn delete_char_at(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, index: i32) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.StringBuffer::deleteCharAt({this:?}, {index})");

        let count: i32 = jvm.get_field(&this, "java/lang/StringBuffer", "count", "I").await?;
        if index < 0 || index >= count {
            return Err(jvm
                .exception(
                    "java/lang/StringIndexOutOfBoundsException",
                    &alloc::format!("index {index}, length {count}"),
                )
                .await);
        }

        let mut value: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(&this, "java/lang/StringBuffer", "value", "[C").await?;
        let tail: Vec<JavaChar> = jvm.load_array(&value, (index + 1) as usize, (count - index - 1) as usize).await?;
        jvm.store_array(&mut value, index as usize, tail).await?;
        jvm.put_field(&mut this, "java/lang/StringBuffer", "count", "I", count - 1).await?;
        Ok(this)
    }

    #[method(public, synchronized)]
    #[method(descriptor = "(IILjava/lang/String;)Ljava/lang/AbstractStringBuilder;", public, bridge, synthetic)]
    async fn replace(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        start: i32,
        end: i32,
        string: ClassInstanceRef<String>,
    ) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.StringBuffer::replace({this:?}, {start}, {end}, {string:?})");

        let count: i32 = jvm.get_field(&this, "java/lang/StringBuffer", "count", "I").await?;
        let end = end.min(count);
        if start < 0 || start > count || start > end {
            return Err(jvm
                .exception(
                    "java/lang/StringIndexOutOfBoundsException",
                    &alloc::format!("start {start}, end {end}, length {count}"),
                )
                .await);
        }
        if string.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "str is null").await);
        }

        let replacement = JavaLangString::to_utf16(jvm, &string).await?;
        let replacement_length = replacement.len() as i32;
        let new_count = count + replacement_length - (end - start);
        let mut value = Self::expand_capacity(jvm, &mut this, new_count).await?;
        let tail: Vec<JavaChar> = jvm.load_array(&value, end as usize, (count - end) as usize).await?;
        jvm.store_array(&mut value, (start + replacement_length) as usize, tail).await?;
        jvm.store_array(&mut value, start as usize, replacement).await?;
        jvm.put_field(&mut this, "java/lang/StringBuffer", "count", "I", new_count).await?;
        Ok(this)
    }

    #[method(public, synchronized)]
    async fn substring(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, start: i32) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.lang.StringBuffer::substring({this:?}, {start})");

        let count: i32 = jvm.get_field(&this, "java/lang/StringBuffer", "count", "I").await?;
        jvm.invoke_virtual(&this, "java/lang/StringBuffer", "substring", "(II)Ljava/lang/String;", (start, count))
            .await
    }

    #[method(name = "substring", public, synchronized)]
    async fn substring_range(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        start: i32,
        end: i32,
    ) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.lang.StringBuffer::substring({this:?}, {start}, {end})");

        let count: i32 = jvm.get_field(&this, "java/lang/StringBuffer", "count", "I").await?;
        if start < 0 || end > count || start > end {
            return Err(jvm
                .exception(
                    "java/lang/StringIndexOutOfBoundsException",
                    &alloc::format!("start {start}, end {end}, length {count}"),
                )
                .await);
        }

        let value: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(&this, "java/lang/StringBuffer", "value", "[C").await?;
        Ok(jvm.new_class("java/lang/String", "([CII)V", (value, start, end - start)).await?.into())
    }

    #[method(public, synchronized)]
    async fn sub_sequence(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        start: i32,
        end: i32,
    ) -> Result<ClassInstanceRef<CharSequence>> {
        tracing::debug!("java.lang.StringBuffer::subSequence({this:?}, {start}, {end})");

        jvm.invoke_virtual(&this, "java/lang/StringBuffer", "substring", "(II)Ljava/lang/String;", (start, end))
            .await
    }

    #[method(public, synchronized)]
    #[method(descriptor = "()Ljava/lang/AbstractStringBuilder;", public, bridge, synthetic)]
    async fn reverse(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Self>> {
        tracing::debug!("java.lang.StringBuffer::reverse({this:?})");

        let count: i32 = jvm.get_field(&this, "java/lang/StringBuffer", "count", "I").await?;
        let mut value: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(&this, "java/lang/StringBuffer", "value", "[C").await?;
        let mut chars: Vec<JavaChar> = jvm.load_array(&value, 0, count as usize).await?;
        chars.reverse();
        jvm.store_array(&mut value, 0, chars).await?;
        Ok(this)
    }

    #[method(public)]
    async fn index_of(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, string: ClassInstanceRef<String>) -> Result<i32> {
        jvm.invoke_special(&this, "java/lang/AbstractStringBuilder", "indexOf", "(Ljava/lang/String;)I", (string,))
            .await
    }

    #[method(name = "indexOf", public, synchronized)]
    async fn index_of_from(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        string: ClassInstanceRef<String>,
        from_index: i32,
    ) -> Result<i32> {
        jvm.invoke_special(
            &this,
            "java/lang/AbstractStringBuilder",
            "indexOf",
            "(Ljava/lang/String;I)I",
            (string, from_index),
        )
        .await
    }

    #[method(public)]
    async fn last_index_of(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, string: ClassInstanceRef<String>) -> Result<i32> {
        jvm.invoke_special(
            &this,
            "java/lang/AbstractStringBuilder",
            "lastIndexOf",
            "(Ljava/lang/String;)I",
            (string,),
        )
        .await
    }

    #[method(name = "lastIndexOf", public, synchronized)]
    async fn last_index_of_from(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        string: ClassInstanceRef<String>,
        from_index: i32,
    ) -> Result<i32> {
        jvm.invoke_special(
            &this,
            "java/lang/AbstractStringBuilder",
            "lastIndexOf",
            "(Ljava/lang/String;I)I",
            (string, from_index),
        )
        .await
    }

    #[method(public, synchronized)]
    async fn to_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        tracing::debug!("java.lang.StringBuffer::toString({this:?})");

        let value: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(&this, "java/lang/StringBuffer", "value", "[C").await?;
        let count: i32 = jvm.get_field(&this, "java/lang/StringBuffer", "count", "I").await?;

        // the buffer stays mutable, so snapshot it instead of handing it to the sharing constructor
        let chars: Vec<JavaChar> = jvm.load_array(&value, 0, count as usize).await?;

        Ok(JavaLangString::from_utf16(jvm, chars).await?.into())
    }

    #[method(public, synchronized)]
    async fn capacity(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.lang.StringBuffer::capacity({this:?})");

        let value: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(&this, "java/lang/StringBuffer", "value", "[C").await?;
        Ok(jvm.array_length(&value).await? as i32)
    }

    #[method(public, synchronized)]
    async fn ensure_capacity(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, minimum_capacity: i32) -> Result<()> {
        tracing::debug!("java.lang.StringBuffer::ensureCapacity({this:?}, {minimum_capacity})");

        if minimum_capacity > 0 {
            Self::expand_capacity(jvm, &mut this, minimum_capacity).await?;
        }
        Ok(())
    }

    #[method(public, synchronized)]
    async fn trim_to_size(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.invoke_special(&this, "java/lang/AbstractStringBuilder", "trimToSize", "()V", ())
            .await
    }

    #[method(public, synchronized)]
    async fn set_length(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, new_length: i32) -> Result<()> {
        tracing::debug!("java.lang.StringBuffer::setLength({this:?}, {new_length})");

        if new_length < 0 {
            return Err(jvm.exception("java/lang/StringIndexOutOfBoundsException", &new_length.to_string()).await);
        }

        let count: i32 = jvm.get_field(&this, "java/lang/StringBuffer", "count", "I").await?;
        if new_length > count {
            let mut value = Self::expand_capacity(jvm, &mut this, new_length).await?;
            jvm.store_array(&mut value, count as usize, vec![0 as JavaChar; (new_length - count) as usize])
                .await?;
        }
        jvm.put_field(&mut this, "java/lang/StringBuffer", "count", "I", new_length).await?;
        Ok(())
    }

    #[method(public, synchronized)]
    async fn length(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.lang.StringBuffer::length({this:?})");

        jvm.get_field(&this, "java/lang/StringBuffer", "count", "I").await
    }

    #[method(public, synchronized)]
    async fn char_at(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, index: i32) -> Result<JavaChar> {
        tracing::debug!("java.lang.StringBuffer::charAt({this:?}, {index})");

        let count: i32 = jvm.get_field(&this, "java/lang/StringBuffer", "count", "I").await?;
        if index < 0 || index >= count {
            return Err(jvm
                .exception(
                    "java/lang/StringIndexOutOfBoundsException",
                    &alloc::format!("index {index}, length {count}"),
                )
                .await);
        }

        let value: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(&this, "java/lang/StringBuffer", "value", "[C").await?;
        Ok(jvm.load_array(&value, index as usize, 1).await?[0])
    }

    #[method(public, synchronized)]
    async fn code_point_at(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, index: i32) -> Result<i32> {
        jvm.invoke_special(&this, "java/lang/AbstractStringBuilder", "codePointAt", "(I)I", (index,))
            .await
    }

    #[method(public, synchronized)]
    async fn code_point_before(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, index: i32) -> Result<i32> {
        jvm.invoke_special(&this, "java/lang/AbstractStringBuilder", "codePointBefore", "(I)I", (index,))
            .await
    }

    #[method(public, synchronized)]
    async fn code_point_count(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, begin: i32, end: i32) -> Result<i32> {
        jvm.invoke_special(&this, "java/lang/AbstractStringBuilder", "codePointCount", "(II)I", (begin, end))
            .await
    }

    #[method(public, synchronized)]
    async fn offset_by_code_points(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, index: i32, offset: i32) -> Result<i32> {
        jvm.invoke_special(&this, "java/lang/AbstractStringBuilder", "offsetByCodePoints", "(II)I", (index, offset))
            .await
    }

    #[method(public, synchronized)]
    async fn set_char_at(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, index: i32, character: JavaChar) -> Result<()> {
        tracing::debug!("java.lang.StringBuffer::setCharAt({this:?}, {index}, {character})");

        let count: i32 = jvm.get_field(&this, "java/lang/StringBuffer", "count", "I").await?;
        if index < 0 || index >= count {
            return Err(jvm
                .exception(
                    "java/lang/StringIndexOutOfBoundsException",
                    &alloc::format!("index {index}, length {count}"),
                )
                .await);
        }

        let mut value: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(&this, "java/lang/StringBuffer", "value", "[C").await?;
        jvm.store_array(&mut value, index as usize, [character]).await
    }

    #[method(public, synchronized)]
    async fn get_chars(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        src_begin: i32,
        src_end: i32,
        mut destination: ClassInstanceRef<Array<JavaChar>>,
        dst_begin: i32,
    ) -> Result<()> {
        tracing::debug!("java.lang.StringBuffer::getChars({this:?}, {src_begin}, {src_end}, {destination:?}, {dst_begin})");

        let count: i32 = jvm.get_field(&this, "java/lang/StringBuffer", "count", "I").await?;
        if src_begin < 0 || src_end > count || src_begin > src_end {
            return Err(jvm
                .exception(
                    "java/lang/StringIndexOutOfBoundsException",
                    &alloc::format!("start {src_begin}, end {src_end}, length {count}"),
                )
                .await);
        }
        if destination.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "dst is null").await);
        }

        let copy_length = src_end - src_begin;
        let destination_length = jvm.array_length(&destination).await? as i32;
        if dst_begin < 0 || dst_begin > destination_length - copy_length {
            return Err(jvm
                .exception(
                    "java/lang/ArrayIndexOutOfBoundsException",
                    &alloc::format!("offset {dst_begin}, length {copy_length}, array length {destination_length}"),
                )
                .await);
        }

        let value: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(&this, "java/lang/StringBuffer", "value", "[C").await?;
        let chars: Vec<JavaChar> = jvm.load_array(&value, src_begin as usize, copy_length as usize).await?;
        jvm.store_array(&mut destination, dst_begin as usize, chars).await
    }

    async fn expand_capacity(jvm: &Jvm, this: &mut ClassInstanceRef<Self>, minimum_capacity: i32) -> Result<ClassInstanceRef<Array<JavaChar>>> {
        let value: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(this, "java/lang/StringBuffer", "value", "[C").await?;
        let old_capacity = jvm.array_length(&value).await? as i32;
        if minimum_capacity <= old_capacity {
            return Ok(value);
        }

        let new_capacity = minimum_capacity.max(old_capacity.saturating_mul(2).saturating_add(2));
        let count: i32 = jvm.get_field(this, "java/lang/StringBuffer", "count", "I").await?;
        let chars: Vec<JavaChar> = jvm.load_array(&value, 0, count as usize).await?;
        let mut new_value = jvm.instantiate_array("C", new_capacity as usize).await?;
        jvm.store_array(&mut new_value, 0, chars).await?;
        jvm.put_field(this, "java/lang/StringBuffer", "value", "[C", new_value.clone()).await?;
        Ok(new_value.into())
    }

    async fn append_utf16(jvm: &Jvm, this: &mut ClassInstanceRef<Self>, chars: Vec<JavaChar>) -> Result<()> {
        let count: i32 = jvm.get_field(this, "java/lang/StringBuffer", "count", "I").await?;
        let new_count = count + chars.len() as i32;
        let mut value = Self::expand_capacity(jvm, this, new_count).await?;
        jvm.store_array(&mut value, count as usize, chars).await?;
        jvm.put_field(this, "java/lang/StringBuffer", "count", "I", new_count).await
    }

    async fn insert_utf16(jvm: &Jvm, this: &mut ClassInstanceRef<Self>, offset: i32, chars: Vec<JavaChar>) -> Result<()> {
        let count: i32 = jvm.get_field(this, "java/lang/StringBuffer", "count", "I").await?;
        if offset < 0 || offset > count {
            return Err(jvm
                .exception(
                    "java/lang/StringIndexOutOfBoundsException",
                    &alloc::format!("offset {offset}, length {count}"),
                )
                .await);
        }

        let inserted_length = chars.len() as i32;
        let new_count = count + inserted_length;
        let mut value = Self::expand_capacity(jvm, this, new_count).await?;
        let tail: Vec<JavaChar> = jvm.load_array(&value, offset as usize, (count - offset) as usize).await?;
        jvm.store_array(&mut value, (offset + inserted_length) as usize, tail).await?;
        jvm.store_array(&mut value, offset as usize, chars).await?;
        jvm.put_field(this, "java/lang/StringBuffer", "count", "I", new_count).await
    }
}
