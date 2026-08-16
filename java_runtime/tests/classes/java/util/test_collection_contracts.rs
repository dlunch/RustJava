use alloc::{boxed::Box, collections::BTreeMap, vec, vec::Vec};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use java_runtime::{RuntimeClassProto, RuntimeContext, classes::java::lang::Object, get_runtime_class_proto};
use jvm::{Array, ClassInstanceRef, JavaError, Jvm, Result, runtime::JavaLangString};
use jvm_rust::ClassDefinitionImpl;

use test_utils::{TestRuntime, create_test_jvm, test_jvm};

struct SnapshotCollection;

impl SnapshotCollection {
    fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "SnapshotCollection",
            parent_class: Some("java/util/AbstractCollection"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "([Ljava/lang/Object;)V", Self::init, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("size", "()I", Self::size, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("iterator", "()Ljava/util/Iterator;", Self::iterator, MethodAccessFlags::PUBLIC),
            ],
            fields: vec![JavaFieldProto::new("elements", "[Ljava/lang/Object;", FieldAccessFlags::PRIVATE)],
            access_flags: ClassAccessFlags::PUBLIC,
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, elements: ClassInstanceRef<Array<Object>>) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/util/AbstractCollection", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "elements", "[Ljava/lang/Object;", elements).await
    }

    async fn size(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        let elements: ClassInstanceRef<Array<Object>> = jvm.get_field(&this, "elements", "[Ljava/lang/Object;").await?;
        Ok(jvm.array_length(&elements).await? as i32)
    }

    async fn iterator(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let elements: ClassInstanceRef<Array<Object>> = jvm.get_field(&this, "elements", "[Ljava/lang/Object;").await?;
        Ok(jvm.new_class("SnapshotIterator", "([Ljava/lang/Object;)V", (elements,)).await?.into())
    }
}

struct SnapshotIterator;

impl SnapshotIterator {
    fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "SnapshotIterator",
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["java/util/Iterator"],
            methods: vec![
                JavaMethodProto::new("<init>", "([Ljava/lang/Object;)V", Self::init, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("hasNext", "()Z", Self::has_next, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("next", "()Ljava/lang/Object;", Self::next, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("remove", "()V", Self::remove, MethodAccessFlags::PUBLIC),
            ],
            fields: vec![
                JavaFieldProto::new("elements", "[Ljava/lang/Object;", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("index", "I", FieldAccessFlags::PRIVATE),
            ],
            access_flags: ClassAccessFlags::PUBLIC,
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, elements: ClassInstanceRef<Array<Object>>) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "elements", "[Ljava/lang/Object;", elements).await?;
        jvm.put_field(&mut this, "index", "I", 0).await
    }

    async fn has_next(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<bool> {
        let elements: ClassInstanceRef<Array<Object>> = jvm.get_field(&this, "elements", "[Ljava/lang/Object;").await?;
        let index: i32 = jvm.get_field(&this, "index", "I").await?;
        Ok(index < jvm.array_length(&elements).await? as i32)
    }

    async fn next(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let elements: ClassInstanceRef<Array<Object>> = jvm.get_field(&this, "elements", "[Ljava/lang/Object;").await?;
        let index: i32 = jvm.get_field(&this, "index", "I").await?;
        if index >= jvm.array_length(&elements).await? as i32 {
            return Err(jvm.exception("java/util/NoSuchElementException", "snapshot iterator exhausted").await);
        }
        let value = jvm.load_array::<ClassInstanceRef<Object>>(&elements, index as usize, 1).await?.remove(0);
        jvm.put_field(&mut this, "index", "I", index + 1).await?;
        Ok(value)
    }

    async fn remove(jvm: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>) -> Result<()> {
        Err(jvm
            .exception("java/lang/UnsupportedOperationException", "immutable test collection")
            .await)
    }
}

struct DirectionalStoredKey;

impl DirectionalStoredKey {
    fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "DirectionalStoredKey",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("hashCode", "()I", Self::hash_code, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("equals", "(Ljava/lang/Object;)Z", Self::equals, MethodAccessFlags::PUBLIC),
            ],
            fields: vec![],
            access_flags: ClassAccessFlags::PUBLIC,
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await
    }

    async fn hash_code(_: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>) -> Result<i32> {
        Ok(0)
    }

    async fn equals(_: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>, _: ClassInstanceRef<Object>) -> Result<bool> {
        Ok(false)
    }
}

struct DirectionalQueryKey;

impl DirectionalQueryKey {
    fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "DirectionalQueryKey",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("hashCode", "()I", Self::hash_code, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("equals", "(Ljava/lang/Object;)Z", Self::equals, MethodAccessFlags::PUBLIC),
            ],
            fields: vec![],
            access_flags: ClassAccessFlags::PUBLIC,
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await
    }

    async fn hash_code(_: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>) -> Result<i32> {
        Ok(0)
    }

    async fn equals(jvm: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>, other: ClassInstanceRef<Object>) -> Result<bool> {
        Ok(!other.is_null() && jvm.is_instance(&**other, "DirectionalStoredKey"))
    }
}

struct ConfigurableEqualsValue;

impl ConfigurableEqualsValue {
    fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "ConfigurableEqualsValue",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(Z)V", Self::init, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("equals", "(Ljava/lang/Object;)Z", Self::equals, MethodAccessFlags::PUBLIC),
            ],
            fields: vec![
                JavaFieldProto::new("result", "Z", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("equalsCalls", "I", FieldAccessFlags::PRIVATE),
            ],
            access_flags: ClassAccessFlags::PUBLIC,
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, result: bool) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "result", "Z", result).await?;
        jvm.put_field(&mut this, "equalsCalls", "I", 0).await
    }

    async fn equals(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, _: ClassInstanceRef<Object>) -> Result<bool> {
        let calls: i32 = jvm.get_field(&this, "equalsCalls", "I").await?;
        jvm.put_field(&mut this, "equalsCalls", "I", calls + 1).await?;
        jvm.get_field(&this, "result", "Z").await
    }
}

struct StatefulMapEntry;

