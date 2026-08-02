use alloc::vec;

use java_class_proto::JavaMethodProto;
use java_constants::{ClassAccessFlags, MethodAccessFlags};
use jvm::{Array, ClassInstanceRef, JavaChar, Jvm, Result, runtime::JavaLangString};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::java::lang::{AbstractStringBuilder, CharSequence, Object, String, StringBuffer},
};

// public final class java.lang.StringBuilder
pub struct StringBuilder;

impl StringBuilder {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/lang/StringBuilder",
            parent_class: Some("java/lang/AbstractStringBuilder"),
            interfaces: vec!["java/io/Serializable", "java/lang/CharSequence"],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("<init>", "(I)V", Self::init_with_capacity, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("<init>", "(Ljava/lang/String;)V", Self::init_with_string, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "<init>",
                    "(Ljava/lang/CharSequence;)V",
                    Self::init_with_char_sequence,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "append",
                    "(Ljava/lang/Object;)Ljava/lang/StringBuilder;",
                    Self::append_object,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "append",
                    "(Ljava/lang/String;)Ljava/lang/StringBuilder;",
                    Self::append_string,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "append",
                    "(Ljava/lang/StringBuffer;)Ljava/lang/StringBuilder;",
                    Self::append_string_buffer,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "append",
                    "(Ljava/lang/CharSequence;)Ljava/lang/StringBuilder;",
                    Self::append_char_sequence,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "append",
                    "(Ljava/lang/CharSequence;II)Ljava/lang/StringBuilder;",
                    Self::append_char_sequence_range,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "append",
                    "([C)Ljava/lang/StringBuilder;",
                    Self::append_char_array,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "append",
                    "([CII)Ljava/lang/StringBuilder;",
                    Self::append_char_array_range,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new("append", "(Z)Ljava/lang/StringBuilder;", Self::append_boolean, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("append", "(C)Ljava/lang/StringBuilder;", Self::append_char, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("append", "(I)Ljava/lang/StringBuilder;", Self::append_int, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("append", "(J)Ljava/lang/StringBuilder;", Self::append_long, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("append", "(F)Ljava/lang/StringBuilder;", Self::append_float, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("append", "(D)Ljava/lang/StringBuilder;", Self::append_double, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "appendCodePoint",
                    "(I)Ljava/lang/StringBuilder;",
                    Self::append_code_point,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new("delete", "(II)Ljava/lang/StringBuilder;", Self::delete, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "deleteCharAt",
                    "(I)Ljava/lang/StringBuilder;",
                    Self::delete_char_at,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "replace",
                    "(IILjava/lang/String;)Ljava/lang/StringBuilder;",
                    Self::replace,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "insert",
                    "(I[CII)Ljava/lang/StringBuilder;",
                    Self::insert_char_array_range,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "insert",
                    "(ILjava/lang/Object;)Ljava/lang/StringBuilder;",
                    Self::insert_object,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "insert",
                    "(ILjava/lang/String;)Ljava/lang/StringBuilder;",
                    Self::insert_string,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "insert",
                    "(I[C)Ljava/lang/StringBuilder;",
                    Self::insert_char_array,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "insert",
                    "(ILjava/lang/CharSequence;)Ljava/lang/StringBuilder;",
                    Self::insert_char_sequence,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "insert",
                    "(ILjava/lang/CharSequence;II)Ljava/lang/StringBuilder;",
                    Self::insert_char_sequence_range,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new("insert", "(IZ)Ljava/lang/StringBuilder;", Self::insert_boolean, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("insert", "(IC)Ljava/lang/StringBuilder;", Self::insert_char, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("insert", "(II)Ljava/lang/StringBuilder;", Self::insert_int, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("insert", "(IJ)Ljava/lang/StringBuilder;", Self::insert_long, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("insert", "(IF)Ljava/lang/StringBuilder;", Self::insert_float, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("insert", "(ID)Ljava/lang/StringBuilder;", Self::insert_double, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("indexOf", "(Ljava/lang/String;)I", Self::index_of, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("indexOf", "(Ljava/lang/String;I)I", Self::index_of_from, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("lastIndexOf", "(Ljava/lang/String;)I", Self::last_index_of, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "lastIndexOf",
                    "(Ljava/lang/String;I)I",
                    Self::last_index_of_from,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new("reverse", "()Ljava/lang/StringBuilder;", Self::reverse, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("toString", "()Ljava/lang/String;", Self::to_string, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "append",
                    "(Ljava/lang/Object;)Ljava/lang/AbstractStringBuilder;",
                    Self::append_object,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::BRIDGE | MethodAccessFlags::SYNTHETIC,
                ),
                JavaMethodProto::new(
                    "append",
                    "(Ljava/lang/String;)Ljava/lang/AbstractStringBuilder;",
                    Self::append_string,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::BRIDGE | MethodAccessFlags::SYNTHETIC,
                ),
                JavaMethodProto::new(
                    "append",
                    "(Ljava/lang/StringBuffer;)Ljava/lang/AbstractStringBuilder;",
                    Self::append_string_buffer,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::BRIDGE | MethodAccessFlags::SYNTHETIC,
                ),
                JavaMethodProto::new(
                    "append",
                    "(Ljava/lang/CharSequence;)Ljava/lang/AbstractStringBuilder;",
                    Self::append_char_sequence,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::BRIDGE | MethodAccessFlags::SYNTHETIC,
                ),
                JavaMethodProto::new(
                    "append",
                    "(Ljava/lang/CharSequence;II)Ljava/lang/AbstractStringBuilder;",
                    Self::append_char_sequence_range,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::BRIDGE | MethodAccessFlags::SYNTHETIC,
                ),
                JavaMethodProto::new(
                    "append",
                    "([C)Ljava/lang/AbstractStringBuilder;",
                    Self::append_char_array,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::BRIDGE | MethodAccessFlags::SYNTHETIC,
                ),
                JavaMethodProto::new(
                    "append",
                    "([CII)Ljava/lang/AbstractStringBuilder;",
                    Self::append_char_array_range,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::BRIDGE | MethodAccessFlags::SYNTHETIC,
                ),
                JavaMethodProto::new(
                    "append",
                    "(Z)Ljava/lang/AbstractStringBuilder;",
                    Self::append_boolean,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::BRIDGE | MethodAccessFlags::SYNTHETIC,
                ),
                JavaMethodProto::new(
                    "append",
                    "(C)Ljava/lang/AbstractStringBuilder;",
                    Self::append_char,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::BRIDGE | MethodAccessFlags::SYNTHETIC,
                ),
                JavaMethodProto::new(
                    "append",
                    "(I)Ljava/lang/AbstractStringBuilder;",
                    Self::append_int,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::BRIDGE | MethodAccessFlags::SYNTHETIC,
                ),
                JavaMethodProto::new(
                    "append",
                    "(J)Ljava/lang/AbstractStringBuilder;",
                    Self::append_long,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::BRIDGE | MethodAccessFlags::SYNTHETIC,
                ),
                JavaMethodProto::new(
                    "append",
                    "(F)Ljava/lang/AbstractStringBuilder;",
                    Self::append_float,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::BRIDGE | MethodAccessFlags::SYNTHETIC,
                ),
                JavaMethodProto::new(
                    "append",
                    "(D)Ljava/lang/AbstractStringBuilder;",
                    Self::append_double,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::BRIDGE | MethodAccessFlags::SYNTHETIC,
                ),
                JavaMethodProto::new(
                    "appendCodePoint",
                    "(I)Ljava/lang/AbstractStringBuilder;",
                    Self::append_code_point,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::BRIDGE | MethodAccessFlags::SYNTHETIC,
                ),
                JavaMethodProto::new(
                    "delete",
                    "(II)Ljava/lang/AbstractStringBuilder;",
                    Self::delete,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::BRIDGE | MethodAccessFlags::SYNTHETIC,
                ),
                JavaMethodProto::new(
                    "deleteCharAt",
                    "(I)Ljava/lang/AbstractStringBuilder;",
                    Self::delete_char_at,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::BRIDGE | MethodAccessFlags::SYNTHETIC,
                ),
                JavaMethodProto::new(
                    "replace",
                    "(IILjava/lang/String;)Ljava/lang/AbstractStringBuilder;",
                    Self::replace,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::BRIDGE | MethodAccessFlags::SYNTHETIC,
                ),
                JavaMethodProto::new(
                    "insert",
                    "(I[CII)Ljava/lang/AbstractStringBuilder;",
                    Self::insert_char_array_range,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::BRIDGE | MethodAccessFlags::SYNTHETIC,
                ),
                JavaMethodProto::new(
                    "insert",
                    "(ILjava/lang/Object;)Ljava/lang/AbstractStringBuilder;",
                    Self::insert_object,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::BRIDGE | MethodAccessFlags::SYNTHETIC,
                ),
                JavaMethodProto::new(
                    "insert",
                    "(ILjava/lang/String;)Ljava/lang/AbstractStringBuilder;",
                    Self::insert_string,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::BRIDGE | MethodAccessFlags::SYNTHETIC,
                ),
                JavaMethodProto::new(
                    "insert",
                    "(I[C)Ljava/lang/AbstractStringBuilder;",
                    Self::insert_char_array,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::BRIDGE | MethodAccessFlags::SYNTHETIC,
                ),
                JavaMethodProto::new(
                    "insert",
                    "(ILjava/lang/CharSequence;)Ljava/lang/AbstractStringBuilder;",
                    Self::insert_char_sequence,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::BRIDGE | MethodAccessFlags::SYNTHETIC,
                ),
                JavaMethodProto::new(
                    "insert",
                    "(ILjava/lang/CharSequence;II)Ljava/lang/AbstractStringBuilder;",
                    Self::insert_char_sequence_range,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::BRIDGE | MethodAccessFlags::SYNTHETIC,
                ),
                JavaMethodProto::new(
                    "insert",
                    "(IZ)Ljava/lang/AbstractStringBuilder;",
                    Self::insert_boolean,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::BRIDGE | MethodAccessFlags::SYNTHETIC,
                ),
                JavaMethodProto::new(
                    "insert",
                    "(IC)Ljava/lang/AbstractStringBuilder;",
                    Self::insert_char,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::BRIDGE | MethodAccessFlags::SYNTHETIC,
                ),
                JavaMethodProto::new(
                    "insert",
                    "(II)Ljava/lang/AbstractStringBuilder;",
                    Self::insert_int,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::BRIDGE | MethodAccessFlags::SYNTHETIC,
                ),
                JavaMethodProto::new(
                    "insert",
                    "(IJ)Ljava/lang/AbstractStringBuilder;",
                    Self::insert_long,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::BRIDGE | MethodAccessFlags::SYNTHETIC,
                ),
                JavaMethodProto::new(
                    "insert",
                    "(IF)Ljava/lang/AbstractStringBuilder;",
                    Self::insert_float,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::BRIDGE | MethodAccessFlags::SYNTHETIC,
                ),
                JavaMethodProto::new(
                    "insert",
                    "(ID)Ljava/lang/AbstractStringBuilder;",
                    Self::insert_double,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::BRIDGE | MethodAccessFlags::SYNTHETIC,
                ),
                JavaMethodProto::new(
                    "reverse",
                    "()Ljava/lang/AbstractStringBuilder;",
                    Self::reverse,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::BRIDGE | MethodAccessFlags::SYNTHETIC,
                ),
                JavaMethodProto::new(
                    "append",
                    "(Ljava/lang/CharSequence;)Ljava/lang/Appendable;",
                    Self::append_char_sequence,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::BRIDGE | MethodAccessFlags::SYNTHETIC,
                ),
                JavaMethodProto::new(
                    "append",
                    "(Ljava/lang/CharSequence;II)Ljava/lang/Appendable;",
                    Self::append_char_sequence_range,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::BRIDGE | MethodAccessFlags::SYNTHETIC,
                ),
                JavaMethodProto::new(
                    "append",
                    "(C)Ljava/lang/Appendable;",
                    Self::append_char,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::BRIDGE | MethodAccessFlags::SYNTHETIC,
                ),
            ],
            fields: vec![],
            access_flags: ClassAccessFlags::PUBLIC | ClassAccessFlags::FINAL,
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.invoke_special(&this, "java/lang/AbstractStringBuilder", "<init>", "(I)V", (16,))
            .await
    }

    async fn init_with_capacity(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, capacity: i32) -> Result<()> {
        jvm.invoke_special(&this, "java/lang/AbstractStringBuilder", "<init>", "(I)V", (capacity,))
            .await
    }

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

    async fn init_with_char_sequence(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        sequence: ClassInstanceRef<CharSequence>,
    ) -> Result<()> {
        if sequence.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "seq is null").await);
        }
        let length: i32 = jvm.invoke_virtual(&sequence, "length", "()I", ()).await?;
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

    async fn index_of(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, string: ClassInstanceRef<String>) -> Result<i32> {
        jvm.invoke_special(&this, "java/lang/AbstractStringBuilder", "indexOf", "(Ljava/lang/String;)I", (string,))
            .await
    }

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

    async fn to_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        let count: i32 = jvm.get_field(&this, "count", "I").await?;
        let value: ClassInstanceRef<Array<JavaChar>> = jvm.get_field(&this, "value", "[C").await?;
        Ok(JavaLangString::from_utf16(jvm, jvm.load_array(&value, 0, count as usize).await?)
            .await?
            .into())
    }
}
