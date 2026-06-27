use alloc::{boxed::Box, collections::BTreeMap, vec};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_runtime::{RuntimeClassProto, RuntimeContext, classes::java::lang::Object, classes::java::util::HashMapEntry};
use jvm::{Array, ClassInstanceRef, JavaError, Jvm, Result, runtime::JavaLangString};
use jvm_rust::ClassDefinitionImpl;

use test_utils::{TestRuntime, create_test_jvm, test_jvm};

struct AsymmetricStoredKey;

impl AsymmetricStoredKey {
    fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "AsymmetricStoredKey",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("hashCode", "()I", Self::hash_code, Default::default()),
                JavaMethodProto::new("equals", "(Ljava/lang/Object;)Z", Self::equals, Default::default()),
            ],
            fields: vec![],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        Ok(())
    }

    async fn hash_code(_: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>) -> Result<i32> {
        Ok(12345)
    }

    async fn equals(_: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>, _: ClassInstanceRef<Object>) -> Result<bool> {
        Ok(false)
    }
}

struct AsymmetricQueryKey;

impl AsymmetricQueryKey {
    fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "AsymmetricQueryKey",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("hashCode", "()I", Self::hash_code, Default::default()),
                JavaMethodProto::new("equals", "(Ljava/lang/Object;)Z", Self::equals, Default::default()),
            ],
            fields: vec![],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        Ok(())
    }

    async fn hash_code(_: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>) -> Result<i32> {
        Ok(12345)
    }

    async fn equals(jvm: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>, other: ClassInstanceRef<Object>) -> Result<bool> {
        if other.is_null() {
            return Ok(false);
        }

        Ok(jvm.is_instance(&**other, "AsymmetricStoredKey"))
    }
}

struct CollisionKey;

impl CollisionKey {
    fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "HashMapCollisionKey",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(I)V", Self::init, Default::default()),
                JavaMethodProto::new("hashCode", "()I", Self::hash_code, Default::default()),
                JavaMethodProto::new("equals", "(Ljava/lang/Object;)Z", Self::equals, Default::default()),
            ],
            fields: vec![JavaFieldProto::new("id", "I", Default::default())],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, id: i32) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "id", "I", id).await?;

        Ok(())
    }

    async fn hash_code(_: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>) -> Result<i32> {
        Ok(777)
    }

    async fn equals(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Object>) -> Result<bool> {
        if other.is_null() || !jvm.is_instance(&**other, "HashMapCollisionKey") {
            return Ok(false);
        }

        let this_id: i32 = jvm.get_field(&this, "id", "I").await?;
        let other_id: i32 = jvm.get_field(&other, "id", "I").await?;

        Ok(this_id == other_id)
    }
}

struct CustomMapEntry;

impl CustomMapEntry {
    fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "CustomMapEntry",
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["java/util/Map$Entry"],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/lang/Object;Ljava/lang/Object;)V", Self::init, Default::default()),
                JavaMethodProto::new("getKey", "()Ljava/lang/Object;", Self::get_key, Default::default()),
                JavaMethodProto::new("getValue", "()Ljava/lang/Object;", Self::get_value, Default::default()),
                JavaMethodProto::new("setValue", "(Ljava/lang/Object;)Ljava/lang/Object;", Self::set_value, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("key", "Ljava/lang/Object;", Default::default()),
                JavaFieldProto::new("value", "Ljava/lang/Object;", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    async fn init(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        key: ClassInstanceRef<Object>,
        value: ClassInstanceRef<Object>,
    ) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "key", "Ljava/lang/Object;", key).await?;
        jvm.put_field(&mut this, "value", "Ljava/lang/Object;", value).await?;

        Ok(())
    }

    async fn get_key(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        jvm.get_field(&this, "key", "Ljava/lang/Object;").await
    }

    async fn get_value(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        jvm.get_field(&this, "value", "Ljava/lang/Object;").await
    }

    async fn set_value(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        value: ClassInstanceRef<Object>,
    ) -> Result<ClassInstanceRef<Object>> {
        let old_value: ClassInstanceRef<Object> = jvm.get_field(&this, "value", "Ljava/lang/Object;").await?;
        jvm.put_field(&mut this, "value", "Ljava/lang/Object;", value).await?;

        Ok(old_value)
    }
}

async fn object_to_optional_string(jvm: &Jvm, value: &ClassInstanceRef<Object>) -> Result<Option<String>> {
    if value.is_null() {
        return Ok(None);
    }

    Ok(Some(JavaLangString::to_rust_string(jvm, value).await?))
}

async fn object_array_to_optional_strings(jvm: &Jvm, array: &ClassInstanceRef<Array<Object>>) -> Result<Vec<Option<String>>> {
    let len = jvm.array_length(array).await?;
    let values: Vec<ClassInstanceRef<Object>> = if len == 0 { Vec::new() } else { jvm.load_array(array, 0, len).await? };
    let mut strings = Vec::with_capacity(values.len());
    for value in values {
        strings.push(object_to_optional_string(jvm, &value).await?);
    }

    Ok(strings)
}

async fn iterator_to_optional_strings(jvm: &Jvm, iterator: &ClassInstanceRef<Object>) -> Result<Vec<Option<String>>> {
    let mut values = Vec::new();
    loop {
        let has_next: bool = jvm.invoke_virtual(iterator, "hasNext", "()Z", ()).await?;
        if !has_next {
            break;
        }

        let value: ClassInstanceRef<Object> = jvm.invoke_virtual(iterator, "next", "()Ljava/lang/Object;", ()).await?;
        values.push(object_to_optional_string(jvm, &value).await?);
    }

    Ok(values)
}

async fn entry_to_optional_strings(jvm: &Jvm, entry: &ClassInstanceRef<Object>) -> Result<(Option<String>, Option<String>)> {
    assert!(jvm.is_instance(&***entry, "java/util/Map$Entry"));

    let key: ClassInstanceRef<Object> = jvm.invoke_virtual(entry, "getKey", "()Ljava/lang/Object;", ()).await?;
    let value: ClassInstanceRef<Object> = jvm.invoke_virtual(entry, "getValue", "()Ljava/lang/Object;", ()).await?;

    Ok((object_to_optional_string(jvm, &key).await?, object_to_optional_string(jvm, &value).await?))
}

