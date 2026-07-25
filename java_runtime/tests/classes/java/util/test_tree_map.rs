use alloc::{
    boxed::Box,
    collections::{BTreeMap, BTreeSet},
    vec,
    vec::Vec,
};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use java_runtime::{
    RuntimeClassProto, RuntimeContext,
    classes::java::{lang::Object, util::TreeMapEntry},
    get_runtime_class_proto,
};
use jvm::{Array, ClassInstanceRef, JavaError, Jvm, Result};
use jvm_rust::ClassDefinitionImpl;

use test_utils::{TestRuntime, create_test_jvm};

struct TreeTestComparator;

impl TreeTestComparator {
    fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "TreeTestComparator",
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["java/util/Comparator"],
            methods: vec![
                JavaMethodProto::new("<init>", "(ZZZ)V", Self::init, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "compare",
                    "(Ljava/lang/Object;Ljava/lang/Object;)I",
                    Self::compare,
                    MethodAccessFlags::PUBLIC,
                ),
            ],
            fields: vec![
                JavaFieldProto::new("reverse", "Z", Default::default()),
                JavaFieldProto::new("allowNull", "Z", Default::default()),
                JavaFieldProto::new("absolute", "Z", Default::default()),
            ],
            access_flags: ClassAccessFlags::PUBLIC,
        }
    }

    async fn init(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        reverse: bool,
        allow_null: bool,
        absolute: bool,
    ) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "reverse", "Z", reverse).await?;
        jvm.put_field(&mut this, "allowNull", "Z", allow_null).await?;
        jvm.put_field(&mut this, "absolute", "Z", absolute).await
    }

    async fn compare(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        left: ClassInstanceRef<Object>,
        right: ClassInstanceRef<Object>,
    ) -> Result<i32> {
        let allow_null: bool = jvm.get_field(&this, "allowNull", "Z").await?;
        let mut comparison = if left.is_null() || right.is_null() {
            if !allow_null {
                return Err(jvm.exception("java/lang/NullPointerException", "null key").await);
            }
            match (left.is_null(), right.is_null()) {
                (true, true) => 0,
                (true, false) => -1,
                (false, true) => 1,
                (false, false) => unreachable!(),
            }
        } else {
            if !jvm.is_instance(left.as_ref(), "java/lang/Integer") || !jvm.is_instance(right.as_ref(), "java/lang/Integer") {
                return Err(jvm.exception("java/lang/ClassCastException", "integer keys required").await);
            }
            let mut left_value: i32 = jvm.invoke_virtual(&left, "intValue", "()I", ()).await?;
            let mut right_value: i32 = jvm.invoke_virtual(&right, "intValue", "()I", ()).await?;
            if jvm.get_field::<bool>(&this, "absolute", "Z").await? {
                left_value = left_value.saturating_abs();
                right_value = right_value.saturating_abs();
            }
            left_value.cmp(&right_value) as i32
        };
        if jvm.get_field::<bool>(&this, "reverse", "Z").await? {
            comparison = -comparison;
        }
        Ok(comparison)
    }
}

struct TreeDirectionalKey;

impl TreeDirectionalKey {
    fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "TreeDirectionalKey",
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["java/lang/Comparable"],
            methods: vec![
                JavaMethodProto::new("<init>", "(IZ)V", Self::init, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("compareTo", "(Ljava/lang/Object;)I", Self::compare_to, MethodAccessFlags::PUBLIC),
            ],
            fields: vec![
                JavaFieldProto::new("value", "I", Default::default()),
                JavaFieldProto::new("fail", "Z", Default::default()),
            ],
            access_flags: ClassAccessFlags::PUBLIC,
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, value: i32, fail: bool) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "value", "I", value).await?;
        jvm.put_field(&mut this, "fail", "Z", fail).await
    }

    async fn compare_to(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Object>) -> Result<i32> {
        if jvm.get_field::<bool>(&this, "fail", "Z").await? {
            return Err(jvm.exception("java/lang/IllegalStateException", "stored key comparison").await);
        }
        if other.is_null() || !jvm.is_instance(other.as_ref(), "TreeDirectionalKey") {
            return Err(jvm.exception("java/lang/ClassCastException", "TreeDirectionalKey required").await);
        }
        Ok(jvm
            .get_field::<i32>(&this, "value", "I")
            .await?
            .cmp(&jvm.get_field::<i32>(&other, "value", "I").await?) as i32)
    }
}

struct TreeEqualsValue;

impl TreeEqualsValue {
    fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "TreeEqualsValue",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(IZ)V", Self::init, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("equals", "(Ljava/lang/Object;)Z", Self::equals, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("hashCode", "()I", Self::hash_code, MethodAccessFlags::PUBLIC),
            ],
            fields: vec![
                JavaFieldProto::new("group", "I", Default::default()),
                JavaFieldProto::new("answer", "Z", Default::default()),
            ],
            access_flags: ClassAccessFlags::PUBLIC,
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, group: i32, answer: bool) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "group", "I", group).await?;
        jvm.put_field(&mut this, "answer", "Z", answer).await
    }

    async fn equals(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Object>) -> Result<bool> {
        if other.is_null() || !jvm.is_instance(other.as_ref(), "TreeEqualsValue") {
            return Ok(false);
        }
        Ok(
            jvm.get_field::<i32>(&this, "group", "I").await? == jvm.get_field::<i32>(&other, "group", "I").await?
                && jvm.get_field::<bool>(&this, "answer", "Z").await?,
        )
    }

    async fn hash_code(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "group", "I").await
    }
}

struct TreeChangingEntry;

impl TreeChangingEntry {
    fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "TreeChangingEntry",
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["java/util/Map$Entry"],
            methods: vec![
                JavaMethodProto::new(
                    "<init>",
                    "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)V",
                    Self::init,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new("getKey", "()Ljava/lang/Object;", Self::get_key, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("getValue", "()Ljava/lang/Object;", Self::get_value, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "setValue",
                    "(Ljava/lang/Object;)Ljava/lang/Object;",
                    Self::set_value,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new("equals", "(Ljava/lang/Object;)Z", Self::equals, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("hashCode", "()I", Self::hash_code, MethodAccessFlags::PUBLIC),
            ],
            fields: vec![
                JavaFieldProto::new("firstKey", "Ljava/lang/Object;", Default::default()),
                JavaFieldProto::new("secondKey", "Ljava/lang/Object;", Default::default()),
                JavaFieldProto::new("value", "Ljava/lang/Object;", Default::default()),
                JavaFieldProto::new("keyCalls", "I", Default::default()),
            ],
            access_flags: ClassAccessFlags::PUBLIC,
        }
    }

    async fn init(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        first_key: ClassInstanceRef<Object>,
        second_key: ClassInstanceRef<Object>,
        value: ClassInstanceRef<Object>,
    ) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "firstKey", "Ljava/lang/Object;", first_key).await?;
        jvm.put_field(&mut this, "secondKey", "Ljava/lang/Object;", second_key).await?;
        jvm.put_field(&mut this, "value", "Ljava/lang/Object;", value).await
    }

    async fn get_key(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let calls: i32 = jvm.get_field(&this, "keyCalls", "I").await?;
        jvm.put_field(&mut this, "keyCalls", "I", calls + 1).await?;
        if calls == 0 {
            jvm.get_field(&this, "firstKey", "Ljava/lang/Object;").await
        } else {
            jvm.get_field(&this, "secondKey", "Ljava/lang/Object;").await
        }
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
        let old = jvm.get_field(&this, "value", "Ljava/lang/Object;").await?;
        jvm.put_field(&mut this, "value", "Ljava/lang/Object;", value).await?;
        Ok(old)
    }

    async fn equals(_: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>, _: ClassInstanceRef<Object>) -> Result<bool> {
        Ok(false)
    }

    async fn hash_code(_: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>) -> Result<i32> {
        Ok(0)
    }
}

struct TreeValueProbeEntry;

impl TreeValueProbeEntry {
    fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "TreeValueProbeEntry",
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["java/util/Map$Entry"],
            methods: vec![
                JavaMethodProto::new(
                    "<init>",
                    "(Ljava/lang/Object;Ljava/lang/Object;Z)V",
                    Self::init,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new("getKey", "()Ljava/lang/Object;", Self::get_key, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("getValue", "()Ljava/lang/Object;", Self::get_value, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "setValue",
                    "(Ljava/lang/Object;)Ljava/lang/Object;",
                    Self::set_value,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new("equals", "(Ljava/lang/Object;)Z", Self::equals, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("hashCode", "()I", Self::hash_code, MethodAccessFlags::PUBLIC),
            ],
            fields: vec![
                JavaFieldProto::new("key", "Ljava/lang/Object;", Default::default()),
                JavaFieldProto::new("value", "Ljava/lang/Object;", Default::default()),
                JavaFieldProto::new("valueCalls", "I", Default::default()),
                JavaFieldProto::new("throwValue", "Z", Default::default()),
            ],
            access_flags: ClassAccessFlags::PUBLIC,
        }
    }

    async fn init(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        key: ClassInstanceRef<Object>,
        value: ClassInstanceRef<Object>,
        throw_value: bool,
    ) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "key", "Ljava/lang/Object;", key).await?;
        jvm.put_field(&mut this, "value", "Ljava/lang/Object;", value).await?;
        jvm.put_field(&mut this, "throwValue", "Z", throw_value).await
    }

    async fn get_key(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        jvm.get_field(&this, "key", "Ljava/lang/Object;").await
    }

    async fn get_value(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let calls: i32 = jvm.get_field(&this, "valueCalls", "I").await?;
        jvm.put_field(&mut this, "valueCalls", "I", calls + 1).await?;
        if jvm.get_field::<bool>(&this, "throwValue", "Z").await? {
            return Err(jvm.exception("java/lang/IllegalStateException", "getValue called").await);
        }
        jvm.get_field(&this, "value", "Ljava/lang/Object;").await
    }

    async fn set_value(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        value: ClassInstanceRef<Object>,
    ) -> Result<ClassInstanceRef<Object>> {
        let old = jvm.get_field(&this, "value", "Ljava/lang/Object;").await?;
        jvm.put_field(&mut this, "value", "Ljava/lang/Object;", value).await?;
        Ok(old)
    }

    async fn equals(_: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>, _: ClassInstanceRef<Object>) -> Result<bool> {
        Ok(false)
    }

    async fn hash_code(_: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>) -> Result<i32> {
        Ok(0)
    }
}

async fn tree_test_jvm() -> Result<Jvm> {
    let runtime = TestRuntime::new(BTreeMap::new());
    let jvm = create_test_jvm(runtime.clone()).await?;
    jvm.register_class(
        Box::new(ClassDefinitionImpl::from_class_proto(
            TreeTestComparator::as_proto(),
            Box::new(runtime.clone()) as Box<_>,
        )),
        None,
    )
    .await?;
    jvm.register_class(
        Box::new(ClassDefinitionImpl::from_class_proto(
            TreeDirectionalKey::as_proto(),
            Box::new(runtime.clone()) as Box<_>,
        )),
        None,
    )
    .await?;
    jvm.register_class(
        Box::new(ClassDefinitionImpl::from_class_proto(
            TreeEqualsValue::as_proto(),
            Box::new(runtime.clone()) as Box<_>,
        )),
        None,
    )
    .await?;
    jvm.register_class(
        Box::new(ClassDefinitionImpl::from_class_proto(
            TreeChangingEntry::as_proto(),
            Box::new(runtime.clone()) as Box<_>,
        )),
        None,
    )
    .await?;
    jvm.register_class(
        Box::new(ClassDefinitionImpl::from_class_proto(
            TreeValueProbeEntry::as_proto(),
            Box::new(runtime) as Box<_>,
        )),
        None,
    )
    .await?;
    Ok(jvm)
}

async fn integer(jvm: &Jvm, value: i32) -> Result<ClassInstanceRef<Object>> {
    Ok(jvm.new_class("java/lang/Integer", "(I)V", (value,)).await?.into())
}

async fn put_integer(jvm: &Jvm, map: &ClassInstanceRef<Object>, key: i32, value: Option<i32>) -> Result<ClassInstanceRef<Object>> {
    let key = integer(jvm, key).await?;
    let value = match value {
        Some(value) => integer(jvm, value).await?,
        None => None.into(),
    };
    jvm.invoke_virtual(map, "put", "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;", (key, value))
        .await
}

async fn ordered_integer_keys(jvm: &Jvm, map: &ClassInstanceRef<Object>) -> Result<Vec<i32>> {
    let keys: ClassInstanceRef<Object> = jvm.invoke_virtual(map, "keySet", "()Ljava/util/Set;", ()).await?;
    let iterator: ClassInstanceRef<Object> = jvm.invoke_virtual(&keys, "iterator", "()Ljava/util/Iterator;", ()).await?;
    let expected_size: i32 = jvm.invoke_virtual(map, "size", "()I", ()).await?;
    let mut result = Vec::with_capacity(expected_size as usize);
    for _ in 0..expected_size {
        if !jvm.invoke_virtual::<_, bool>(&iterator, "hasNext", "()Z", ()).await? {
            break;
        }
        let key: ClassInstanceRef<Object> = jvm.invoke_virtual(&iterator, "next", "()Ljava/lang/Object;", ()).await?;
        result.push(jvm.invoke_virtual(&key, "intValue", "()I", ()).await?);
    }
    assert!(
        !jvm.invoke_virtual::<_, bool>(&iterator, "hasNext", "()Z", ()).await?,
        "TreeMap iterator exceeded map size, indicating a cycle or duplicate traversal"
    );
    Ok(result)
}