impl StatefulMapEntry {
    fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "StatefulMapEntry",
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["java/util/Map$Entry"],
            methods: vec![
                JavaMethodProto::new(
                    "<init>",
                    "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;I)V",
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
            ],
            fields: vec![
                JavaFieldProto::new("firstKey", "Ljava/lang/Object;", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("laterKey", "Ljava/lang/Object;", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("value", "Ljava/lang/Object;", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("mode", "I", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("keyCalls", "I", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("valueCalls", "I", FieldAccessFlags::PRIVATE),
            ],
            access_flags: ClassAccessFlags::PUBLIC,
        }
    }

    async fn init(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        first_key: ClassInstanceRef<Object>,
        later_key: ClassInstanceRef<Object>,
        value: ClassInstanceRef<Object>,
        mode: i32,
    ) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "firstKey", "Ljava/lang/Object;", first_key).await?;
        jvm.put_field(&mut this, "laterKey", "Ljava/lang/Object;", later_key).await?;
        jvm.put_field(&mut this, "value", "Ljava/lang/Object;", value).await?;
        jvm.put_field(&mut this, "mode", "I", mode).await?;
        jvm.put_field(&mut this, "keyCalls", "I", 0).await?;
        jvm.put_field(&mut this, "valueCalls", "I", 0).await
    }

    async fn get_key(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let calls: i32 = jvm.get_field(&this, "keyCalls", "I").await?;
        jvm.put_field(&mut this, "keyCalls", "I", calls + 1).await?;
        if jvm.get_field::<i32>(&this, "mode", "I").await? == 1 {
            return Err(jvm.exception("java/lang/IllegalStateException", "getKey failure").await);
        }
        if calls == 0 {
            jvm.get_field(&this, "firstKey", "Ljava/lang/Object;").await
        } else {
            jvm.get_field(&this, "laterKey", "Ljava/lang/Object;").await
        }
    }

    async fn get_value(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let calls: i32 = jvm.get_field(&this, "valueCalls", "I").await?;
        jvm.put_field(&mut this, "valueCalls", "I", calls + 1).await?;
        if jvm.get_field::<i32>(&this, "mode", "I").await? == 2 {
            return Err(jvm.exception("java/lang/IllegalStateException", "getValue failure").await);
        }
        jvm.get_field(&this, "value", "Ljava/lang/Object;").await
    }

    async fn set_value(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        value: ClassInstanceRef<Object>,
    ) -> Result<ClassInstanceRef<Object>> {
        let previous: ClassInstanceRef<Object> = jvm.get_field(&this, "value", "Ljava/lang/Object;").await?;
        jvm.put_field(&mut this, "value", "Ljava/lang/Object;", value).await?;
        Ok(previous)
    }
}

struct ThrowingMap;

impl ThrowingMap {
    fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "ThrowingMap",
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["java/util/Map"],
            methods: vec![
                JavaMethodProto::new("<init>", "(II)V", Self::init, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("size", "()I", Self::size, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("get", "(Ljava/lang/Object;)Ljava/lang/Object;", Self::get, MethodAccessFlags::PUBLIC),
            ],
            fields: vec![
                JavaFieldProto::new("size", "I", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("mode", "I", FieldAccessFlags::PRIVATE),
            ],
            access_flags: ClassAccessFlags::PUBLIC,
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, size: i32, mode: i32) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "size", "I", size).await?;
        jvm.put_field(&mut this, "mode", "I", mode).await
    }

    async fn size(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        jvm.get_field(&this, "size", "I").await
    }

    async fn get(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, _: ClassInstanceRef<Object>) -> Result<ClassInstanceRef<Object>> {
        if jvm.get_field::<i32>(&this, "mode", "I").await? == 0 {
            Err(jvm.exception("java/lang/NullPointerException", "test map get failure").await)
        } else {
            Err(jvm.exception("java/lang/ClassCastException", "test map get failure").await)
        }
    }
}

async fn collection_contract_fixture_jvm() -> Result<Jvm> {
    let runtime = TestRuntime::new(BTreeMap::new());
    let jvm = create_test_jvm(runtime.clone()).await?;
    for proto in [
        SnapshotCollection::as_proto(),
        SnapshotIterator::as_proto(),
        DirectionalStoredKey::as_proto(),
        DirectionalQueryKey::as_proto(),
        ConfigurableEqualsValue::as_proto(),
        StatefulMapEntry::as_proto(),
        ThrowingMap::as_proto(),
    ] {
        jvm.register_class(
            Box::new(ClassDefinitionImpl::from_class_proto(proto, Box::new(runtime.clone()) as Box<_>)),
            None,
        )
        .await?;
    }

    Ok(jvm)
}

#[tokio::test]
async fn col_01_to_06_interfaces_have_exact_descriptors_and_flags() -> Result<()> {
    for (class_name, interfaces, methods) in [
        (
            "java/util/Collection",
            Vec::new(),
            vec![
                ("containsAll", "(Ljava/util/Collection;)Z"),
                ("addAll", "(Ljava/util/Collection;)Z"),
                ("removeAll", "(Ljava/util/Collection;)Z"),
                ("retainAll", "(Ljava/util/Collection;)Z"),
                ("toArray", "([Ljava/lang/Object;)[Ljava/lang/Object;"),
            ],
        ),
        (
            "java/util/List",
            vec!["java/util/Collection"],
            vec![
                ("addAll", "(ILjava/util/Collection;)Z"),
                ("indexOf", "(Ljava/lang/Object;)I"),
                ("lastIndexOf", "(Ljava/lang/Object;)I"),
                ("listIterator", "()Ljava/util/ListIterator;"),
                ("listIterator", "(I)Ljava/util/ListIterator;"),
                ("subList", "(II)Ljava/util/List;"),
            ],
        ),
        (
            "java/util/Set",
            vec!["java/util/Collection"],
            vec![
                ("size", "()I"),
                ("isEmpty", "()Z"),
                ("contains", "(Ljava/lang/Object;)Z"),
                ("iterator", "()Ljava/util/Iterator;"),
                ("toArray", "()[Ljava/lang/Object;"),
                ("toArray", "([Ljava/lang/Object;)[Ljava/lang/Object;"),
                ("add", "(Ljava/lang/Object;)Z"),
                ("remove", "(Ljava/lang/Object;)Z"),
                ("containsAll", "(Ljava/util/Collection;)Z"),
                ("addAll", "(Ljava/util/Collection;)Z"),
                ("retainAll", "(Ljava/util/Collection;)Z"),
                ("removeAll", "(Ljava/util/Collection;)Z"),
                ("clear", "()V"),
                ("equals", "(Ljava/lang/Object;)Z"),
                ("hashCode", "()I"),
            ],
        ),
        ("java/util/Map", Vec::new(), vec![("putAll", "(Ljava/util/Map;)V")]),
        (
            "java/util/Comparator",
            Vec::new(),
            vec![
                ("compare", "(Ljava/lang/Object;Ljava/lang/Object;)I"),
                ("equals", "(Ljava/lang/Object;)Z"),
            ],
        ),
        (
            "java/util/ListIterator",
            vec!["java/util/Iterator"],
            vec![
                ("hasNext", "()Z"),
                ("next", "()Ljava/lang/Object;"),
                ("hasPrevious", "()Z"),
                ("previous", "()Ljava/lang/Object;"),
                ("nextIndex", "()I"),
                ("previousIndex", "()I"),
                ("remove", "()V"),
                ("set", "(Ljava/lang/Object;)V"),
                ("add", "(Ljava/lang/Object;)V"),
            ],
        ),
    ] {
        let proto = get_runtime_class_proto(class_name).unwrap_or_else(|| panic!("missing {class_name}"));
        assert_eq!(
            proto.access_flags,
            ClassAccessFlags::PUBLIC | ClassAccessFlags::INTERFACE | ClassAccessFlags::ABSTRACT
        );
        assert_eq!(proto.interfaces, interfaces);
        for (name, descriptor) in methods {
            let method = proto
                .methods
                .iter()
                .find(|method| method.name == name && method.descriptor == descriptor)
                .unwrap_or_else(|| panic!("missing {class_name}.{name}{descriptor}"));
            assert_eq!(
                method.access_flags,
                MethodAccessFlags::PUBLIC | MethodAccessFlags::ABSTRACT,
                "{class_name}.{name}{descriptor}"
            );
        }
    }
    let set = get_runtime_class_proto("java/util/Set").unwrap();
    assert_eq!(set.parent_class, None);
    assert!(set.fields.is_empty());
    assert_eq!(set.methods.len(), 15, "java/util/Set must redeclare the complete J2SE 1.2 contract");

    let jvm = test_jvm().await?;
    let list_iterator = jvm.resolve_class("java/util/ListIterator").await?;
    assert!(
        list_iterator
            .definition
            .interface_names()
            .iter()
            .any(|interface| interface == "java/util/Iterator")
    );

    for class_name in [
        "java/util/ArrayList",
        "java/util/Vector",
        "java/util/HashSet",
        "java/util/HashMap",
        "java/util/Hashtable",
    ] {
        assert_eq!(
            get_runtime_class_proto(class_name).unwrap().access_flags,
            ClassAccessFlags::PUBLIC,
            "{class_name}"
        );
    }
    for (class_name, name, descriptor, flags) in [
        ("java/util/ArrayList", "<init>", "(Ljava/util/Collection;)V", MethodAccessFlags::PUBLIC),
        ("java/util/Vector", "<init>", "(Ljava/util/Collection;)V", MethodAccessFlags::PUBLIC),
        (
            "java/util/Vector",
            "addAll",
            "(Ljava/util/Collection;)Z",
            MethodAccessFlags::PUBLIC | MethodAccessFlags::SYNCHRONIZED,
        ),
        (
            "java/util/Vector",
            "addAll",
            "(ILjava/util/Collection;)Z",
            MethodAccessFlags::PUBLIC | MethodAccessFlags::SYNCHRONIZED,
        ),
        (
            "java/util/Vector",
            "containsAll",
            "(Ljava/util/Collection;)Z",
            MethodAccessFlags::PUBLIC | MethodAccessFlags::SYNCHRONIZED,
        ),
        (
            "java/util/Vector",
            "removeAll",
            "(Ljava/util/Collection;)Z",
            MethodAccessFlags::PUBLIC | MethodAccessFlags::SYNCHRONIZED,
        ),
        (
            "java/util/Vector",
            "retainAll",
            "(Ljava/util/Collection;)Z",
            MethodAccessFlags::PUBLIC | MethodAccessFlags::SYNCHRONIZED,
        ),
        (
            "java/util/Vector",
            "toArray",
            "([Ljava/lang/Object;)[Ljava/lang/Object;",
            MethodAccessFlags::PUBLIC | MethodAccessFlags::SYNCHRONIZED,
        ),
        (
            "java/util/Vector",
            "listIterator",
            "()Ljava/util/ListIterator;",
            MethodAccessFlags::PUBLIC | MethodAccessFlags::SYNCHRONIZED,
        ),
        (
            "java/util/Vector",
            "listIterator",
            "(I)Ljava/util/ListIterator;",
            MethodAccessFlags::PUBLIC | MethodAccessFlags::SYNCHRONIZED,
        ),
        ("java/util/HashSet", "<init>", "(Ljava/util/Collection;)V", MethodAccessFlags::PUBLIC),
        ("java/util/HashMap", "<init>", "(Ljava/util/Map;)V", MethodAccessFlags::PUBLIC),
        ("java/util/HashMap", "putAll", "(Ljava/util/Map;)V", MethodAccessFlags::PUBLIC),
        ("java/util/Hashtable", "<init>", "(Ljava/util/Map;)V", MethodAccessFlags::PUBLIC),
        (
            "java/util/Hashtable",
            "putAll",
            "(Ljava/util/Map;)V",
            MethodAccessFlags::PUBLIC | MethodAccessFlags::SYNCHRONIZED,
        ),
        (
            "java/util/Hashtable",
            "equals",
            "(Ljava/lang/Object;)Z",
            MethodAccessFlags::PUBLIC | MethodAccessFlags::SYNCHRONIZED,
        ),
        (
            "java/util/Hashtable",
            "hashCode",
            "()I",
            MethodAccessFlags::PUBLIC | MethodAccessFlags::SYNCHRONIZED,
        ),
    ] {
        let proto = get_runtime_class_proto(class_name).unwrap();
        let method = proto
            .methods
            .iter()
            .find(|method| method.name == name && method.descriptor == descriptor)
            .unwrap();
        assert_eq!(method.access_flags, flags, "{class_name}.{name}{descriptor}");
    }
    let entry = get_runtime_class_proto("java/util/Hashtable$Entry").unwrap();
    for (name, descriptor) in [("equals", "(Ljava/lang/Object;)Z"), ("hashCode", "()I")] {
        let method = entry
            .methods
            .iter()
            .find(|method| method.name == name && method.descriptor == descriptor)
            .unwrap_or_else(|| panic!("missing java/util/Hashtable$Entry.{name}{descriptor}"));
        assert_eq!(method.access_flags, MethodAccessFlags::PUBLIC);
    }

    Ok(())
}

#[tokio::test]
async fn col_01_bulk_operations_handle_null_self_and_mutation_results() -> Result<()> {
    let jvm = test_jvm().await?;
    let list = jvm.new_class("java/util/ArrayList", "()V", ()).await?;
    let first = JavaLangString::from_rust_string(&jvm, "first").await?;
    let second = JavaLangString::from_rust_string(&jvm, "second").await?;
    let _: bool = jvm
        .invoke_virtual(&list, &list.class_definition().name(), "add", "(Ljava/lang/Object;)Z", (first.clone(),))
        .await?;
    let _: bool = jvm
        .invoke_virtual(&list, &list.class_definition().name(), "add", "(Ljava/lang/Object;)Z", (second.clone(),))
        .await?;

    let contains_all: bool = jvm
        .invoke_virtual(
            &list,
            &list.class_definition().name(),
            "containsAll",
            "(Ljava/util/Collection;)Z",
            (list.clone(),),
        )
        .await?;
    assert!(contains_all);
    let retained: bool = jvm
        .invoke_virtual(
            &list,
            &list.class_definition().name(),
            "retainAll",
            "(Ljava/util/Collection;)Z",
            (list.clone(),),
        )
        .await?;
    assert!(!retained);

    let added: bool = jvm
        .invoke_virtual(
            &list,
            &list.class_definition().name(),
            "addAll",
            "(Ljava/util/Collection;)Z",
            (list.clone(),),
        )
        .await?;
    assert!(added);
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&list, &list.class_definition().name(), "size", "()I", ())
            .await?,
        4
    );

    let filter = jvm.new_class("java/util/HashSet", "()V", ()).await?;
    let _: bool = jvm
        .invoke_virtual(
            &filter,
            &filter.class_definition().name(),
            "add",
            "(Ljava/lang/Object;)Z",
            (first.clone(),),
        )
        .await?;
    let removed: bool = jvm
        .invoke_virtual(
            &list,
            &list.class_definition().name(),
            "removeAll",
            "(Ljava/util/Collection;)Z",
            (filter.clone(),),
        )
        .await?;
    assert!(removed);
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&list, &list.class_definition().name(), "size", "()I", ())
            .await?,
        2
    );
    let removed_again: bool = jvm
        .invoke_virtual(
            &list,
            &list.class_definition().name(),
            "removeAll",
            "(Ljava/util/Collection;)Z",
            (filter,),
        )
        .await?;
    assert!(!removed_again);

    let removed_self: bool = jvm
        .invoke_virtual(
            &list,
            &list.class_definition().name(),
            "removeAll",
            "(Ljava/util/Collection;)Z",
            (list.clone(),),
        )
        .await?;
    assert!(removed_self);
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&list, &list.class_definition().name(), "size", "()I", ())
            .await?,
        0
    );

    let null_collection: ClassInstanceRef<Object> = None.into();
    for (name, descriptor) in [
        ("containsAll", "(Ljava/util/Collection;)Z"),
        ("addAll", "(Ljava/util/Collection;)Z"),
        ("removeAll", "(Ljava/util/Collection;)Z"),
        ("retainAll", "(Ljava/util/Collection;)Z"),
    ] {
        let result: Result<bool> = jvm
            .invoke_virtual(&list, &list.class_definition().name(), name, descriptor, (null_collection.clone(),))
            .await;
        let Err(JavaError::JavaException(exception)) = result else {
            panic!("{name} must reject null");
        };
        assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));
    }

    Ok(())
}

