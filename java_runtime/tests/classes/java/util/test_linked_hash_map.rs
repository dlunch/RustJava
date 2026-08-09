use alloc::{boxed::Box, collections::BTreeMap, vec, vec::Vec};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, MethodAccessFlags};
use java_runtime::{RuntimeClassProto, RuntimeContext, classes::java::lang::Object, get_runtime_class_proto};
use jvm::{Array, ClassInstanceRef, JavaError, Jvm, Result, runtime::JavaLangString};
use jvm_rust::ClassDefinitionImpl;

use test_utils::{TestRuntime, create_test_jvm, test_jvm};

struct LimitedLinkedHashMap;

impl LimitedLinkedHashMap {
    fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "LimitedLinkedHashMap",
            parent_class: Some("java/util/LinkedHashMap"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(I)V", Self::init, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "removeEldestEntry",
                    "(Ljava/util/Map$Entry;)Z",
                    Self::remove_eldest_entry,
                    MethodAccessFlags::PROTECTED,
                ),
            ],
            fields: vec![
                JavaFieldProto::new("limit", "I", Default::default()),
                JavaFieldProto::new("callbacks", "I", Default::default()),
            ],
            access_flags: ClassAccessFlags::PUBLIC,
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, limit: i32) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/util/LinkedHashMap", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "limit", "I", limit).await?;
        jvm.put_field(&mut this, "callbacks", "I", 0).await
    }

    async fn remove_eldest_entry(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, _: ClassInstanceRef<Object>) -> Result<bool> {
        let callbacks: i32 = jvm.get_field(&this, "callbacks", "I").await?;
        jvm.put_field(&mut this, "callbacks", "I", callbacks + 1).await?;
        let size: i32 = jvm.invoke_virtual(&this, "size", "()I", ()).await?;
        let limit: i32 = jvm.get_field(&this, "limit", "I").await?;

        Ok(size > limit)
    }
}

async fn view_strings(jvm: &Jvm, map: &ClassInstanceRef<Object>, method: &str, descriptor: &str) -> Result<Vec<Option<String>>> {
    let view: ClassInstanceRef<Object> = jvm.invoke_virtual(map, method, descriptor, ()).await?;
    let iterator: ClassInstanceRef<Object> = jvm.invoke_virtual(&view, "iterator", "()Ljava/util/Iterator;", ()).await?;
    let mut values = Vec::new();
    while jvm.invoke_virtual::<_, bool>(&iterator, "hasNext", "()Z", ()).await? {
        let value: ClassInstanceRef<Object> = jvm.invoke_virtual(&iterator, "next", "()Ljava/lang/Object;", ()).await?;
        values.push(if value.is_null() {
            None
        } else {
            Some(JavaLangString::to_rust_string(jvm, &value).await?)
        });
    }

    Ok(values)
}

async fn entry_strings(jvm: &Jvm, map: &ClassInstanceRef<Object>) -> Result<Vec<(Option<String>, Option<String>)>> {
    let view: ClassInstanceRef<Object> = jvm.invoke_virtual(map, "entrySet", "()Ljava/util/Set;", ()).await?;
    let iterator: ClassInstanceRef<Object> = jvm.invoke_virtual(&view, "iterator", "()Ljava/util/Iterator;", ()).await?;
    let mut entries = Vec::new();
    while jvm.invoke_virtual::<_, bool>(&iterator, "hasNext", "()Z", ()).await? {
        let entry: ClassInstanceRef<Object> = jvm.invoke_virtual(&iterator, "next", "()Ljava/lang/Object;", ()).await?;
        let key: ClassInstanceRef<Object> = jvm.invoke_virtual(&entry, "getKey", "()Ljava/lang/Object;", ()).await?;
        let value: ClassInstanceRef<Object> = jvm.invoke_virtual(&entry, "getValue", "()Ljava/lang/Object;", ()).await?;
        entries.push((
            if key.is_null() {
                None
            } else {
                Some(JavaLangString::to_rust_string(jvm, &key).await?)
            },
            if value.is_null() {
                None
            } else {
                Some(JavaLangString::to_rust_string(jvm, &value).await?)
            },
        ));
    }

    Ok(entries)
}