async fn iterator_to_entry_pairs(jvm: &Jvm, iterator: &ClassInstanceRef<Object>) -> Result<Vec<(Option<String>, Option<String>)>> {
    let mut entries = Vec::new();
    loop {
        let has_next: bool = jvm.invoke_virtual(iterator, "hasNext", "()Z", ()).await?;
        if !has_next {
            break;
        }

        let entry: ClassInstanceRef<Object> = jvm.invoke_virtual(iterator, "next", "()Ljava/lang/Object;", ()).await?;
        entries.push(entry_to_optional_strings(jvm, &entry).await?);
    }

    Ok(entries)
}

async fn assert_next_throws_no_such_element(jvm: &Jvm, iterator: &ClassInstanceRef<Object>) -> Result<()> {
    let result: Result<ClassInstanceRef<Object>> = jvm.invoke_virtual(iterator, "next", "()Ljava/lang/Object;", ()).await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("Expected JavaException, got {:?}", result);
    };
    assert!(jvm.is_instance(&*exception, "java/util/NoSuchElementException"));

    Ok(())
}

async fn assert_remove_throws_unsupported(jvm: &Jvm, iterator: &ClassInstanceRef<Object>) -> Result<()> {
    let result: Result<()> = jvm.invoke_virtual(iterator, "remove", "()V", ()).await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("Expected JavaException, got {:?}", result);
    };
    assert!(jvm.is_instance(&*exception, "java/lang/UnsupportedOperationException"));

    Ok(())
}

fn sorted_optional_strings(mut values: Vec<Option<String>>) -> Vec<Option<String>> {
    values.sort();
    values
}

fn sorted_entry_pairs(mut values: Vec<(Option<String>, Option<String>)>) -> Vec<(Option<String>, Option<String>)> {
    values.sort();
    values
}

#[tokio::test]
async fn test_hash_map_basic_put_get_overwrite_and_remove() -> Result<()> {
    let jvm = test_jvm().await?;

    let hash_map = jvm.new_class("java/util/HashMap", "()V", ()).await?;
    assert!(jvm.is_instance(&*hash_map, "java/util/Map"));
    assert!(jvm.is_instance(&*hash_map, "java/util/AbstractMap"));

    let size: i32 = jvm.invoke_virtual(&hash_map, "size", "()I", ()).await?;
    assert_eq!(size, 0);
    let is_empty: bool = jvm.invoke_virtual(&hash_map, "isEmpty", "()Z", ()).await?;
    assert!(is_empty);

    let key = JavaLangString::from_rust_string(&jvm, "key").await?;
    let equal_key = JavaLangString::from_rust_string(&jvm, "key").await?;
    let missing_key = JavaLangString::from_rust_string(&jvm, "missing").await?;
    let value = JavaLangString::from_rust_string(&jvm, "value").await?;
    let equal_value = JavaLangString::from_rust_string(&jvm, "value").await?;
    let replacement = JavaLangString::from_rust_string(&jvm, "replacement").await?;

    let absent: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&hash_map, "get", "(Ljava/lang/Object;)Ljava/lang/Object;", (missing_key.clone(),))
        .await?;
    assert!(absent.is_null());

    let absent: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&hash_map, "remove", "(Ljava/lang/Object;)Ljava/lang/Object;", (missing_key.clone(),))
        .await?;
    assert!(absent.is_null());

    let old: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &hash_map,
            "put",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            (key.clone(), value.clone()),
        )
        .await?;
    assert!(old.is_null());

    let size: i32 = jvm.invoke_virtual(&hash_map, "size", "()I", ()).await?;
    assert_eq!(size, 1);
    let is_empty: bool = jvm.invoke_virtual(&hash_map, "isEmpty", "()Z", ()).await?;
    assert!(!is_empty);

    let found: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&hash_map, "get", "(Ljava/lang/Object;)Ljava/lang/Object;", (equal_key.clone(),))
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &found).await?, "value");

    let contains_key: bool = jvm
        .invoke_virtual(&hash_map, "containsKey", "(Ljava/lang/Object;)Z", (equal_key.clone(),))
        .await?;
    assert!(contains_key);
    let contains_value: bool = jvm
        .invoke_virtual(&hash_map, "containsValue", "(Ljava/lang/Object;)Z", (equal_value.clone(),))
        .await?;
    assert!(contains_value);

    let old: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &hash_map,
            "put",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            (equal_key.clone(), replacement.clone()),
        )
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &old).await?, "value");

    let size: i32 = jvm.invoke_virtual(&hash_map, "size", "()I", ()).await?;
    assert_eq!(size, 1);

    let found: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&hash_map, "get", "(Ljava/lang/Object;)Ljava/lang/Object;", (key.clone(),))
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &found).await?, "replacement");

    let removed: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&hash_map, "remove", "(Ljava/lang/Object;)Ljava/lang/Object;", (equal_key.clone(),))
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &removed).await?, "replacement");

    let size: i32 = jvm.invoke_virtual(&hash_map, "size", "()I", ()).await?;
    assert_eq!(size, 0);
    let contains_key: bool = jvm
        .invoke_virtual(&hash_map, "containsKey", "(Ljava/lang/Object;)Z", (key.clone(),))
        .await?;
    assert!(!contains_key);

    let found: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&hash_map, "get", "(Ljava/lang/Object;)Ljava/lang/Object;", (key,))
        .await?;
    assert!(found.is_null());

    Ok(())
}

