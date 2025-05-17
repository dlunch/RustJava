use alloc::{boxed::Box, sync::Arc, vec, vec::Vec};
use core::mem;

use hashbrown::HashMap;
use parking_lot::Mutex;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm::{ClassInstance, ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::Object};

// I'm too lazy to implement hashmap in java, so i'm leveraging rust hashmap here...
// We can't use java object as hashmap key as we need `await` to call `equals()`
type RustHashMap = Arc<Mutex<HashMap<i32, Vec<(Box<dyn ClassInstance>, Box<dyn ClassInstance>)>>>>;

// class java.util.Hashtable
pub struct Hashtable;

impl Hashtable {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/Hashtable",
            parent_class: Some("java/util/Dictionary"),
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

        let _: () = jvm.invoke_special(&this, "java/util/Dictionary", "<init>", "()V", ()).await?;

        let rust_hash_map: RustHashMap = Arc::new(Mutex::new(HashMap::new()));
        jvm.put_rust_object_field(&mut this, "raw", rust_hash_map).await?;

        Ok(())
    }

    // TODO we need to add synchronized
    async fn contains_key(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, key: ClassInstanceRef<Object>) -> Result<bool> {
        tracing::debug!("java.util.Hashtable::containsKey({:?}, {:?})", &this, &key);

        let rust_hash_map = Self::get_rust_hashmap(jvm, &this).await?;
        let key_hash: i32 = jvm.invoke_virtual(&key, "hashCode", "()I", ()).await?;

        let vec = rust_hash_map.lock().get(&key_hash).cloned();

        if let Some(x) = vec {
            for (key, _) in &x {
                let equals = jvm.invoke_virtual(key, "equals", "(Ljava/lang/Object;)Z", ((*key).clone(),)).await?;
                if equals {
                    return Ok(true);
                }
            }
        }
        Ok(false)
    }

    // TODO we need to add synchronized
    async fn get(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, key: ClassInstanceRef<Object>) -> Result<ClassInstanceRef<Object>> {
        tracing::debug!("java.util.Hashtable::get({:?}, {:?})", &this, &key);

        let rust_hash_map = Self::get_rust_hashmap(jvm, &this).await?;
        let key_hash: i32 = jvm.invoke_virtual(&key, "hashCode", "()I", ()).await?;

        let vec = rust_hash_map.lock().get(&key_hash).cloned();

        if let Some(x) = vec {
            for (key, value) in &x {
                let equals = jvm.invoke_virtual(key, "equals", "(Ljava/lang/Object;)Z", ((*key).clone(),)).await?;
                if equals {
                    return Ok(value.clone().into());
                }
            }
        }

        Ok(None.into())
    }

    // TODO we need to add synchronized
    async fn remove(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        key: ClassInstanceRef<Object>,
    ) -> Result<ClassInstanceRef<Object>> {
        tracing::debug!("java.util.Hashtable::remove({:?}, {:?})", &this, &key);

        let rust_hash_map = Self::get_rust_hashmap(jvm, &this).await?;
        let key_hash: i32 = jvm.invoke_virtual(&key, "hashCode", "()I", ()).await?;

        let vec = rust_hash_map.lock().get(&key_hash).cloned();

        if let Some(x) = vec {
            for (i, (bucket_key, _)) in x.iter().enumerate() {
                let equals = jvm.invoke_virtual(bucket_key, "equals", "(Ljava/lang/Object;)Z", (key.clone(),)).await?;
                if equals {
                    let (_, old_value) = rust_hash_map.lock().get_mut(&key_hash).unwrap().remove(i);

                    return Ok(old_value.into());
                }
            }
        }

        Ok(None.into())
    }

    // TODO we need to add synchronized
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

        let vec = {
            let mut rust_hash_map = rust_hash_map.lock();
            if !rust_hash_map.contains_key(&key_hash) {
                rust_hash_map.insert(key_hash, Vec::new());
            }

            rust_hash_map.get(&key_hash).cloned().unwrap()
        };

        for (i, (bucket_key, _)) in vec.iter().enumerate() {
            let equals = jvm.invoke_virtual(bucket_key, "equals", "(Ljava/lang/Object;)Z", (key.clone(),)).await?;
            if equals {
                let mut rust_hash_map = rust_hash_map.lock();
                let vec = rust_hash_map.get_mut(&key_hash).unwrap();

                let (_, old_value) = mem::replace(&mut vec[i], (key.into(), value.into()));

                return Ok(old_value.into());
            }
        }

        rust_hash_map.lock().get_mut(&key_hash).unwrap().push((key.into(), value.into()));

        Ok(None.into())
    }

    async fn get_rust_hashmap(jvm: &Jvm, this: &ClassInstanceRef<Self>) -> Result<RustHashMap> {
        jvm.get_rust_object_field(this, "raw").await
    }
}
