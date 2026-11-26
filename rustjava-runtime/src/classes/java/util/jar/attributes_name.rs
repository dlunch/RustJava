use alloc::boxed::Box;

use jvm::{ClassInstance, ClassInstanceRef, Jvm, Result, runtime::JavaLangString};
use jvm_class_proto::{java_class, java_methods};

use crate::{
    RuntimeContext,
    classes::java::lang::{Object, String},
};

// class java.util.jar.Attributes$Name
#[java_class(name = "java/util/jar/Attributes$Name", extends = "java/lang/Object", public)]
pub struct AttributesName {
    #[field(private, final)]
    name: ClassInstanceRef<String>,
    #[field(public, static, final)]
    MANIFEST_VERSION: ClassInstanceRef<Self>,
    #[field(public, static, final)]
    SIGNATURE_VERSION: ClassInstanceRef<Self>,
    #[field(public, static, final)]
    CONTENT_TYPE: ClassInstanceRef<Self>,
    #[field(public, static, final)]
    CLASS_PATH: ClassInstanceRef<Self>,
    #[field(public, static, final)]
    MAIN_CLASS: ClassInstanceRef<Self>,
    #[field(public, static, final)]
    SEALED: ClassInstanceRef<Self>,
    #[field(public, static, final)]
    EXTENSION_LIST: ClassInstanceRef<Self>,
    #[field(public, static, final)]
    EXTENSION_NAME: ClassInstanceRef<Self>,
    #[field(public, static, final)]
    EXTENSION_INSTALLATION: ClassInstanceRef<Self>,
    #[field(public, static, final)]
    IMPLEMENTATION_TITLE: ClassInstanceRef<Self>,
    #[field(public, static, final)]
    IMPLEMENTATION_VERSION: ClassInstanceRef<Self>,
    #[field(public, static, final)]
    IMPLEMENTATION_VENDOR: ClassInstanceRef<Self>,
    #[field(public, static, final)]
    IMPLEMENTATION_VENDOR_ID: ClassInstanceRef<Self>,
    #[field(public, static, final)]
    IMPLEMENTATION_URL: ClassInstanceRef<Self>,
    #[field(public, static, final)]
    SPECIFICATION_TITLE: ClassInstanceRef<Self>,
    #[field(public, static, final)]
    SPECIFICATION_VERSION: ClassInstanceRef<Self>,
    #[field(public, static, final)]
    SPECIFICATION_VENDOR: ClassInstanceRef<Self>,
}

#[java_methods]
impl AttributesName {
    // Each constant is stored in the static field named after its spelling in upper case with `-` replaced by `_`.
    const NAMES: [&str; 17] = [
        "Manifest-Version",
        "Signature-Version",
        "Content-Type",
        "Class-Path",
        "Main-Class",
        "Sealed",
        "Extension-List",
        "Extension-Name",
        "Extension-Installation",
        "Implementation-Title",
        "Implementation-Version",
        "Implementation-Vendor",
        "Implementation-Vendor-Id",
        "Implementation-URL",
        "Specification-Title",
        "Specification-Version",
        "Specification-Vendor",
    ];

    #[method(static_initializer)]
    async fn clinit(jvm: &Jvm, _: &mut RuntimeContext) -> Result<()> {
        for spelling in Self::NAMES {
            let name = JavaLangString::from_rust_string(jvm, spelling).await?;
            let value = jvm.new_class("java/util/jar/Attributes$Name", "(Ljava/lang/String;)V", (name,)).await?;
            let field = spelling.to_ascii_uppercase().replace('-', "_");
            jvm.put_static_field("java/util/jar/Attributes$Name", &field, "Ljava/util/jar/Attributes$Name;", value)
                .await?;
        }
        Ok(())
    }

    #[method(constructor, public)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, name: ClassInstanceRef<String>) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        if name.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "name is null").await);
        }
        let spelling = JavaLangString::to_rust_string(jvm, &name).await?;
        if !(1..=70).contains(&spelling.len()) || !spelling.bytes().all(|byte| byte.is_ascii_alphanumeric() || byte == b'-' || byte == b'_') {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "invalid attribute name").await);
        }
        jvm.put_field(&mut this, "java/util/jar/Attributes$Name", "name", "Ljava/lang/String;", name)
            .await
    }

    #[method(public)]
    async fn equals(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Object>) -> Result<bool> {
        if other.is_null() || !jvm.is_instance(&**other, "java/util/jar/Attributes$Name") {
            return Ok(false);
        }
        let name: Box<dyn ClassInstance> = jvm
            .get_field(&this, "java/util/jar/Attributes$Name", "name", "Ljava/lang/String;")
            .await?;
        let other_name: Box<dyn ClassInstance> = jvm
            .get_field(&other, "java/util/jar/Attributes$Name", "name", "Ljava/lang/String;")
            .await?;
        Ok(JavaLangString::to_rust_string(jvm, &name)
            .await?
            .eq_ignore_ascii_case(&JavaLangString::to_rust_string(jvm, &other_name).await?))
    }

    #[method(public)]
    async fn hash_code(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        let name: Box<dyn ClassInstance> = jvm
            .get_field(&this, "java/util/jar/Attributes$Name", "name", "Ljava/lang/String;")
            .await?;
        let spelling = JavaLangString::to_rust_string(jvm, &name).await?;
        Ok(spelling
            .bytes()
            .fold(0i32, |hash, byte| hash.wrapping_mul(31).wrapping_add(byte.to_ascii_lowercase() as i32)))
    }

    #[method(public)]
    async fn to_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        jvm.get_field(&this, "java/util/jar/Attributes$Name", "name", "Ljava/lang/String;").await
    }
}