#[tokio::test]
async fn test_hash_map_remove_unlinks_collision_chain_positions() -> Result<()> {
    let runtime = TestRuntime::new(BTreeMap::new());
    let jvm = create_test_jvm(runtime.clone()).await?;

    let collision_class = Box::new(ClassDefinitionImpl::from_class_proto(
        CollisionKey::as_proto(),
        Box::new(runtime.clone()) as Box<_>,
    ));
    jvm.register_class(collision_class, None).await?;

    let hash_map = jvm.new_class("java/util/HashMap", "(I)V", (3,)).await?;
    let key1 = jvm.new_class("HashMapCollisionKey", "(I)V", (1,)).await?;
    let key2 = jvm.new_class("HashMapCollisionKey", "(I)V", (2,)).await?;
    let key3 = jvm.new_class("HashMapCollisionKey", "(I)V", (3,)).await?;
    let value1 = JavaLangString::from_rust_string(&jvm, "value-1").await?;
    let value2 = JavaLangString::from_rust_string(&jvm, "value-2").await?;
    let value3 = JavaLangString::from_rust_string(&jvm, "value-3").await?;

    for (key, value) in [(key1, value1.clone()), (key2, value2.clone()), (key3, value3.clone())] {
        let old: ClassInstanceRef<Object> = jvm
            .invoke_virtual(&hash_map, "put", "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;", (key, value))
            .await?;
        assert!(old.is_null());
    }

    let remove_middle = jvm.new_class("HashMapCollisionKey", "(I)V", (2,)).await?;
    let removed: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&hash_map, "remove", "(Ljava/lang/Object;)Ljava/lang/Object;", (remove_middle,))
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &removed).await?, "value-2");

    let query1 = jvm.new_class("HashMapCollisionKey", "(I)V", (1,)).await?;
    let query2 = jvm.new_class("HashMapCollisionKey", "(I)V", (2,)).await?;
    let query3 = jvm.new_class("HashMapCollisionKey", "(I)V", (3,)).await?;
    let found1: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&hash_map, "get", "(Ljava/lang/Object;)Ljava/lang/Object;", (query1.clone(),))
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &found1).await?, "value-1");
    let found2: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&hash_map, "get", "(Ljava/lang/Object;)Ljava/lang/Object;", (query2,))
        .await?;
    assert!(found2.is_null());
    let found3: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&hash_map, "get", "(Ljava/lang/Object;)Ljava/lang/Object;", (query3.clone(),))
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &found3).await?, "value-3");

    let size: i32 = jvm.invoke_virtual(&hash_map, "size", "()I", ()).await?;
    assert_eq!(size, 2);

    let removed: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&hash_map, "remove", "(Ljava/lang/Object;)Ljava/lang/Object;", (query3,))
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &removed).await?, "value-3");

    let removed: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&hash_map, "remove", "(Ljava/lang/Object;)Ljava/lang/Object;", (query1.clone(),))
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &removed).await?, "value-1");

    let size: i32 = jvm.invoke_virtual(&hash_map, "size", "()I", ()).await?;
    assert_eq!(size, 0);
    let found1: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&hash_map, "get", "(Ljava/lang/Object;)Ljava/lang/Object;", (query1,))
        .await?;
    assert!(found1.is_null());

    Ok(())
}

#[tokio::test]
async fn test_hash_map_null_key_and_null_value() -> Result<()> {
    let jvm = test_jvm().await?;

    let hash_map = jvm.new_class("java/util/HashMap", "()V", ()).await?;
    let null_ref: ClassInstanceRef<Object> = None.into();
    let null_key_value = JavaLangString::from_rust_string(&jvm, "null-key-value").await?;
    let null_value_key = JavaLangString::from_rust_string(&jvm, "null-value-key").await?;

    let old: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &hash_map,
            "put",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            (null_ref.clone(), null_key_value.clone()),
        )
        .await?;
    assert!(old.is_null());

    let contains_key: bool = jvm
        .invoke_virtual(&hash_map, "containsKey", "(Ljava/lang/Object;)Z", (null_ref.clone(),))
        .await?;
    assert!(contains_key);

    let found: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&hash_map, "get", "(Ljava/lang/Object;)Ljava/lang/Object;", (null_ref.clone(),))
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &found).await?, "null-key-value");

    let old: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &hash_map,
            "put",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            (null_value_key.clone(), null_ref.clone()),
        )
        .await?;
    assert!(old.is_null());

    let found: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&hash_map, "get", "(Ljava/lang/Object;)Ljava/lang/Object;", (null_value_key.clone(),))
        .await?;
    assert!(found.is_null());

    let contains_key: bool = jvm
        .invoke_virtual(&hash_map, "containsKey", "(Ljava/lang/Object;)Z", (null_value_key.clone(),))
        .await?;
    assert!(contains_key);

    let contains_value: bool = jvm
        .invoke_virtual(&hash_map, "containsValue", "(Ljava/lang/Object;)Z", (null_ref.clone(),))
        .await?;
    assert!(contains_value);

    let removed: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&hash_map, "remove", "(Ljava/lang/Object;)Ljava/lang/Object;", (null_ref.clone(),))
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &removed).await?, "null-key-value");

    let contains_key: bool = jvm.invoke_virtual(&hash_map, "containsKey", "(Ljava/lang/Object;)Z", (null_ref,)).await?;
    assert!(!contains_key);

    Ok(())
}

#[tokio::test]
async fn test_hash_map_zero_and_negative_capacity() -> Result<()> {
    let jvm = test_jvm().await?;

    let hash_map = jvm.new_class("java/util/HashMap", "(I)V", (0,)).await?;
    let table: ClassInstanceRef<Array<HashMapEntry>> = jvm.get_field(&hash_map, "table", "[Ljava/util/HashMap$Entry;").await?;
    assert_eq!(jvm.array_length(&table).await?, 0);

    let key = JavaLangString::from_rust_string(&jvm, "zero-key").await?;
    let value = JavaLangString::from_rust_string(&jvm, "zero-value").await?;

    let contains_key: bool = jvm
        .invoke_virtual(&hash_map, "containsKey", "(Ljava/lang/Object;)Z", (key.clone(),))
        .await?;
    assert!(!contains_key);

    let old: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &hash_map,
            "put",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            (key.clone(), value.clone()),
        )
        .await?;
    assert!(old.is_null());

    let table: ClassInstanceRef<Array<HashMapEntry>> = jvm.get_field(&hash_map, "table", "[Ljava/util/HashMap$Entry;").await?;
    assert!(jvm.array_length(&table).await? >= 1);

    let found: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&hash_map, "get", "(Ljava/lang/Object;)Ljava/lang/Object;", (key.clone(),))
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &found).await?, "zero-value");

    let removed: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&hash_map, "remove", "(Ljava/lang/Object;)Ljava/lang/Object;", (key.clone(),))
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &removed).await?, "zero-value");

    let contains_key: bool = jvm.invoke_virtual(&hash_map, "containsKey", "(Ljava/lang/Object;)Z", (key,)).await?;
    assert!(!contains_key);

    let result = jvm.new_class("java/util/HashMap", "(I)V", (-1,)).await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("Expected JavaException, got {:?}", result);
    };
    assert!(jvm.is_instance(&*exception, "java/lang/IllegalArgumentException"));

    Ok(())
}