#[tokio::test]
async fn linked_hash_map_exposes_the_cdc_11_class_shape_and_constructor_validation() -> Result<()> {
    let proto = get_runtime_class_proto("java/util/LinkedHashMap").expect("LinkedHashMap must be registered");
    assert_eq!(proto.parent_class, Some("java/util/HashMap"));
    assert_eq!(proto.interfaces, vec!["java/util/Map"]);
    assert_eq!(proto.access_flags, ClassAccessFlags::PUBLIC);
    for descriptor in ["()V", "(I)V", "(IF)V", "(IFZ)V", "(Ljava/util/Map;)V"] {
        let method = proto
            .methods
            .iter()
            .find(|method| method.name == "<init>" && method.descriptor == descriptor)
            .unwrap_or_else(|| panic!("missing LinkedHashMap{descriptor}"));
        assert_eq!(method.access_flags, MethodAccessFlags::PUBLIC);
    }
    for (name, descriptor, flags) in [
        ("containsValue", "(Ljava/lang/Object;)Z", MethodAccessFlags::PUBLIC),
        ("get", "(Ljava/lang/Object;)Ljava/lang/Object;", MethodAccessFlags::PUBLIC),
        ("clear", "()V", MethodAccessFlags::PUBLIC),
        ("removeEldestEntry", "(Ljava/util/Map$Entry;)Z", MethodAccessFlags::PROTECTED),
    ] {
        let method = proto
            .methods
            .iter()
            .find(|method| method.name == name && method.descriptor == descriptor)
            .unwrap_or_else(|| panic!("missing {name}{descriptor}"));
        assert_eq!(method.access_flags, flags);
    }

    let jvm = test_jvm().await?;
    let map = jvm.new_class("java/util/LinkedHashMap", "()V", ()).await?;
    assert!(jvm.is_instance(&*map, "java/util/HashMap"));
    assert!(jvm.is_instance(&*map, "java/util/Map"));

    for result in [
        jvm.new_class("java/util/LinkedHashMap", "(I)V", (-1,)).await,
        jvm.new_class("java/util/LinkedHashMap", "(IF)V", (1, 0.0f32)).await,
        jvm.new_class("java/util/LinkedHashMap", "(IF)V", (1, -1.0f32)).await,
        jvm.new_class("java/util/LinkedHashMap", "(IFZ)V", (1, f32::NAN, false)).await,
    ] {
        let Err(JavaError::JavaException(exception)) = result else {
            panic!("invalid LinkedHashMap constructor arguments must fail");
        };
        assert!(jvm.is_instance(&*exception, "java/lang/IllegalArgumentException"));
    }

    Ok(())
}

#[tokio::test]
async fn linked_hash_map_preserves_insertion_order_across_views_nulls_and_rehash() -> Result<()> {
    let jvm = test_jvm().await?;
    let map: ClassInstanceRef<Object> = jvm.new_class("java/util/LinkedHashMap", "(IF)V", (1, 0.75f32)).await?.into();

    for index in 0..24 {
        let key = JavaLangString::from_rust_string(&jvm, &format!("k{index:02}")).await?;
        let value = JavaLangString::from_rust_string(&jvm, &format!("v{index:02}")).await?;
        let _: ClassInstanceRef<Object> = jvm
            .invoke_virtual(&map, "put", "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;", (key, value))
            .await?;
    }
    let replacement_key = JavaLangString::from_rust_string(&jvm, "k05").await?;
    let replacement = JavaLangString::from_rust_string(&jvm, "changed").await?;
    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &map,
            "put",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            (replacement_key, replacement),
        )
        .await?;
    let null: ClassInstanceRef<Object> = None.into();
    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &map,
            "put",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            (null.clone(), null),
        )
        .await?;

    let mut expected_keys: Vec<Option<String>> = (0..24).map(|index| Some(format!("k{index:02}"))).collect();
    expected_keys.push(None);
    assert_eq!(view_strings(&jvm, &map, "keySet", "()Ljava/util/Set;").await?, expected_keys);

    let values = view_strings(&jvm, &map, "values", "()Ljava/util/Collection;").await?;
    assert_eq!(values[5], Some("changed".into()));
    assert_eq!(values.last(), Some(&None));
    let entries = entry_strings(&jvm, &map).await?;
    assert_eq!(entries[5], (Some("k05".into()), Some("changed".into())));
    assert_eq!(entries.last(), Some(&(None, None)));

    let keys: ClassInstanceRef<Object> = jvm.invoke_virtual(&map, "keySet", "()Ljava/util/Set;", ()).await?;
    let iterator: ClassInstanceRef<Object> = jvm.invoke_virtual(&keys, "iterator", "()Ljava/util/Iterator;", ()).await?;
    let key = JavaLangString::from_rust_string(&jvm, "k00").await?;
    let _: ClassInstanceRef<Object> = jvm.invoke_virtual(&map, "get", "(Ljava/lang/Object;)Ljava/lang/Object;", (key,)).await?;
    let first: ClassInstanceRef<Object> = jvm.invoke_virtual(&iterator, "next", "()Ljava/lang/Object;", ()).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &first).await?, "k00");

    let text: ClassInstanceRef<Object> = jvm.invoke_virtual(&map, "toString", "()Ljava/lang/String;", ()).await?;
    let text = JavaLangString::to_rust_string(&jvm, &text).await?;
    assert!(text.starts_with("{k00=v00, k01=v01, k02=v02"));
    assert!(text.ends_with(", null=null}"));

    Ok(())
}