async fn assert_red_black_invariants(jvm: &Jvm, map: &ClassInstanceRef<Object>) -> Result<()> {
    let root: ClassInstanceRef<TreeMapEntry> = jvm.get_field(map, "root", "Ljava/util/TreeMap$Entry;").await?;
    let expected_size: i32 = jvm.invoke_virtual(map, "size", "()I", ()).await?;
    if root.is_null() {
        assert_eq!(expected_size, 0);
        return Ok(());
    }
    assert!(jvm.get_field::<bool>(&root, "color", "Z").await?, "root must be black");
    let root_parent: ClassInstanceRef<TreeMapEntry> = jvm.get_field(&root, "parent", "Ljava/util/TreeMap$Entry;").await?;
    assert!(root_parent.is_null());

    let mut seen = BTreeSet::new();
    let mut black_height = None;
    let mut stack = vec![(root, None::<i32>, None::<i32>, 0i32)];
    while let Some((node, lower, upper, black_count)) = stack.pop() {
        assert!(seen.insert(node.identity()), "tree must not contain a cycle");
        let key: ClassInstanceRef<Object> = jvm.get_field(&node, "key", "Ljava/lang/Object;").await?;
        let key: i32 = jvm.invoke_virtual(&key, "intValue", "()I", ()).await?;
        assert!(lower.is_none_or(|lower| key > lower));
        assert!(upper.is_none_or(|upper| key < upper));
        let black = jvm.get_field::<bool>(&node, "color", "Z").await?;
        let next_black_count = black_count + i32::from(black);
        let left: ClassInstanceRef<TreeMapEntry> = jvm.get_field(&node, "left", "Ljava/util/TreeMap$Entry;").await?;
        let right: ClassInstanceRef<TreeMapEntry> = jvm.get_field(&node, "right", "Ljava/util/TreeMap$Entry;").await?;
        if !black {
            if !left.is_null() {
                assert!(jvm.get_field::<bool>(&left, "color", "Z").await?, "red node has red left child");
            }
            if !right.is_null() {
                assert!(jvm.get_field::<bool>(&right, "color", "Z").await?, "red node has red right child");
            }
        }
        for (child, child_lower, child_upper) in [(left, lower, Some(key)), (right, Some(key), upper)] {
            if child.is_null() {
                let leaf_height = next_black_count + 1;
                assert_eq!(*black_height.get_or_insert(leaf_height), leaf_height);
            } else {
                let parent: ClassInstanceRef<TreeMapEntry> = jvm.get_field(&child, "parent", "Ljava/util/TreeMap$Entry;").await?;
                assert!(!parent.is_null() && parent.identity() == node.identity());
                stack.push((child, child_lower, child_upper, next_black_count));
            }
        }
    }
    assert_eq!(seen.len(), expected_size as usize);
    Ok(())
}

#[test]
fn tm_01_ts_01_sorted_interfaces_and_tree_classes_are_registered() {
    for (name, parent, methods) in [
        (
            "java/util/SortedMap",
            "java/util/Map",
            vec![
                ("comparator", "()Ljava/util/Comparator;"),
                ("subMap", "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/SortedMap;"),
                ("headMap", "(Ljava/lang/Object;)Ljava/util/SortedMap;"),
                ("tailMap", "(Ljava/lang/Object;)Ljava/util/SortedMap;"),
                ("firstKey", "()Ljava/lang/Object;"),
                ("lastKey", "()Ljava/lang/Object;"),
            ],
        ),
        (
            "java/util/SortedSet",
            "java/util/Set",
            vec![
                ("comparator", "()Ljava/util/Comparator;"),
                ("subSet", "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/SortedSet;"),
                ("headSet", "(Ljava/lang/Object;)Ljava/util/SortedSet;"),
                ("tailSet", "(Ljava/lang/Object;)Ljava/util/SortedSet;"),
                ("first", "()Ljava/lang/Object;"),
                ("last", "()Ljava/lang/Object;"),
            ],
        ),
    ] {
        let proto = get_runtime_class_proto(name).unwrap_or_else(|| panic!("missing {name}"));
        assert_eq!(proto.parent_class, None);
        assert_eq!(proto.interfaces, vec![parent]);
        assert!(proto.fields.is_empty());
        assert_eq!(
            proto.access_flags,
            ClassAccessFlags::PUBLIC | ClassAccessFlags::INTERFACE | ClassAccessFlags::ABSTRACT
        );
        for (method_name, descriptor) in methods {
            let method = proto
                .methods
                .iter()
                .find(|method| method.name == method_name && method.descriptor == descriptor)
                .unwrap_or_else(|| panic!("missing {name}.{method_name}{descriptor}"));
            assert_eq!(method.access_flags, MethodAccessFlags::PUBLIC | MethodAccessFlags::ABSTRACT);
        }
    }

    for name in [
        "java/util/TreeMap",
        "java/util/TreeMap$Entry",
        "java/util/TreeMap$SubMap",
        "java/util/TreeMap$KeySet",
        "java/util/TreeMap$Values",
        "java/util/TreeMap$EntrySet",
        "java/util/TreeMap$PrivateEntryIterator",
        "java/util/TreeMap$KeyIterator",
        "java/util/TreeMap$ValueIterator",
        "java/util/TreeMap$EntryIterator",
        "java/util/TreeSet",
    ] {
        assert!(get_runtime_class_proto(name).is_some(), "missing {name}");
    }

    let sub_map = get_runtime_class_proto("java/util/TreeMap$SubMap").unwrap();
    assert_eq!(sub_map.access_flags, ClassAccessFlags::empty());
    assert!(sub_map.methods.iter().any(|method| {
        method.name == "containsValue" && method.descriptor == "(Ljava/lang/Object;)Z" && method.access_flags == MethodAccessFlags::PUBLIC
    }));

    let tree_map = get_runtime_class_proto("java/util/TreeMap").unwrap();
    assert_eq!(tree_map.parent_class, Some("java/util/AbstractMap"));
    assert_eq!(
        tree_map.interfaces,
        vec!["java/util/SortedMap", "java/lang/Cloneable", "java/io/Serializable"]
    );
    assert_eq!(tree_map.access_flags, ClassAccessFlags::PUBLIC);
    for (name, descriptor) in [
        ("<init>", "()V"),
        ("<init>", "(Ljava/util/Comparator;)V"),
        ("<init>", "(Ljava/util/Map;)V"),
        ("<init>", "(Ljava/util/SortedMap;)V"),
        ("size", "()I"),
        ("containsKey", "(Ljava/lang/Object;)Z"),
        ("containsValue", "(Ljava/lang/Object;)Z"),
        ("get", "(Ljava/lang/Object;)Ljava/lang/Object;"),
        ("put", "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"),
        ("putAll", "(Ljava/util/Map;)V"),
        ("remove", "(Ljava/lang/Object;)Ljava/lang/Object;"),
        ("clear", "()V"),
        ("comparator", "()Ljava/util/Comparator;"),
        ("firstKey", "()Ljava/lang/Object;"),
        ("lastKey", "()Ljava/lang/Object;"),
        ("subMap", "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/SortedMap;"),
        ("headMap", "(Ljava/lang/Object;)Ljava/util/SortedMap;"),
        ("tailMap", "(Ljava/lang/Object;)Ljava/util/SortedMap;"),
        ("keySet", "()Ljava/util/Set;"),
        ("values", "()Ljava/util/Collection;"),
        ("entrySet", "()Ljava/util/Set;"),
    ] {
        let method = tree_map
            .methods
            .iter()
            .find(|method| method.name == name && method.descriptor == descriptor)
            .unwrap_or_else(|| panic!("missing TreeMap.{name}{descriptor}"));
        assert_eq!(method.access_flags, MethodAccessFlags::PUBLIC);
    }
    for (name, descriptor, access_flags) in [
        (
            "root",
            "Ljava/util/TreeMap$Entry;",
            FieldAccessFlags::PRIVATE | FieldAccessFlags::TRANSIENT,
        ),
        ("size", "I", FieldAccessFlags::PRIVATE | FieldAccessFlags::TRANSIENT),
        ("comparator", "Ljava/util/Comparator;", FieldAccessFlags::PRIVATE),
    ] {
        let field = tree_map
            .fields
            .iter()
            .find(|field| field.name == name && field.descriptor == descriptor)
            .unwrap_or_else(|| panic!("missing TreeMap.{name}:{descriptor}"));
        assert_eq!(field.access_flags, access_flags);
    }

    for (name, descriptor) in [
        ("size", "()I"),
        ("containsKey", "(Ljava/lang/Object;)Z"),
        ("containsValue", "(Ljava/lang/Object;)Z"),
        ("get", "(Ljava/lang/Object;)Ljava/lang/Object;"),
        ("put", "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"),
        ("remove", "(Ljava/lang/Object;)Ljava/lang/Object;"),
        ("clear", "()V"),
        ("comparator", "()Ljava/util/Comparator;"),
        ("firstKey", "()Ljava/lang/Object;"),
        ("lastKey", "()Ljava/lang/Object;"),
        ("subMap", "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/SortedMap;"),
        ("headMap", "(Ljava/lang/Object;)Ljava/util/SortedMap;"),
        ("tailMap", "(Ljava/lang/Object;)Ljava/util/SortedMap;"),
        ("keySet", "()Ljava/util/Set;"),
        ("values", "()Ljava/util/Collection;"),
        ("entrySet", "()Ljava/util/Set;"),
    ] {
        let method = sub_map
            .methods
            .iter()
            .find(|method| method.name == name && method.descriptor == descriptor)
            .unwrap_or_else(|| panic!("missing TreeMap$SubMap.{name}{descriptor}"));
        assert_eq!(method.access_flags, MethodAccessFlags::PUBLIC);
    }

    let entry = get_runtime_class_proto("java/util/TreeMap$Entry").unwrap();
    assert_eq!(entry.interfaces, vec!["java/util/Map$Entry"]);
    assert_eq!(entry.access_flags, ClassAccessFlags::FINAL);
    for (name, descriptor) in [
        ("getKey", "()Ljava/lang/Object;"),
        ("getValue", "()Ljava/lang/Object;"),
        ("setValue", "(Ljava/lang/Object;)Ljava/lang/Object;"),
        ("equals", "(Ljava/lang/Object;)Z"),
        ("hashCode", "()I"),
    ] {
        let method = entry
            .methods
            .iter()
            .find(|method| method.name == name && method.descriptor == descriptor)
            .unwrap_or_else(|| panic!("missing TreeMap$Entry.{name}{descriptor}"));
        assert_eq!(method.access_flags, MethodAccessFlags::PUBLIC);
    }

    let tree_set = get_runtime_class_proto("java/util/TreeSet").unwrap();
    assert_eq!(tree_set.parent_class, Some("java/util/AbstractSet"));
    assert_eq!(
        tree_set.interfaces,
        vec!["java/util/SortedSet", "java/lang/Cloneable", "java/io/Serializable"]
    );
    assert_eq!(tree_set.access_flags, ClassAccessFlags::PUBLIC);
    for (name, descriptor) in [
        ("<init>", "()V"),
        ("<init>", "(Ljava/util/Comparator;)V"),
        ("<init>", "(Ljava/util/Collection;)V"),
        ("<init>", "(Ljava/util/SortedSet;)V"),
        ("size", "()I"),
        ("contains", "(Ljava/lang/Object;)Z"),
        ("add", "(Ljava/lang/Object;)Z"),
        ("remove", "(Ljava/lang/Object;)Z"),
        ("clear", "()V"),
        ("iterator", "()Ljava/util/Iterator;"),
        ("comparator", "()Ljava/util/Comparator;"),
        ("first", "()Ljava/lang/Object;"),
        ("last", "()Ljava/lang/Object;"),
        ("subSet", "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/SortedSet;"),
        ("headSet", "(Ljava/lang/Object;)Ljava/util/SortedSet;"),
        ("tailSet", "(Ljava/lang/Object;)Ljava/util/SortedSet;"),
    ] {
        let method = tree_set
            .methods
            .iter()
            .find(|method| method.name == name && method.descriptor == descriptor)
            .unwrap_or_else(|| panic!("missing TreeSet.{name}{descriptor}"));
        assert_eq!(method.access_flags, MethodAccessFlags::PUBLIC);
    }
    for (name, descriptor, access_flags) in [
        ("m", "Ljava/util/SortedMap;", FieldAccessFlags::PRIVATE | FieldAccessFlags::TRANSIENT),
        (
            "PRESENT",
            "Ljava/lang/Object;",
            FieldAccessFlags::PRIVATE | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
        ),
    ] {
        let field = tree_set
            .fields
            .iter()
            .find(|field| field.name == name && field.descriptor == descriptor)
            .unwrap_or_else(|| panic!("missing TreeSet.{name}:{descriptor}"));
        assert_eq!(field.access_flags, access_flags);
    }

    let private_iterator = get_runtime_class_proto("java/util/TreeMap$PrivateEntryIterator").unwrap();
    assert_eq!(private_iterator.access_flags, ClassAccessFlags::ABSTRACT);
    for (name, descriptor, access_flags) in [
        ("hasNext", "()Z", MethodAccessFlags::PUBLIC),
        ("next", "()Ljava/lang/Object;", MethodAccessFlags::PUBLIC | MethodAccessFlags::ABSTRACT),
        ("remove", "()V", MethodAccessFlags::PUBLIC),
    ] {
        let method = private_iterator
            .methods
            .iter()
            .find(|method| method.name == name && method.descriptor == descriptor)
            .unwrap_or_else(|| panic!("missing TreeMap$PrivateEntryIterator.{name}{descriptor}"));
        assert_eq!(method.access_flags, access_flags);
    }
    for name in [
        "java/util/TreeMap$KeyIterator",
        "java/util/TreeMap$ValueIterator",
        "java/util/TreeMap$EntryIterator",
    ] {
        let proto = get_runtime_class_proto(name).unwrap();
        let next = proto
            .methods
            .iter()
            .find(|method| method.name == "next" && method.descriptor == "()Ljava/lang/Object;")
            .unwrap_or_else(|| panic!("missing {name}.next"));
        assert_eq!(next.access_flags, MethodAccessFlags::PUBLIC);
    }
}