#[tokio::test]
async fn test_hash_map_rehash_keeps_entries() -> Result<()> {
    let jvm = test_jvm().await?;

    let hash_map = jvm.new_class("java/util/HashMap", "(I)V", (1,)).await?;
    for index in 0..40 {
        let key = JavaLangString::from_rust_string(&jvm, &format!("key-{index}")).await?;
        let value = JavaLangString::from_rust_string(&jvm, &format!("value-{index}")).await?;
        let old: ClassInstanceRef<Object> = jvm
            .invoke_virtual(&hash_map, "put", "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;", (key, value))
            .await?;
        assert!(old.is_null());
    }

    let size: i32 = jvm.invoke_virtual(&hash_map, "size", "()I", ()).await?;
    assert_eq!(size, 40);

    for index in 0..40 {
        let key = JavaLangString::from_rust_string(&jvm, &format!("key-{index}")).await?;
        let value: ClassInstanceRef<Object> = jvm
            .invoke_virtual(&hash_map, "get", "(Ljava/lang/Object;)Ljava/lang/Object;", (key,))
            .await?;
        assert_eq!(JavaLangString::to_rust_string(&jvm, &value).await?, format!("value-{index}"));
    }

    Ok(())
}

#[tokio::test]
async fn test_hash_map_clear_removes_entries_and_bucket_chains() -> Result<()> {
    let jvm = test_jvm().await?;

    let hash_map = jvm.new_class("java/util/HashMap", "()V", ()).await?;
    let first_key = JavaLangString::from_rust_string(&jvm, "first").await?;
    let first_value = JavaLangString::from_rust_string(&jvm, "first-value").await?;
    let second_key = JavaLangString::from_rust_string(&jvm, "second").await?;
    let second_value = JavaLangString::from_rust_string(&jvm, "second-value").await?;

    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &hash_map,
            "put",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            (first_key.clone(), first_value.clone()),
        )
        .await?;
    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &hash_map,
            "put",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            (second_key.clone(), second_value.clone()),
        )
        .await?;

    let _: () = jvm.invoke_virtual(&hash_map, "clear", "()V", ()).await?;

    let size: i32 = jvm.invoke_virtual(&hash_map, "size", "()I", ()).await?;
    assert_eq!(size, 0);
    let is_empty: bool = jvm.invoke_virtual(&hash_map, "isEmpty", "()Z", ()).await?;
    assert!(is_empty);

    let found: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&hash_map, "get", "(Ljava/lang/Object;)Ljava/lang/Object;", (first_key.clone(),))
        .await?;
    assert!(found.is_null());
    let contains_key: bool = jvm
        .invoke_virtual(&hash_map, "containsKey", "(Ljava/lang/Object;)Z", (first_key,))
        .await?;
    assert!(!contains_key);
    let contains_value: bool = jvm
        .invoke_virtual(&hash_map, "containsValue", "(Ljava/lang/Object;)Z", (second_value,))
        .await?;
    assert!(!contains_value);

    let table: ClassInstanceRef<Array<HashMapEntry>> = jvm.get_field(&hash_map, "table", "[Ljava/util/HashMap$Entry;").await?;
    let table_len = jvm.array_length(&table).await?;
    let buckets: Vec<ClassInstanceRef<HashMapEntry>> = jvm.load_array(&table, 0, table_len).await?;
    assert!(buckets.iter().all(ClassInstanceRef::is_null));

    Ok(())
}

#[tokio::test]
async fn test_hash_map_entry_direct_methods_and_assignability() -> Result<()> {
    let jvm = test_jvm().await?;

    let key = JavaLangString::from_rust_string(&jvm, "entry-key").await?;
    let value = JavaLangString::from_rust_string(&jvm, "entry-value").await?;
    let replacement = JavaLangString::from_rust_string(&jvm, "entry-replacement").await?;
    let next: ClassInstanceRef<HashMapEntry> = None.into();

    let entry: ClassInstanceRef<HashMapEntry> = jvm
        .new_class(
            "java/util/HashMap$Entry",
            "(ILjava/lang/Object;Ljava/lang/Object;Ljava/util/HashMap$Entry;)V",
            (7, key.clone(), value.clone(), next),
        )
        .await?
        .into();

    assert!(jvm.is_instance(&**entry, "java/util/Map$Entry"));

    let found_key: ClassInstanceRef<Object> = jvm.invoke_virtual(&entry, "getKey", "()Ljava/lang/Object;", ()).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &found_key).await?, "entry-key");

    let found_value: ClassInstanceRef<Object> = jvm.invoke_virtual(&entry, "getValue", "()Ljava/lang/Object;", ()).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &found_value).await?, "entry-value");

    let old_value: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&entry, "setValue", "(Ljava/lang/Object;)Ljava/lang/Object;", (replacement.clone(),))
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &old_value).await?, "entry-value");

    let found_value: ClassInstanceRef<Object> = jvm.invoke_virtual(&entry, "getValue", "()Ljava/lang/Object;", ()).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &found_value).await?, "entry-replacement");

    Ok(())
}