#[tokio::test]
async fn linked_hash_map_access_order_tracks_only_documented_accesses() -> Result<()> {
    let jvm = test_jvm().await?;
    let map: ClassInstanceRef<Object> = jvm.new_class("java/util/LinkedHashMap", "(IFZ)V", (4, 0.75f32, true)).await?.into();
    for key in ["a", "b", "c"] {
        let value = JavaLangString::from_rust_string(&jvm, key).await?;
        let key = JavaLangString::from_rust_string(&jvm, key).await?;
        let _: ClassInstanceRef<Object> = jvm
            .invoke_virtual(&map, "put", "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;", (key, value))
            .await?;
    }

    let keys: ClassInstanceRef<Object> = jvm.invoke_virtual(&map, "keySet", "()Ljava/util/Set;", ()).await?;
    let iterator: ClassInstanceRef<Object> = jvm.invoke_virtual(&keys, "iterator", "()Ljava/util/Iterator;", ()).await?;
    let tail = JavaLangString::from_rust_string(&jvm, "c").await?;
    let _: ClassInstanceRef<Object> = jvm.invoke_virtual(&map, "get", "(Ljava/lang/Object;)Ljava/lang/Object;", (tail,)).await?;
    let first: ClassInstanceRef<Object> = jvm.invoke_virtual(&iterator, "next", "()Ljava/lang/Object;", ()).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &first).await?, "a");

    let iterator: ClassInstanceRef<Object> = jvm.invoke_virtual(&keys, "iterator", "()Ljava/util/Iterator;", ()).await?;
    let tail = JavaLangString::from_rust_string(&jvm, "c").await?;
    let value = JavaLangString::from_rust_string(&jvm, "C").await?;
    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&map, "put", "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;", (tail, value))
        .await?;
    let first: ClassInstanceRef<Object> = jvm.invoke_virtual(&iterator, "next", "()Ljava/lang/Object;", ()).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &first).await?, "a");

    let iterator: ClassInstanceRef<Object> = jvm.invoke_virtual(&keys, "iterator", "()Ljava/util/Iterator;", ()).await?;
    let missing = JavaLangString::from_rust_string(&jvm, "missing").await?;
    let missing_value: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&map, "get", "(Ljava/lang/Object;)Ljava/lang/Object;", (missing,))
        .await?;
    assert!(missing_value.is_null());
    let first: ClassInstanceRef<Object> = jvm.invoke_virtual(&iterator, "next", "()Ljava/lang/Object;", ()).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &first).await?, "a");

    let iterator: ClassInstanceRef<Object> = jvm.invoke_virtual(&keys, "iterator", "()Ljava/util/Iterator;", ()).await?;
    let key = JavaLangString::from_rust_string(&jvm, "a").await?;
    let _: ClassInstanceRef<Object> = jvm.invoke_virtual(&map, "get", "(Ljava/lang/Object;)Ljava/lang/Object;", (key,)).await?;
    let result: Result<ClassInstanceRef<Object>> = jvm.invoke_virtual(&iterator, "next", "()Ljava/lang/Object;", ()).await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("access-order get must invalidate an existing iterator");
    };
    assert!(jvm.is_instance(&*exception, "java/util/ConcurrentModificationException"));
    assert_eq!(
        view_strings(&jvm, &map, "keySet", "()Ljava/util/Set;").await?,
        vec![Some("b".into()), Some("c".into()), Some("a".into())]
    );

    let key = JavaLangString::from_rust_string(&jvm, "b").await?;
    assert!(
        jvm.invoke_virtual::<_, bool>(&map, "containsKey", "(Ljava/lang/Object;)Z", (key,))
            .await?
    );
    assert_eq!(
        view_strings(&jvm, &map, "keySet", "()Ljava/util/Set;").await?,
        vec![Some("b".into()), Some("c".into()), Some("a".into())]
    );

    let key = JavaLangString::from_rust_string(&jvm, "b").await?;
    let value = JavaLangString::from_rust_string(&jvm, "B").await?;
    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&map, "put", "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;", (key, value))
        .await?;
    assert_eq!(
        view_strings(&jvm, &map, "keySet", "()Ljava/util/Set;").await?,
        vec![Some("c".into()), Some("a".into()), Some("b".into())]
    );

    let source: ClassInstanceRef<Object> = jvm.new_class("java/util/LinkedHashMap", "()V", ()).await?.into();
    for key in ["a", "d"] {
        let value = JavaLangString::from_rust_string(&jvm, key).await?;
        let key = JavaLangString::from_rust_string(&jvm, key).await?;
        let _: ClassInstanceRef<Object> = jvm
            .invoke_virtual(&source, "put", "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;", (key, value))
            .await?;
    }
    let _: () = jvm.invoke_virtual(&map, "putAll", "(Ljava/util/Map;)V", (source,)).await?;
    assert_eq!(
        view_strings(&jvm, &map, "keySet", "()Ljava/util/Set;").await?,
        vec![Some("c".into()), Some("b".into()), Some("a".into()), Some("d".into())]
    );

    Ok(())
}