#[tokio::test]
async fn tm_02_tm_03_red_black_invariants_hold_for_deterministic_insert_delete_permutations() -> Result<()> {
    let permutations = [
        (0..31).collect::<Vec<_>>(),
        (0..31).rev().collect::<Vec<_>>(),
        vec![
            15, 7, 23, 3, 11, 19, 27, 1, 5, 9, 13, 17, 21, 25, 29, 0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30,
        ],
        vec![
            12, 3, 27, 8, 19, 1, 25, 14, 6, 30, 10, 21, 4, 17, 28, 0, 15, 9, 24, 2, 18, 7, 29, 13, 5, 23, 11, 26, 16, 20, 22,
        ],
    ];
    for (permutation_index, insertion_order) in permutations.iter().enumerate() {
        let jvm = tree_test_jvm().await?;
        let map: ClassInstanceRef<Object> = jvm.new_class("java/util/TreeMap", "()V", ()).await?.into();
        let mut expected = BTreeSet::new();
        for key in insertion_order {
            assert!(put_integer(&jvm, &map, *key, Some(*key * 10)).await?.is_null());
            expected.insert(*key);
            assert_red_black_invariants(&jvm, &map).await?;
            assert_eq!(ordered_integer_keys(&jvm, &map).await?, expected.iter().copied().collect::<Vec<_>>());
        }

        let mut deletion_order = insertion_order.clone();
        if permutation_index % 2 == 0 {
            deletion_order.reverse();
        } else {
            deletion_order.rotate_left(11);
        }
        for key in deletion_order {
            let removed: ClassInstanceRef<Object> = jvm
                .invoke_virtual(&map, "remove", "(Ljava/lang/Object;)Ljava/lang/Object;", (integer(&jvm, key).await?,))
                .await?;
            assert!(!removed.is_null());
            expected.remove(&key);
            assert_red_black_invariants(&jvm, &map).await?;
            assert_eq!(ordered_integer_keys(&jvm, &map).await?, expected.iter().copied().collect::<Vec<_>>());
        }
        assert_eq!(jvm.invoke_virtual::<_, i32>(&map, "size", "()I", ()).await?, 0);
    }
    Ok(())
}

#[tokio::test]
async fn tm_02_tm_03_red_black_invariants_hold_for_seeded_randomized_mutations() -> Result<()> {
    for seed in 1u64..=12 {
        let jvm = tree_test_jvm().await?;
        let map: ClassInstanceRef<Object> = jvm.new_class("java/util/TreeMap", "()V", ()).await?.into();
        let mut state = seed.wrapping_mul(0x9e37_79b9_7f4a_7c15);
        let mut insertion_order = (0..40).collect::<Vec<_>>();
        for index in (1..insertion_order.len()).rev() {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            insertion_order.swap(index, state as usize % (index + 1));
        }

        let mut expected = BTreeSet::new();
        for key in insertion_order {
            assert!(put_integer(&jvm, &map, key, Some(key.wrapping_mul(17))).await?.is_null());
            expected.insert(key);
            assert_red_black_invariants(&jvm, &map).await?;
            assert_eq!(ordered_integer_keys(&jvm, &map).await?, expected.iter().copied().collect::<Vec<_>>());
        }

        let mut deletion_order = (0..40).collect::<Vec<_>>();
        for index in (1..deletion_order.len()).rev() {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            deletion_order.swap(index, state as usize % (index + 1));
        }
        for key in deletion_order {
            let removed: ClassInstanceRef<Object> = jvm
                .invoke_virtual(&map, "remove", "(Ljava/lang/Object;)Ljava/lang/Object;", (integer(&jvm, key).await?,))
                .await?;
            assert!(!removed.is_null());
            expected.remove(&key);
            assert_red_black_invariants(&jvm, &map).await?;
            assert_eq!(ordered_integer_keys(&jvm, &map).await?, expected.iter().copied().collect::<Vec<_>>());
        }
        assert!(expected.is_empty());
    }
    Ok(())
}

#[tokio::test]
async fn tm_02_tm_03_natural_and_custom_comparator_contracts() -> Result<()> {
    let jvm = tree_test_jvm().await?;
    let map: ClassInstanceRef<Object> = jvm.new_class("java/util/TreeMap", "()V", ()).await?.into();

    for method in ["firstKey", "lastKey"] {
        let result: Result<ClassInstanceRef<Object>> = jvm.invoke_virtual(&map, method, "()Ljava/lang/Object;", ()).await;
        let Err(JavaError::JavaException(exception)) = result else {
            panic!("empty TreeMap.{method} must throw");
        };
        assert!(jvm.is_instance(exception.as_ref(), "java/util/NoSuchElementException"));
    }

    let null_result: Result<bool> = jvm
        .invoke_virtual(&map, "containsKey", "(Ljava/lang/Object;)Z", (ClassInstanceRef::<Object>::from(None),))
        .await;
    let Err(JavaError::JavaException(exception)) = null_result else {
        panic!("natural-order null query must throw");
    };
    assert!(jvm.is_instance(exception.as_ref(), "java/lang/NullPointerException"));

    let plain_object: ClassInstanceRef<Object> = jvm.new_class("java/lang/Object", "()V", ()).await?.into();
    let non_comparable: Result<bool> = jvm
        .invoke_virtual(&map, "containsKey", "(Ljava/lang/Object;)Z", (plain_object.clone(),))
        .await;
    let Err(JavaError::JavaException(exception)) = non_comparable else {
        panic!("natural-order non-comparable query must throw");
    };
    assert!(jvm.is_instance(exception.as_ref(), "java/lang/ClassCastException"));

    let failing_map: ClassInstanceRef<Object> = jvm.new_class("java/util/TreeMap", "()V", ()).await?.into();
    let failing_key: ClassInstanceRef<Object> = jvm.new_class("TreeDirectionalKey", "(IZ)V", (1, true)).await?.into();
    let failing_put: Result<ClassInstanceRef<Object>> = jvm
        .invoke_virtual(
            &failing_map,
            "put",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            (failing_key, integer(&jvm, 1).await?),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = failing_put else {
        panic!("first natural-order put must compare key with itself");
    };
    assert!(jvm.is_instance(exception.as_ref(), "java/lang/IllegalStateException"));
    assert_eq!(jvm.invoke_virtual::<_, i32>(&failing_map, "size", "()I", ()).await?, 0);

    let stored: ClassInstanceRef<Object> = jvm.new_class("TreeDirectionalKey", "(IZ)V", (7, false)).await?.into();
    let stored_for_mutation = stored.clone();
    let value = integer(&jvm, 70).await?;
    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &map,
            "put",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            (stored, value.clone()),
        )
        .await?;
    let mut stored_for_mutation = stored_for_mutation;
    jvm.put_field(&mut stored_for_mutation, "fail", "Z", true).await?;
    let query: ClassInstanceRef<Object> = jvm.new_class("TreeDirectionalKey", "(IZ)V", (7, false)).await?.into();
    let found: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&map, "get", "(Ljava/lang/Object;)Ljava/lang/Object;", (query,))
        .await?;
    assert_eq!(found.identity(), value.identity(), "lookup must call query.compareTo(stored)");

    let reverse: ClassInstanceRef<Object> = jvm.new_class("TreeTestComparator", "(ZZZ)V", (true, false, false)).await?.into();
    let reverse_map: ClassInstanceRef<Object> = jvm
        .new_class("java/util/TreeMap", "(Ljava/util/Comparator;)V", (reverse.clone(),))
        .await?
        .into();
    assert!(
        !jvm.invoke_virtual::<_, bool>(&reverse_map, "containsKey", "(Ljava/lang/Object;)Z", (plain_object.clone(),))
            .await?
    );
    let invalid_first_put: Result<ClassInstanceRef<Object>> = jvm
        .invoke_virtual(
            &reverse_map,
            "put",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            (plain_object, integer(&jvm, 1).await?),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = invalid_first_put else {
        panic!("first put must validate comparator compatibility");
    };
    assert!(jvm.is_instance(exception.as_ref(), "java/lang/ClassCastException"));
    for key in [1, 3, 2] {
        put_integer(&jvm, &reverse_map, key, Some(key)).await?;
    }
    assert_eq!(ordered_integer_keys(&jvm, &reverse_map).await?, vec![3, 2, 1]);
    let comparator: ClassInstanceRef<Object> = jvm.invoke_virtual(&reverse_map, "comparator", "()Ljava/util/Comparator;", ()).await?;
    assert_eq!(comparator.identity(), reverse.identity());

    let nulls_first: ClassInstanceRef<Object> = jvm.new_class("TreeTestComparator", "(ZZZ)V", (false, true, false)).await?.into();
    let null_map: ClassInstanceRef<Object> = jvm
        .new_class("java/util/TreeMap", "(Ljava/util/Comparator;)V", (nulls_first,))
        .await?
        .into();
    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &null_map,
            "put",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            (ClassInstanceRef::<Object>::from(None), integer(&jvm, 9).await?),
        )
        .await?;
    put_integer(&jvm, &null_map, 2, Some(2)).await?;
    assert!(
        jvm.invoke_virtual::<_, bool>(
            &null_map,
            "containsKey",
            "(Ljava/lang/Object;)Z",
            (ClassInstanceRef::<Object>::from(None),)
        )
        .await?
    );
    let first: ClassInstanceRef<Object> = jvm.invoke_virtual(&null_map, "firstKey", "()Ljava/lang/Object;", ()).await?;
    assert!(first.is_null());

    let absolute: ClassInstanceRef<Object> = jvm.new_class("TreeTestComparator", "(ZZZ)V", (false, false, true)).await?.into();
    let equivalent_map: ClassInstanceRef<Object> = jvm.new_class("java/util/TreeMap", "(Ljava/util/Comparator;)V", (absolute,)).await?.into();
    assert!(put_integer(&jvm, &equivalent_map, -4, Some(1)).await?.is_null());
    let replaced = put_integer(&jvm, &equivalent_map, 4, Some(2)).await?;
    assert_eq!(jvm.invoke_virtual::<_, i32>(&replaced, "intValue", "()I", ()).await?, 1);
    assert_eq!(jvm.invoke_virtual::<_, i32>(&equivalent_map, "size", "()I", ()).await?, 1);
    let key: ClassInstanceRef<Object> = jvm.invoke_virtual(&equivalent_map, "firstKey", "()Ljava/lang/Object;", ()).await?;
    assert_eq!(jvm.invoke_virtual::<_, i32>(&key, "intValue", "()I", ()).await?, -4);

    Ok(())
}

