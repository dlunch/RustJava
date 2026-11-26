use alloc::format;

use jvm::{ClassInstanceRef, Jvm, Result, runtime::JavaLangString};
use jvm_class_proto::{java_class, java_methods};

use crate::{RuntimeContext, classes::java::lang::String};

// class java/lang/StringIndexOutOfBoundsException
#[java_class(
    name = "java/lang/StringIndexOutOfBoundsException",
    extends = "java/lang/IndexOutOfBoundsException",
    public
)]
pub struct StringIndexOutOfBoundsException;

#[java_methods]
impl StringIndexOutOfBoundsException {
    #[method(constructor, public)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.lang.StringIndexOutOfBoundsException::<init>({this:?})");

        let _: () = jvm
            .invoke_special(&this, "java/lang/IndexOutOfBoundsException", "<init>", "()V", ())
            .await?;

        Ok(())
    }

    #[method(constructor, public)]
    async fn init_with_message(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, message: ClassInstanceRef<String>) -> Result<()> {
        tracing::debug!("java.lang.StringIndexOutOfBoundsException::<init>({this:?}, {message:?})");

        let _: () = jvm
            .invoke_special(
                &this,
                "java/lang/IndexOutOfBoundsException",
                "<init>",
                "(Ljava/lang/String;)V",
                (message,),
            )
            .await?;

        Ok(())
    }

    #[method(constructor, public)]
    async fn init_with_index(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, index: i32) -> Result<()> {
        tracing::debug!("java.lang.StringIndexOutOfBoundsException::<init>({this:?}, {index:?})");

        let message = JavaLangString::from_rust_string(jvm, &format!("String index out of range: {index}")).await?;
        let _: () = jvm
            .invoke_special(
                &this,
                "java/lang/IndexOutOfBoundsException",
                "<init>",
                "(Ljava/lang/String;)V",
                (message,),
            )
            .await?;

        Ok(())
    }
}