#[tokio::test]
async fn col_02_07_09_list_defaults_and_collection_copy_constructors() -> Result<()> {
    let jvm = test_jvm().await?;
    let source = jvm.new_class("java/util/ArrayList", "()V", ()).await?;
    for value in ["a", "b", "a"] {
        let value = JavaLangString::from_rust_string(&jvm, value).await?;
        let _: bool = jvm
            .invoke_virtual(&source, &source.class_definition().name(), "add", "(Ljava/lang/Object;)Z", (value,))
            .await?;
    }

    let index: i32 = jvm
        .invoke_virtual(
            &source,
            &source.class_definition().name(),
            "lastIndexOf",
            "(Ljava/lang/Object;)I",
            (JavaLangString::from_rust_string(&jvm, "a").await?,),
        )
        .await?;
    assert_eq!(index, 2);

    let inserted = jvm.new_class("java/util/ArrayList", "()V", ()).await?;
    let _: bool = jvm
        .invoke_virtual(
            &inserted,
            &inserted.class_definition().name(),
            "add",
            "(Ljava/lang/Object;)Z",
            (JavaLangString::from_rust_string(&jvm, "x").await?,),
        )
        .await?;
    let changed: bool = jvm
        .invoke_virtual(
            &source,
            &source.class_definition().name(),
            "addAll",
            "(ILjava/util/Collection;)Z",
            (1, inserted),
        )
        .await?;
    assert!(changed);
    let value: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&source, &source.class_definition().name(), "get", "(I)Ljava/lang/Object;", (1,))
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &value).await?, "x");

    for (class_name, descriptor) in [
        ("java/util/ArrayList", "(Ljava/util/Collection;)V"),
        ("java/util/Vector", "(Ljava/util/Collection;)V"),
        ("java/util/HashSet", "(Ljava/util/Collection;)V"),
    ] {
        let copy = jvm.new_class(class_name, descriptor, (source.clone(),)).await?;
        assert_eq!(
            jvm.invoke_virtual::<_, i32>(&copy, &copy.class_definition().name(), "size", "()I", ())
                .await?,
            if class_name.ends_with("HashSet") { 3 } else { 4 }
        );
        assert!(
            jvm.invoke_virtual::<_, bool>(
                &copy,
                &copy.class_definition().name(),
                "contains",
                "(Ljava/lang/Object;)Z",
                (JavaLangString::from_rust_string(&jvm, "x").await?,)
            )
            .await?
        );
    }

    let null_collection: ClassInstanceRef<Object> = None.into();
    for class_name in ["java/util/ArrayList", "java/util/Vector", "java/util/HashSet"] {
        let result = jvm.new_class(class_name, "(Ljava/util/Collection;)V", (null_collection.clone(),)).await;
        let Err(JavaError::JavaException(exception)) = result else {
            panic!("{class_name} copy constructor must reject null");
        };
        assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));
    }

    Ok(())
}

#[tokio::test]
async fn col_10_11_map_copy_and_put_all_validate_before_mutation() -> Result<()> {
    let jvm = test_jvm().await?;
    let source = jvm.new_class("java/util/HashMap", "()V", ()).await?;
    for (key, value) in [("one", "1"), ("two", "2")] {
        let _: ClassInstanceRef<Object> = jvm
            .invoke_virtual(
                &source,
                &source.class_definition().name(),
                "put",
                "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
                (
                    JavaLangString::from_rust_string(&jvm, key).await?,
                    JavaLangString::from_rust_string(&jvm, value).await?,
                ),
            )
            .await?;
    }

    let hash_map = jvm.new_class("java/util/HashMap", "(Ljava/util/Map;)V", (source.clone(),)).await?;
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&hash_map, &hash_map.class_definition().name(), "size", "()I", ())
            .await?,
        2
    );
    let _: () = jvm
        .invoke_virtual(
            &hash_map,
            &hash_map.class_definition().name(),
            "putAll",
            "(Ljava/util/Map;)V",
            (hash_map.clone(),),
        )
        .await?;
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&hash_map, &hash_map.class_definition().name(), "size", "()I", ())
            .await?,
        2
    );

    let hashtable = jvm.new_class("java/util/Hashtable", "(Ljava/util/Map;)V", (source,)).await?;
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&hashtable, &hashtable.class_definition().name(), "size", "()I", ())
            .await?,
        2
    );
    let _: () = jvm
        .invoke_virtual(
            &hashtable,
            &hashtable.class_definition().name(),
            "putAll",
            "(Ljava/util/Map;)V",
            (hashtable.clone(),),
        )
        .await?;
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&hashtable, &hashtable.class_definition().name(), "size", "()I", ())
            .await?,
        2
    );

    let invalid = jvm.new_class("java/util/HashMap", "()V", ()).await?;
    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &invalid,
            &invalid.class_definition().name(),
            "put",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            (
                JavaLangString::from_rust_string(&jvm, "valid").await?,
                JavaLangString::from_rust_string(&jvm, "value").await?,
            ),
        )
        .await?;
    let null_value: ClassInstanceRef<Object> = None.into();
    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &invalid,
            &invalid.class_definition().name(),
            "put",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            (JavaLangString::from_rust_string(&jvm, "invalid").await?, null_value),
        )
        .await?;

    let target = jvm.new_class("java/util/Hashtable", "()V", ()).await?;
    let original_key = JavaLangString::from_rust_string(&jvm, "original").await?;
    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &target,
            &target.class_definition().name(),
            "put",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            (original_key.clone(), JavaLangString::from_rust_string(&jvm, "kept").await?),
        )
        .await?;
    let result: Result<()> = jvm
        .invoke_virtual(
            &target,
            &target.class_definition().name(),
            "putAll",
            "(Ljava/util/Map;)V",
            (invalid.clone(),),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("Hashtable.putAll must reject null values");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&target, &target.class_definition().name(), "size", "()I", ())
            .await?,
        1
    );
    let original: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &target,
            &target.class_definition().name(),
            "get",
            "(Ljava/lang/Object;)Ljava/lang/Object;",
            (original_key,),
        )
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &original).await?, "kept");

    let result = jvm.new_class("java/util/Hashtable", "(Ljava/util/Map;)V", (invalid,)).await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("Hashtable copy constructor must reject null values");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));

    Ok(())
}

#[tokio::test]
async fn col_12_typed_to_array_reuses_grows_terminates_and_preserves_on_ase() -> Result<()> {
    let jvm = test_jvm().await?;
    let list = jvm.new_class("java/util/ArrayList", "()V", ()).await?;
    for value in ["first", "second"] {
        let _: bool = jvm
            .invoke_virtual(
                &list,
                &list.class_definition().name(),
                "add",
                "(Ljava/lang/Object;)Z",
                (JavaLangString::from_rust_string(&jvm, value).await?,),
            )
            .await?;
    }

    let sentinel = JavaLangString::from_rust_string(&jvm, "sentinel").await?;
    let mut oversized: ClassInstanceRef<Array<Object>> = jvm.instantiate_array("Ljava/lang/String;", 4).await?.into();
    jvm.store_array(
        &mut oversized,
        0,
        [sentinel.clone(), sentinel.clone(), sentinel.clone(), sentinel.clone()],
    )
    .await?;
    let oversized_identity = oversized.identity();
    let reused: ClassInstanceRef<Array<Object>> = jvm
        .invoke_virtual(
            &list,
            &list.class_definition().name(),
            "toArray",
            "([Ljava/lang/Object;)[Ljava/lang/Object;",
            (oversized,),
        )
        .await?;
    assert_eq!(reused.identity(), oversized_identity);
    assert_eq!(reused.class_definition().name(), "[Ljava/lang/String;");
    let values: Vec<ClassInstanceRef<Object>> = jvm.load_array(&reused, 0, 4).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &values[0]).await?, "first");
    assert_eq!(JavaLangString::to_rust_string(&jvm, &values[1]).await?, "second");
    assert!(values[2].is_null());
    assert_eq!(JavaLangString::to_rust_string(&jvm, &values[3]).await?, "sentinel");

    let small: ClassInstanceRef<Array<Object>> = jvm.instantiate_array("Ljava/lang/String;", 1).await?.into();
    let small_identity = small.identity();
    let grown: ClassInstanceRef<Array<Object>> = jvm
        .invoke_virtual(
            &list,
            &list.class_definition().name(),
            "toArray",
            "([Ljava/lang/Object;)[Ljava/lang/Object;",
            (small,),
        )
        .await?;
    assert_ne!(grown.identity(), small_identity);
    assert_eq!(grown.class_definition().name(), "[Ljava/lang/String;");
    assert_eq!(jvm.array_length(&grown).await?, 2);

    for map_class in ["java/util/HashMap", "java/util/Hashtable"] {
        let map = jvm.new_class(map_class, "()V", ()).await?;
        let key = JavaLangString::from_rust_string(&jvm, "key").await?;
        let value = JavaLangString::from_rust_string(&jvm, "value").await?;
        let _: ClassInstanceRef<Object> = jvm
            .invoke_virtual(
                &map,
                &map.class_definition().name(),
                "put",
                "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
                (key.clone(), value.clone()),
            )
            .await?;

        for (view_method, view_descriptor, component_descriptor, expected_class) in [
            ("keySet", "()Ljava/util/Set;", "Ljava/lang/String;", "[Ljava/lang/String;"),
            ("values", "()Ljava/util/Collection;", "Ljava/lang/String;", "[Ljava/lang/String;"),
            ("entrySet", "()Ljava/util/Set;", "Ljava/util/Map$Entry;", "[Ljava/util/Map$Entry;"),
        ] {
            let view: ClassInstanceRef<Object> = jvm
                .invoke_virtual(&map, &map.class_definition().name(), view_method, view_descriptor, ())
                .await?;
            let destination: ClassInstanceRef<Array<Object>> = jvm.instantiate_array(component_descriptor, 0).await?.into();
            let typed: ClassInstanceRef<Array<Object>> = jvm
                .invoke_virtual(
                    &view,
                    &view.class_definition().name(),
                    "toArray",
                    "([Ljava/lang/Object;)[Ljava/lang/Object;",
                    (destination,),
                )
                .await?;
            assert_eq!(typed.class_definition().name(), expected_class);
            assert_eq!(jvm.array_length(&typed).await?, 1);
        }

        let values: ClassInstanceRef<Object> = jvm
            .invoke_virtual(&map, &map.class_definition().name(), "values", "()Ljava/util/Collection;", ())
            .await?;
        let removal = jvm.new_class("java/util/ArrayList", "()V", ()).await?;
        let _: bool = jvm
            .invoke_virtual(&removal, &removal.class_definition().name(), "add", "(Ljava/lang/Object;)Z", (value,))
            .await?;
        let changed: bool = jvm
            .invoke_virtual(
                &values,
                &values.class_definition().name(),
                "removeAll",
                "(Ljava/util/Collection;)Z",
                (removal,),
            )
            .await?;
        assert!(changed);
        assert_eq!(
            jvm.invoke_virtual::<_, i32>(&map, &map.class_definition().name(), "size", "()I", ())
                .await?,
            0
        );

        let _: ClassInstanceRef<Object> = jvm
            .invoke_virtual(
                &map,
                &map.class_definition().name(),
                "put",
                "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
                (key, JavaLangString::from_rust_string(&jvm, "again").await?),
            )
            .await?;
        let entries: ClassInstanceRef<Object> = jvm
            .invoke_virtual(&map, &map.class_definition().name(), "entrySet", "()Ljava/util/Set;", ())
            .await?;
        let changed: bool = jvm
            .invoke_virtual(
                &entries,
                &entries.class_definition().name(),
                "removeAll",
                "(Ljava/util/Collection;)Z",
                (entries.clone(),),
            )
            .await?;
        assert!(changed);
        assert_eq!(
            jvm.invoke_virtual::<_, i32>(&map, &map.class_definition().name(), "size", "()I", ())
                .await?,
            0
        );
    }

    let incompatible = jvm.new_class("java/util/ArrayList", "()V", ()).await?;
    let _: bool = jvm
        .invoke_virtual(
            &incompatible,
            &incompatible.class_definition().name(),
            "add",
            "(Ljava/lang/Object;)Z",
            (JavaLangString::from_rust_string(&jvm, "compatible").await?,),
        )
        .await?;
    let _: bool = jvm
        .invoke_virtual(
            &incompatible,
            &incompatible.class_definition().name(),
            "add",
            "(Ljava/lang/Object;)Z",
            (jvm.new_class("java/lang/Object", "()V", ()).await?,),
        )
        .await?;
    let before_first = JavaLangString::from_rust_string(&jvm, "before-first").await?;
    let before_second = JavaLangString::from_rust_string(&jvm, "before-second").await?;
    let mut destination: ClassInstanceRef<Array<Object>> = jvm.instantiate_array("Ljava/lang/String;", 2).await?.into();
    jvm.store_array(&mut destination, 0, [before_first, before_second]).await?;
    let result: Result<ClassInstanceRef<Array<Object>>> = jvm
        .invoke_virtual(
            &incompatible,
            &incompatible.class_definition().name(),
            "toArray",
            "([Ljava/lang/Object;)[Ljava/lang/Object;",
            (destination.clone(),),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("typed toArray must reject incompatible elements");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/ArrayStoreException"));
    let partially_written: Vec<ClassInstanceRef<Object>> = jvm.load_array(&destination, 0, 2).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &partially_written[0]).await?, "compatible");
    assert_eq!(JavaLangString::to_rust_string(&jvm, &partially_written[1]).await?, "before-second");

    let null_array: ClassInstanceRef<Array<Object>> = None.into();
    let result: Result<ClassInstanceRef<Array<Object>>> = jvm
        .invoke_virtual(
            &list,
            &list.class_definition().name(),
            "toArray",
            "([Ljava/lang/Object;)[Ljava/lang/Object;",
            (null_array,),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("typed toArray must reject null");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));

    Ok(())
}