#[tokio::test]
async fn tm_02_tm_03_map_constructors_put_all_values_and_clear() -> Result<()> {
    let jvm = tree_test_jvm().await?;
    let source: ClassInstanceRef<Object> = jvm.new_class("java/util/HashMap", "()V", ()).await?.into();
    for (key, value) in [(3, 30), (1, 10), (2, 20)] {
        let _: ClassInstanceRef<Object> = jvm
            .invoke_virtual(
                &source,
                "put",
                "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
                (integer(&jvm, key).await?, integer(&jvm, value).await?),
            )
            .await?;
    }
    let map: ClassInstanceRef<Object> = jvm.new_class("java/util/TreeMap", "(Ljava/util/Map;)V", (source.clone(),)).await?.into();
    assert_eq!(ordered_integer_keys(&jvm, &map).await?, vec![1, 2, 3]);
    assert!(
        jvm.invoke_virtual::<_, ClassInstanceRef<Object>>(&map, "comparator", "()Ljava/util/Comparator;", ())
            .await?
            .is_null()
    );
    assert!(
        jvm.invoke_virtual::<_, bool>(&map, "containsValue", "(Ljava/lang/Object;)Z", (integer(&jvm, 20).await?,))
            .await?
    );
    assert!(
        !jvm.invoke_virtual::<_, bool>(&map, "containsValue", "(Ljava/lang/Object;)Z", (integer(&jvm, 99).await?,))
            .await?
    );
    put_integer(&jvm, &map, 4, None).await?;
    assert!(
        jvm.invoke_virtual::<_, bool>(&map, "containsValue", "(Ljava/lang/Object;)Z", (ClassInstanceRef::<Object>::from(None),))
            .await?
    );

    let reverse: ClassInstanceRef<Object> = jvm.new_class("TreeTestComparator", "(ZZZ)V", (true, false, false)).await?.into();
    let sorted_source: ClassInstanceRef<Object> = jvm
        .new_class("java/util/TreeMap", "(Ljava/util/Comparator;)V", (reverse.clone(),))
        .await?
        .into();
    for key in [2, 1, 3] {
        put_integer(&jvm, &sorted_source, key, Some(key)).await?;
    }
    let sorted_copy: ClassInstanceRef<Object> = jvm
        .new_class("java/util/TreeMap", "(Ljava/util/SortedMap;)V", (sorted_source.clone(),))
        .await?
        .into();
    let copied_comparator: ClassInstanceRef<Object> = jvm.invoke_virtual(&sorted_copy, "comparator", "()Ljava/util/Comparator;", ()).await?;
    assert_eq!(copied_comparator.identity(), reverse.identity());
    assert_eq!(ordered_integer_keys(&jvm, &sorted_copy).await?, vec![3, 2, 1]);

    let natural_copy: ClassInstanceRef<Object> = jvm.new_class("java/util/TreeMap", "(Ljava/util/Map;)V", (sorted_source,)).await?.into();
    assert!(
        jvm.invoke_virtual::<_, ClassInstanceRef<Object>>(&natural_copy, "comparator", "()Ljava/util/Comparator;", ())
            .await?
            .is_null()
    );
    assert_eq!(ordered_integer_keys(&jvm, &natural_copy).await?, vec![1, 2, 3]);

    let destination: ClassInstanceRef<Object> = jvm.new_class("java/util/TreeMap", "()V", ()).await?.into();
    let _: () = jvm.invoke_virtual(&destination, "putAll", "(Ljava/util/Map;)V", (source,)).await?;
    assert_eq!(ordered_integer_keys(&jvm, &destination).await?, vec![1, 2, 3]);
    let _: () = jvm
        .invoke_virtual(&destination, "putAll", "(Ljava/util/Map;)V", (destination.clone(),))
        .await?;
    assert_eq!(ordered_integer_keys(&jvm, &destination).await?, vec![1, 2, 3]);
    let _: () = jvm.invoke_virtual(&destination, "clear", "()V", ()).await?;
    assert_eq!(jvm.invoke_virtual::<_, i32>(&destination, "size", "()I", ()).await?, 0);
    assert_red_black_invariants(&jvm, &destination).await?;

    for descriptor in ["(Ljava/util/Map;)V", "(Ljava/util/SortedMap;)V"] {
        let result: Result<ClassInstanceRef<Object>> = jvm
            .new_class("java/util/TreeMap", descriptor, (ClassInstanceRef::<Object>::from(None),))
            .await
            .map(Into::into);
        let Err(JavaError::JavaException(exception)) = result else {
            panic!("TreeMap{descriptor} null source must throw");
        };
        assert!(jvm.is_instance(exception.as_ref(), "java/lang/NullPointerException"));
    }
    Ok(())
}

#[tokio::test]
async fn tm_04_sm_01_submaps_are_live_bounded_and_validate_nested_ranges() -> Result<()> {
    let jvm = tree_test_jvm().await?;
    let map: ClassInstanceRef<Object> = jvm.new_class("java/util/TreeMap", "()V", ()).await?.into();
    for key in 0..10 {
        put_integer(&jvm, &map, key, Some(key * 10)).await?;
    }
    let sub_map: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &map,
            "subMap",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/SortedMap;",
            (integer(&jvm, 2).await?, integer(&jvm, 8).await?),
        )
        .await?;
    assert_eq!(ordered_integer_keys(&jvm, &sub_map).await?, vec![2, 3, 4, 5, 6, 7]);
    assert_eq!(jvm.invoke_virtual::<_, i32>(&sub_map, "size", "()I", ()).await?, 6);
    let first: ClassInstanceRef<Object> = jvm.invoke_virtual(&sub_map, "firstKey", "()Ljava/lang/Object;", ()).await?;
    let last: ClassInstanceRef<Object> = jvm.invoke_virtual(&sub_map, "lastKey", "()Ljava/lang/Object;", ()).await?;
    assert_eq!(jvm.invoke_virtual::<_, i32>(&first, "intValue", "()I", ()).await?, 2);
    assert_eq!(jvm.invoke_virtual::<_, i32>(&last, "intValue", "()I", ()).await?, 7);
    assert!(
        jvm.invoke_virtual::<_, bool>(&sub_map, "containsValue", "(Ljava/lang/Object;)Z", (integer(&jvm, 50).await?,))
            .await?
    );
    assert!(
        !jvm.invoke_virtual::<_, bool>(&sub_map, "containsValue", "(Ljava/lang/Object;)Z", (integer(&jvm, 90).await?,))
            .await?
    );

    for key in [1, 8] {
        let key = integer(&jvm, key).await?;
        assert!(
            !jvm.invoke_virtual::<_, bool>(&sub_map, "containsKey", "(Ljava/lang/Object;)Z", (key.clone(),))
                .await?
        );
        assert!(
            jvm.invoke_virtual::<_, ClassInstanceRef<Object>>(&sub_map, "get", "(Ljava/lang/Object;)Ljava/lang/Object;", (key.clone(),))
                .await?
                .is_null()
        );
        assert!(
            jvm.invoke_virtual::<_, ClassInstanceRef<Object>>(&sub_map, "remove", "(Ljava/lang/Object;)Ljava/lang/Object;", (key.clone(),))
                .await?
                .is_null()
        );
        let result: Result<ClassInstanceRef<Object>> = jvm
            .invoke_virtual(
                &sub_map,
                "put",
                "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
                (key, integer(&jvm, 100).await?),
            )
            .await;
        let Err(JavaError::JavaException(exception)) = result else {
            panic!("out-of-range SubMap.put must throw");
        };
        assert!(jvm.is_instance(exception.as_ref(), "java/lang/IllegalArgumentException"));
    }

    let old = put_integer(&jvm, &sub_map, 3, Some(333)).await?;
    assert_eq!(jvm.invoke_virtual::<_, i32>(&old, "intValue", "()I", ()).await?, 30);
    let from_root: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&map, "get", "(Ljava/lang/Object;)Ljava/lang/Object;", (integer(&jvm, 3).await?,))
        .await?;
    assert_eq!(jvm.invoke_virtual::<_, i32>(&from_root, "intValue", "()I", ()).await?, 333);
    put_integer(&jvm, &map, 6, Some(666)).await?;
    let from_view: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&sub_map, "get", "(Ljava/lang/Object;)Ljava/lang/Object;", (integer(&jvm, 6).await?,))
        .await?;
    assert_eq!(jvm.invoke_virtual::<_, i32>(&from_view, "intValue", "()I", ()).await?, 666);

    let nested: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &sub_map,
            "subMap",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/SortedMap;",
            (integer(&jvm, 3).await?, integer(&jvm, 7).await?),
        )
        .await?;
    assert_eq!(ordered_integer_keys(&jvm, &nested).await?, vec![3, 4, 5, 6]);
    let same_upper: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &sub_map,
            "headMap",
            "(Ljava/lang/Object;)Ljava/util/SortedMap;",
            (integer(&jvm, 8).await?,),
        )
        .await?;
    assert_eq!(ordered_integer_keys(&jvm, &same_upper).await?, vec![2, 3, 4, 5, 6, 7]);
    let empty_at_lower: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &sub_map,
            "headMap",
            "(Ljava/lang/Object;)Ljava/util/SortedMap;",
            (integer(&jvm, 2).await?,),
        )
        .await?;
    assert_eq!(jvm.invoke_virtual::<_, i32>(&empty_at_lower, "size", "()I", ()).await?, 0);
    for method in ["firstKey", "lastKey"] {
        let result: Result<ClassInstanceRef<Object>> = jvm.invoke_virtual(&empty_at_lower, method, "()Ljava/lang/Object;", ()).await;
        let Err(JavaError::JavaException(exception)) = result else {
            panic!("empty nested range {method} must throw");
        };
        assert!(jvm.is_instance(exception.as_ref(), "java/util/NoSuchElementException"));
    }

    for (method, descriptor, endpoint) in [
        ("headMap", "(Ljava/lang/Object;)Ljava/util/SortedMap;", 1),
        ("tailMap", "(Ljava/lang/Object;)Ljava/util/SortedMap;", 8),
        ("tailMap", "(Ljava/lang/Object;)Ljava/util/SortedMap;", 9),
    ] {
        let result: Result<ClassInstanceRef<Object>> = jvm.invoke_virtual(&sub_map, method, descriptor, (integer(&jvm, endpoint).await?,)).await;
        let Err(JavaError::JavaException(exception)) = result else {
            panic!("nested out-of-range endpoint must throw");
        };
        assert!(jvm.is_instance(exception.as_ref(), "java/lang/IllegalArgumentException"));
    }
    let reversed: Result<ClassInstanceRef<Object>> = jvm
        .invoke_virtual(
            &sub_map,
            "subMap",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/SortedMap;",
            (integer(&jvm, 7).await?, integer(&jvm, 3).await?),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = reversed else {
        panic!("reversed nested range must throw");
    };
    assert!(jvm.is_instance(exception.as_ref(), "java/lang/IllegalArgumentException"));

    let incompatible_upper: ClassInstanceRef<Object> = jvm.new_class("java/lang/Object", "()V", ()).await?.into();
    let out_of_range_first: Result<ClassInstanceRef<Object>> = jvm
        .invoke_virtual(
            &sub_map,
            "subMap",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/SortedMap;",
            (integer(&jvm, 1).await?, incompatible_upper),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = out_of_range_first else {
        panic!("nested subMap must reject the from endpoint before comparing endpoints");
    };
    assert!(
        jvm.is_instance(exception.as_ref(), "java/lang/IllegalArgumentException"),
        "from endpoint validation must precede a ClassCastException from the to endpoint"
    );

    let null_bound: Result<ClassInstanceRef<Object>> = jvm
        .invoke_virtual(
            &map,
            "subMap",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/SortedMap;",
            (ClassInstanceRef::<Object>::from(None), integer(&jvm, 3).await?),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = null_bound else {
        panic!("natural-order null bound must throw");
    };
    assert!(jvm.is_instance(exception.as_ref(), "java/lang/NullPointerException"));

    let _: () = jvm.invoke_virtual(&nested, "clear", "()V", ()).await?;
    assert_eq!(ordered_integer_keys(&jvm, &map).await?, vec![0, 1, 2, 7, 8, 9]);
    assert_red_black_invariants(&jvm, &map).await?;
    Ok(())
}

#[tokio::test]
async fn tm_03_sm_01_views_entries_and_iterators_are_live_and_mutable() -> Result<()> {
    let jvm = tree_test_jvm().await?;
    let map: ClassInstanceRef<Object> = jvm.new_class("java/util/TreeMap", "()V", ()).await?.into();
    for key in [4, 2, 6, 1, 3, 5, 7] {
        put_integer(&jvm, &map, key, Some(key * 10)).await?;
    }

    let keys: ClassInstanceRef<Object> = jvm.invoke_virtual(&map, "keySet", "()Ljava/util/Set;", ()).await?;
    let values: ClassInstanceRef<Object> = jvm.invoke_virtual(&map, "values", "()Ljava/util/Collection;", ()).await?;
    let entries: ClassInstanceRef<Object> = jvm.invoke_virtual(&map, "entrySet", "()Ljava/util/Set;", ()).await?;
    assert_eq!(jvm.invoke_virtual::<_, i32>(&keys, "size", "()I", ()).await?, 7);
    assert!(
        jvm.invoke_virtual::<_, bool>(&values, "contains", "(Ljava/lang/Object;)Z", (integer(&jvm, 50).await?,))
            .await?
    );

    let entry_iterator: ClassInstanceRef<Object> = jvm.invoke_virtual(&entries, "iterator", "()Ljava/util/Iterator;", ()).await?;
    let first_entry: ClassInstanceRef<Object> = jvm.invoke_virtual(&entry_iterator, "next", "()Ljava/lang/Object;", ()).await?;
    let first_key: ClassInstanceRef<Object> = jvm.invoke_virtual(&first_entry, "getKey", "()Ljava/lang/Object;", ()).await?;
    assert_eq!(jvm.invoke_virtual::<_, i32>(&first_key, "intValue", "()I", ()).await?, 1);
    let replacement = integer(&jvm, 111).await?;
    let old: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&first_entry, "setValue", "(Ljava/lang/Object;)Ljava/lang/Object;", (replacement.clone(),))
        .await?;
    assert_eq!(jvm.invoke_virtual::<_, i32>(&old, "intValue", "()I", ()).await?, 10);
    let stored: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&map, "get", "(Ljava/lang/Object;)Ljava/lang/Object;", (integer(&jvm, 1).await?,))
        .await?;
    assert_eq!(stored.identity(), replacement.identity());

    assert!(
        jvm.invoke_virtual::<_, bool>(&keys, "remove", "(Ljava/lang/Object;)Z", (integer(&jvm, 2).await?,))
            .await?
    );
    assert!(
        !jvm.invoke_virtual::<_, bool>(&map, "containsKey", "(Ljava/lang/Object;)Z", (integer(&jvm, 2).await?,))
            .await?
    );
    assert!(
        jvm.invoke_virtual::<_, bool>(&values, "remove", "(Ljava/lang/Object;)Z", (integer(&jvm, 30).await?,))
            .await?
    );
    assert!(
        !jvm.invoke_virtual::<_, bool>(&map, "containsKey", "(Ljava/lang/Object;)Z", (integer(&jvm, 3).await?,))
            .await?
    );

    let candidate_map: ClassInstanceRef<Object> = jvm.new_class("java/util/TreeMap", "()V", ()).await?.into();
    put_integer(&jvm, &candidate_map, 5, Some(50)).await?;
    let candidate_entries: ClassInstanceRef<Object> = jvm.invoke_virtual(&candidate_map, "entrySet", "()Ljava/util/Set;", ()).await?;
    let candidate_iterator: ClassInstanceRef<Object> = jvm.invoke_virtual(&candidate_entries, "iterator", "()Ljava/util/Iterator;", ()).await?;
    let candidate: ClassInstanceRef<Object> = jvm.invoke_virtual(&candidate_iterator, "next", "()Ljava/lang/Object;", ()).await?;
    assert!(
        jvm.invoke_virtual::<_, bool>(&entries, "contains", "(Ljava/lang/Object;)Z", (candidate.clone(),))
            .await?
    );
    assert!(
        jvm.invoke_virtual::<_, bool>(&entries, "remove", "(Ljava/lang/Object;)Z", (candidate,))
            .await?
    );
    assert!(
        !jvm.invoke_virtual::<_, bool>(&map, "containsKey", "(Ljava/lang/Object;)Z", (integer(&jvm, 5).await?,))
            .await?
    );

    let live_map: ClassInstanceRef<Object> = jvm.new_class("java/util/TreeMap", "()V", ()).await?.into();
    for key in [1, 3] {
        put_integer(&jvm, &live_map, key, Some(key)).await?;
    }
    let live_keys: ClassInstanceRef<Object> = jvm.invoke_virtual(&live_map, "keySet", "()Ljava/util/Set;", ()).await?;
    let live_iterator: ClassInstanceRef<Object> = jvm.invoke_virtual(&live_keys, "iterator", "()Ljava/util/Iterator;", ()).await?;
    put_integer(&jvm, &live_map, 2, Some(2)).await?;
    let mut seen = Vec::new();
    for _ in 0..3 {
        assert!(jvm.invoke_virtual::<_, bool>(&live_iterator, "hasNext", "()Z", ()).await?);
        let key: ClassInstanceRef<Object> = jvm.invoke_virtual(&live_iterator, "next", "()Ljava/lang/Object;", ()).await?;
        seen.push(jvm.invoke_virtual::<_, i32>(&key, "intValue", "()I", ()).await?);
    }
    assert!(
        !jvm.invoke_virtual::<_, bool>(&live_iterator, "hasNext", "()Z", ()).await?,
        "live TreeMap iterator must terminate after each key exactly once"
    );
    assert_eq!(seen, vec![1, 2, 3], "iterator must traverse the live tree rather than a snapshot");

    let remove_map: ClassInstanceRef<Object> = jvm.new_class("java/util/TreeMap", "()V", ()).await?.into();
    for key in [4, 2, 6, 1, 3, 5, 7] {
        put_integer(&jvm, &remove_map, key, Some(key)).await?;
    }
    let remove_keys: ClassInstanceRef<Object> = jvm.invoke_virtual(&remove_map, "keySet", "()Ljava/util/Set;", ()).await?;
    let iterator: ClassInstanceRef<Object> = jvm.invoke_virtual(&remove_keys, "iterator", "()Ljava/util/Iterator;", ()).await?;
    let before_next: Result<()> = jvm.invoke_virtual(&iterator, "remove", "()V", ()).await;
    let Err(JavaError::JavaException(exception)) = before_next else {
        panic!("iterator.remove before next must throw");
    };
    assert!(jvm.is_instance(exception.as_ref(), "java/lang/IllegalStateException"));
    let mut visited = Vec::new();
    while jvm.invoke_virtual::<_, bool>(&iterator, "hasNext", "()Z", ()).await? {
        let key: ClassInstanceRef<Object> = jvm.invoke_virtual(&iterator, "next", "()Ljava/lang/Object;", ()).await?;
        let key = jvm.invoke_virtual::<_, i32>(&key, "intValue", "()I", ()).await?;
        visited.push(key);
        if key == 4 {
            let _: () = jvm.invoke_virtual(&iterator, "remove", "()V", ()).await?;
            let second_remove: Result<()> = jvm.invoke_virtual(&iterator, "remove", "()V", ()).await;
            let Err(JavaError::JavaException(exception)) = second_remove else {
                panic!("iterator.remove twice must throw");
            };
            assert!(jvm.is_instance(exception.as_ref(), "java/lang/IllegalStateException"));
        }
    }
    assert_eq!(visited, vec![1, 2, 3, 4, 5, 6, 7]);
    assert_eq!(ordered_integer_keys(&jvm, &remove_map).await?, vec![1, 2, 3, 5, 6, 7]);
    assert_red_black_invariants(&jvm, &remove_map).await?;
    let exhausted: Result<ClassInstanceRef<Object>> = jvm.invoke_virtual(&iterator, "next", "()Ljava/lang/Object;", ()).await;
    let Err(JavaError::JavaException(exception)) = exhausted else {
        panic!("exhausted iterator.next must throw");
    };
    assert!(jvm.is_instance(exception.as_ref(), "java/util/NoSuchElementException"));

    let range: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &remove_map,
            "subMap",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/SortedMap;",
            (integer(&jvm, 2).await?, integer(&jvm, 7).await?),
        )
        .await?;
    let range_values: ClassInstanceRef<Object> = jvm.invoke_virtual(&range, "values", "()Ljava/util/Collection;", ()).await?;
    let _: () = jvm.invoke_virtual(&range_values, "clear", "()V", ()).await?;
    assert_eq!(ordered_integer_keys(&jvm, &remove_map).await?, vec![1, 7]);
    assert_red_black_invariants(&jvm, &remove_map).await?;
    Ok(())
}

