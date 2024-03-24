use alloc::{sync::Arc, vec, vec::Vec};
use core::mem;

use hashbrown::HashMap;
use spin::RwLock;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{classes::java::lang::Object, RuntimeClassProto, RuntimeContext};

// I'm too lazy to implement hashmap in java, so i'm leveraging rust hashmap here...
// We can't use java object as hashmap key as we need `await` to call `equals()`
type RustHashMap = Arc<RwLock<HashMap<i32, Vec<(ClassInstanceRef<Object>, ClassInstanceRef<Object>)>>>>;

// class java.util.Hashtable
pub struct Hashtable {}

impl Hashtable {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("containsKey", "(Ljava/lang/Object;)Z", Self::contains_key, Default::default()),
                JavaMethodProto::new(
                    "put",
                    "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
                    Self::put,
                    Default::default(),
                ),
                JavaMethodProto::new("get", "(Ljava/lang/Object;)Ljava/lang/Object;", Self::get, Default::default()),
                JavaMethodProto::new("remove", "(Ljava/lang/Object;)Ljava/lang/Object;", Self::remove, Default::default()),
            ],
            fields: vec![JavaFieldProto::new("raw", "[B", Default::default())],
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.Hashtable::<init>({:?})", &this);

        let rust_hash_map: RustHashMap = Arc::new(RwLock::new(HashMap::new()));
        jvm.put_rust_object_field(&mut this, "raw", rust_hash_map).await?;

        Ok(())
    }

    // TODO we need to add synchronized
    #[allow(clippy::await_holding_refcell_ref)]
    async fn contains_key(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, key: ClassInstanceRef<Object>) -> Result<bool> {
        tracing::debug!("java.util.Hashtable::containsKey({:?}, {:?})", &this, &key);

        let rust_hash_map = Self::get_rust_hashmap(jvm, &this).await?;
        let key_hash: i32 = jvm.invoke_virtual(&key, "hashCode", "()I", ()).await?;

        let rust_hash_map = rust_hash_map.read();
        let vec = rust_hash_map.get(&key_hash);

        if vec.is_some() {
            for (key, _) in vec.unwrap() {
                let equals = jvm.invoke_virtual(key, "equals", "(Ljava/lang/Object;)Z", ((*key).clone(),)).await?;
                if equals {
                    return Ok(true);
                }
            }
        }
        Ok(false)
    }

    // TODO we need to add synchronized
    #[allow(clippy::await_holding_refcell_ref)]
    async fn get(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, key: ClassInstanceRef<Object>) -> Result<ClassInstanceRef<Object>> {
        tracing::debug!("java.util.Hashtable::get({:?}, {:?})", &this, &key);

        let rust_hash_map = Self::get_rust_hashmap(jvm, &this).await?;
        let key_hash: i32 = jvm.invoke_virtual(&key, "hashCode", "()I", ()).await?;

        let rust_hash_map = rust_hash_map.read();
        let vec = rust_hash_map.get(&key_hash);

        if vec.is_some() {
            for (key, value) in vec.unwrap() {
                let equals = jvm.invoke_virtual(key, "equals", "(Ljava/lang/Object;)Z", ((*key).clone(),)).await?;
                if equals {
                    return Ok(value.clone());
                }
            }
        }

        Ok(None.into())
    }

    // TODO we need to add synchronized
    #[allow(clippy::await_holding_refcell_ref)]
    async fn remove(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        key: ClassInstanceRef<Object>,
    ) -> Result<ClassInstanceRef<Object>> {
        tracing::debug!("java.util.Hashtable::remove({:?}, {:?})", &this, &key);

        let rust_hash_map = Self::get_rust_hashmap(jvm, &this).await?;
        let key_hash: i32 = jvm.invoke_virtual(&key, "hashCode", "()I", ()).await?;

        let mut rust_hash_map = rust_hash_map.write();
        let vec = rust_hash_map.get_mut(&key_hash);

        if vec.is_some() {
            for (i, (bucket_key, _)) in vec.as_ref().unwrap().iter().enumerate() {
                let equals = jvm.invoke_virtual(bucket_key, "equals", "(Ljava/lang/Object;)Z", (key.clone(),)).await?;
                if equals {
                    let (_, old_value) = vec.unwrap().remove(i);

                    return Ok(old_value);
                }
            }
        }

        Ok(None.into())
    }

    // TODO we need to add synchronized
    #[allow(clippy::await_holding_refcell_ref)]
    async fn put(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        key: ClassInstanceRef<Object>,
        value: ClassInstanceRef<Object>,
    ) -> Result<ClassInstanceRef<Object>> {
        tracing::debug!("java.util.Hashtable::put({:?}, {:?}, {:?})", &this, &key, &value);

        let rust_hash_map = Self::get_rust_hashmap(jvm, &this).await?;
        let key_hash: i32 = jvm.invoke_virtual(&key, "hashCode", "()I", ()).await?;

        let mut rust_hash_map = rust_hash_map.write();
        let vec = rust_hash_map.entry(key_hash).or_insert_with(Vec::new);

        for (i, (bucket_key, _)) in vec.iter().enumerate() {
            let equals = jvm.invoke_virtual(bucket_key, "equals", "(Ljava/lang/Object;)Z", (key.clone(),)).await?;
            if equals {
                let (_, old_value) = mem::replace(&mut vec[i], (key, value));

                return Ok(old_value);
            }
        }

        vec.push((key, value));

        Ok(None.into())
    }

    async fn get_rust_hashmap(jvm: &Jvm, this: &ClassInstanceRef<Self>) -> Result<RustHashMap> {
        jvm.get_rust_object_field(this, "raw").await
    }
}

#[cfg(test)]
mod test {
    use jvm::{runtime::JavaLangString, ClassInstanceRef, Result};

    use crate::{classes::java::lang::Object, test::test_jvm};

    #[futures_test::test]
    async fn test_hashmap() -> Result<()> {
        let jvm = test_jvm().await?;

        let hash_map = jvm.new_class("java/util/Hashtable", "()V", ()).await?;

        let test_key = JavaLangString::from_rust_string(&jvm, "testKey").await?;
        let test_value = JavaLangString::from_rust_string(&jvm, "testValue").await?;

        let _: ClassInstanceRef<Object> = jvm
            .invoke_virtual(
                &hash_map,
                "put",
                "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
                (test_key.clone(), test_value),
            )
            .await?;

        let value = jvm
            .invoke_virtual(&hash_map, "get", "(Ljava/lang/Object;)Ljava/lang/Object;", (test_key.clone(),))
            .await?;

        let value_string = JavaLangString::to_rust_string(&jvm, &value).await?;
        assert_eq!(value_string, "testValue");

        let value = jvm
            .invoke_virtual(&hash_map, "remove", "(Ljava/lang/Object;)Ljava/lang/Object;", (test_key.clone(),))
            .await?;

        let value_string = JavaLangString::to_rust_string(&jvm, &value).await?;
        assert_eq!(value_string, "testValue");

        let value: ClassInstanceRef<Object> = jvm
            .invoke_virtual(&hash_map, "get", "(Ljava/lang/Object;)Ljava/lang/Object;", (test_key,))
            .await?;

        assert!(value.is_null());

        Ok(())
    }
}