#[tokio::test]
async fn test_hash_map_uses_query_key_equals_direction() -> Result<()> {
    let runtime = TestRuntime::new(BTreeMap::new());
    let jvm = create_test_jvm(runtime.clone()).await?;

    let stored_class = Box::new(ClassDefinitionImpl::from_class_proto(
        AsymmetricStoredKey::as_proto(),
        Box::new(runtime.clone()) as Box<_>,
    ));
    jvm.register_class(stored_class, None).await?;
    let query_class = Box::new(ClassDefinitionImpl::from_class_proto(
        AsymmetricQueryKey::as_proto(),
        Box::new(runtime.clone()) as Box<_>,
    ));
    jvm.register_class(query_class, None).await?;

    let hash_map = jvm.new_class("java/util/HashMap", "()V", ()).await?;
    let stored_key = jvm.new_class("AsymmetricStoredKey", "()V", ()).await?;
    let query_key = jvm.new_class("AsymmetricQueryKey", "()V", ()).await?;
    let value = JavaLangString::from_rust_string(&jvm, "direction-value").await?;

    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &hash_map,
            "put",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            (stored_key, value.clone()),
        )
        .await?;

    let contains_key: bool = jvm
        .invoke_virtual(&hash_map, "containsKey", "(Ljava/lang/Object;)Z", (query_key.clone(),))
        .await?;
    assert!(contains_key);

    let found: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&hash_map, "get", "(Ljava/lang/Object;)Ljava/lang/Object;", (query_key.clone(),))
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &found).await?, "direction-value");

    let removed: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&hash_map, "remove", "(Ljava/lang/Object;)Ljava/lang/Object;", (query_key,))
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &removed).await?, "direction-value");

    let size: i32 = jvm.invoke_virtual(&hash_map, "size", "()I", ()).await?;
    assert_eq!(size, 0);

    Ok(())
}

#[tokio::test]
async fn test_hash_map_key_set_view_backing_and_iterator_snapshot() -> Result<()> {
    let jvm = test_jvm().await?;

    let hash_map = jvm.new_class("java/util/HashMap", "()V", ()).await?;
    let first_key = JavaLangString::from_rust_string(&jvm, "first").await?;
    let second_key = JavaLangString::from_rust_string(&jvm, "second").await?;
    let third_key = JavaLangString::from_rust_string(&jvm, "third").await?;
    let missing_key = JavaLangString::from_rust_string(&jvm, "missing").await?;
    let first_value = JavaLangString::from_rust_string(&jvm, "first-value").await?;
    let second_value = JavaLangString::from_rust_string(&jvm, "second-value").await?;
    let third_value = JavaLangString::from_rust_string(&jvm, "third-value").await?;

    for (key, value) in [(first_key.clone(), first_value), (second_key.clone(), second_value)] {
        let old: ClassInstanceRef<Object> = jvm
            .invoke_virtual(&hash_map, "put", "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;", (key, value))
            .await?;
        assert!(old.is_null());
    }

    let key_set: ClassInstanceRef<Object> = jvm.invoke_virtual(&hash_map, "keySet", "()Ljava/util/Set;", ()).await?;
    assert!(jvm.is_instance(&**key_set, "java/util/Set"));
    assert!(jvm.is_instance(&**key_set, "java/util/Collection"));
    assert!(jvm.is_instance(&**key_set, "java/util/AbstractSet"));

    let size: i32 = jvm.invoke_virtual(&key_set, "size", "()I", ()).await?;
    assert_eq!(size, 2);
    let contains: bool = jvm
        .invoke_virtual(&key_set, "contains", "(Ljava/lang/Object;)Z", (first_key.clone(),))
        .await?;
    assert!(contains);
    let contains: bool = jvm
        .invoke_virtual(&key_set, "contains", "(Ljava/lang/Object;)Z", (missing_key.clone(),))
        .await?;
    assert!(!contains);

    let key_array: ClassInstanceRef<Array<Object>> = jvm.invoke_virtual(&key_set, "toArray", "()[Ljava/lang/Object;", ()).await?;
    let key_array_values = sorted_optional_strings(object_array_to_optional_strings(&jvm, &key_array).await?);
    assert_eq!(key_array_values, vec![Some("first".to_string()), Some("second".to_string())]);

    let iterator: ClassInstanceRef<Object> = jvm.invoke_virtual(&key_set, "iterator", "()Ljava/util/Iterator;", ()).await?;
    assert_eq!(iterator.class_definition().name(), "java/util/HashMap$KeyIterator");
    assert!(jvm.is_instance(&**iterator, "java/util/Iterator"));
    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &hash_map,
            "put",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            (third_key.clone(), third_value),
        )
        .await?;

    let snapshot = sorted_optional_strings(iterator_to_optional_strings(&jvm, &iterator).await?);
    assert_eq!(snapshot, vec![Some("first".to_string()), Some("second".to_string())]);
    assert_next_throws_no_such_element(&jvm, &iterator).await?;
    assert_remove_throws_unsupported(&jvm, &iterator).await?;

    let size: i32 = jvm.invoke_virtual(&key_set, "size", "()I", ()).await?;
    assert_eq!(size, 3);
    let contains: bool = jvm
        .invoke_virtual(&key_set, "contains", "(Ljava/lang/Object;)Z", (third_key.clone(),))
        .await?;
    assert!(contains);

    let null_value_key = JavaLangString::from_rust_string(&jvm, "null-value-key").await?;
    let null_ref: ClassInstanceRef<Object> = None.into();
    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &hash_map,
            "put",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            (null_value_key.clone(), null_ref),
        )
        .await?;

    let removed: bool = jvm
        .invoke_virtual(&key_set, "remove", "(Ljava/lang/Object;)Z", (second_key.clone(),))
        .await?;
    assert!(removed);
    let removed: bool = jvm
        .invoke_virtual(&key_set, "remove", "(Ljava/lang/Object;)Z", (null_value_key.clone(),))
        .await?;
    assert!(removed);
    let removed: bool = jvm.invoke_virtual(&key_set, "remove", "(Ljava/lang/Object;)Z", (missing_key,)).await?;
    assert!(!removed);
    let contains_key: bool = jvm
        .invoke_virtual(&hash_map, "containsKey", "(Ljava/lang/Object;)Z", (second_key,))
        .await?;
    assert!(!contains_key);
    let contains_key: bool = jvm
        .invoke_virtual(&hash_map, "containsKey", "(Ljava/lang/Object;)Z", (null_value_key,))
        .await?;
    assert!(!contains_key);

    let _: () = jvm.invoke_virtual(&key_set, "clear", "()V", ()).await?;
    let size: i32 = jvm.invoke_virtual(&hash_map, "size", "()I", ()).await?;
    assert_eq!(size, 0);
    let is_empty: bool = jvm.invoke_virtual(&key_set, "isEmpty", "()Z", ()).await?;
    assert!(is_empty);

    let empty_iterator: ClassInstanceRef<Object> = jvm.invoke_virtual(&key_set, "iterator", "()Ljava/util/Iterator;", ()).await?;
    let has_next: bool = jvm.invoke_virtual(&empty_iterator, "hasNext", "()Z", ()).await?;
    assert!(!has_next);
    assert_next_throws_no_such_element(&jvm, &empty_iterator).await?;

    Ok(())
}