#[tokio::test]
async fn linked_hash_map_views_and_iterators_remove_in_order_and_fail_fast() -> Result<()> {
    let jvm = test_jvm().await?;
    let map: ClassInstanceRef<Object> = jvm.new_class("java/util/LinkedHashMap", "()V", ()).await?.into();
    for (key, value) in [("a", "same"), ("b", "same"), ("c", "other")] {
        let key = JavaLangString::from_rust_string(&jvm, key).await?;
        let value = JavaLangString::from_rust_string(&jvm, value).await?;
        let _: ClassInstanceRef<Object> = jvm
            .invoke_virtual(&map, "put", "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;", (key, value))
            .await?;
    }

    let values: ClassInstanceRef<Object> = jvm.invoke_virtual(&map, "values", "()Ljava/util/Collection;", ()).await?;
    let same = JavaLangString::from_rust_string(&jvm, "same").await?;
    assert!(jvm.invoke_virtual::<_, bool>(&values, "remove", "(Ljava/lang/Object;)Z", (same,)).await?);
    assert_eq!(
        view_strings(&jvm, &map, "keySet", "()Ljava/util/Set;").await?,
        vec![Some("b".into()), Some("c".into())]
    );

    let keys: ClassInstanceRef<Object> = jvm.invoke_virtual(&map, "keySet", "()Ljava/util/Set;", ()).await?;
    let iterator: ClassInstanceRef<Object> = jvm.invoke_virtual(&keys, "iterator", "()Ljava/util/Iterator;", ()).await?;
    let first: ClassInstanceRef<Object> = jvm.invoke_virtual(&iterator, "next", "()Ljava/lang/Object;", ()).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &first).await?, "b");
    let _: () = jvm.invoke_virtual(&iterator, "remove", "()V", ()).await?;
    let result: Result<()> = jvm.invoke_virtual(&iterator, "remove", "()V", ()).await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("repeated iterator remove must fail");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/IllegalStateException"));
    assert_eq!(view_strings(&jvm, &map, "keySet", "()Ljava/util/Set;").await?, vec![Some("c".into())]);

    let iterator: ClassInstanceRef<Object> = jvm.invoke_virtual(&keys, "iterator", "()Ljava/util/Iterator;", ()).await?;
    let key = JavaLangString::from_rust_string(&jvm, "d").await?;
    let value = JavaLangString::from_rust_string(&jvm, "other").await?;
    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&map, "put", "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;", (key, value))
        .await?;
    let result: Result<ClassInstanceRef<Object>> = jvm.invoke_virtual(&iterator, "next", "()Ljava/lang/Object;", ()).await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("structural modification must invalidate iterator");
    };
    assert!(jvm.is_instance(&*exception, "java/util/ConcurrentModificationException"));

    let _: () = jvm.invoke_virtual(&map, "clear", "()V", ()).await?;
    assert!(view_strings(&jvm, &map, "keySet", "()Ljava/util/Set;").await?.is_empty());
    let key = JavaLangString::from_rust_string(&jvm, "reused").await?;
    let value = JavaLangString::from_rust_string(&jvm, "value").await?;
    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&map, "put", "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;", (key, value))
        .await?;
    assert_eq!(
        view_strings(&jvm, &map, "keySet", "()Ljava/util/Set;").await?,
        vec![Some("reused".into())]
    );

    let keys: ClassInstanceRef<Object> = jvm.invoke_virtual(&map, "keySet", "()Ljava/util/Set;", ()).await?;
    let iterator: ClassInstanceRef<Object> = jvm.invoke_virtual(&keys, "iterator", "()Ljava/util/Iterator;", ()).await?;
    let _: ClassInstanceRef<Object> = jvm.invoke_virtual(&iterator, "next", "()Ljava/lang/Object;", ()).await?;
    let result: Result<ClassInstanceRef<Object>> = jvm.invoke_virtual(&iterator, "next", "()Ljava/lang/Object;", ()).await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("exhausted iterator must fail");
    };
    assert!(jvm.is_instance(&*exception, "java/util/NoSuchElementException"));

    Ok(())
}