#[tokio::test]
async fn col_07_08_copy_constructors_preserve_custom_snapshot_order_duplicates_and_nulls() -> Result<()> {
    let jvm = collection_contract_fixture_jvm().await?;
    let beta: ClassInstanceRef<Object> = JavaLangString::from_rust_string(&jvm, "beta").await?.into();
    let alpha: ClassInstanceRef<Object> = JavaLangString::from_rust_string(&jvm, "alpha").await?.into();
    let null: ClassInstanceRef<Object> = None.into();
    let mut elements: ClassInstanceRef<Array<Object>> = jvm.instantiate_array("Ljava/lang/Object;", 5).await?.into();
    jvm.store_array(&mut elements, 0, [beta.clone(), null.clone(), alpha.clone(), beta.clone(), null])
        .await?;
    let source = jvm.new_class("SnapshotCollection", "([Ljava/lang/Object;)V", (elements,)).await?;

    for class_name in ["java/util/ArrayList", "java/util/Vector"] {
        let copy = jvm.new_class(class_name, "(Ljava/util/Collection;)V", (source.clone(),)).await?;
        assert_eq!(
            jvm.invoke_virtual::<_, i32>(&copy, &copy.class_definition().name(), "size", "()I", ())
                .await?,
            5
        );
        for (index, expected) in [Some("beta"), None, Some("alpha"), Some("beta"), None].into_iter().enumerate() {
            let value: ClassInstanceRef<Object> = jvm
                .invoke_virtual(&copy, &copy.class_definition().name(), "get", "(I)Ljava/lang/Object;", (index as i32,))
                .await?;
            match expected {
                Some(expected) => assert_eq!(JavaLangString::to_rust_string(&jvm, &value).await?, expected),
                None => assert!(value.is_null()),
            }
        }
    }

    let set = jvm.new_class("java/util/HashSet", "(Ljava/util/Collection;)V", (source,)).await?;
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&set, &set.class_definition().name(), "size", "()I", ())
            .await?,
        3
    );
    assert!(
        jvm.invoke_virtual::<_, bool>(&set, &set.class_definition().name(), "contains", "(Ljava/lang/Object;)Z", (beta,))
            .await?
    );
    assert!(
        jvm.invoke_virtual::<_, bool>(&set, &set.class_definition().name(), "contains", "(Ljava/lang/Object;)Z", (alpha,))
            .await?
    );
    let null: ClassInstanceRef<Object> = None.into();
    assert!(
        jvm.invoke_virtual::<_, bool>(&set, &set.class_definition().name(), "contains", "(Ljava/lang/Object;)Z", (null,))
            .await?
    );

    Ok(())
}

#[tokio::test]
async fn col_10_11_put_all_preserves_equals_direction_and_hashtable_prevalidates_both_null_forms() -> Result<()> {
    let jvm = collection_contract_fixture_jvm().await?;
    let stored_key: ClassInstanceRef<Object> = jvm.new_class("DirectionalStoredKey", "()V", ()).await?.into();
    let query_key: ClassInstanceRef<Object> = jvm.new_class("DirectionalQueryKey", "()V", ()).await?.into();
    let target = jvm.new_class("java/util/HashMap", "()V", ()).await?;
    let source = jvm.new_class("java/util/HashMap", "()V", ()).await?;
    let old_value = JavaLangString::from_rust_string(&jvm, "old").await?;
    let new_value = JavaLangString::from_rust_string(&jvm, "new").await?;
    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &target,
            &target.class_definition().name(),
            "put",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            (stored_key.clone(), old_value),
        )
        .await?;
    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &source,
            &source.class_definition().name(),
            "put",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            (query_key.clone(), new_value.clone()),
        )
        .await?;

    let _: () = jvm
        .invoke_virtual(
            &target,
            &target.class_definition().name(),
            "putAll",
            "(Ljava/util/Map;)V",
            (source.clone(),),
        )
        .await?;
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&target, &target.class_definition().name(), "size", "()I", ())
            .await?,
        1
    );
    let replaced: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &target,
            &target.class_definition().name(),
            "get",
            "(Ljava/lang/Object;)Ljava/lang/Object;",
            (query_key.clone(),),
        )
        .await?;
    assert_eq!(replaced.identity(), new_value.identity());
    let target_entries: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&target, &target.class_definition().name(), "entrySet", "()Ljava/util/Set;", ())
        .await?;
    let target_entries: ClassInstanceRef<Array<Object>> = jvm
        .invoke_virtual(
            &target_entries,
            &target_entries.class_definition().name(),
            "toArray",
            "()[Ljava/lang/Object;",
            (),
        )
        .await?;
    let target_entry: ClassInstanceRef<Object> = jvm.load_array(&target_entries, 0, 1).await?.pop().unwrap();
    let retained_key: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &target_entry,
            &target_entry.class_definition().name(),
            "getKey",
            "()Ljava/lang/Object;",
            (),
        )
        .await?;
    assert_eq!(retained_key.identity(), stored_key.identity());

    let copy = jvm.new_class("java/util/HashMap", "(Ljava/util/Map;)V", (source,)).await?;
    let copy_entries: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&copy, &copy.class_definition().name(), "entrySet", "()Ljava/util/Set;", ())
        .await?;
    let copy_entries: ClassInstanceRef<Array<Object>> = jvm
        .invoke_virtual(
            &copy_entries,
            &copy_entries.class_definition().name(),
            "toArray",
            "()[Ljava/lang/Object;",
            (),
        )
        .await?;
    let copy_entry: ClassInstanceRef<Object> = jvm.load_array(&copy_entries, 0, 1).await?.pop().unwrap();
    let copied_key: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&copy_entry, &copy_entry.class_definition().name(), "getKey", "()Ljava/lang/Object;", ())
        .await?;
    assert_eq!(copied_key.identity(), query_key.identity());

    let hashtable = jvm.new_class("java/util/Hashtable", "()V", ()).await?;
    let original_key = JavaLangString::from_rust_string(&jvm, "original-key").await?;
    let original_value = JavaLangString::from_rust_string(&jvm, "original-value").await?;
    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &hashtable,
            &hashtable.class_definition().name(),
            "put",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            (original_key.clone(), original_value.clone()),
        )
        .await?;

    let null_key_source = jvm.new_class("java/util/HashMap", "()V", ()).await?;
    let null_key: ClassInstanceRef<Object> = None.into();
    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &null_key_source,
            &null_key_source.class_definition().name(),
            "put",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            (null_key, JavaLangString::from_rust_string(&jvm, "invalid-null-key").await?),
        )
        .await?;
    let valid_after_null_key: ClassInstanceRef<Object> = jvm.new_class("DirectionalStoredKey", "()V", ()).await?.into();
    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &null_key_source,
            &null_key_source.class_definition().name(),
            "put",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            (
                valid_after_null_key.clone(),
                JavaLangString::from_rust_string(&jvm, "valid-before-null-key").await?,
            ),
        )
        .await?;
    let result: Result<()> = jvm
        .invoke_virtual(
            &hashtable,
            &hashtable.class_definition().name(),
            "putAll",
            "(Ljava/util/Map;)V",
            (null_key_source,),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("Hashtable.putAll must reject a null key");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&hashtable, &hashtable.class_definition().name(), "size", "()I", ())
            .await?,
        1
    );
    assert!(
        !jvm.invoke_virtual::<_, bool>(
            &hashtable,
            &hashtable.class_definition().name(),
            "containsKey",
            "(Ljava/lang/Object;)Z",
            (valid_after_null_key,)
        )
        .await?
    );

    let null_value_source = jvm.new_class("java/util/HashMap", "()V", ()).await?;
    let invalid_value_key: ClassInstanceRef<Object> = jvm.new_class("DirectionalQueryKey", "()V", ()).await?.into();
    let null_value: ClassInstanceRef<Object> = None.into();
    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &null_value_source,
            &null_value_source.class_definition().name(),
            "put",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            (invalid_value_key, null_value),
        )
        .await?;
    let valid_after_null_value: ClassInstanceRef<Object> = jvm.new_class("DirectionalStoredKey", "()V", ()).await?.into();
    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &null_value_source,
            &null_value_source.class_definition().name(),
            "put",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            (
                valid_after_null_value.clone(),
                JavaLangString::from_rust_string(&jvm, "valid-before-null-value").await?,
            ),
        )
        .await?;
    let result: Result<()> = jvm
        .invoke_virtual(
            &hashtable,
            &hashtable.class_definition().name(),
            "putAll",
            "(Ljava/util/Map;)V",
            (null_value_source,),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("Hashtable.putAll must reject a null value");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&hashtable, &hashtable.class_definition().name(), "size", "()I", ())
            .await?,
        1
    );
    assert!(
        !jvm.invoke_virtual::<_, bool>(
            &hashtable,
            &hashtable.class_definition().name(),
            "containsKey",
            "(Ljava/lang/Object;)Z",
            (valid_after_null_value,)
        )
        .await?
    );
    let preserved: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &hashtable,
            &hashtable.class_definition().name(),
            "get",
            "(Ljava/lang/Object;)Ljava/lang/Object;",
            (original_key,),
        )
        .await?;
    assert_eq!(preserved.identity(), original_value.identity());

    let null_map: ClassInstanceRef<Object> = None.into();
    for map in [&target, &hashtable] {
        let result: Result<()> = jvm
            .invoke_virtual(map, &map.class_definition().name(), "putAll", "(Ljava/util/Map;)V", (null_map.clone(),))
            .await;
        let Err(JavaError::JavaException(exception)) = result else {
            panic!("putAll(null) must throw NullPointerException");
        };
        assert!(jvm.is_instance(&*exception, "java/lang/NullPointerException"));
    }
    let target_size: i32 = jvm.invoke_virtual(&target, &target.class_definition().name(), "size", "()I", ()).await?;
    let _: () = jvm
        .invoke_virtual(
            &target,
            &target.class_definition().name(),
            "putAll",
            "(Ljava/util/Map;)V",
            (target.clone(),),
        )
        .await?;
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&target, &target.class_definition().name(), "size", "()I", ())
            .await?,
        target_size
    );
    let hashtable_size: i32 = jvm
        .invoke_virtual(&hashtable, &hashtable.class_definition().name(), "size", "()I", ())
        .await?;
    let _: () = jvm
        .invoke_virtual(
            &hashtable,
            &hashtable.class_definition().name(),
            "putAll",
            "(Ljava/util/Map;)V",
            (hashtable.clone(),),
        )
        .await?;
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&hashtable, &hashtable.class_definition().name(), "size", "()I", ())
            .await?,
        hashtable_size
    );

    Ok(())
}