#[tokio::test]
async fn test_hash_map_values_view_contains_clear_and_iterator_snapshot() -> Result<()> {
    let jvm = test_jvm().await?;

    let hash_map = jvm.new_class("java/util/HashMap", "()V", ()).await?;
    let null_ref: ClassInstanceRef<Object> = None.into();
    let null_value_key = JavaLangString::from_rust_string(&jvm, "null-value-key").await?;
    let equal_value_key = JavaLangString::from_rust_string(&jvm, "equal-value-key").await?;
    let third_key = JavaLangString::from_rust_string(&jvm, "third-key").await?;
    let equal_value = JavaLangString::from_rust_string(&jvm, "same-value").await?;
    let equal_value_distinct = JavaLangString::from_rust_string(&jvm, "same-value").await?;
    let third_value = JavaLangString::from_rust_string(&jvm, "third-value").await?;
    let missing_value = JavaLangString::from_rust_string(&jvm, "missing-value").await?;

    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &hash_map,
            "put",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            (null_value_key, null_ref.clone()),
        )
        .await?;
    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &hash_map,
            "put",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            (equal_value_key, equal_value),
        )
        .await?;

    let values: ClassInstanceRef<Object> = jvm.invoke_virtual(&hash_map, "values", "()Ljava/util/Collection;", ()).await?;
    assert!(jvm.is_instance(&**values, "java/util/Collection"));
    assert!(jvm.is_instance(&**values, "java/util/AbstractCollection"));
    assert!(!jvm.is_instance(&**values, "java/util/Set"));

    let size: i32 = jvm.invoke_virtual(&values, "size", "()I", ()).await?;
    assert_eq!(size, 2);
    let contains_null: bool = jvm
        .invoke_virtual(&values, "contains", "(Ljava/lang/Object;)Z", (null_ref.clone(),))
        .await?;
    assert!(contains_null);
    let contains_equal: bool = jvm
        .invoke_virtual(&values, "contains", "(Ljava/lang/Object;)Z", (equal_value_distinct.clone(),))
        .await?;
    assert!(contains_equal);
    let contains_missing: bool = jvm.invoke_virtual(&values, "contains", "(Ljava/lang/Object;)Z", (missing_value,)).await?;
    assert!(!contains_missing);

    let values_array: ClassInstanceRef<Array<Object>> = jvm.invoke_virtual(&values, "toArray", "()[Ljava/lang/Object;", ()).await?;
    let values_array = sorted_optional_strings(object_array_to_optional_strings(&jvm, &values_array).await?);
    assert_eq!(values_array, vec![None, Some("same-value".to_string())]);

    let iterator: ClassInstanceRef<Object> = jvm.invoke_virtual(&values, "iterator", "()Ljava/util/Iterator;", ()).await?;
    assert_eq!(iterator.class_definition().name(), "java/util/HashMap$ValueIterator");
    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &hash_map,
            "put",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            (third_key, third_value),
        )
        .await?;

    let snapshot = sorted_optional_strings(iterator_to_optional_strings(&jvm, &iterator).await?);
    assert_eq!(snapshot, vec![None, Some("same-value".to_string())]);
    assert_next_throws_no_such_element(&jvm, &iterator).await?;
    assert_remove_throws_unsupported(&jvm, &iterator).await?;

    let size: i32 = jvm.invoke_virtual(&values, "size", "()I", ()).await?;
    assert_eq!(size, 3);

    let _: () = jvm.invoke_virtual(&values, "clear", "()V", ()).await?;
    let size: i32 = jvm.invoke_virtual(&hash_map, "size", "()I", ()).await?;
    assert_eq!(size, 0);
    let is_empty: bool = jvm.invoke_virtual(&values, "isEmpty", "()Z", ()).await?;
    assert!(is_empty);

    let values_array: ClassInstanceRef<Array<Object>> = jvm.invoke_virtual(&values, "toArray", "()[Ljava/lang/Object;", ()).await?;
    assert_eq!(jvm.array_length(&values_array).await?, 0);

    Ok(())
}

