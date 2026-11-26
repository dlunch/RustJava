use jvm::{Array, ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::{RuntimeContext, classes::java::lang::String};

// interface java.io.DataOutput
#[java_class(name = "java/io/DataOutput", interface)]
pub struct DataOutput;

#[java_methods]
impl DataOutput {
    #[method(public)]
    async fn write(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: i32) -> Result<()>;

    #[method(name = "write", public)]
    async fn write_bytes(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, buffer: ClassInstanceRef<Array<i8>>) -> Result<()>;

    #[method(name = "write", public)]
    async fn write_bytes_offset(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        bytes: ClassInstanceRef<Array<i8>>,
        offset: i32,
        length: i32,
    ) -> Result<()>;

    #[method(public)]
    async fn write_byte(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, v: i32) -> Result<()>;

    #[method(public)]
    async fn write_boolean(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, v: bool) -> Result<()>;

    #[method(public)]
    async fn write_int(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, i: i32) -> Result<()>;

    #[method(public)]
    async fn write_short(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, s: i32) -> Result<()>;

    #[method(public)]
    async fn write_char(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: i32) -> Result<()>;

    #[method(public)]
    async fn write_long(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, l: i64) -> Result<()>;

    #[method(public)]
    async fn write_float(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: f32) -> Result<()>;

    #[method(public)]
    async fn write_double(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, value: f64) -> Result<()>;

    #[method(name = "writeBytes", public)]
    async fn write_bytes_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, s: ClassInstanceRef<String>) -> Result<()>;

    #[method(public)]
    async fn write_chars(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, s: ClassInstanceRef<String>) -> Result<()>;

    #[method(name = "writeUTF", public)]
    async fn write_utf(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, s: ClassInstanceRef<String>) -> Result<()>;
}