#[tokio::test]
async fn sm_01_bounded_entry_views_delegate_comparator_equality_and_set_value() -> Result<()> {
    let jvm = tree_test_jvm().await?;
    let reverse: ClassInstanceRef<Object> = jvm.new_class("TreeTestComparator", "(ZZZ)V", (true, false, false)).await?.into();
    let map: ClassInstanceRef<Object> = jvm
        .new_class("java/util/TreeMap", "(Ljava/util/Comparator;)V", (reverse.clone(),))
        .await?
        .into();
    for key in 1..=7 {
        put_integer(&jvm, &map, key, Some(key * 10)).await?;
    }
    let range: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &map,
            "subMap",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/SortedMap;",
            (integer(&jvm, 6).await?, integer(&jvm, 2).await?),
        )
        .await?;
    let comparator: ClassInstanceRef<Object> = jvm.invoke_virtual(&range, "comparator", "()Ljava/util/Comparator;", ()).await?;
    assert_eq!(comparator.identity(), reverse.identity());
    assert_eq!(ordered_integer_keys(&jvm, &range).await?, vec![6, 5, 4, 3]);

    let entries: ClassInstanceRef<Object> = jvm.invoke_virtual(&range, "entrySet", "()Ljava/util/Set;", ()).await?;
    let iterator: ClassInstanceRef<Object> = jvm.invoke_virtual(&entries, "iterator", "()Ljava/util/Iterator;", ()).await?;
    let entry: ClassInstanceRef<Object> = jvm.invoke_virtual(&iterator, "next", "()Ljava/lang/Object;", ()).await?;
    let key: ClassInstanceRef<Object> = jvm.invoke_virtual(&entry, "getKey", "()Ljava/lang/Object;", ()).await?;
    assert_eq!(jvm.invoke_virtual::<_, i32>(&key, "intValue", "()I", ()).await?, 6);
    let new_value = integer(&jvm, 600).await?;
    let old: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&entry, "setValue", "(Ljava/lang/Object;)Ljava/lang/Object;", (new_value.clone(),))
        .await?;
    assert_eq!(jvm.invoke_virtual::<_, i32>(&old, "intValue", "()I", ()).await?, 60);
    let root_value: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&map, "get", "(Ljava/lang/Object;)Ljava/lang/Object;", (integer(&jvm, 6).await?,))
        .await?;
    assert_eq!(root_value.identity(), new_value.identity());

    let same_entry_map: ClassInstanceRef<Object> = jvm.new_class("java/util/TreeMap", "(Ljava/util/Comparator;)V", (reverse,)).await?.into();
    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &same_entry_map,
            "put",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            (integer(&jvm, 6).await?, new_value),
        )
        .await?;
    let same_entries: ClassInstanceRef<Object> = jvm.invoke_virtual(&same_entry_map, "entrySet", "()Ljava/util/Set;", ()).await?;
    let same_iterator: ClassInstanceRef<Object> = jvm.invoke_virtual(&same_entries, "iterator", "()Ljava/util/Iterator;", ()).await?;
    let same_entry: ClassInstanceRef<Object> = jvm.invoke_virtual(&same_iterator, "next", "()Ljava/lang/Object;", ()).await?;
    assert!(
        jvm.invoke_virtual::<_, bool>(&entry, "equals", "(Ljava/lang/Object;)Z", (same_entry.clone(),))
            .await?
    );
    assert!(
        jvm.invoke_virtual::<_, bool>(&same_entry, "equals", "(Ljava/lang/Object;)Z", (entry.clone(),))
            .await?
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&entry, "hashCode", "()I", ()).await?,
        jvm.invoke_virtual::<_, i32>(&same_entry, "hashCode", "()I", ()).await?
    );
    assert!(
        jvm.invoke_virtual::<_, bool>(&entries, "contains", "(Ljava/lang/Object;)Z", (same_entry.clone(),))
            .await?
    );
    assert!(
        jvm.invoke_virtual::<_, bool>(&entries, "remove", "(Ljava/lang/Object;)Z", (same_entry,))
            .await?
    );
    assert!(
        !jvm.invoke_virtual::<_, bool>(&map, "containsKey", "(Ljava/lang/Object;)Z", (integer(&jvm, 6).await?,))
            .await?
    );
    assert_eq!(ordered_integer_keys(&jvm, &range).await?, vec![5, 4, 3]);
    Ok(())
}

#[tokio::test]
async fn tm_03_views_use_jdk_value_equals_directions() -> Result<()> {
    let jvm = tree_test_jvm().await?;
    let map: ClassInstanceRef<Object> = jvm.new_class("java/util/TreeMap", "()V", ()).await?.into();
    let stored_false: ClassInstanceRef<Object> = jvm.new_class("TreeEqualsValue", "(IZ)V", (1, false)).await?.into();
    let query_true: ClassInstanceRef<Object> = jvm.new_class("TreeEqualsValue", "(IZ)V", (1, true)).await?.into();
    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &map,
            "put",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            (integer(&jvm, 1).await?, stored_false),
        )
        .await?;
    assert!(
        jvm.invoke_virtual::<_, bool>(&map, "containsValue", "(Ljava/lang/Object;)Z", (query_true.clone(),))
            .await?,
        "TreeMap.containsValue uses query.equals(stored)"
    );

    let values: ClassInstanceRef<Object> = jvm.invoke_virtual(&map, "values", "()Ljava/util/Collection;", ()).await?;
    assert!(
        !jvm.invoke_virtual::<_, bool>(&values, "remove", "(Ljava/lang/Object;)Z", (query_true.clone(),))
            .await?,
        "TreeMap.Values.remove follows JDK stored.equals(query)"
    );

    let candidate_map: ClassInstanceRef<Object> = jvm.new_class("java/util/TreeMap", "()V", ()).await?.into();
    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &candidate_map,
            "put",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            (integer(&jvm, 1).await?, query_true),
        )
        .await?;
    let candidate_entries: ClassInstanceRef<Object> = jvm.invoke_virtual(&candidate_map, "entrySet", "()Ljava/util/Set;", ()).await?;
    let candidate_iterator: ClassInstanceRef<Object> = jvm.invoke_virtual(&candidate_entries, "iterator", "()Ljava/util/Iterator;", ()).await?;
    let candidate: ClassInstanceRef<Object> = jvm.invoke_virtual(&candidate_iterator, "next", "()Ljava/lang/Object;", ()).await?;
    let entries: ClassInstanceRef<Object> = jvm.invoke_virtual(&map, "entrySet", "()Ljava/util/Set;", ()).await?;
    assert!(
        !jvm.invoke_virtual::<_, bool>(&entries, "contains", "(Ljava/lang/Object;)Z", (candidate,))
            .await?,
        "TreeMap.EntrySet.contains follows JDK storedValue.equals(candidateValue)"
    );

    let stored_true: ClassInstanceRef<Object> = jvm.new_class("TreeEqualsValue", "(IZ)V", (2, true)).await?.into();
    let query_false: ClassInstanceRef<Object> = jvm.new_class("TreeEqualsValue", "(IZ)V", (2, false)).await?.into();
    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &map,
            "put",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            (integer(&jvm, 2).await?, stored_true),
        )
        .await?;
    assert!(
        jvm.invoke_virtual::<_, bool>(&values, "remove", "(Ljava/lang/Object;)Z", (query_false,))
            .await?
    );
    assert!(
        !jvm.invoke_virtual::<_, bool>(&map, "containsKey", "(Ljava/lang/Object;)Z", (integer(&jvm, 2).await?,))
            .await?
    );

    let stored_true: ClassInstanceRef<Object> = jvm.new_class("TreeEqualsValue", "(IZ)V", (3, true)).await?.into();
    let candidate_false: ClassInstanceRef<Object> = jvm.new_class("TreeEqualsValue", "(IZ)V", (3, false)).await?.into();
    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &map,
            "put",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            (integer(&jvm, 3).await?, stored_true),
        )
        .await?;
    let candidate_map: ClassInstanceRef<Object> = jvm.new_class("java/util/TreeMap", "()V", ()).await?.into();
    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &candidate_map,
            "put",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            (integer(&jvm, 3).await?, candidate_false),
        )
        .await?;
    let candidate_entries: ClassInstanceRef<Object> = jvm.invoke_virtual(&candidate_map, "entrySet", "()Ljava/util/Set;", ()).await?;
    let candidate_iterator: ClassInstanceRef<Object> = jvm.invoke_virtual(&candidate_entries, "iterator", "()Ljava/util/Iterator;", ()).await?;
    let candidate: ClassInstanceRef<Object> = jvm.invoke_virtual(&candidate_iterator, "next", "()Ljava/lang/Object;", ()).await?;
    assert!(
        jvm.invoke_virtual::<_, bool>(&entries, "contains", "(Ljava/lang/Object;)Z", (candidate.clone(),))
            .await?
    );
    assert!(
        jvm.invoke_virtual::<_, bool>(&entries, "remove", "(Ljava/lang/Object;)Z", (candidate,))
            .await?
    );
    assert!(
        !jvm.invoke_virtual::<_, bool>(&map, "containsKey", "(Ljava/lang/Object;)Z", (integer(&jvm, 3).await?,))
            .await?
    );

    let changing_map: ClassInstanceRef<Object> = jvm.new_class("java/util/TreeMap", "()V", ()).await?.into();
    put_integer(&jvm, &changing_map, 10, Some(100)).await?;
    put_integer(&jvm, &changing_map, 20, Some(100)).await?;
    let changing_entry: ClassInstanceRef<Object> = jvm
        .new_class(
            "TreeChangingEntry",
            "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)V",
            (integer(&jvm, 10).await?, integer(&jvm, 20).await?, integer(&jvm, 100).await?),
        )
        .await?
        .into();
    let changing_entries: ClassInstanceRef<Object> = jvm.invoke_virtual(&changing_map, "entrySet", "()Ljava/util/Set;", ()).await?;
    assert!(
        jvm.invoke_virtual::<_, bool>(&changing_entries, "remove", "(Ljava/lang/Object;)Z", (changing_entry.clone(),))
            .await?
    );
    assert_eq!(jvm.get_field::<i32>(&changing_entry, "keyCalls", "I").await?, 1);
    assert!(
        !jvm.invoke_virtual::<_, bool>(&changing_map, "containsKey", "(Ljava/lang/Object;)Z", (integer(&jvm, 10).await?,))
            .await?
    );
    assert!(
        jvm.invoke_virtual::<_, bool>(&changing_map, "containsKey", "(Ljava/lang/Object;)Z", (integer(&jvm, 20).await?,))
            .await?
    );
    Ok(())
}