#[tokio::test]
async fn test_hash_map_entry_set_view_backing_clear_and_iterator_snapshot() -> Result<()> {
    let jvm = test_jvm().await?;

    let hash_map = jvm.new_class("java/util/HashMap", "()V", ()).await?;
    let first_key = JavaLangString::from_rust_string(&jvm, "first").await?;
    let second_key = JavaLangString::from_rust_string(&jvm, "second").await?;
    let third_key = JavaLangString::from_rust_string(&jvm, "third").await?;
    let first_value = JavaLangString::from_rust_string(&jvm, "first-value").await?;
    let second_value = JavaLangString::from_rust_string(&jvm, "second-value").await?;
    let third_value = JavaLangString::from_rust_string(&jvm, "third-value").await?;

    for (key, value) in [(first_key.clone(), first_value), (second_key.clone(), second_value)] {
        let old: ClassInstanceRef<Object> = jvm
            .invoke_virtual(&hash_map, "put", "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;", (key, value))
            .await?;
        assert!(old.is_null());
    }

    let entry_set: ClassInstanceRef<Object> = jvm.invoke_virtual(&hash_map, "entrySet", "()Ljava/util/Set;", ()).await?;
    assert!(jvm.is_instance(&**entry_set, "java/util/Set"));
    assert!(jvm.is_instance(&**entry_set, "java/util/Collection"));
    assert!(jvm.is_instance(&**entry_set, "java/util/AbstractSet"));

    let size: i32 = jvm.invoke_virtual(&entry_set, "size", "()I", ()).await?;
    assert_eq!(size, 2);
    let is_empty: bool = jvm.invoke_virtual(&entry_set, "isEmpty", "()Z", ()).await?;
    assert!(!is_empty);

    let iterator: ClassInstanceRef<Object> = jvm.invoke_virtual(&entry_set, "iterator", "()Ljava/util/Iterator;", ()).await?;
    assert_eq!(iterator.class_definition().name(), "java/util/HashMap$EntryIterator");
    assert!(jvm.is_instance(&**iterator, "java/util/Iterator"));
    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &hash_map,
            "put",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            (third_key, third_value),
        )
        .await?;

    let snapshot = sorted_entry_pairs(iterator_to_entry_pairs(&jvm, &iterator).await?);
    assert_eq!(
        snapshot,
        vec![
            (Some("first".to_string()), Some("first-value".to_string())),
            (Some("second".to_string()), Some("second-value".to_string()))
        ]
    );
    assert_next_throws_no_such_element(&jvm, &iterator).await?;
    assert_remove_throws_unsupported(&jvm, &iterator).await?;

    let size: i32 = jvm.invoke_virtual(&entry_set, "size", "()I", ()).await?;
    assert_eq!(size, 3);

    let _: () = jvm.invoke_virtual(&entry_set, "clear", "()V", ()).await?;
    let size: i32 = jvm.invoke_virtual(&hash_map, "size", "()I", ()).await?;
    assert_eq!(size, 0);
    let is_empty: bool = jvm.invoke_virtual(&entry_set, "isEmpty", "()Z", ()).await?;
    assert!(is_empty);

    let empty_iterator: ClassInstanceRef<Object> = jvm.invoke_virtual(&entry_set, "iterator", "()Ljava/util/Iterator;", ()).await?;
    let has_next: bool = jvm.invoke_virtual(&empty_iterator, "hasNext", "()Z", ()).await?;
    assert!(!has_next);
    assert_next_throws_no_such_element(&jvm, &empty_iterator).await?;

    Ok(())
}

#[tokio::test]
async fn test_hash_map_entry_set_iterator_set_value_updates_backing_map() -> Result<()> {
    let jvm = test_jvm().await?;

    let hash_map = jvm.new_class("java/util/HashMap", "()V", ()).await?;
    let key = JavaLangString::from_rust_string(&jvm, "mutable-key").await?;
    let value = JavaLangString::from_rust_string(&jvm, "old-value").await?;
    let replacement = JavaLangString::from_rust_string(&jvm, "new-value").await?;
    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &hash_map,
            "put",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            (key.clone(), value.clone()),
        )
        .await?;

    let entry_set: ClassInstanceRef<Object> = jvm.invoke_virtual(&hash_map, "entrySet", "()Ljava/util/Set;", ()).await?;
    let iterator: ClassInstanceRef<Object> = jvm.invoke_virtual(&entry_set, "iterator", "()Ljava/util/Iterator;", ()).await?;
    let entry: ClassInstanceRef<Object> = jvm.invoke_virtual(&iterator, "next", "()Ljava/lang/Object;", ()).await?;
    assert!(jvm.is_instance(&**entry, "java/util/Map$Entry"));

    let old_value: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&entry, "setValue", "(Ljava/lang/Object;)Ljava/lang/Object;", (replacement.clone(),))
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &old_value).await?, "old-value");

    let found: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&hash_map, "get", "(Ljava/lang/Object;)Ljava/lang/Object;", (key,))
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &found).await?, "new-value");

    let entry_value: ClassInstanceRef<Object> = jvm.invoke_virtual(&entry, "getValue", "()Ljava/lang/Object;", ()).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &entry_value).await?, "new-value");

    Ok(())
}