#[tokio::test]
async fn col_12_typed_to_array_preserves_multidimensional_components_and_jdk_ase_timing() -> Result<()> {
    let jvm = test_jvm().await?;
    let strings = jvm.new_class("java/util/ArrayList", "()V", ()).await?;
    let string_row_1: ClassInstanceRef<Array<Object>> = jvm.instantiate_array("Ljava/lang/String;", 1).await?.into();
    let string_row_2: ClassInstanceRef<Array<Object>> = jvm.instantiate_array("Ljava/lang/String;", 2).await?.into();
    for row in [string_row_1.clone(), string_row_2.clone()] {
        let _: bool = jvm
            .invoke_virtual(&strings, &strings.class_definition().name(), "add", "(Ljava/lang/Object;)Z", (row,))
            .await?;
    }

    let small: ClassInstanceRef<Array<Object>> = jvm.instantiate_array("[Ljava/lang/String;", 0).await?.into();
    let grown: ClassInstanceRef<Array<Object>> = jvm
        .invoke_virtual(
            &strings,
            &strings.class_definition().name(),
            "toArray",
            "([Ljava/lang/Object;)[Ljava/lang/Object;",
            (small,),
        )
        .await?;
    assert_eq!(grown.class_definition().name(), "[[Ljava/lang/String;");
    let grown_rows: Vec<ClassInstanceRef<Object>> = jvm.load_array(&grown, 0, 2).await?;
    assert_eq!(grown_rows[0].identity(), string_row_1.identity());
    assert_eq!(grown_rows[1].identity(), string_row_2.identity());

    let object_matrix: ClassInstanceRef<Array<Object>> = jvm.instantiate_array("[Ljava/lang/Object;", 0).await?.into();
    let covariant: ClassInstanceRef<Array<Object>> = jvm
        .invoke_virtual(
            &strings,
            &strings.class_definition().name(),
            "toArray",
            "([Ljava/lang/Object;)[Ljava/lang/Object;",
            (object_matrix,),
        )
        .await?;
    assert_eq!(covariant.class_definition().name(), "[[Ljava/lang/Object;");

    let ints = jvm.new_class("java/util/ArrayList", "()V", ()).await?;
    let int_row_1: ClassInstanceRef<Array<i32>> = jvm.instantiate_array("I", 1).await?.into();
    let int_row_2: ClassInstanceRef<Array<i32>> = jvm.instantiate_array("I", 2).await?.into();
    for row in [int_row_1.clone(), int_row_2.clone()] {
        let _: bool = jvm
            .invoke_virtual(&ints, &ints.class_definition().name(), "add", "(Ljava/lang/Object;)Z", (row,))
            .await?;
    }
    let sentinel_int_row: ClassInstanceRef<Array<i32>> = jvm.instantiate_array("I", 3).await?.into();
    let mut int_matrix: ClassInstanceRef<Array<Object>> = jvm.instantiate_array("[I", 3).await?.into();
    jvm.store_array(
        &mut int_matrix,
        0,
        [sentinel_int_row.clone(), sentinel_int_row.clone(), sentinel_int_row.clone()],
    )
    .await?;
    let int_matrix_identity = int_matrix.identity();
    let reused: ClassInstanceRef<Array<Object>> = jvm
        .invoke_virtual(
            &ints,
            &ints.class_definition().name(),
            "toArray",
            "([Ljava/lang/Object;)[Ljava/lang/Object;",
            (int_matrix,),
        )
        .await?;
    assert_eq!(reused.identity(), int_matrix_identity);
    assert_eq!(reused.class_definition().name(), "[[I");
    let reused_rows: Vec<ClassInstanceRef<Object>> = jvm.load_array(&reused, 0, 3).await?;
    assert_eq!(reused_rows[0].identity(), int_row_1.identity());
    assert_eq!(reused_rows[1].identity(), int_row_2.identity());
    assert!(reused_rows[2].is_null());

    let incompatible = jvm.new_class("java/util/ArrayList", "()V", ()).await?;
    let object_row: ClassInstanceRef<Array<Object>> = jvm.instantiate_array("Ljava/lang/Object;", 1).await?.into();
    let _: bool = jvm
        .invoke_virtual(
            &incompatible,
            &incompatible.class_definition().name(),
            "add",
            "(Ljava/lang/Object;)Z",
            (string_row_1.clone(),),
        )
        .await?;
    let _: bool = jvm
        .invoke_virtual(
            &incompatible,
            &incompatible.class_definition().name(),
            "add",
            "(Ljava/lang/Object;)Z",
            (object_row,),
        )
        .await?;
    let sentinel_string_row: ClassInstanceRef<Array<Object>> = jvm.instantiate_array("Ljava/lang/String;", 4).await?.into();
    let mut destination: ClassInstanceRef<Array<Object>> = jvm.instantiate_array("[Ljava/lang/String;", 3).await?.into();
    jvm.store_array(
        &mut destination,
        0,
        [sentinel_string_row.clone(), sentinel_string_row.clone(), sentinel_string_row.clone()],
    )
    .await?;
    let result: Result<ClassInstanceRef<Array<Object>>> = jvm
        .invoke_virtual(
            &incompatible,
            &incompatible.class_definition().name(),
            "toArray",
            "([Ljava/lang/Object;)[Ljava/lang/Object;",
            (destination.clone(),),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("String[][] must reject an Object[] element");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/ArrayStoreException"));
    let partial: Vec<ClassInstanceRef<Object>> = jvm.load_array(&destination, 0, 3).await?;
    assert_eq!(partial[0].identity(), string_row_1.identity());
    assert_eq!(partial[1].identity(), sentinel_string_row.identity());
    assert_eq!(partial[2].identity(), sentinel_string_row.identity());

    Ok(())
}

#[tokio::test]
async fn col_01_bulk_unsupported_mutations_respect_empty_and_pre_mutation_state() -> Result<()> {
    let jvm = collection_contract_fixture_jvm().await?;
    let first = JavaLangString::from_rust_string(&jvm, "first").await?;
    let second = JavaLangString::from_rust_string(&jvm, "second").await?;
    let mut elements: ClassInstanceRef<Array<Object>> = jvm.instantiate_array("Ljava/lang/Object;", 2).await?.into();
    jvm.store_array(&mut elements, 0, [first.clone(), second.clone()]).await?;
    let immutable = jvm.new_class("SnapshotCollection", "([Ljava/lang/Object;)V", (elements,)).await?;
    let empty_elements: ClassInstanceRef<Array<Object>> = jvm.instantiate_array("Ljava/lang/Object;", 0).await?.into();
    let empty = jvm.new_class("SnapshotCollection", "([Ljava/lang/Object;)V", (empty_elements,)).await?;

    for (name, descriptor) in [("addAll", "(Ljava/util/Collection;)Z"), ("removeAll", "(Ljava/util/Collection;)Z")] {
        let changed: bool = jvm
            .invoke_virtual(&immutable, &immutable.class_definition().name(), name, descriptor, (empty.clone(),))
            .await?;
        assert!(!changed);
    }
    let retained: bool = jvm
        .invoke_virtual(
            &immutable,
            &immutable.class_definition().name(),
            "retainAll",
            "(Ljava/util/Collection;)Z",
            (immutable.clone(),),
        )
        .await?;
    assert!(!retained);

    for (name, argument) in [
        ("addAll", immutable.clone()),
        ("removeAll", immutable.clone()),
        ("retainAll", empty.clone()),
    ] {
        let result: Result<bool> = jvm
            .invoke_virtual(
                &immutable,
                &immutable.class_definition().name(),
                name,
                "(Ljava/util/Collection;)Z",
                (argument,),
            )
            .await;
        let Err(JavaError::JavaException(exception)) = result else {
            panic!("{name} must reject an unsupported mutation");
        };
        assert!(jvm.is_instance(&*exception, "java/lang/UnsupportedOperationException"));
        assert_eq!(
            jvm.invoke_virtual::<_, i32>(&immutable, &immutable.class_definition().name(), "size", "()I", ())
                .await?,
            2
        );
        assert!(
            jvm.invoke_virtual::<_, bool>(
                &immutable,
                &immutable.class_definition().name(),
                "contains",
                "(Ljava/lang/Object;)Z",
                (first.clone(),)
            )
            .await?
        );
        assert!(
            jvm.invoke_virtual::<_, bool>(
                &immutable,
                &immutable.class_definition().name(),
                "contains",
                "(Ljava/lang/Object;)Z",
                (second.clone(),)
            )
            .await?
        );
    }

    let map = jvm.new_class("java/util/HashMap", "()V", ()).await?;
    let map_key_1 = JavaLangString::from_rust_string(&jvm, "map-key-1").await?;
    let map_key_2 = JavaLangString::from_rust_string(&jvm, "map-key-2").await?;
    let map_value = JavaLangString::from_rust_string(&jvm, "map-value").await?;
    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &map,
            &map.class_definition().name(),
            "put",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            (map_key_1.clone(), map_value.clone()),
        )
        .await?;
    let keys: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&map, &map.class_definition().name(), "keySet", "()Ljava/util/Set;", ())
        .await?;
    let snapshot_iterator: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&keys, &keys.class_definition().name(), "iterator", "()Ljava/util/Iterator;", ())
        .await?;
    let empty_add: bool = jvm
        .invoke_virtual(
            &keys,
            &keys.class_definition().name(),
            "addAll",
            "(Ljava/util/Collection;)Z",
            (empty.clone(),),
        )
        .await?;
    assert!(!empty_add);
    let result: Result<bool> = jvm
        .invoke_virtual(
            &keys,
            &keys.class_definition().name(),
            "addAll",
            "(Ljava/util/Collection;)Z",
            (immutable.clone(),),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("keySet.addAll(nonEmpty) must throw UnsupportedOperationException");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/UnsupportedOperationException"));
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&map, &map.class_definition().name(), "size", "()I", ())
            .await?,
        1
    );

    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &map,
            &map.class_definition().name(),
            "put",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            (map_key_2.clone(), map_value),
        )
        .await?;
    let target = jvm.new_class("java/util/ArrayList", "()V", ()).await?;
    let _: bool = jvm
        .invoke_virtual(&target, &target.class_definition().name(), "addAll", "(Ljava/util/Collection;)Z", (keys,))
        .await?;
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&target, &target.class_definition().name(), "size", "()I", ())
            .await?,
        2
    );
    let snapshot_first: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &snapshot_iterator,
            &snapshot_iterator.class_definition().name(),
            "next",
            "()Ljava/lang/Object;",
            (),
        )
        .await?;
    assert_eq!(snapshot_first.identity(), map_key_1.identity());
    assert!(
        !jvm.invoke_virtual::<_, bool>(&snapshot_iterator, &snapshot_iterator.class_definition().name(), "hasNext", "()Z", ())
            .await?
    );
    assert!(
        jvm.invoke_virtual::<_, bool>(
            &target,
            &target.class_definition().name(),
            "contains",
            "(Ljava/lang/Object;)Z",
            (map_key_1,)
        )
        .await?
    );
    assert!(
        jvm.invoke_virtual::<_, bool>(
            &target,
            &target.class_definition().name(),
            "contains",
            "(Ljava/lang/Object;)Z",
            (map_key_2,)
        )
        .await?
    );

    Ok(())
}

