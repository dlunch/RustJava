use jvm::{Array, ClassInstanceRef, JavaChar, Jvm, Result, runtime::JavaLangString};
use jvm_class_proto::{java_class, java_methods};

use crate::{
    RuntimeContext,
    classes::java::lang::{AbstractStringBuilder, CharSequence, Object, String, StringBuffer},
};

// public final class java.lang.StringBuilder
#[java_class(
    name = "java/lang/StringBuilder",
    extends = "java/lang/AbstractStringBuilder",
    implements("java/io/Serializable", "java/lang/CharSequence"),
    public,
    final
)]
pub struct StringBuilder;

#[java_methods]
impl StringBuilder {
    #[method(constructor, public)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.invoke_special(&this, "java/lang/AbstractStringBuilder", "<init>", "(I)V", (16,))
            .await
    }

    #[method(constructor, public)]
    async fn init_with_capacity(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, capacity: i32) -> Result<()> {
        jvm.invoke_special(&this, "java/lang/AbstractStringBuilder", "<init>", "(I)V", (capacity,))
            .await
    }

    #[method(constructor, public)]
    async fn init_with_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, string: ClassInstanceRef<String>) -> Result<()> {
        if string.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "str is null").await);
        }
        let characters = JavaLangString::to_utf16(jvm, &string).await?;
        let _: () = jvm
            .invoke_special(
                &this,
                "java/lang/AbstractStringBuilder",
                "<init>",
                "(I)V",
                ((characters.len() + 16) as i32,),
            )
            .await?;
        let _: ClassInstanceRef<AbstractStringBuilder> = jvm
            .invoke_special(
                &this,
                "java/lang/AbstractStringBuilder",
                "append",
                "(Ljava/lang/String;)Ljava/lang/AbstractStringBuilder;",
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
        let _: ClassInstanceRef<AbstractStringBuilder> = jvm
            .invoke_special(
                &this,
                "java/lang/AbstractStringBuilder",
                "append",
                "(Ljava/lang/CharSequence;)Ljava/lang/AbstractStringBuilder;",
                (sequence,),
            )
            .await?;
        Ok(())
    }

    #[method(name = "append", public)]
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
        let _: ClassInstanceRef<AbstractStringBuilder> = jvm
            .invoke_special(
                &this,
                "java/lang/AbstractStringBuilder",
                "append",
                "(Ljava/lang/Object;)Ljava/lang/AbstractStringBuilder;",
                (object,),
            )
            .await?;
        Ok(this)
    }

    #[method(name = "append", public)]
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
        this: ClassInstanceRef<Self>,
        string: ClassInstanceRef<String>,
    ) -> Result<ClassInstanceRef<Self>> {
        let _: ClassInstanceRef<AbstractStringBuilder> = jvm
            .invoke_special(
                &this,
                "java/lang/AbstractStringBuilder",
                "append",
                "(Ljava/lang/String;)Ljava/lang/AbstractStringBuilder;",
                (string,),
            )
            .await?;
        Ok(this)
    }

    #[method(name = "append", public)]
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
        buffer: ClassInstanceRef<StringBuffer>,
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

    #[method(name = "append", public)]
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

    #[method(name = "append", public)]
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

    #[method(name = "append", public)]
    #[method(name = "append", descriptor = "([C)Ljava/lang/AbstractStringBuilder;", public, bridge, synthetic)]
    async fn append_char_array(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        array: ClassInstanceRef<Array<JavaChar>>,
    ) -> Result<ClassInstanceRef<Self>> {
        let _: ClassInstanceRef<AbstractStringBuilder> = jvm
            .invoke_special(
                &this,
                "java/lang/AbstractStringBuilder",
                "append",
                "([C)Ljava/lang/AbstractStringBuilder;",
                (array,),
            )
            .await?;
        Ok(this)
    }

    #[method(name = "append", public)]
    #[method(name = "append", descriptor = "([CII)Ljava/lang/AbstractStringBuilder;", public, bridge, synthetic)]
    async fn append_char_array_range(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        array: ClassInstanceRef<Array<JavaChar>>,
        offset: i32,
        length: i32,
    ) -> Result<ClassInstanceRef<Self>> {
        let _: ClassInstanceRef<AbstractStringBuilder> = jvm
            .invoke_special(
                &this,
                "java/lang/AbstractStringBuilder",
                "append",
                "([CII)Ljava/lang/AbstractStringBuilder;",
                (array, offset, length),
            )
            .await?;
        Ok(this)
    }

    #[method(name = "append", public)]
    #[method(name = "append", descriptor = "(Z)Ljava/lang/AbstractStringBuilder;", public, bridge, synthetic)]
    async fn append_boolean(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: bool) -> Result<ClassInstanceRef<Self>> {
        let _: ClassInstanceRef<AbstractStringBuilder> = jvm
            .invoke_special(
                &this,
                "java/lang/AbstractStringBuilder",
                "append",
                "(Z)Ljava/lang/AbstractStringBuilder;",
                (value,),
            )
            .await?;
        Ok(this)
    }

    #[method(name = "append", public)]
    #[method(name = "append", descriptor = "(C)Ljava/lang/AbstractStringBuilder;", public, bridge, synthetic)]
    #[method(name = "append", descriptor = "(C)Ljava/lang/Appendable;", public, bridge, synthetic)]
    async fn append_char(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: JavaChar) -> Result<ClassInstanceRef<Self>> {
        let _: ClassInstanceRef<AbstractStringBuilder> = jvm
            .invoke_special(
                &this,
                "java/lang/AbstractStringBuilder",
                "append",
                "(C)Ljava/lang/AbstractStringBuilder;",
                (value,),
            )
            .await?;
        Ok(this)
    }

    #[method(name = "append", public)]
    #[method(name = "append", descriptor = "(I)Ljava/lang/AbstractStringBuilder;", public, bridge, synthetic)]
    async fn append_int(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: i32) -> Result<ClassInstanceRef<Self>> {
        let _: ClassInstanceRef<AbstractStringBuilder> = jvm
            .invoke_special(
                &this,
                "java/lang/AbstractStringBuilder",
                "append",
                "(I)Ljava/lang/AbstractStringBuilder;",
                (value,),
            )
            .await?;
        Ok(this)
    }

    #[method(name = "append", public)]
    #[method(name = "append", descriptor = "(J)Ljava/lang/AbstractStringBuilder;", public, bridge, synthetic)]
    async fn append_long(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: i64) -> Result<ClassInstanceRef<Self>> {
        let _: ClassInstanceRef<AbstractStringBuilder> = jvm
            .invoke_special(
                &this,
                "java/lang/AbstractStringBuilder",
                "append",
                "(J)Ljava/lang/AbstractStringBuilder;",
                (value,),
            )
            .await?;
        Ok(this)
    }

    #[method(name = "append", public)]
    #[method(name = "append", descriptor = "(F)Ljava/lang/AbstractStringBuilder;", public, bridge, synthetic)]
    async fn append_float(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: f32) -> Result<ClassInstanceRef<Self>> {
        let _: ClassInstanceRef<AbstractStringBuilder> = jvm
            .invoke_special(
                &this,
                "java/lang/AbstractStringBuilder",
                "append",
                "(F)Ljava/lang/AbstractStringBuilder;",
                (value,),
            )
            .await?;
        Ok(this)
    }

    #[method(name = "append", public)]
    #[method(name = "append", descriptor = "(D)Ljava/lang/AbstractStringBuilder;", public, bridge, synthetic)]
    async fn append_double(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: f64) -> Result<ClassInstanceRef<Self>> {
        let _: ClassInstanceRef<AbstractStringBuilder> = jvm
            .invoke_special(
                &this,
                "java/lang/AbstractStringBuilder",
                "append",
                "(D)Ljava/lang/AbstractStringBuilder;",
                (value,),
            )
            .await?;
        Ok(this)
    }

    #[method(public)]
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

    #[method(public)]
    #[method(descriptor = "(II)Ljava/lang/AbstractStringBuilder;", public, bridge, synthetic)]
    async fn delete(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, start: i32, end: i32) -> Result<ClassInstanceRef<Self>> {
        let _: ClassInstanceRef<AbstractStringBuilder> = jvm
            .invoke_special(
                &this,
                "java/lang/AbstractStringBuilder",
                "delete",
                "(II)Ljava/lang/AbstractStringBuilder;",
                (start, end),
            )
            .await?;
        Ok(this)
    }

    #[method(public)]
    #[method(descriptor = "(I)Ljava/lang/AbstractStringBuilder;", public, bridge, synthetic)]
    async fn delete_char_at(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, index: i32) -> Result<ClassInstanceRef<Self>> {
        let _: ClassInstanceRef<AbstractStringBuilder> = jvm
            .invoke_special(
                &this,
                "java/lang/AbstractStringBuilder",
                "deleteCharAt",
                "(I)Ljava/lang/AbstractStringBuilder;",
                (index,),
            )
            .await?;
        Ok(this)
    }

    #[method(public)]
    #[method(descriptor = "(IILjava/lang/String;)Ljava/lang/AbstractStringBuilder;", public, bridge, synthetic)]
    async fn replace(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        start: i32,
        end: i32,
        string: ClassInstanceRef<String>,
    ) -> Result<ClassInstanceRef<Self>> {
        let _: ClassInstanceRef<AbstractStringBuilder> = jvm
            .invoke_special(
                &this,
                "java/lang/AbstractStringBuilder",
                "replace",
                "(IILjava/lang/String;)Ljava/lang/AbstractStringBuilder;",
                (start, end, string),
            )
            .await?;
        Ok(this)
    }

    #[method(name = "insert", public)]
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
        let _: ClassInstanceRef<AbstractStringBuilder> = jvm
            .invoke_special(
                &this,
                "java/lang/AbstractStringBuilder",
                "insert",
                "(ILjava/lang/Object;)Ljava/lang/AbstractStringBuilder;",
                (offset, object),
            )
            .await?;
        Ok(this)
    }

    #[method(name = "insert", public)]
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
        this: ClassInstanceRef<Self>,
        offset: i32,
        string: ClassInstanceRef<String>,
    ) -> Result<ClassInstanceRef<Self>> {
        let _: ClassInstanceRef<AbstractStringBuilder> = jvm
            .invoke_special(
                &this,
                "java/lang/AbstractStringBuilder",
                "insert",
                "(ILjava/lang/String;)Ljava/lang/AbstractStringBuilder;",
                (offset, string),
            )
            .await?;
        Ok(this)
    }

    #[method(name = "insert", public)]
    #[method(name = "insert", descriptor = "(I[C)Ljava/lang/AbstractStringBuilder;", public, bridge, synthetic)]
    async fn insert_char_array(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        offset: i32,
        array: ClassInstanceRef<Array<JavaChar>>,
    ) -> Result<ClassInstanceRef<Self>> {
        let _: ClassInstanceRef<AbstractStringBuilder> = jvm
            .invoke_special(
                &this,
                "java/lang/AbstractStringBuilder",
                "insert",
                "(I[C)Ljava/lang/AbstractStringBuilder;",
                (offset, array),
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
        let _: ClassInstanceRef<AbstractStringBuilder> = jvm
            .invoke_special(
                &this,
                "java/lang/AbstractStringBuilder",
                "insert",
                "(ILjava/lang/CharSequence;)Ljava/lang/AbstractStringBuilder;",
                (offset, sequence),
            )
            .await?;
        Ok(this)
    }

    #[method(name = "insert", public)]
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

    #[method(name = "insert", public)]
    #[method(name = "insert", descriptor = "(IZ)Ljava/lang/AbstractStringBuilder;", public, bridge, synthetic)]
    async fn insert_boolean(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        offset: i32,
        value: bool,
    ) -> Result<ClassInstanceRef<Self>> {
        let _: ClassInstanceRef<AbstractStringBuilder> = jvm
            .invoke_special(
                &this,
                "java/lang/AbstractStringBuilder",
                "insert",
                "(IZ)Ljava/lang/AbstractStringBuilder;",
                (offset, value),
            )
            .await?;
        Ok(this)
    }

    #[method(name = "insert", public)]
    #[method(name = "insert", descriptor = "(IC)Ljava/lang/AbstractStringBuilder;", public, bridge, synthetic)]
    async fn insert_char(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        offset: i32,
        value: JavaChar,
    ) -> Result<ClassInstanceRef<Self>> {
        let _: ClassInstanceRef<AbstractStringBuilder> = jvm
            .invoke_special(
                &this,
                "java/lang/AbstractStringBuilder",
                "insert",
                "(IC)Ljava/lang/AbstractStringBuilder;",
                (offset, value),
            )
            .await?;
        Ok(this)
    }

    #[method(name = "insert", public)]
    #[method(name = "insert", descriptor = "(II)Ljava/lang/AbstractStringBuilder;", public, bridge, synthetic)]
    async fn insert_int(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, offset: i32, value: i32) -> Result<ClassInstanceRef<Self>> {
        let _: ClassInstanceRef<AbstractStringBuilder> = jvm
            .invoke_special(
                &this,
                "java/lang/AbstractStringBuilder",
                "insert",
                "(II)Ljava/lang/AbstractStringBuilder;",
                (offset, value),
            )
            .await?;
        Ok(this)
    }

    #[method(name = "insert", public)]
    #[method(name = "insert", descriptor = "(IJ)Ljava/lang/AbstractStringBuilder;", public, bridge, synthetic)]
    async fn insert_long(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, offset: i32, value: i64) -> Result<ClassInstanceRef<Self>> {
        let _: ClassInstanceRef<AbstractStringBuilder> = jvm
            .invoke_special(
                &this,
                "java/lang/AbstractStringBuilder",
                "insert",
                "(IJ)Ljava/lang/AbstractStringBuilder;",
                (offset, value),
            )
            .await?;
        Ok(this)
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
        let _: ClassInstanceRef<AbstractStringBuilder> = jvm
            .invoke_special(
                &this,
                "java/lang/AbstractStringBuilder",
                "insert",
                "(IF)Ljava/lang/AbstractStringBuilder;",
                (offset, value),
            )
            .await?;
        Ok(this)
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
        let _: ClassInstanceRef<AbstractStringBuilder> = jvm
            .invoke_special(
                &this,
                "java/lang/AbstractStringBuilder",
                "insert",
                "(ID)Ljava/lang/AbstractStringBuilder;",
                (offset, value),
            )
            .await?;
        Ok(this)
    }

    #[method(public)]
    async fn index_of(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, string: ClassInstanceRef<String>) -> Result<i32> {
        jvm.invoke_special(&this, "java/lang/AbstractStringBuilder", "indexOf", "(Ljava/lang/String;)I", (string,))
            .await
    }

    #[method(name = "indexOf", public)]
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

    #[method(name = "lastIndexOf", public)]
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

    #[method(public)]
    #[method(descriptor = "()Ljava/lang/AbstractStringBuilder;", public, bridge, synthetic)]
    async fn reverse(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Self>> {
        let _: ClassInstanceRef<AbstractStringBuilder> = jvm
            .invoke_special(
                &this,
                "java/lang/AbstractStringBuilder",
                "reverse",
                "()Ljava/lang/AbstractStringBuilder;",
                (),
            )
            .await?;
        Ok(this)
    }

    #[method(public)]
    async fn to_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        let count: i32 = jvm.get_field(&this, "java/lang/StringBuilder", "count", "I").await?;
        let value: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(&this, "java/lang/StringBuilder", "value", "[C").await?;
        Ok(JavaLangString::from_utf16(jvm, jvm.load_array(&value, 0, count as usize).await?)
            .await?
            .into())
    }
}