#[tokio::test]
async fn test_hash_map_entry_set_contains_candidates_and_null_values() -> Result<()> {
    let jvm = test_jvm().await?;

    let hash_map = jvm.new_class("java/util/HashMap", "()V", ()).await?;
    let key = JavaLangString::from_rust_string(&jvm, "contains-key").await?;
    let value = JavaLangString::from_rust_string(&jvm, "contains-value").await?;
    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &hash_map,
            "put",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            (key.clone(), value.clone()),
        )
        .await?;

    let entry_set: ClassInstanceRef<Object> = jvm.invoke_virtual(&hash_map, "entrySet", "()Ljava/util/Set;", ()).await?;
    let iterator: ClassInstanceRef<Object> = jvm.invoke_virtual(&entry_set, "iterator", "()Ljava/util/Iterator;", ()).await?;
    let backing_entry: ClassInstanceRef<Object> = jvm.invoke_virtual(&iterator, "next", "()Ljava/lang/Object;", ()).await?;
    let contains: bool = jvm
        .invoke_virtual(&entry_set, "contains", "(Ljava/lang/Object;)Z", (backing_entry.clone(),))
        .await?;
    assert!(contains);

    let null_ref: ClassInstanceRef<Object> = None.into();
    let contains: bool = jvm
        .invoke_virtual(&entry_set, "contains", "(Ljava/lang/Object;)Z", (null_ref.clone(),))
        .await?;
    assert!(!contains);

    let equal_key = JavaLangString::from_rust_string(&jvm, "contains-key").await?;
    let equal_value = JavaLangString::from_rust_string(&jvm, "contains-value").await?;
    let next: ClassInstanceRef<HashMapEntry> = None.into();
    let candidate_equal: ClassInstanceRef<HashMapEntry> = jvm
        .new_class(
            "java/util/HashMap$Entry",
            "(ILjava/lang/Object;Ljava/lang/Object;Ljava/util/HashMap$Entry;)V",
            (0, equal_key.clone(), equal_value, next),
        )
        .await?
        .into();
    let contains: bool = jvm
        .invoke_virtual(&entry_set, "contains", "(Ljava/lang/Object;)Z", (candidate_equal,))
        .await?;
    assert!(contains);

    let different_value = JavaLangString::from_rust_string(&jvm, "different-value").await?;
    let next: ClassInstanceRef<HashMapEntry> = None.into();
    let candidate_different_value: ClassInstanceRef<HashMapEntry> = jvm
        .new_class(
            "java/util/HashMap$Entry",
            "(ILjava/lang/Object;Ljava/lang/Object;Ljava/util/HashMap$Entry;)V",
            (0, equal_key, different_value, next),
        )
        .await?
        .into();
    let contains: bool = jvm
        .invoke_virtual(&entry_set, "contains", "(Ljava/lang/Object;)Z", (candidate_different_value,))
        .await?;
    assert!(!contains);

    let equal_key = JavaLangString::from_rust_string(&jvm, "contains-key").await?;
    let next: ClassInstanceRef<HashMapEntry> = None.into();
    let candidate_null_for_non_null_value: ClassInstanceRef<HashMapEntry> = jvm
        .new_class(
            "java/util/HashMap$Entry",
            "(ILjava/lang/Object;Ljava/lang/Object;Ljava/util/HashMap$Entry;)V",
            (0, equal_key, null_ref.clone(), next),
        )
        .await?
        .into();
    let contains: bool = jvm
        .invoke_virtual(&entry_set, "contains", "(Ljava/lang/Object;)Z", (candidate_null_for_non_null_value,))
        .await?;
    assert!(!contains);

    let null_key_value = JavaLangString::from_rust_string(&jvm, "null-key-value").await?;
    let equal_null_key_value = JavaLangString::from_rust_string(&jvm, "null-key-value").await?;
    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &hash_map,
            "put",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            (null_ref.clone(), null_key_value),
        )
        .await?;

    let next: ClassInstanceRef<HashMapEntry> = None.into();
    let candidate_null_key: ClassInstanceRef<HashMapEntry> = jvm
        .new_class(
            "java/util/HashMap$Entry",
            "(ILjava/lang/Object;Ljava/lang/Object;Ljava/util/HashMap$Entry;)V",
            (0, null_ref.clone(), equal_null_key_value, next),
        )
        .await?
        .into();
    let contains: bool = jvm
        .invoke_virtual(&entry_set, "contains", "(Ljava/lang/Object;)Z", (candidate_null_key,))
        .await?;
    assert!(contains);

    let null_value_key = JavaLangString::from_rust_string(&jvm, "null-value-key").await?;
    let equal_null_value_key = JavaLangString::from_rust_string(&jvm, "null-value-key").await?;
    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &hash_map,
            "put",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            (null_value_key, null_ref.clone()),
        )
        .await?;

    let next: ClassInstanceRef<HashMapEntry> = None.into();
    let candidate_null_value: ClassInstanceRef<HashMapEntry> = jvm
        .new_class(
            "java/util/HashMap$Entry",
            "(ILjava/lang/Object;Ljava/lang/Object;Ljava/util/HashMap$Entry;)V",
            (0, equal_null_value_key, null_ref, next),
        )
        .await?
        .into();
    let contains: bool = jvm
        .invoke_virtual(&entry_set, "contains", "(Ljava/lang/Object;)Z", (candidate_null_value,))
        .await?;
    assert!(contains);

    let non_entry = JavaLangString::from_rust_string(&jvm, "not-an-entry").await?;
    let contains: bool = jvm.invoke_virtual(&entry_set, "contains", "(Ljava/lang/Object;)Z", (non_entry,)).await?;
    assert!(!contains);

    Ok(())
}

#[tokio::test]
async fn test_hash_map_entry_set_contains_custom_map_entry() -> Result<()> {
    let runtime = TestRuntime::new(BTreeMap::new());
    let jvm = create_test_jvm(runtime.clone()).await?;

    let custom_entry_class = Box::new(ClassDefinitionImpl::from_class_proto(
        CustomMapEntry::as_proto(),
        Box::new(runtime.clone()) as Box<_>,
    ));
    jvm.register_class(custom_entry_class, None).await?;

    let hash_map = jvm.new_class("java/util/HashMap", "()V", ()).await?;
    let key = JavaLangString::from_rust_string(&jvm, "custom-key").await?;
    let value = JavaLangString::from_rust_string(&jvm, "custom-value").await?;
    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &hash_map,
            "put",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            (key.clone(), value.clone()),
        )
        .await?;

    let entry_set: ClassInstanceRef<Object> = jvm.invoke_virtual(&hash_map, "entrySet", "()Ljava/util/Set;", ()).await?;
    let equal_key = JavaLangString::from_rust_string(&jvm, "custom-key").await?;
    let equal_value = JavaLangString::from_rust_string(&jvm, "custom-value").await?;
    let custom_equal = jvm
        .new_class("CustomMapEntry", "(Ljava/lang/Object;Ljava/lang/Object;)V", (equal_key, equal_value))
        .await?;
    assert!(jvm.is_instance(&*custom_equal, "java/util/Map$Entry"));

    let contains: bool = jvm
        .invoke_virtual(&entry_set, "contains", "(Ljava/lang/Object;)Z", (custom_equal,))
        .await?;
    assert!(contains);

    let equal_key = JavaLangString::from_rust_string(&jvm, "custom-key").await?;
    let different_value = JavaLangString::from_rust_string(&jvm, "different-value").await?;
    let custom_different = jvm
        .new_class("CustomMapEntry", "(Ljava/lang/Object;Ljava/lang/Object;)V", (equal_key, different_value))
        .await?;
    let contains: bool = jvm
        .invoke_virtual(&entry_set, "contains", "(Ljava/lang/Object;)Z", (custom_different,))
        .await?;
    assert!(!contains);

    Ok(())
}