#[tokio::test]
async fn coll_08_entry_set_contains_and_remove_snapshot_candidates_and_use_stored_value_equals() -> Result<()> {
    let jvm = collection_contract_fixture_jvm().await?;
    let null: ClassInstanceRef<Object> = None.into();

    let asymmetric_key_map = jvm.new_class("java/util/HashMap", "()V", ()).await?;
    let stored_key: ClassInstanceRef<Object> = jvm.new_class("DirectionalStoredKey", "()V", ()).await?.into();
    let query_key: ClassInstanceRef<Object> = jvm.new_class("DirectionalQueryKey", "()V", ()).await?.into();
    let stored_key_value = JavaLangString::from_rust_string(&jvm, "stored-key-value").await?;
    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &asymmetric_key_map,
            &asymmetric_key_map.class_definition().name(),
            "put",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            (stored_key.clone(), stored_key_value.clone()),
        )
        .await?;
    assert!(
        jvm.invoke_virtual::<_, bool>(
            &asymmetric_key_map,
            &asymmetric_key_map.class_definition().name(),
            "containsKey",
            "(Ljava/lang/Object;)Z",
            (query_key.clone(),),
        )
        .await?,
        "the query key must find the stored key before EntrySet applies stored-key equality"
    );
    let asymmetric_entries: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &asymmetric_key_map,
            &asymmetric_key_map.class_definition().name(),
            "entrySet",
            "()Ljava/util/Set;",
            (),
        )
        .await?;
    let asymmetric_contains: ClassInstanceRef<Object> = jvm
        .new_class(
            "StatefulMapEntry",
            "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;I)V",
            (query_key.clone(), query_key.clone(), null.clone(), 2),
        )
        .await?
        .into();
    assert!(
        !jvm.invoke_virtual::<_, bool>(
            &asymmetric_entries,
            &asymmetric_entries.class_definition().name(),
            "contains",
            "(Ljava/lang/Object;)Z",
            (asymmetric_contains.clone(),),
        )
        .await?
    );
    assert_eq!(jvm.get_field::<i32>(&asymmetric_contains, "keyCalls", "I").await?, 1);
    assert_eq!(jvm.get_field::<i32>(&asymmetric_contains, "valueCalls", "I").await?, 0);
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&asymmetric_key_map, &asymmetric_key_map.class_definition().name(), "size", "()I", ())
            .await?,
        1
    );

    let asymmetric_remove: ClassInstanceRef<Object> = jvm
        .new_class(
            "StatefulMapEntry",
            "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;I)V",
            (query_key.clone(), query_key, null.clone(), 2),
        )
        .await?
        .into();
    assert!(
        !jvm.invoke_virtual::<_, bool>(
            &asymmetric_entries,
            &asymmetric_entries.class_definition().name(),
            "remove",
            "(Ljava/lang/Object;)Z",
            (asymmetric_remove.clone(),),
        )
        .await?
    );
    assert_eq!(jvm.get_field::<i32>(&asymmetric_remove, "keyCalls", "I").await?, 1);
    assert_eq!(jvm.get_field::<i32>(&asymmetric_remove, "valueCalls", "I").await?, 0);
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&asymmetric_key_map, &asymmetric_key_map.class_definition().name(), "size", "()I", ())
            .await?,
        1
    );
    let preserved_iterator: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &asymmetric_entries,
            &asymmetric_entries.class_definition().name(),
            "iterator",
            "()Ljava/util/Iterator;",
            (),
        )
        .await?;
    let preserved_entry: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &preserved_iterator,
            &preserved_iterator.class_definition().name(),
            "next",
            "()Ljava/lang/Object;",
            (),
        )
        .await?;
    let preserved_key: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &preserved_entry,
            &preserved_entry.class_definition().name(),
            "getKey",
            "()Ljava/lang/Object;",
            (),
        )
        .await?;
    let preserved_value: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &preserved_entry,
            &preserved_entry.class_definition().name(),
            "getValue",
            "()Ljava/lang/Object;",
            (),
        )
        .await?;
    assert_eq!(preserved_key.identity(), stored_key.identity());
    assert_eq!(preserved_value.identity(), stored_key_value.identity());
    assert!(
        !jvm.invoke_virtual::<_, bool>(&preserved_iterator, &preserved_iterator.class_definition().name(), "hasNext", "()Z", ())
            .await?
    );

    let null_key_map = jvm.new_class("java/util/HashMap", "()V", ()).await?;
    let null_key_value = JavaLangString::from_rust_string(&jvm, "null-key-value").await?;
    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &null_key_map,
            &null_key_map.class_definition().name(),
            "put",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            (null.clone(), null_key_value.clone()),
        )
        .await?;
    let null_key_entries: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &null_key_map,
            &null_key_map.class_definition().name(),
            "entrySet",
            "()Ljava/util/Set;",
            (),
        )
        .await?;
    let null_key_contains: ClassInstanceRef<Object> = jvm
        .new_class(
            "StatefulMapEntry",
            "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;I)V",
            (null.clone(), null.clone(), null_key_value.clone(), 0),
        )
        .await?
        .into();
    assert!(
        jvm.invoke_virtual::<_, bool>(
            &null_key_entries,
            &null_key_entries.class_definition().name(),
            "contains",
            "(Ljava/lang/Object;)Z",
            (null_key_contains.clone(),),
        )
        .await?
    );
    assert_eq!(jvm.get_field::<i32>(&null_key_contains, "keyCalls", "I").await?, 1);
    assert_eq!(jvm.get_field::<i32>(&null_key_contains, "valueCalls", "I").await?, 1);
    let null_key_remove: ClassInstanceRef<Object> = jvm
        .new_class(
            "StatefulMapEntry",
            "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;I)V",
            (null.clone(), null.clone(), null_key_value, 0),
        )
        .await?
        .into();
    assert!(
        jvm.invoke_virtual::<_, bool>(
            &null_key_entries,
            &null_key_entries.class_definition().name(),
            "remove",
            "(Ljava/lang/Object;)Z",
            (null_key_remove.clone(),),
        )
        .await?
    );
    assert_eq!(jvm.get_field::<i32>(&null_key_remove, "keyCalls", "I").await?, 1);
    assert_eq!(jvm.get_field::<i32>(&null_key_remove, "valueCalls", "I").await?, 1);
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&null_key_map, &null_key_map.class_definition().name(), "size", "()I", ())
            .await?,
        0
    );

    for map_class in ["java/util/HashMap", "java/util/Hashtable"] {
        let map: ClassInstanceRef<Object> = jvm.new_class(map_class, "()V", ()).await?.into();
        let key = JavaLangString::from_rust_string(&jvm, "direction-key").await?;
        let stored_value: ClassInstanceRef<Object> = jvm.new_class("ConfigurableEqualsValue", "(Z)V", (true,)).await?.into();
        let candidate_value: ClassInstanceRef<Object> = jvm.new_class("ConfigurableEqualsValue", "(Z)V", (false,)).await?.into();
        let _: ClassInstanceRef<Object> = jvm
            .invoke_virtual(
                &map,
                &map.class_definition().name(),
                "put",
                "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
                (key.clone(), stored_value.clone()),
            )
            .await?;
        let entries: ClassInstanceRef<Object> = jvm
            .invoke_virtual(&map, &map.class_definition().name(), "entrySet", "()Ljava/util/Set;", ())
            .await?;
        let contains_candidate: ClassInstanceRef<Object> = jvm
            .new_class(
                "StatefulMapEntry",
                "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;I)V",
                (key.clone(), key.clone(), candidate_value.clone(), 0),
            )
            .await?
            .into();
        assert!(
            jvm.invoke_virtual::<_, bool>(
                &entries,
                &entries.class_definition().name(),
                "contains",
                "(Ljava/lang/Object;)Z",
                (contains_candidate.clone(),)
            )
            .await?,
            "{map_class} EntrySet.contains must use storedValue.equals(candidateValue)"
        );
        assert_eq!(jvm.get_field::<i32>(&contains_candidate, "keyCalls", "I").await?, 1);
        assert_eq!(jvm.get_field::<i32>(&contains_candidate, "valueCalls", "I").await?, 1);
        assert_eq!(jvm.get_field::<i32>(&stored_value, "equalsCalls", "I").await?, 1);
        assert_eq!(jvm.get_field::<i32>(&candidate_value, "equalsCalls", "I").await?, 0);

        let remove_candidate: ClassInstanceRef<Object> = jvm
            .new_class(
                "StatefulMapEntry",
                "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;I)V",
                (key.clone(), key, candidate_value.clone(), 0),
            )
            .await?
            .into();
        assert!(
            jvm.invoke_virtual::<_, bool>(
                &entries,
                &entries.class_definition().name(),
                "remove",
                "(Ljava/lang/Object;)Z",
                (remove_candidate.clone(),)
            )
            .await?,
            "{map_class} EntrySet.remove must use storedValue.equals(candidateValue)"
        );
        assert_eq!(jvm.get_field::<i32>(&remove_candidate, "keyCalls", "I").await?, 1);
        assert_eq!(jvm.get_field::<i32>(&remove_candidate, "valueCalls", "I").await?, 1);
        assert_eq!(jvm.get_field::<i32>(&stored_value, "equalsCalls", "I").await?, 2);
        assert_eq!(jvm.get_field::<i32>(&candidate_value, "equalsCalls", "I").await?, 0);
        assert_eq!(
            jvm.invoke_virtual::<_, i32>(&map, &map.class_definition().name(), "size", "()I", ())
                .await?,
            0
        );

        let opposite_map: ClassInstanceRef<Object> = jvm.new_class(map_class, "()V", ()).await?.into();
        let opposite_key = JavaLangString::from_rust_string(&jvm, "opposite-key").await?;
        let rejecting_stored: ClassInstanceRef<Object> = jvm.new_class("ConfigurableEqualsValue", "(Z)V", (false,)).await?.into();
        let accepting_candidate: ClassInstanceRef<Object> = jvm.new_class("ConfigurableEqualsValue", "(Z)V", (true,)).await?.into();
        let _: ClassInstanceRef<Object> = jvm
            .invoke_virtual(
                &opposite_map,
                &opposite_map.class_definition().name(),
                "put",
                "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
                (opposite_key.clone(), rejecting_stored.clone()),
            )
            .await?;
        let opposite_entries: ClassInstanceRef<Object> = jvm
            .invoke_virtual(
                &opposite_map,
                &opposite_map.class_definition().name(),
                "entrySet",
                "()Ljava/util/Set;",
                (),
            )
            .await?;
        let opposite_candidate: ClassInstanceRef<Object> = jvm
            .new_class(
                "StatefulMapEntry",
                "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;I)V",
                (opposite_key.clone(), opposite_key, accepting_candidate.clone(), 0),
            )
            .await?
            .into();
        assert!(
            !jvm.invoke_virtual::<_, bool>(
                &opposite_entries,
                &opposite_entries.class_definition().name(),
                "contains",
                "(Ljava/lang/Object;)Z",
                (opposite_candidate,),
            )
            .await?
        );
        assert_eq!(jvm.get_field::<i32>(&rejecting_stored, "equalsCalls", "I").await?, 1);
        assert_eq!(jvm.get_field::<i32>(&accepting_candidate, "equalsCalls", "I").await?, 0);

        let stateful_map: ClassInstanceRef<Object> = jvm.new_class(map_class, "()V", ()).await?.into();
        let first_key = JavaLangString::from_rust_string(&jvm, "first-key").await?;
        let later_key = JavaLangString::from_rust_string(&jvm, "later-key").await?;
        let first_value = JavaLangString::from_rust_string(&jvm, "first-value").await?;
        let later_value = JavaLangString::from_rust_string(&jvm, "later-value").await?;
        for (key, value) in [(first_key.clone(), first_value.clone()), (later_key.clone(), later_value.clone())] {
            let _: ClassInstanceRef<Object> = jvm
                .invoke_virtual(
                    &stateful_map,
                    &stateful_map.class_definition().name(),
                    "put",
                    "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
                    (key, value),
                )
                .await?;
        }
        let stateful_entries: ClassInstanceRef<Object> = jvm
            .invoke_virtual(
                &stateful_map,
                &stateful_map.class_definition().name(),
                "entrySet",
                "()Ljava/util/Set;",
                (),
            )
            .await?;
        let alternating: ClassInstanceRef<Object> = jvm
            .new_class(
                "StatefulMapEntry",
                "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;I)V",
                (
                    first_key.clone(),
                    later_key.clone(),
                    JavaLangString::from_rust_string(&jvm, "first-value").await?,
                    0,
                ),
            )
            .await?
            .into();
        assert!(
            jvm.invoke_virtual::<_, bool>(
                &stateful_entries,
                &stateful_entries.class_definition().name(),
                "remove",
                "(Ljava/lang/Object;)Z",
                (alternating.clone(),)
            )
            .await?
        );
        assert_eq!(jvm.get_field::<i32>(&alternating, "keyCalls", "I").await?, 1);
        assert_eq!(jvm.get_field::<i32>(&alternating, "valueCalls", "I").await?, 1);
        assert!(
            !jvm.invoke_virtual::<_, bool>(
                &stateful_map,
                &stateful_map.class_definition().name(),
                "containsKey",
                "(Ljava/lang/Object;)Z",
                (first_key,)
            )
            .await?
        );
        assert!(
            jvm.invoke_virtual::<_, bool>(
                &stateful_map,
                &stateful_map.class_definition().name(),
                "containsKey",
                "(Ljava/lang/Object;)Z",
                (later_key.clone(),)
            )
            .await?
        );
        let preserved: ClassInstanceRef<Object> = jvm
            .invoke_virtual(
                &stateful_map,
                &stateful_map.class_definition().name(),
                "get",
                "(Ljava/lang/Object;)Ljava/lang/Object;",
                (later_key.clone(),),
            )
            .await?;
        assert_eq!(JavaLangString::to_rust_string(&jvm, &preserved).await?, "later-value");

        let missing_key = JavaLangString::from_rust_string(&jvm, "missing-key").await?;
        let missing_contains: ClassInstanceRef<Object> = jvm
            .new_class(
                "StatefulMapEntry",
                "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;I)V",
                (missing_key.clone(), missing_key.clone(), null.clone(), 2),
            )
            .await?
            .into();
        assert!(
            !jvm.invoke_virtual::<_, bool>(
                &stateful_entries,
                &stateful_entries.class_definition().name(),
                "contains",
                "(Ljava/lang/Object;)Z",
                (missing_contains.clone(),),
            )
            .await?
        );
        assert_eq!(jvm.get_field::<i32>(&missing_contains, "keyCalls", "I").await?, 1);
        assert_eq!(jvm.get_field::<i32>(&missing_contains, "valueCalls", "I").await?, 0);

        let missing_remove: ClassInstanceRef<Object> = jvm
            .new_class(
                "StatefulMapEntry",
                "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;I)V",
                (missing_key.clone(), missing_key, null.clone(), 2),
            )
            .await?
            .into();
        assert!(
            !jvm.invoke_virtual::<_, bool>(
                &stateful_entries,
                &stateful_entries.class_definition().name(),
                "remove",
                "(Ljava/lang/Object;)Z",
                (missing_remove.clone(),),
            )
            .await?
        );
        assert_eq!(jvm.get_field::<i32>(&missing_remove, "keyCalls", "I").await?, 1);
        assert_eq!(jvm.get_field::<i32>(&missing_remove, "valueCalls", "I").await?, 0);

        let throwing_key: ClassInstanceRef<Object> = jvm
            .new_class(
                "StatefulMapEntry",
                "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;I)V",
                (later_key.clone(), later_key.clone(), null.clone(), 1),
            )
            .await?
            .into();
        let result: Result<bool> = jvm
            .invoke_virtual(
                &stateful_entries,
                &stateful_entries.class_definition().name(),
                "contains",
                "(Ljava/lang/Object;)Z",
                (throwing_key.clone(),),
            )
            .await;
        let Err(JavaError::JavaException(exception)) = result else {
            panic!("{map_class} EntrySet.contains must propagate getKey exceptions");
        };
        assert!(jvm.is_instance(&*exception, "java/lang/IllegalStateException"));
        assert_eq!(jvm.get_field::<i32>(&throwing_key, "keyCalls", "I").await?, 1);
        assert_eq!(jvm.get_field::<i32>(&throwing_key, "valueCalls", "I").await?, 0);

        let throwing_remove_key: ClassInstanceRef<Object> = jvm
            .new_class(
                "StatefulMapEntry",
                "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;I)V",
                (later_key.clone(), later_key.clone(), null.clone(), 1),
            )
            .await?
            .into();
        let result: Result<bool> = jvm
            .invoke_virtual(
                &stateful_entries,
                &stateful_entries.class_definition().name(),
                "remove",
                "(Ljava/lang/Object;)Z",
                (throwing_remove_key.clone(),),
            )
            .await;
        let Err(JavaError::JavaException(exception)) = result else {
            panic!("{map_class} EntrySet.remove must propagate getKey exceptions");
        };
        assert!(jvm.is_instance(&*exception, "java/lang/IllegalStateException"));
        assert_eq!(jvm.get_field::<i32>(&throwing_remove_key, "keyCalls", "I").await?, 1);
        assert_eq!(jvm.get_field::<i32>(&throwing_remove_key, "valueCalls", "I").await?, 0);

        let throwing_contains_value: ClassInstanceRef<Object> = jvm
            .new_class(
                "StatefulMapEntry",
                "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;I)V",
                (later_key.clone(), later_key.clone(), null.clone(), 2),
            )
            .await?
            .into();
        let result: Result<bool> = jvm
            .invoke_virtual(
                &stateful_entries,
                &stateful_entries.class_definition().name(),
                "contains",
                "(Ljava/lang/Object;)Z",
                (throwing_contains_value.clone(),),
            )
            .await;
        let Err(JavaError::JavaException(exception)) = result else {
            panic!("{map_class} EntrySet.contains must propagate getValue exceptions for a found key");
        };
        assert!(jvm.is_instance(&*exception, "java/lang/IllegalStateException"));
        assert_eq!(jvm.get_field::<i32>(&throwing_contains_value, "keyCalls", "I").await?, 1);
        assert_eq!(jvm.get_field::<i32>(&throwing_contains_value, "valueCalls", "I").await?, 1);

        let throwing_value: ClassInstanceRef<Object> = jvm
            .new_class(
                "StatefulMapEntry",
                "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;I)V",
                (later_key.clone(), later_key.clone(), null.clone(), 2),
            )
            .await?
            .into();
        let result: Result<bool> = jvm
            .invoke_virtual(
                &stateful_entries,
                &stateful_entries.class_definition().name(),
                "remove",
                "(Ljava/lang/Object;)Z",
                (throwing_value.clone(),),
            )
            .await;
        let Err(JavaError::JavaException(exception)) = result else {
            panic!("{map_class} EntrySet.remove must propagate getValue exceptions for a found key");
        };
        assert!(jvm.is_instance(&*exception, "java/lang/IllegalStateException"));
        assert_eq!(jvm.get_field::<i32>(&throwing_value, "keyCalls", "I").await?, 1);
        assert_eq!(jvm.get_field::<i32>(&throwing_value, "valueCalls", "I").await?, 1);
        assert!(
            jvm.invoke_virtual::<_, bool>(
                &stateful_map,
                &stateful_map.class_definition().name(),
                "containsKey",
                "(Ljava/lang/Object;)Z",
                (later_key.clone(),)
            )
            .await?
        );

        let non_entry: ClassInstanceRef<Object> = jvm.new_class("java/lang/Object", "()V", ()).await?.into();
        for candidate in [null.clone(), non_entry] {
            assert!(
                !jvm.invoke_virtual::<_, bool>(
                    &stateful_entries,
                    &stateful_entries.class_definition().name(),
                    "contains",
                    "(Ljava/lang/Object;)Z",
                    (candidate.clone(),)
                )
                .await?
            );
            assert!(
                !jvm.invoke_virtual::<_, bool>(
                    &stateful_entries,
                    &stateful_entries.class_definition().name(),
                    "remove",
                    "(Ljava/lang/Object;)Z",
                    (candidate,)
                )
                .await?
            );
        }

        if map_class == "java/util/Hashtable" {
            let null_key: ClassInstanceRef<Object> = jvm
                .new_class(
                    "StatefulMapEntry",
                    "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;I)V",
                    (null.clone(), null.clone(), null.clone(), 2),
                )
                .await?
                .into();
            assert!(
                !jvm.invoke_virtual::<_, bool>(
                    &stateful_entries,
                    &stateful_entries.class_definition().name(),
                    "contains",
                    "(Ljava/lang/Object;)Z",
                    (null_key.clone(),)
                )
                .await?
            );
            assert!(
                !jvm.invoke_virtual::<_, bool>(
                    &stateful_entries,
                    &stateful_entries.class_definition().name(),
                    "remove",
                    "(Ljava/lang/Object;)Z",
                    (null_key.clone(),)
                )
                .await?
            );
            assert_eq!(jvm.get_field::<i32>(&null_key, "keyCalls", "I").await?, 2);
            assert_eq!(jvm.get_field::<i32>(&null_key, "valueCalls", "I").await?, 0);
        }
    }

    Ok(())
}

