use alloc::format;

use jvm::{ClassInstanceRef, JavaChar, Jvm, Result, runtime::JavaLangString};
use jvm_class_proto::{java_class, java_methods};

use crate::{
    RuntimeContext,
    classes::java::lang::{Class, String},
};

// public class java.util.IllegalFormatConversionException
#[java_class(name = "java/util/IllegalFormatConversionException", extends = "java/util/IllegalFormatException", public)]
pub struct IllegalFormatConversionException {
    #[field(private)]
    conversion: JavaChar,
    #[field(private)]
    argumentClass: ClassInstanceRef<Class>,
}

#[java_methods]
impl IllegalFormatConversionException {
    #[method(constructor, public)]
    async fn init(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        conversion: JavaChar,
        argument_class: ClassInstanceRef<Class>,
    ) -> Result<()> {
        if argument_class.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "argumentClass is null").await);
        }
        let _: () = jvm.invoke_special(&this, "java/util/IllegalFormatException", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "java/util/IllegalFormatConversionException", "conversion", "C", conversion)
            .await?;
        jvm.put_field(
            &mut this,
            "java/util/IllegalFormatConversionException",
            "argumentClass",
            "Ljava/lang/Class;",
            argument_class,
        )
        .await
    }

    #[method(public)]
    async fn get_conversion(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<JavaChar> {
        jvm.get_field(&this, "java/util/IllegalFormatConversionException", "conversion", "C")
            .await
    }

    #[method(public)]
    async fn get_argument_class(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Class>> {
        jvm.get_field(&this, "java/util/IllegalFormatConversionException", "argumentClass", "Ljava/lang/Class;")
            .await
    }

    #[method(public)]
    async fn get_message(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        let conversion: JavaChar = jvm
            .get_field(&this, "java/util/IllegalFormatConversionException", "conversion", "C")
            .await?;
        let argument_class: ClassInstanceRef<Class> = jvm
            .get_field(&this, "java/util/IllegalFormatConversionException", "argumentClass", "Ljava/lang/Class;")
            .await?;
        let class_name: ClassInstanceRef<String> = jvm
            .invoke_virtual(&argument_class, "java/lang/Class", "getName", "()Ljava/lang/String;", ())
            .await?;
        Ok(JavaLangString::from_rust_string(
            jvm,
            &format!(
                "{} != {}",
                char::from_u32(conversion as u32).unwrap_or('\u{fffd}'),
                JavaLangString::to_rust_string(jvm, &class_name).await?
            ),
        )
        .await?
        .into())
    }
}