#[tokio::test]
async fn tm_03_sm_01_typed_to_array_preserves_component_and_live_view_contracts() -> Result<()> {
    let jvm = tree_test_jvm().await?;
    let map: ClassInstanceRef<Object> = jvm.new_class("java/util/TreeMap", "()V", ()).await?.into();
    for key in 1..=4 {
        put_integer(&jvm, &map, key, Some(key * 10)).await?;
    }
    let sub_map: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &map,
            "subMap",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/SortedMap;",
            (integer(&jvm, 2).await?, integer(&jvm, 4).await?),
        )
        .await?;

    let keys: ClassInstanceRef<Object> = jvm.invoke_virtual(&map, "keySet", "()Ljava/util/Set;", ()).await?;
    let values: ClassInstanceRef<Object> = jvm.invoke_virtual(&map, "values", "()Ljava/util/Collection;", ()).await?;
    let entries: ClassInstanceRef<Object> = jvm.invoke_virtual(&map, "entrySet", "()Ljava/util/Set;", ()).await?;
    let sub_keys: ClassInstanceRef<Object> = jvm.invoke_virtual(&sub_map, "keySet", "()Ljava/util/Set;", ()).await?;
    let sub_values: ClassInstanceRef<Object> = jvm.invoke_virtual(&sub_map, "values", "()Ljava/util/Collection;", ()).await?;
    let sub_entries: ClassInstanceRef<Object> = jvm.invoke_virtual(&sub_map, "entrySet", "()Ljava/util/Set;", ()).await?;

    let sentinel = integer(&jvm, -1).await?;
    let mut key_destination: ClassInstanceRef<Array<Object>> = jvm.instantiate_array("Ljava/lang/Integer;", 6).await?.into();
    jvm.store_array(
        &mut key_destination,
        0,
        [
            sentinel.clone(),
            sentinel.clone(),
            sentinel.clone(),
            sentinel.clone(),
            sentinel.clone(),
            sentinel.clone(),
        ],
    )
    .await?;
    let key_result: ClassInstanceRef<Array<Object>> = jvm
        .invoke_virtual(&keys, "toArray", "([Ljava/lang/Object;)[Ljava/lang/Object;", (key_destination.clone(),))
        .await?;
    assert_eq!(key_result.identity(), key_destination.identity());
    let key_elements = jvm.load_array::<ClassInstanceRef<Object>>(&key_result, 0, 6).await?;
    for (index, expected) in [1, 2, 3, 4].into_iter().enumerate() {
        assert_eq!(jvm.invoke_virtual::<_, i32>(&key_elements[index], "intValue", "()I", ()).await?, expected);
    }
    assert!(key_elements[4].is_null(), "a reused oversized destination needs a null terminator");
    assert_eq!(
        key_elements[5].identity(),
        sentinel.identity(),
        "elements after the terminator must be preserved"
    );

    let value_destination: ClassInstanceRef<Array<Object>> = jvm.instantiate_array("Ljava/lang/Integer;", 0).await?.into();
    let value_result: ClassInstanceRef<Array<Object>> = jvm
        .invoke_virtual(
            &values,
            "toArray",
            "([Ljava/lang/Object;)[Ljava/lang/Object;",
            (value_destination.clone(),),
        )
        .await?;
    assert_ne!(value_result.identity(), value_destination.identity());
    assert_eq!(value_result.class_definition().name(), "[Ljava/lang/Integer;");
    assert_eq!(jvm.array_length(&value_result).await?, 4);
    let value_elements = jvm.load_array::<ClassInstanceRef<Object>>(&value_result, 0, 4).await?;
    for (index, expected) in [10, 20, 30, 40].into_iter().enumerate() {
        assert_eq!(
            jvm.invoke_virtual::<_, i32>(&value_elements[index], "intValue", "()I", ()).await?,
            expected
        );
    }

    let entry_destination: ClassInstanceRef<Array<Object>> = jvm.instantiate_array("Ljava/util/Map$Entry;", 0).await?.into();
    let entry_result: ClassInstanceRef<Array<Object>> = jvm
        .invoke_virtual(&entries, "toArray", "([Ljava/lang/Object;)[Ljava/lang/Object;", (entry_destination,))
        .await?;
    assert_eq!(entry_result.class_definition().name(), "[Ljava/util/Map$Entry;");
    let root_entries = jvm.load_array::<ClassInstanceRef<Object>>(&entry_result, 0, 4).await?;
    let replacement = integer(&jvm, 111).await?;
    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &root_entries[0],
            "setValue",
            "(Ljava/lang/Object;)Ljava/lang/Object;",
            (replacement.clone(),),
        )
        .await?;
    let stored: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&map, "get", "(Ljava/lang/Object;)Ljava/lang/Object;", (integer(&jvm, 1).await?,))
        .await?;
    assert_eq!(
        stored.identity(),
        replacement.identity(),
        "typed entry arrays must retain live map entries"
    );

    let mut sub_key_destination: ClassInstanceRef<Array<Object>> = jvm.instantiate_array("Ljava/lang/Integer;", 4).await?.into();
    jvm.store_array(
        &mut sub_key_destination,
        0,
        [sentinel.clone(), sentinel.clone(), sentinel.clone(), sentinel.clone()],
    )
    .await?;
    let sub_key_result: ClassInstanceRef<Array<Object>> = jvm
        .invoke_virtual(
            &sub_keys,
            "toArray",
            "([Ljava/lang/Object;)[Ljava/lang/Object;",
            (sub_key_destination.clone(),),
        )
        .await?;
    assert_eq!(sub_key_result.identity(), sub_key_destination.identity());
    let sub_key_elements = jvm.load_array::<ClassInstanceRef<Object>>(&sub_key_result, 0, 4).await?;
    assert_eq!(jvm.invoke_virtual::<_, i32>(&sub_key_elements[0], "intValue", "()I", ()).await?, 2);
    assert_eq!(jvm.invoke_virtual::<_, i32>(&sub_key_elements[1], "intValue", "()I", ()).await?, 3);
    assert!(sub_key_elements[2].is_null());
    assert_eq!(sub_key_elements[3].identity(), sentinel.identity());

    let sub_value_destination: ClassInstanceRef<Array<Object>> = jvm.instantiate_array("Ljava/lang/Integer;", 0).await?.into();
    let sub_value_result: ClassInstanceRef<Array<Object>> = jvm
        .invoke_virtual(
            &sub_values,
            "toArray",
            "([Ljava/lang/Object;)[Ljava/lang/Object;",
            (sub_value_destination,),
        )
        .await?;
    assert_eq!(sub_value_result.class_definition().name(), "[Ljava/lang/Integer;");
    let sub_value_elements = jvm.load_array::<ClassInstanceRef<Object>>(&sub_value_result, 0, 2).await?;
    assert_eq!(jvm.invoke_virtual::<_, i32>(&sub_value_elements[0], "intValue", "()I", ()).await?, 20);
    assert_eq!(jvm.invoke_virtual::<_, i32>(&sub_value_elements[1], "intValue", "()I", ()).await?, 30);

    let sub_entry_destination: ClassInstanceRef<Array<Object>> = jvm.instantiate_array("Ljava/util/Map$Entry;", 0).await?.into();
    let sub_entry_result: ClassInstanceRef<Array<Object>> = jvm
        .invoke_virtual(
            &sub_entries,
            "toArray",
            "([Ljava/lang/Object;)[Ljava/lang/Object;",
            (sub_entry_destination,),
        )
        .await?;
    let sub_entry_elements = jvm.load_array::<ClassInstanceRef<Object>>(&sub_entry_result, 0, 2).await?;
    let replacement = integer(&jvm, 222).await?;
    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &sub_entry_elements[0],
            "setValue",
            "(Ljava/lang/Object;)Ljava/lang/Object;",
            (replacement.clone(),),
        )
        .await?;
    let stored: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&map, "get", "(Ljava/lang/Object;)Ljava/lang/Object;", (integer(&jvm, 2).await?,))
        .await?;
    assert_eq!(
        stored.identity(),
        replacement.identity(),
        "SubMap typed entry arrays must remain root-backed"
    );

    for (name, view) in [
        ("TreeMap.keySet", keys.clone()),
        ("TreeMap.values", values.clone()),
        ("SubMap.keySet", sub_keys.clone()),
        ("SubMap.values", sub_values.clone()),
    ] {
        let incompatible: ClassInstanceRef<Array<Object>> = jvm.instantiate_array("LTreeEqualsValue;", 4).await?.into();
        let result: Result<ClassInstanceRef<Array<Object>>> = jvm
            .invoke_virtual(&view, "toArray", "([Ljava/lang/Object;)[Ljava/lang/Object;", (incompatible,))
            .await;
        let Err(JavaError::JavaException(exception)) = result else {
            panic!("{name}.toArray must reject an incompatible component type");
        };
        assert!(jvm.is_instance(exception.as_ref(), "java/lang/ArrayStoreException"));
    }
    for (name, view) in [("TreeMap.entrySet", entries), ("SubMap.entrySet", sub_entries)] {
        let incompatible: ClassInstanceRef<Array<Object>> = jvm.instantiate_array("Ljava/lang/Integer;", 4).await?.into();
        let result: Result<ClassInstanceRef<Array<Object>>> = jvm
            .invoke_virtual(&view, "toArray", "([Ljava/lang/Object;)[Ljava/lang/Object;", (incompatible,))
            .await;
        let Err(JavaError::JavaException(exception)) = result else {
            panic!("{name}.toArray must reject an incompatible component type");
        };
        assert!(jvm.is_instance(exception.as_ref(), "java/lang/ArrayStoreException"));
    }

    let partial_map: ClassInstanceRef<Object> = jvm.new_class("java/util/TreeMap", "()V", ()).await?.into();
    let first_value: ClassInstanceRef<Object> = jvm.new_class("TreeEqualsValue", "(IZ)V", (1, true)).await?.into();
    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &partial_map,
            "put",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            (integer(&jvm, 1).await?, first_value.clone()),
        )
        .await?;
    put_integer(&jvm, &partial_map, 2, Some(20)).await?;
    let third_value: ClassInstanceRef<Object> = jvm.new_class("TreeEqualsValue", "(IZ)V", (3, true)).await?.into();
    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &partial_map,
            "put",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            (integer(&jvm, 3).await?, third_value),
        )
        .await?;
    let partial_sub_map: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &partial_map,
            "subMap",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/SortedMap;",
            (integer(&jvm, 1).await?, integer(&jvm, 3).await?),
        )
        .await?;
    let partial_values: ClassInstanceRef<Object> = jvm.invoke_virtual(&partial_map, "values", "()Ljava/util/Collection;", ()).await?;
    let partial_sub_values: ClassInstanceRef<Object> = jvm.invoke_virtual(&partial_sub_map, "values", "()Ljava/util/Collection;", ()).await?;
    for (name, view) in [("TreeMap.values", partial_values), ("SubMap.values", partial_sub_values)] {
        let array_sentinel: ClassInstanceRef<Object> = jvm.new_class("TreeEqualsValue", "(IZ)V", (99, true)).await?.into();
        let mut destination: ClassInstanceRef<Array<Object>> = jvm.instantiate_array("LTreeEqualsValue;", 4).await?.into();
        jvm.store_array(
            &mut destination,
            0,
            [
                array_sentinel.clone(),
                array_sentinel.clone(),
                array_sentinel.clone(),
                array_sentinel.clone(),
            ],
        )
        .await?;
        let result: Result<ClassInstanceRef<Array<Object>>> = jvm
            .invoke_virtual(&view, "toArray", "([Ljava/lang/Object;)[Ljava/lang/Object;", (destination.clone(),))
            .await;
        let Err(JavaError::JavaException(exception)) = result else {
            panic!("{name}.toArray must fail at the first incompatible element");
        };
        assert!(jvm.is_instance(exception.as_ref(), "java/lang/ArrayStoreException"));
        let after = jvm.load_array::<ClassInstanceRef<Object>>(&destination, 0, 4).await?;
        assert_eq!(
            after[0].identity(),
            first_value.identity(),
            "compatible prefix must be written before ASE"
        );
        assert_eq!(after[1].identity(), array_sentinel.identity(), "failing slot must remain unchanged");
        assert_eq!(after[2].identity(), array_sentinel.identity());
        assert_eq!(after[3].identity(), array_sentinel.identity());
    }
    Ok(())
}