#[tokio::test]
async fn hashtable_equals_and_hash_code_match_map_value_contracts() -> Result<()> {
    let jvm = collection_contract_fixture_jvm().await?;
    let hashtable: ClassInstanceRef<Object> = jvm.new_class("java/util/Hashtable", "()V", ()).await?.into();
    let hash_map: ClassInstanceRef<Object> = jvm.new_class("java/util/HashMap", "()V", ()).await?.into();
    let tree_map: ClassInstanceRef<Object> = jvm.new_class("java/util/TreeMap", "()V", ()).await?.into();

    for map in [&hashtable, &hash_map, &tree_map] {
        for (key, value) in [("alpha", 17), ("beta", 29)] {
            let key: ClassInstanceRef<Object> = JavaLangString::from_rust_string(&jvm, key).await?.into();
            let value: ClassInstanceRef<Object> = jvm.new_class("java/lang/Integer", "(I)V", (value,)).await?.into();
            let _: ClassInstanceRef<Object> = jvm
                .invoke_virtual(
                    map,
                    &map.class_definition().name(),
                    "put",
                    "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
                    (key, value),
                )
                .await?;
        }
    }
    let unmodifiable: ClassInstanceRef<Object> = jvm
        .invoke_static(
            "java/util/Collections",
            "unmodifiableMap",
            "(Ljava/util/Map;)Ljava/util/Map;",
            (hash_map.clone(),),
        )
        .await?;
    let maps = [&hashtable, &hash_map, &tree_map, &unmodifiable];
    let expected_hash: i32 = jvm
        .invoke_virtual(&hashtable, &hashtable.class_definition().name(), "hashCode", "()I", ())
        .await?;
    for left in maps {
        assert_eq!(
            jvm.invoke_virtual::<_, i32>(left, &left.class_definition().name(), "hashCode", "()I", ())
                .await?,
            expected_hash
        );
        for right in maps {
            assert!(
                jvm.invoke_virtual::<_, bool>(left, &left.class_definition().name(), "equals", "(Ljava/lang/Object;)Z", (right.clone(),))
                    .await?,
                "{} must equal {}",
                left.class_definition().name(),
                right.class_definition().name()
            );
        }
    }

    assert!(
        jvm.invoke_virtual::<_, bool>(
            &hashtable,
            &hashtable.class_definition().name(),
            "equals",
            "(Ljava/lang/Object;)Z",
            (hashtable.clone(),)
        )
        .await?
    );
    let null: ClassInstanceRef<Object> = None.into();
    assert!(
        !jvm.invoke_virtual::<_, bool>(
            &hashtable,
            &hashtable.class_definition().name(),
            "equals",
            "(Ljava/lang/Object;)Z",
            (null,)
        )
        .await?
    );
    let non_map: ClassInstanceRef<Object> = jvm.new_class("java/lang/Object", "()V", ()).await?.into();
    assert!(
        !jvm.invoke_virtual::<_, bool>(
            &hashtable,
            &hashtable.class_definition().name(),
            "equals",
            "(Ljava/lang/Object;)Z",
            (non_map,)
        )
        .await?
    );
    let different_size: ClassInstanceRef<Object> = jvm.new_class("java/util/HashMap", "()V", ()).await?.into();
    assert!(
        !jvm.invoke_virtual::<_, bool>(
            &hashtable,
            &hashtable.class_definition().name(),
            "equals",
            "(Ljava/lang/Object;)Z",
            (different_size,)
        )
        .await?
    );

    for mode in [0, 1] {
        let throwing: ClassInstanceRef<Object> = jvm.new_class("ThrowingMap", "(II)V", (2, mode)).await?.into();
        assert!(
            !jvm.invoke_virtual::<_, bool>(
                &hashtable,
                &hashtable.class_definition().name(),
                "equals",
                "(Ljava/lang/Object;)Z",
                (throwing,)
            )
            .await?,
            "Hashtable.equals must convert mode {mode} NPE/CCE to false"
        );
    }

    Ok(())
}

