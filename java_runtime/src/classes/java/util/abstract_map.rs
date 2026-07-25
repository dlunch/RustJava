use alloc::vec;

use java_class_proto::JavaMethodProto;
use java_constants::{ClassAccessFlags, MethodAccessFlags};
use jvm::{Array, ClassInstanceRef, JavaError, Jvm, Result, runtime::JavaLangString};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::Object};

// abstract class java.util.AbstractMap
pub struct AbstractMap;

impl AbstractMap {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/AbstractMap",
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["java/util/Map"],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, MethodAccessFlags::PROTECTED),
                JavaMethodProto::new_abstract("size", "()I", MethodAccessFlags::PUBLIC | MethodAccessFlags::ABSTRACT),
                JavaMethodProto::new("isEmpty", "()Z", Self::is_empty, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("putAll", "(Ljava/util/Map;)V", Self::put_all, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("equals", "(Ljava/lang/Object;)Z", Self::equals, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("hashCode", "()I", Self::hash_code, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("toString", "()Ljava/lang/String;", Self::to_string, MethodAccessFlags::PUBLIC),
            ],
            fields: vec![],
            access_flags: ClassAccessFlags::PUBLIC | ClassAccessFlags::ABSTRACT,
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.AbstractMap::<init>({this:?})");

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        Ok(())
    }

    async fn is_empty(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::debug!("java.util.AbstractMap::isEmpty({this:?})");

        let size: i32 = jvm.invoke_virtual(&this, "size", "()I", ()).await?;

        Ok(size == 0)
    }

    async fn put_all(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, map: ClassInstanceRef<Object>) -> Result<()> {
        tracing::debug!("java.util.AbstractMap::putAll({this:?}, {map:?})");

        if map.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "map").await);
        }
        if this.identity() == map.identity() {
            return Ok(());
        }

        let entry_set: ClassInstanceRef<Object> = jvm.invoke_virtual(&map, "entrySet", "()Ljava/util/Set;", ()).await?;
        let entries: ClassInstanceRef<Array<Object>> = jvm.invoke_virtual(&entry_set, "toArray", "()[Ljava/lang/Object;", ()).await?;
        let count = jvm.array_length(&entries).await?;
        for entry in jvm.load_array::<ClassInstanceRef<Object>>(&entries, 0, count).await? {
            let key: ClassInstanceRef<Object> = jvm.invoke_virtual(&entry, "getKey", "()Ljava/lang/Object;", ()).await?;
            let value: ClassInstanceRef<Object> = jvm.invoke_virtual(&entry, "getValue", "()Ljava/lang/Object;", ()).await?;
            let _: ClassInstanceRef<Object> = jvm
                .invoke_virtual(&this, "put", "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;", (key, value))
                .await?;
        }

        Ok(())
    }

    async fn equals(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Object>) -> Result<bool> {
        if other.is_null() || !jvm.is_instance(other.as_ref(), "java/util/Map") {
            return Ok(false);
        }
        if this.identity() == other.identity() {
            return Ok(true);
        }
        let this_size: i32 = jvm.invoke_virtual(&this, "size", "()I", ()).await?;
        let other_size: i32 = jvm.invoke_virtual(&other, "size", "()I", ()).await?;
        if this_size != other_size {
            return Ok(false);
        }

        let comparison: Result<bool> = async {
            let entries: ClassInstanceRef<Object> = jvm.invoke_virtual(&this, "entrySet", "()Ljava/util/Set;", ()).await?;
            let iterator: ClassInstanceRef<Object> = jvm.invoke_virtual(&entries, "iterator", "()Ljava/util/Iterator;", ()).await?;
            while jvm.invoke_virtual::<_, bool>(&iterator, "hasNext", "()Z", ()).await? {
                let entry: ClassInstanceRef<Object> = jvm.invoke_virtual(&iterator, "next", "()Ljava/lang/Object;", ()).await?;
                let key: ClassInstanceRef<Object> = jvm.invoke_virtual(&entry, "getKey", "()Ljava/lang/Object;", ()).await?;
                let value: ClassInstanceRef<Object> = jvm.invoke_virtual(&entry, "getValue", "()Ljava/lang/Object;", ()).await?;
                let other_value: ClassInstanceRef<Object> = jvm
                    .invoke_virtual(&other, "get", "(Ljava/lang/Object;)Ljava/lang/Object;", (key.clone(),))
                    .await?;
                if value.is_null() {
                    if !other_value.is_null()
                        || !jvm
                            .invoke_virtual::<_, bool>(&other, "containsKey", "(Ljava/lang/Object;)Z", (key,))
                            .await?
                    {
                        return Ok(false);
                    }
                } else if !jvm
                    .invoke_virtual::<_, bool>(&value, "equals", "(Ljava/lang/Object;)Z", (other_value,))
                    .await?
                {
                    return Ok(false);
                }
            }
            Ok(true)
        }
        .await;
        match comparison {
            Err(JavaError::JavaException(exception))
                if jvm.is_instance(exception.as_ref(), "java/lang/ClassCastException")
                    || jvm.is_instance(exception.as_ref(), "java/lang/NullPointerException") =>
            {
                Ok(false)
            }
            result => result,
        }
    }

    async fn hash_code(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        let entries: ClassInstanceRef<Object> = jvm.invoke_virtual(&this, "entrySet", "()Ljava/util/Set;", ()).await?;
        let iterator: ClassInstanceRef<Object> = jvm.invoke_virtual(&entries, "iterator", "()Ljava/util/Iterator;", ()).await?;
        let mut hash = 0i32;
        while jvm.invoke_virtual::<_, bool>(&iterator, "hasNext", "()Z", ()).await? {
            let entry: ClassInstanceRef<Object> = jvm.invoke_virtual(&iterator, "next", "()Ljava/lang/Object;", ()).await?;
            hash = hash.wrapping_add(jvm.invoke_virtual::<_, i32>(&entry, "hashCode", "()I", ()).await?);
        }
        Ok(hash)
    }

    async fn to_string(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let buffer: ClassInstanceRef<Object> = jvm.new_class("java/lang/StringBuffer", "()V", ()).await?.into();
        let open = JavaLangString::from_rust_string(jvm, "{").await?;
        let _: ClassInstanceRef<Object> = jvm
            .invoke_virtual(&buffer, "append", "(Ljava/lang/String;)Ljava/lang/StringBuffer;", (open,))
            .await?;
        let entries: ClassInstanceRef<Object> = jvm.invoke_virtual(&this, "entrySet", "()Ljava/util/Set;", ()).await?;
        let iterator: ClassInstanceRef<Object> = jvm.invoke_virtual(&entries, "iterator", "()Ljava/util/Iterator;", ()).await?;
        let mut first = true;
        while jvm.invoke_virtual::<_, bool>(&iterator, "hasNext", "()Z", ()).await? {
            if first {
                first = false;
            } else {
                let separator = JavaLangString::from_rust_string(jvm, ", ").await?;
                let _: ClassInstanceRef<Object> = jvm
                    .invoke_virtual(&buffer, "append", "(Ljava/lang/String;)Ljava/lang/StringBuffer;", (separator,))
                    .await?;
            }
            let entry: ClassInstanceRef<Object> = jvm.invoke_virtual(&iterator, "next", "()Ljava/lang/Object;", ()).await?;
            let key: ClassInstanceRef<Object> = jvm.invoke_virtual(&entry, "getKey", "()Ljava/lang/Object;", ()).await?;
            let value: ClassInstanceRef<Object> = jvm.invoke_virtual(&entry, "getValue", "()Ljava/lang/Object;", ()).await?;
            if !key.is_null() && key.identity() == this.identity() {
                let recursive = JavaLangString::from_rust_string(jvm, "(this Map)").await?;
                let _: ClassInstanceRef<Object> = jvm
                    .invoke_virtual(&buffer, "append", "(Ljava/lang/String;)Ljava/lang/StringBuffer;", (recursive,))
                    .await?;
            } else {
                let _: ClassInstanceRef<Object> = jvm
                    .invoke_virtual(&buffer, "append", "(Ljava/lang/Object;)Ljava/lang/StringBuffer;", (key,))
                    .await?;
            }
            let equals = JavaLangString::from_rust_string(jvm, "=").await?;
            let _: ClassInstanceRef<Object> = jvm
                .invoke_virtual(&buffer, "append", "(Ljava/lang/String;)Ljava/lang/StringBuffer;", (equals,))
                .await?;
            if !value.is_null() && value.identity() == this.identity() {
                let recursive = JavaLangString::from_rust_string(jvm, "(this Map)").await?;
                let _: ClassInstanceRef<Object> = jvm
                    .invoke_virtual(&buffer, "append", "(Ljava/lang/String;)Ljava/lang/StringBuffer;", (recursive,))
                    .await?;
            } else {
                let _: ClassInstanceRef<Object> = jvm
                    .invoke_virtual(&buffer, "append", "(Ljava/lang/Object;)Ljava/lang/StringBuffer;", (value,))
                    .await?;
            }
        }
        let close = JavaLangString::from_rust_string(jvm, "}").await?;
        let _: ClassInstanceRef<Object> = jvm
            .invoke_virtual(&buffer, "append", "(Ljava/lang/String;)Ljava/lang/StringBuffer;", (close,))
            .await?;
        jvm.invoke_virtual(&buffer, "toString", "()Ljava/lang/String;", ()).await
    }
}