#[tokio::test]
async fn tm_03_entry_equals_short_circuits_before_reading_value() -> Result<()> {
    let jvm = tree_test_jvm().await?;
    let map: ClassInstanceRef<Object> = jvm.new_class("java/util/TreeMap", "()V", ()).await?.into();
    put_integer(&jvm, &map, 1, Some(10)).await?;
    let entries: ClassInstanceRef<Object> = jvm.invoke_virtual(&map, "entrySet", "()Ljava/util/Set;", ()).await?;
    let iterator: ClassInstanceRef<Object> = jvm.invoke_virtual(&entries, "iterator", "()Ljava/util/Iterator;", ()).await?;
    let entry: ClassInstanceRef<Object> = jvm.invoke_virtual(&iterator, "next", "()Ljava/lang/Object;", ()).await?;

    let different_key: ClassInstanceRef<Object> = jvm
        .new_class(
            "TreeValueProbeEntry",
            "(Ljava/lang/Object;Ljava/lang/Object;Z)V",
            (integer(&jvm, 2).await?, integer(&jvm, 10).await?, true),
        )
        .await?
        .into();
    assert!(
        !jvm.invoke_virtual::<_, bool>(&entry, "equals", "(Ljava/lang/Object;)Z", (different_key.clone(),))
            .await?
    );
    assert_eq!(
        jvm.get_field::<i32>(&different_key, "valueCalls", "I").await?,
        0,
        "TreeMap.Entry.equals must not call getValue after a key mismatch"
    );

    let equal_key: ClassInstanceRef<Object> = jvm
        .new_class(
            "TreeValueProbeEntry",
            "(Ljava/lang/Object;Ljava/lang/Object;Z)V",
            (integer(&jvm, 1).await?, integer(&jvm, 10).await?, true),
        )
        .await?
        .into();
    let result: Result<bool> = jvm.invoke_virtual(&entry, "equals", "(Ljava/lang/Object;)Z", (equal_key.clone(),)).await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("TreeMap.Entry.equals must call getValue after an equal key and propagate its exception");
    };
    assert!(jvm.is_instance(exception.as_ref(), "java/lang/IllegalStateException"));
    assert_eq!(jvm.get_field::<i32>(&equal_key, "valueCalls", "I").await?, 1);
    Ok(())
}

#[tokio::test]
async fn tm_03_ts_02_cross_implementation_equals_and_hash_code_follow_jdk_contracts() -> Result<()> {
    let jvm = tree_test_jvm().await?;
    let tree_map: ClassInstanceRef<Object> = jvm.new_class("java/util/TreeMap", "()V", ()).await?.into();
    let hash_map: ClassInstanceRef<Object> = jvm.new_class("java/util/HashMap", "()V", ()).await?.into();
    for map in [tree_map.clone(), hash_map.clone()] {
        put_integer(&jvm, &map, 1, None).await?;
        put_integer(&jvm, &map, 2, Some(20)).await?;
    }
    assert!(
        jvm.invoke_virtual::<_, bool>(&tree_map, "equals", "(Ljava/lang/Object;)Z", (hash_map.clone(),))
            .await?
    );
    assert!(
        jvm.invoke_virtual::<_, bool>(&hash_map, "equals", "(Ljava/lang/Object;)Z", (tree_map.clone(),))
            .await?
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&tree_map, "hashCode", "()I", ()).await?,
        jvm.invoke_virtual::<_, i32>(&hash_map, "hashCode", "()I", ()).await?
    );

    let incompatible_hash_map: ClassInstanceRef<Object> = jvm.new_class("java/util/HashMap", "()V", ()).await?.into();
    let plain_key: ClassInstanceRef<Object> = jvm.new_class("java/lang/Object", "()V", ()).await?.into();
    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &incompatible_hash_map,
            "put",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            (plain_key, ClassInstanceRef::<Object>::from(None)),
        )
        .await?;
    assert!(
        !jvm.invoke_virtual::<_, bool>(&incompatible_hash_map, "equals", "(Ljava/lang/Object;)Z", (tree_map.clone(),))
            .await?,
        "AbstractMap.equals must convert TreeMap CCE to false"
    );
    let null_key_hash_map: ClassInstanceRef<Object> = jvm.new_class("java/util/HashMap", "()V", ()).await?.into();
    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &null_key_hash_map,
            "put",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            (ClassInstanceRef::<Object>::from(None), ClassInstanceRef::<Object>::from(None)),
        )
        .await?;
    let single_tree_map: ClassInstanceRef<Object> = jvm.new_class("java/util/TreeMap", "()V", ()).await?.into();
    put_integer(&jvm, &single_tree_map, 1, None).await?;
    assert!(
        !jvm.invoke_virtual::<_, bool>(&null_key_hash_map, "equals", "(Ljava/lang/Object;)Z", (single_tree_map.clone(),))
            .await?,
        "AbstractMap.equals must convert TreeMap NPE to false"
    );

    let absolute: ClassInstanceRef<Object> = jvm.new_class("TreeTestComparator", "(ZZZ)V", (false, false, true)).await?.into();
    let equivalent_tree_map: ClassInstanceRef<Object> = jvm
        .new_class("java/util/TreeMap", "(Ljava/util/Comparator;)V", (absolute.clone(),))
        .await?
        .into();
    put_integer(&jvm, &equivalent_tree_map, -5, None).await?;
    let same_key_hash_map: ClassInstanceRef<Object> = jvm.new_class("java/util/HashMap", "()V", ()).await?.into();
    put_integer(&jvm, &same_key_hash_map, -5, None).await?;
    assert!(
        jvm.invoke_virtual::<_, bool>(&equivalent_tree_map, "equals", "(Ljava/lang/Object;)Z", (same_key_hash_map.clone(),))
            .await?
    );
    assert!(
        jvm.invoke_virtual::<_, bool>(&same_key_hash_map, "equals", "(Ljava/lang/Object;)Z", (equivalent_tree_map.clone(),))
            .await?
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&equivalent_tree_map, "hashCode", "()I", ()).await?,
        jvm.invoke_virtual::<_, i32>(&same_key_hash_map, "hashCode", "()I", ()).await?
    );

    let comparator_equivalent_hash_map: ClassInstanceRef<Object> = jvm.new_class("java/util/HashMap", "()V", ()).await?.into();
    put_integer(&jvm, &comparator_equivalent_hash_map, 5, None).await?;
    assert!(
        !jvm.invoke_virtual::<_, bool>(
            &equivalent_tree_map,
            "equals",
            "(Ljava/lang/Object;)Z",
            (comparator_equivalent_hash_map.clone(),)
        )
        .await?,
        "TreeMap equality iterates its retained key and therefore uses Object.equals in the peer map"
    );
    assert!(
        jvm.invoke_virtual::<_, bool>(
            &comparator_equivalent_hash_map,
            "equals",
            "(Ljava/lang/Object;)Z",
            (equivalent_tree_map.clone(),)
        )
        .await?,
        "HashMap equality queries TreeMap with the comparator-equivalent key"
    );
    assert_ne!(
        jvm.invoke_virtual::<_, i32>(&equivalent_tree_map, "hashCode", "()I", ()).await?,
        jvm.invoke_virtual::<_, i32>(&comparator_equivalent_hash_map, "hashCode", "()I", ())
            .await?,
        "a comparator inconsistent with equals intentionally violates the general Map contract"
    );

    let tree_set: ClassInstanceRef<Object> = jvm.new_class("java/util/TreeSet", "()V", ()).await?.into();
    let hash_set: ClassInstanceRef<Object> = jvm.new_class("java/util/HashSet", "()V", ()).await?.into();
    for set in [tree_set.clone(), hash_set.clone()] {
        for key in [1, 2, 3] {
            assert!(
                jvm.invoke_virtual::<_, bool>(&set, "add", "(Ljava/lang/Object;)Z", (integer(&jvm, key).await?,))
                    .await?
            );
        }
    }
    assert!(
        jvm.invoke_virtual::<_, bool>(&tree_set, "equals", "(Ljava/lang/Object;)Z", (hash_set.clone(),))
            .await?
    );
    assert!(
        jvm.invoke_virtual::<_, bool>(&hash_set, "equals", "(Ljava/lang/Object;)Z", (tree_set.clone(),))
            .await?
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&tree_set, "hashCode", "()I", ()).await?,
        jvm.invoke_virtual::<_, i32>(&hash_set, "hashCode", "()I", ()).await?
    );

    let nulls_first: ClassInstanceRef<Object> = jvm.new_class("TreeTestComparator", "(ZZZ)V", (false, true, false)).await?.into();
    let null_tree_set: ClassInstanceRef<Object> = jvm
        .new_class("java/util/TreeSet", "(Ljava/util/Comparator;)V", (nulls_first,))
        .await?
        .into();
    let null_hash_set: ClassInstanceRef<Object> = jvm.new_class("java/util/HashSet", "()V", ()).await?.into();
    for set in [null_tree_set.clone(), null_hash_set.clone()] {
        assert!(
            jvm.invoke_virtual::<_, bool>(&set, "add", "(Ljava/lang/Object;)Z", (ClassInstanceRef::<Object>::from(None),))
                .await?
        );
        assert!(
            jvm.invoke_virtual::<_, bool>(&set, "add", "(Ljava/lang/Object;)Z", (integer(&jvm, 1).await?,))
                .await?
        );
    }
    assert!(
        jvm.invoke_virtual::<_, bool>(&null_tree_set, "equals", "(Ljava/lang/Object;)Z", (null_hash_set.clone(),))
            .await?
    );
    assert!(
        jvm.invoke_virtual::<_, bool>(&null_hash_set, "equals", "(Ljava/lang/Object;)Z", (null_tree_set.clone(),))
            .await?
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&null_tree_set, "hashCode", "()I", ()).await?,
        jvm.invoke_virtual::<_, i32>(&null_hash_set, "hashCode", "()I", ()).await?
    );

    let equivalent_tree_set: ClassInstanceRef<Object> = jvm.new_class("java/util/TreeSet", "(Ljava/util/Comparator;)V", (absolute,)).await?.into();
    assert!(
        jvm.invoke_virtual::<_, bool>(&equivalent_tree_set, "add", "(Ljava/lang/Object;)Z", (integer(&jvm, -5).await?,))
            .await?
    );
    let same_key_hash_set: ClassInstanceRef<Object> = jvm.new_class("java/util/HashSet", "()V", ()).await?.into();
    assert!(
        jvm.invoke_virtual::<_, bool>(&same_key_hash_set, "add", "(Ljava/lang/Object;)Z", (integer(&jvm, -5).await?,))
            .await?
    );
    assert!(
        jvm.invoke_virtual::<_, bool>(&equivalent_tree_set, "equals", "(Ljava/lang/Object;)Z", (same_key_hash_set.clone(),))
            .await?
    );
    assert!(
        jvm.invoke_virtual::<_, bool>(&same_key_hash_set, "equals", "(Ljava/lang/Object;)Z", (equivalent_tree_set.clone(),))
            .await?
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&equivalent_tree_set, "hashCode", "()I", ()).await?,
        jvm.invoke_virtual::<_, i32>(&same_key_hash_set, "hashCode", "()I", ()).await?
    );

    let comparator_equivalent_hash_set: ClassInstanceRef<Object> = jvm.new_class("java/util/HashSet", "()V", ()).await?.into();
    assert!(
        jvm.invoke_virtual::<_, bool>(
            &comparator_equivalent_hash_set,
            "add",
            "(Ljava/lang/Object;)Z",
            (integer(&jvm, 5).await?,)
        )
        .await?
    );
    assert!(
        jvm.invoke_virtual::<_, bool>(
            &equivalent_tree_set,
            "equals",
            "(Ljava/lang/Object;)Z",
            (comparator_equivalent_hash_set.clone(),)
        )
        .await?,
        "TreeSet.containsAll uses comparator equivalence"
    );
    assert!(
        !jvm.invoke_virtual::<_, bool>(
            &comparator_equivalent_hash_set,
            "equals",
            "(Ljava/lang/Object;)Z",
            (equivalent_tree_set.clone(),)
        )
        .await?,
        "HashSet.containsAll uses Object.equals"
    );
    assert_ne!(
        jvm.invoke_virtual::<_, i32>(&equivalent_tree_set, "hashCode", "()I", ()).await?,
        jvm.invoke_virtual::<_, i32>(&comparator_equivalent_hash_set, "hashCode", "()I", ())
            .await?,
        "a comparator inconsistent with equals intentionally violates the general Set contract"
    );
    Ok(())
}

