use jvm::{Array, ClassInstanceRef, JavaChar, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::{RuntimeContext, classes::java::lang::String};

// interface java.io.DataInput
#[java_class(name = "java/io/DataInput", interface)]
pub struct DataInput;

#[java_methods]
impl DataInput {
    #[method(public)]
    async fn read_boolean(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool>;

    #[method(public)]
    async fn read_byte(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i8>;

    #[method(public)]
    async fn read_char(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<JavaChar>;

    #[method(public)]
    async fn read_double(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<f64>;

    #[method(public)]
    async fn read_float(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<f32>;

    #[method(public)]
    async fn read_fully(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, b: ClassInstanceRef<Array<i8>>) -> Result<()>;

    #[method(name = "readFully", public)]
    async fn read_fully_offset_length(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        b: ClassInstanceRef<Array<i8>>,
        off: i32,
        len: i32,
    ) -> Result<()>;

    #[method(public)]
    async fn read_int(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32>;

    #[method(public)]
    async fn read_long(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i64>;

    #[method(public)]
    async fn read_short(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i16>;

    #[method(public)]
    async fn read_unsigned_byte(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32>;

    #[method(public)]
    async fn read_unsigned_short(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32>;

    #[method(name = "readUTF", public)]
    async fn read_utf(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>>;

    #[method(public)]
    async fn skip_bytes(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, n: i32) -> Result<i32>;
}