#[tokio::test]
async fn hashtable_hash_code_guards_self_key_and_value_recursion() -> Result<()> {
    let jvm = collection_contract_fixture_jvm().await?;

    let self_value: ClassInstanceRef<Object> = jvm.new_class("java/util/Hashtable", "()V", ()).await?.into();
    let value_key: ClassInstanceRef<Object> = JavaLangString::from_rust_string(&jvm, "self-value").await?.into();
    let expected_value_hash: i32 = jvm
        .invoke_virtual(&value_key, &value_key.class_definition().name(), "hashCode", "()I", ())
        .await?;
    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &self_value,
            &self_value.class_definition().name(),
            "put",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            (value_key, self_value.clone()),
        )
        .await?;
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&self_value, &self_value.class_definition().name(), "hashCode", "()I", ())
            .await?,
        expected_value_hash
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&self_value, &self_value.class_definition().name(), "hashCode", "()I", ())
            .await?,
        expected_value_hash
    );

    let self_key: ClassInstanceRef<Object> = jvm.new_class("java/util/Hashtable", "()V", ()).await?.into();
    let key_value: ClassInstanceRef<Object> = jvm.new_class("java/lang/Integer", "(I)V", (37,)).await?.into();
    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &self_key,
            &self_key.class_definition().name(),
            "put",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            (self_key.clone(), key_value),
        )
        .await?;
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&self_key, &self_key.class_definition().name(), "hashCode", "()I", ())
            .await?,
        37
    );

    let self_both: ClassInstanceRef<Object> = jvm.new_class("java/util/Hashtable", "()V", ()).await?.into();
    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &self_both,
            &self_both.class_definition().name(),
            "put",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            (self_both.clone(), self_both.clone()),
        )
        .await?;
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&self_both, &self_both.class_definition().name(), "hashCode", "()I", ())
            .await?,
        0
    );

    Ok(())
}

#[tokio::test]
async fn hashtable_entry_equals_and_hash_code_follow_map_entry_contract() -> Result<()> {
    let jvm = collection_contract_fixture_jvm().await?;
    let table: ClassInstanceRef<Object> = jvm.new_class("java/util/Hashtable", "()V", ()).await?.into();
    let key: ClassInstanceRef<Object> = JavaLangString::from_rust_string(&jvm, "entry-key").await?.into();
    let value: ClassInstanceRef<Object> = JavaLangString::from_rust_string(&jvm, "entry-value").await?.into();
    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &table,
            &table.class_definition().name(),
            "put",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            (key.clone(), value.clone()),
        )
        .await?;
    let entries: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&table, &table.class_definition().name(), "entrySet", "()Ljava/util/Set;", ())
        .await?;
    let iterator: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&entries, &entries.class_definition().name(), "iterator", "()Ljava/util/Iterator;", ())
        .await?;
    let entry: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&iterator, &iterator.class_definition().name(), "next", "()Ljava/lang/Object;", ())
        .await?;

    let different_key: ClassInstanceRef<Object> = JavaLangString::from_rust_string(&jvm, "different").await?.into();
    let short_circuit: ClassInstanceRef<Object> = jvm
        .new_class(
            "StatefulMapEntry",
            "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;I)V",
            (different_key.clone(), different_key, value.clone(), 2),
        )
        .await?
        .into();
    assert!(
        !jvm.invoke_virtual::<_, bool>(
            &entry,
            &entry.class_definition().name(),
            "equals",
            "(Ljava/lang/Object;)Z",
            (short_circuit.clone(),)
        )
        .await?
    );
    assert_eq!(jvm.get_field::<i32>(&short_circuit, "keyCalls", "I").await?, 1);
    assert_eq!(jvm.get_field::<i32>(&short_circuit, "valueCalls", "I").await?, 0);

    let throwing_key: ClassInstanceRef<Object> = jvm
        .new_class(
            "StatefulMapEntry",
            "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;I)V",
            (key.clone(), key.clone(), value.clone(), 1),
        )
        .await?
        .into();
    let result: Result<bool> = jvm
        .invoke_virtual(
            &entry,
            &entry.class_definition().name(),
            "equals",
            "(Ljava/lang/Object;)Z",
            (throwing_key.clone(),),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("Hashtable.Entry.equals must propagate getKey exceptions");
    };
    assert!(jvm.is_instance(exception.as_ref(), "java/lang/IllegalStateException"));
    assert_eq!(jvm.get_field::<i32>(&throwing_key, "keyCalls", "I").await?, 1);
    assert_eq!(jvm.get_field::<i32>(&throwing_key, "valueCalls", "I").await?, 0);

    let throwing_value: ClassInstanceRef<Object> = jvm
        .new_class(
            "StatefulMapEntry",
            "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;I)V",
            (key.clone(), key.clone(), value.clone(), 2),
        )
        .await?
        .into();
    let result: Result<bool> = jvm
        .invoke_virtual(
            &entry,
            &entry.class_definition().name(),
            "equals",
            "(Ljava/lang/Object;)Z",
            (throwing_value.clone(),),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("Hashtable.Entry.equals must propagate getValue exceptions after matching keys");
    };
    assert!(jvm.is_instance(exception.as_ref(), "java/lang/IllegalStateException"));
    assert_eq!(jvm.get_field::<i32>(&throwing_value, "keyCalls", "I").await?, 1);
    assert_eq!(jvm.get_field::<i32>(&throwing_value, "valueCalls", "I").await?, 1);

    let directional_table: ClassInstanceRef<Object> = jvm.new_class("java/util/Hashtable", "()V", ()).await?.into();
    let stored_key: ClassInstanceRef<Object> = jvm.new_class("ConfigurableEqualsValue", "(Z)V", (true,)).await?.into();
    let candidate_key: ClassInstanceRef<Object> = jvm.new_class("ConfigurableEqualsValue", "(Z)V", (false,)).await?.into();
    let stored_value: ClassInstanceRef<Object> = jvm.new_class("ConfigurableEqualsValue", "(Z)V", (true,)).await?.into();
    let candidate_value: ClassInstanceRef<Object> = jvm.new_class("ConfigurableEqualsValue", "(Z)V", (false,)).await?.into();
    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &directional_table,
            &directional_table.class_definition().name(),
            "put",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            (stored_key.clone(), stored_value.clone()),
        )
        .await?;
    let directional_entries: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &directional_table,
            &directional_table.class_definition().name(),
            "entrySet",
            "()Ljava/util/Set;",
            (),
        )
        .await?;
    let directional_iterator: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &directional_entries,
            &directional_entries.class_definition().name(),
            "iterator",
            "()Ljava/util/Iterator;",
            (),
        )
        .await?;
    let directional_entry: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &directional_iterator,
            &directional_iterator.class_definition().name(),
            "next",
            "()Ljava/lang/Object;",
            (),
        )
        .await?;
    let directional_candidate: ClassInstanceRef<Object> = jvm
        .new_class(
            "StatefulMapEntry",
            "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;I)V",
            (candidate_key.clone(), candidate_key.clone(), candidate_value.clone(), 0),
        )
        .await?
        .into();
    assert!(
        jvm.invoke_virtual::<_, bool>(
            &directional_entry,
            &directional_entry.class_definition().name(),
            "equals",
            "(Ljava/lang/Object;)Z",
            (directional_candidate.clone(),),
        )
        .await?
    );
    assert_eq!(jvm.get_field::<i32>(&directional_candidate, "keyCalls", "I").await?, 1);
    assert_eq!(jvm.get_field::<i32>(&directional_candidate, "valueCalls", "I").await?, 1);
    assert_eq!(jvm.get_field::<i32>(&stored_key, "equalsCalls", "I").await?, 1);
    assert_eq!(jvm.get_field::<i32>(&candidate_key, "equalsCalls", "I").await?, 0);
    assert_eq!(jvm.get_field::<i32>(&stored_value, "equalsCalls", "I").await?, 1);
    assert_eq!(jvm.get_field::<i32>(&candidate_value, "equalsCalls", "I").await?, 0);

    let integer_key: ClassInstanceRef<Object> = jvm.new_class("java/lang/Integer", "(I)V", (7,)).await?.into();
    let integer_value: ClassInstanceRef<Object> = jvm.new_class("java/lang/Integer", "(I)V", (11,)).await?.into();
    let null: ClassInstanceRef<Object> = None.into();
    let integer_entry: ClassInstanceRef<Object> = jvm
        .new_class(
            "java/util/Hashtable$Entry",
            "(ILjava/lang/Object;Ljava/lang/Object;Ljava/util/Hashtable$Entry;)V",
            (7, integer_key, integer_value, null.clone()),
        )
        .await?
        .into();
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&integer_entry, &integer_entry.class_definition().name(), "hashCode", "()I", ())
            .await?,
        7 ^ 11
    );

    let null_entry: ClassInstanceRef<Object> = jvm
        .new_class(
            "java/util/Hashtable$Entry",
            "(ILjava/lang/Object;Ljava/lang/Object;Ljava/util/Hashtable$Entry;)V",
            (0, null.clone(), null.clone(), null.clone()),
        )
        .await?
        .into();
    let null_candidate: ClassInstanceRef<Object> = jvm
        .new_class(
            "StatefulMapEntry",
            "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;I)V",
            (null.clone(), null.clone(), null.clone(), 0),
        )
        .await?
        .into();
    assert!(
        jvm.invoke_virtual::<_, bool>(
            &null_entry,
            &null_entry.class_definition().name(),
            "equals",
            "(Ljava/lang/Object;)Z",
            (null_candidate.clone(),)
        )
        .await?
    );
    assert_eq!(jvm.get_field::<i32>(&null_candidate, "keyCalls", "I").await?, 1);
    assert_eq!(jvm.get_field::<i32>(&null_candidate, "valueCalls", "I").await?, 1);
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&null_entry, &null_entry.class_definition().name(), "hashCode", "()I", ())
            .await?,
        0
    );
    assert!(
        !jvm.invoke_virtual::<_, bool>(
            &null_entry,
            &null_entry.class_definition().name(),
            "equals",
            "(Ljava/lang/Object;)Z",
            (null.clone(),)
        )
        .await?
    );
    let non_entry: ClassInstanceRef<Object> = jvm.new_class("java/lang/Object", "()V", ()).await?.into();
    assert!(
        !jvm.invoke_virtual::<_, bool>(
            &null_entry,
            &null_entry.class_definition().name(),
            "equals",
            "(Ljava/lang/Object;)Z",
            (non_entry,)
        )
        .await?
    );

    Ok(())
}