#[tokio::test]
async fn ts_02_ts_03_tree_set_core_constructors_and_comparator_equivalence() -> Result<()> {
    let jvm = tree_test_jvm().await?;
    let set: ClassInstanceRef<Object> = jvm.new_class("java/util/TreeSet", "()V", ()).await?.into();
    for (key, expected_added) in [(3, true), (1, true), (2, true), (2, false)] {
        assert_eq!(
            jvm.invoke_virtual::<_, bool>(&set, "add", "(Ljava/lang/Object;)Z", (integer(&jvm, key).await?,))
                .await?,
            expected_added
        );
    }
    assert_eq!(jvm.invoke_virtual::<_, i32>(&set, "size", "()I", ()).await?, 3);
    assert!(
        jvm.invoke_virtual::<_, bool>(&set, "contains", "(Ljava/lang/Object;)Z", (integer(&jvm, 2).await?,))
            .await?
    );
    let first: ClassInstanceRef<Object> = jvm.invoke_virtual(&set, "first", "()Ljava/lang/Object;", ()).await?;
    let last: ClassInstanceRef<Object> = jvm.invoke_virtual(&set, "last", "()Ljava/lang/Object;", ()).await?;
    assert_eq!(jvm.invoke_virtual::<_, i32>(&first, "intValue", "()I", ()).await?, 1);
    assert_eq!(jvm.invoke_virtual::<_, i32>(&last, "intValue", "()I", ()).await?, 3);
    assert!(
        jvm.invoke_virtual::<_, bool>(&set, "remove", "(Ljava/lang/Object;)Z", (integer(&jvm, 2).await?,))
            .await?
    );
    assert!(
        !jvm.invoke_virtual::<_, bool>(&set, "remove", "(Ljava/lang/Object;)Z", (integer(&jvm, 2).await?,))
            .await?
    );

    let collection: ClassInstanceRef<Object> = jvm.new_class("java/util/ArrayList", "()V", ()).await?.into();
    for key in [4, 2, 3, 2] {
        let _: bool = jvm
            .invoke_virtual(&collection, "add", "(Ljava/lang/Object;)Z", (integer(&jvm, key).await?,))
            .await?;
    }
    let collection_set: ClassInstanceRef<Object> = jvm
        .new_class("java/util/TreeSet", "(Ljava/util/Collection;)V", (collection,))
        .await?
        .into();
    let mut collection_values = Vec::new();
    let iterator: ClassInstanceRef<Object> = jvm.invoke_virtual(&collection_set, "iterator", "()Ljava/util/Iterator;", ()).await?;
    while jvm.invoke_virtual::<_, bool>(&iterator, "hasNext", "()Z", ()).await? {
        let value: ClassInstanceRef<Object> = jvm.invoke_virtual(&iterator, "next", "()Ljava/lang/Object;", ()).await?;
        collection_values.push(jvm.invoke_virtual::<_, i32>(&value, "intValue", "()I", ()).await?);
    }
    assert_eq!(collection_values, vec![2, 3, 4]);
    assert!(
        jvm.invoke_virtual::<_, ClassInstanceRef<Object>>(&collection_set, "comparator", "()Ljava/util/Comparator;", ())
            .await?
            .is_null()
    );

    let reverse: ClassInstanceRef<Object> = jvm.new_class("TreeTestComparator", "(ZZZ)V", (true, false, false)).await?.into();
    let reverse_set: ClassInstanceRef<Object> = jvm
        .new_class("java/util/TreeSet", "(Ljava/util/Comparator;)V", (reverse.clone(),))
        .await?
        .into();
    for key in [1, 3, 2] {
        let _: bool = jvm
            .invoke_virtual(&reverse_set, "add", "(Ljava/lang/Object;)Z", (integer(&jvm, key).await?,))
            .await?;
    }
    let sorted_copy: ClassInstanceRef<Object> = jvm
        .new_class("java/util/TreeSet", "(Ljava/util/SortedSet;)V", (reverse_set.clone(),))
        .await?
        .into();
    let copied_comparator: ClassInstanceRef<Object> = jvm.invoke_virtual(&sorted_copy, "comparator", "()Ljava/util/Comparator;", ()).await?;
    assert_eq!(copied_comparator.identity(), reverse.identity());
    let mut copied_values = Vec::new();
    let iterator: ClassInstanceRef<Object> = jvm.invoke_virtual(&sorted_copy, "iterator", "()Ljava/util/Iterator;", ()).await?;
    while jvm.invoke_virtual::<_, bool>(&iterator, "hasNext", "()Z", ()).await? {
        let value: ClassInstanceRef<Object> = jvm.invoke_virtual(&iterator, "next", "()Ljava/lang/Object;", ()).await?;
        copied_values.push(jvm.invoke_virtual::<_, i32>(&value, "intValue", "()I", ()).await?);
    }
    assert_eq!(copied_values, vec![3, 2, 1]);

    let collection_copy: ClassInstanceRef<Object> = jvm
        .new_class("java/util/TreeSet", "(Ljava/util/Collection;)V", (reverse_set,))
        .await?
        .into();
    assert!(
        jvm.invoke_virtual::<_, ClassInstanceRef<Object>>(&collection_copy, "comparator", "()Ljava/util/Comparator;", ())
            .await?
            .is_null()
    );
    let mut natural_values = Vec::new();
    let iterator: ClassInstanceRef<Object> = jvm.invoke_virtual(&collection_copy, "iterator", "()Ljava/util/Iterator;", ()).await?;
    while jvm.invoke_virtual::<_, bool>(&iterator, "hasNext", "()Z", ()).await? {
        let value: ClassInstanceRef<Object> = jvm.invoke_virtual(&iterator, "next", "()Ljava/lang/Object;", ()).await?;
        natural_values.push(jvm.invoke_virtual::<_, i32>(&value, "intValue", "()I", ()).await?);
    }
    assert_eq!(natural_values, vec![1, 2, 3]);

    let absolute: ClassInstanceRef<Object> = jvm.new_class("TreeTestComparator", "(ZZZ)V", (false, false, true)).await?.into();
    let equivalent: ClassInstanceRef<Object> = jvm.new_class("java/util/TreeSet", "(Ljava/util/Comparator;)V", (absolute,)).await?.into();
    assert!(
        jvm.invoke_virtual::<_, bool>(&equivalent, "add", "(Ljava/lang/Object;)Z", (integer(&jvm, -5).await?,))
            .await?
    );
    assert!(
        !jvm.invoke_virtual::<_, bool>(&equivalent, "add", "(Ljava/lang/Object;)Z", (integer(&jvm, 5).await?,))
            .await?
    );
    let retained: ClassInstanceRef<Object> = jvm.invoke_virtual(&equivalent, "first", "()Ljava/lang/Object;", ()).await?;
    assert_eq!(jvm.invoke_virtual::<_, i32>(&retained, "intValue", "()I", ()).await?, -5);

    for descriptor in ["(Ljava/util/Collection;)V", "(Ljava/util/SortedSet;)V"] {
        let result: Result<ClassInstanceRef<Object>> = jvm
            .new_class("java/util/TreeSet", descriptor, (ClassInstanceRef::<Object>::from(None),))
            .await
            .map(Into::into);
        let Err(JavaError::JavaException(exception)) = result else {
            panic!("TreeSet{descriptor} null source must throw");
        };
        assert!(jvm.is_instance(exception.as_ref(), "java/lang/NullPointerException"));
    }

    let _: () = jvm.invoke_virtual(&set, "clear", "()V", ()).await?;
    for method in ["first", "last"] {
        let result: Result<ClassInstanceRef<Object>> = jvm.invoke_virtual(&set, method, "()Ljava/lang/Object;", ()).await;
        let Err(JavaError::JavaException(exception)) = result else {
            panic!("empty TreeSet.{method} must throw");
        };
        assert!(jvm.is_instance(exception.as_ref(), "java/util/NoSuchElementException"));
    }
    Ok(())
}

#[tokio::test]
async fn ts_03_tree_set_ranges_are_live_bounded_and_iterator_mutable() -> Result<()> {
    let jvm = tree_test_jvm().await?;
    let set: ClassInstanceRef<Object> = jvm.new_class("java/util/TreeSet", "()V", ()).await?.into();
    for key in 0..10 {
        let _: bool = jvm
            .invoke_virtual(&set, "add", "(Ljava/lang/Object;)Z", (integer(&jvm, key).await?,))
            .await?;
    }
    let range: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &set,
            "subSet",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/SortedSet;",
            (integer(&jvm, 2).await?, integer(&jvm, 8).await?),
        )
        .await?;
    assert_eq!(jvm.invoke_virtual::<_, i32>(&range, "size", "()I", ()).await?, 6);
    assert!(
        jvm.invoke_virtual::<_, bool>(&range, "contains", "(Ljava/lang/Object;)Z", (integer(&jvm, 2).await?,))
            .await?
    );
    assert!(
        !jvm.invoke_virtual::<_, bool>(&range, "contains", "(Ljava/lang/Object;)Z", (integer(&jvm, 8).await?,))
            .await?
    );

    let nested: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&range, "tailSet", "(Ljava/lang/Object;)Ljava/util/SortedSet;", (integer(&jvm, 4).await?,))
        .await?;
    let iterator: ClassInstanceRef<Object> = jvm.invoke_virtual(&nested, "iterator", "()Ljava/util/Iterator;", ()).await?;
    let mut nested_values = Vec::new();
    while jvm.invoke_virtual::<_, bool>(&iterator, "hasNext", "()Z", ()).await? {
        let value: ClassInstanceRef<Object> = jvm.invoke_virtual(&iterator, "next", "()Ljava/lang/Object;", ()).await?;
        let value = jvm.invoke_virtual::<_, i32>(&value, "intValue", "()I", ()).await?;
        nested_values.push(value);
        if value == 5 {
            let _: () = jvm.invoke_virtual(&iterator, "remove", "()V", ()).await?;
        }
    }
    assert_eq!(nested_values, vec![4, 5, 6, 7]);
    assert!(
        !jvm.invoke_virtual::<_, bool>(&set, "contains", "(Ljava/lang/Object;)Z", (integer(&jvm, 5).await?,))
            .await?
    );

    assert!(
        jvm.invoke_virtual::<_, bool>(&range, "add", "(Ljava/lang/Object;)Z", (integer(&jvm, 5).await?,))
            .await?
    );
    assert!(
        jvm.invoke_virtual::<_, bool>(&set, "contains", "(Ljava/lang/Object;)Z", (integer(&jvm, 5).await?,))
            .await?
    );
    let outside_add: Result<bool> = jvm
        .invoke_virtual(&range, "add", "(Ljava/lang/Object;)Z", (integer(&jvm, 8).await?,))
        .await;
    let Err(JavaError::JavaException(exception)) = outside_add else {
        panic!("range add at exclusive upper bound must throw");
    };
    assert!(jvm.is_instance(exception.as_ref(), "java/lang/IllegalArgumentException"));

    let _: () = jvm.invoke_virtual(&nested, "clear", "()V", ()).await?;
    let mut remaining = Vec::new();
    let iterator: ClassInstanceRef<Object> = jvm.invoke_virtual(&set, "iterator", "()Ljava/util/Iterator;", ()).await?;
    while jvm.invoke_virtual::<_, bool>(&iterator, "hasNext", "()Z", ()).await? {
        let value: ClassInstanceRef<Object> = jvm.invoke_virtual(&iterator, "next", "()Ljava/lang/Object;", ()).await?;
        remaining.push(jvm.invoke_virtual::<_, i32>(&value, "intValue", "()I", ()).await?);
    }
    assert_eq!(remaining, vec![0, 1, 2, 3, 8, 9]);

    let nulls_first: ClassInstanceRef<Object> = jvm.new_class("TreeTestComparator", "(ZZZ)V", (false, true, false)).await?.into();
    let null_set: ClassInstanceRef<Object> = jvm
        .new_class("java/util/TreeSet", "(Ljava/util/Comparator;)V", (nulls_first.clone(),))
        .await?
        .into();
    assert!(
        jvm.invoke_virtual::<_, bool>(&null_set, "add", "(Ljava/lang/Object;)Z", (ClassInstanceRef::<Object>::from(None),))
            .await?
    );
    assert!(
        jvm.invoke_virtual::<_, bool>(&null_set, "add", "(Ljava/lang/Object;)Z", (integer(&jvm, 1).await?,))
            .await?
    );
    let first: ClassInstanceRef<Object> = jvm.invoke_virtual(&null_set, "first", "()Ljava/lang/Object;", ()).await?;
    assert!(first.is_null());
    let comparator: ClassInstanceRef<Object> = jvm.invoke_virtual(&null_set, "comparator", "()Ljava/util/Comparator;", ()).await?;
    assert_eq!(comparator.identity(), nulls_first.identity());
    let null_tail: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &null_set,
            "tailSet",
            "(Ljava/lang/Object;)Ljava/util/SortedSet;",
            (ClassInstanceRef::<Object>::from(None),),
        )
        .await?;
    assert_eq!(jvm.invoke_virtual::<_, i32>(&null_tail, "size", "()I", ()).await?, 2);
    let first: ClassInstanceRef<Object> = jvm.invoke_virtual(&null_tail, "first", "()Ljava/lang/Object;", ()).await?;
    assert!(first.is_null(), "null range bound must not be mistaken for an unbounded view");
    Ok(())
}
