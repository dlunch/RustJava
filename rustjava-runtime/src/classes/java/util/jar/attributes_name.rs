use alloc::vec;

use jvm::{ClassInstanceRef, Jvm, Result, runtime::JavaLangString};
use jvm_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm_types::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};

use crate::{
    RuntimeClassProto, RuntimeContext,
    classes::java::lang::{Object, String},
};

pub struct AttributesName;

impl AttributesName {
    const NAMES: [(&str, &str); 17] = [
        ("MANIFEST_VERSION", "Manifest-Version"),
        ("SIGNATURE_VERSION", "Signature-Version"),
        ("CONTENT_TYPE", "Content-Type"),
        ("CLASS_PATH", "Class-Path"),
        ("MAIN_CLASS", "Main-Class"),
        ("SEALED", "Sealed"),
        ("EXTENSION_LIST", "Extension-List"),
        ("EXTENSION_NAME", "Extension-Name"),
        ("EXTENSION_INSTALLATION", "Extension-Installation"),
        ("IMPLEMENTATION_TITLE", "Implementation-Title"),
        ("IMPLEMENTATION_VERSION", "Implementation-Version"),
        ("IMPLEMENTATION_VENDOR", "Implementation-Vendor"),
        ("IMPLEMENTATION_VENDOR_ID", "Implementation-Vendor-Id"),
        ("IMPLEMENTATION_URL", "Implementation-URL"),
        ("SPECIFICATION_TITLE", "Specification-Title"),
        ("SPECIFICATION_VERSION", "Specification-Version"),
        ("SPECIFICATION_VENDOR", "Specification-Vendor"),
    ];

    pub fn as_proto() -> RuntimeClassProto {
        let mut fields = vec![JavaFieldProto::new(
            "name",
            "Ljava/lang/String;",
            FieldAccessFlags::PRIVATE | FieldAccessFlags::FINAL,
        )];
        fields.extend(Self::NAMES.iter().map(|(field, _)| {
            JavaFieldProto::new(
                field,
                "Ljava/util/jar/Attributes$Name;",
                FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
            )
        }));
        RuntimeClassProto {
            name: "java/util/jar/Attributes$Name",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<clinit>", "()V", Self::clinit, MethodAccessFlags::STATIC),
                JavaMethodProto::new("<init>", "(Ljava/lang/String;)V", Self::init, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("equals", "(Ljava/lang/Object;)Z", Self::equals, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("hashCode", "()I", Self::hash_code, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("toString", "()Ljava/lang/String;", Self::to_string, MethodAccessFlags::PUBLIC),
            ],
            fields,
            access_flags: ClassAccessFlags::PUBLIC,
        }
    }

    async fn clinit(jvm: &Jvm, _: &mut RuntimeContext) -> Result<()> {
        for (field, spelling) in Self::NAMES {
            let name = JavaLangString::from_rust_string(jvm, spelling).await?;
            let value = jvm.new_class("java/util/jar/Attributes$Name", "(Ljava/lang/String;)V", (name,)).await?;
            jvm.put_static_field("java/util/jar/Attributes$Name", field, "Ljava/util/jar/Attributes$Name;", value)
                .await?;
        }
        Ok(())
    }

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

    async fn equals(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Object>) -> Result<bool> {
        if other.is_null() || !jvm.is_instance(&**other, "java/util/jar/Attributes$Name") {
            return Ok(false);
        }
        let name = jvm
            .get_field(&this, "java/util/jar/Attributes$Name", "name", "Ljava/lang/String;")
            .await?;
        let other_name = jvm
            .get_field(&other, "java/util/jar/Attributes$Name", "name", "Ljava/lang/String;")
            .await?;
        Ok(JavaLangString::to_rust_string(jvm, &name)
            .await?
            .eq_ignore_ascii_case(&JavaLangString::to_rust_string(jvm, &other_name).await?))
    }

    async fn hash_code(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        let name = jvm
            .get_field(&this, "java/util/jar/Attributes$Name", "name", "Ljava/lang/String;")
            .await?;
        let spelling = JavaLangString::to_rust_string(jvm, &name).await?;
        Ok(spelling
            .bytes()
            .fold(0i32, |hash, byte| hash.wrapping_mul(31).wrapping_add(byte.to_ascii_lowercase() as i32)))
    }

    async fn to_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<String>> {
        jvm.get_field(&this, "java/util/jar/Attributes$Name", "name", "Ljava/lang/String;").await
    }
}