#[tokio::test]
async fn linked_hash_map_copy_and_remove_eldest_entry_preserve_policy_and_order() -> Result<()> {
    let runtime = TestRuntime::new(BTreeMap::new());
    let jvm = create_test_jvm(runtime.clone()).await?;
    jvm.register_class(
        Box::new(ClassDefinitionImpl::from_class_proto(
            LimitedLinkedHashMap::as_proto(),
            Box::new(runtime) as Box<_>,
        )),
        None,
    )
    .await?;

    let limited: ClassInstanceRef<Object> = jvm.new_class("LimitedLinkedHashMap", "(I)V", (2,)).await?.into();
    for key in ["a", "b", "c"] {
        let key = JavaLangString::from_rust_string(&jvm, key).await?;
        let value = key.clone();
        let _: ClassInstanceRef<Object> = jvm
            .invoke_virtual(&limited, "put", "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;", (key, value))
            .await?;
    }
    assert_eq!(
        view_strings(&jvm, &limited, "keySet", "()Ljava/util/Set;").await?,
        vec![Some("b".into()), Some("c".into())]
    );
    assert_eq!(jvm.get_field::<i32>(&limited, "callbacks", "I").await?, 3);

    let key = JavaLangString::from_rust_string(&jvm, "c").await?;
    let value = JavaLangString::from_rust_string(&jvm, "C").await?;
    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&limited, "put", "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;", (key, value))
        .await?;
    assert_eq!(jvm.get_field::<i32>(&limited, "callbacks", "I").await?, 3);

    let copy: ClassInstanceRef<Object> = jvm.new_class("java/util/LinkedHashMap", "(Ljava/util/Map;)V", (limited,)).await?.into();
    assert_eq!(
        view_strings(&jvm, &copy, "keySet", "()Ljava/util/Set;").await?,
        vec![Some("b".into()), Some("c".into())]
    );
    assert_eq!(entry_strings(&jvm, &copy).await?[1], (Some("c".into()), Some("C".into())));

    let key_set: ClassInstanceRef<Object> = jvm.invoke_virtual(&copy, "keySet", "()Ljava/util/Set;", ()).await?;
    let array: ClassInstanceRef<Array<Object>> = jvm.invoke_virtual(&key_set, "toArray", "()[Ljava/lang/Object;", ()).await?;
    let keys = jvm.load_array::<ClassInstanceRef<Object>>(&array, 0, 2).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &keys[0]).await?, "b");
    assert_eq!(JavaLangString::to_rust_string(&jvm, &keys[1]).await?, "c");

    Ok(())
}
