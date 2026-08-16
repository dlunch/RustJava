use alloc::{boxed::Box, collections::BTreeMap, vec, vec::Vec};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use java_runtime::{RuntimeClassProto, RuntimeContext, classes::java::lang::Object, get_runtime_class_proto};
use jvm::{Array, ClassInstanceRef, JavaError, Jvm, Result, runtime::JavaLangString};
use jvm_rust::ClassDefinitionImpl;

use test_utils::{TestRuntime, create_test_jvm, test_jvm};

struct CollectionsSortValue;

impl CollectionsSortValue {
    fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "CollectionsSortValue",
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["java/lang/Comparable"],
            methods: vec![
                JavaMethodProto::new("<init>", "(IIZ)V", Self::init, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("compareTo", "(Ljava/lang/Object;)I", Self::compare_to, MethodAccessFlags::PUBLIC),
            ],
            fields: vec![
                JavaFieldProto::new("key", "I", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("id", "I", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("fail", "Z", FieldAccessFlags::PRIVATE),
            ],
            access_flags: ClassAccessFlags::empty(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, key: i32, id: i32, fail: bool) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "key", "I", key).await?;
        jvm.put_field(&mut this, "id", "I", id).await?;
        jvm.put_field(&mut this, "fail", "Z", fail).await
    }

    async fn compare_to(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, other: ClassInstanceRef<Object>) -> Result<i32> {
        if other.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "other").await);
        }
        if !jvm.is_instance(other.as_ref(), "CollectionsSortValue") {
            return Err(jvm.exception("java/lang/ClassCastException", "other").await);
        }
        if jvm.get_field::<bool>(&this, "fail", "Z").await? || jvm.get_field::<bool>(&other, "fail", "Z").await? {
            return Err(jvm.exception("java/lang/IllegalStateException", "comparison failure").await);
        }

        Ok(jvm
            .get_field::<i32>(&this, "key", "I")
            .await?
            .cmp(&jvm.get_field::<i32>(&other, "key", "I").await?) as i32)
    }
}

struct CollectionsComparator;

impl CollectionsComparator {
    fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "CollectionsComparator",
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["java/util/Comparator"],
            methods: vec![
                JavaMethodProto::new("<init>", "(ZZ)V", Self::init, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("<init>", "(ZZZ)V", Self::init_with_nulls, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "compare",
                    "(Ljava/lang/Object;Ljava/lang/Object;)I",
                    Self::compare,
                    MethodAccessFlags::PUBLIC,
                ),
            ],
            fields: vec![
                JavaFieldProto::new("reverse", "Z", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("fail", "Z", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("allowNull", "Z", FieldAccessFlags::PRIVATE),
            ],
            access_flags: ClassAccessFlags::empty(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, reverse: bool, fail: bool) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "reverse", "Z", reverse).await?;
        jvm.put_field(&mut this, "fail", "Z", fail).await?;
        jvm.put_field(&mut this, "allowNull", "Z", false).await
    }

    async fn init_with_nulls(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        reverse: bool,
        fail: bool,
        allow_null: bool,
    ) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "reverse", "Z", reverse).await?;
        jvm.put_field(&mut this, "fail", "Z", fail).await?;
        jvm.put_field(&mut this, "allowNull", "Z", allow_null).await
    }

    async fn compare(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        this: ClassInstanceRef<Self>,
        left: ClassInstanceRef<Object>,
        right: ClassInstanceRef<Object>,
    ) -> Result<i32> {
        if jvm.get_field::<bool>(&this, "fail", "Z").await? {
            return Err(jvm.exception("java/lang/IllegalStateException", "comparison failure").await);
        }
        let comparison = if left.is_null() || right.is_null() {
            if !jvm.get_field::<bool>(&this, "allowNull", "Z").await? {
                return Err(jvm.exception("java/lang/NullPointerException", "value").await);
            }
            match (left.is_null(), right.is_null()) {
                (true, true) => 0,
                (true, false) => -1,
                (false, true) => 1,
                (false, false) => unreachable!(),
            }
        } else {
            jvm.get_field::<i32>(&left, "key", "I")
                .await?
                .cmp(&jvm.get_field::<i32>(&right, "key", "I").await?) as i32
        };
        Ok(if jvm.get_field::<bool>(&this, "reverse", "Z").await? {
            -comparison
        } else {
            comparison
        })
    }
}

struct CollectionsProbeList;

impl CollectionsProbeList {
    fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "CollectionsProbeList",
            parent_class: Some("java/util/AbstractList"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "([Ljava/lang/Object;)V", Self::init, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("size", "()I", Self::size, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("get", "(I)Ljava/lang/Object;", Self::get, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("set", "(ILjava/lang/Object;)Ljava/lang/Object;", Self::set, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new(
                    "listIterator",
                    "()Ljava/util/ListIterator;",
                    Self::list_iterator,
                    MethodAccessFlags::PUBLIC,
                ),
                JavaMethodProto::new(
                    "listIterator",
                    "(I)Ljava/util/ListIterator;",
                    Self::list_iterator_at,
                    MethodAccessFlags::PUBLIC,
                ),
            ],
            fields: vec![
                JavaFieldProto::new("elements", "[Ljava/lang/Object;", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("listIteratorCalls", "I", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("setCalls", "I", FieldAccessFlags::PRIVATE),
            ],
            access_flags: ClassAccessFlags::empty(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, elements: ClassInstanceRef<Array<Object>>) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/util/AbstractList", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "elements", "[Ljava/lang/Object;", elements).await?;
        jvm.put_field(&mut this, "listIteratorCalls", "I", 0).await?;
        jvm.put_field(&mut this, "setCalls", "I", 0).await
    }

    async fn size(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<i32> {
        let elements: ClassInstanceRef<Array<Object>> = jvm.get_field(&this, "elements", "[Ljava/lang/Object;").await?;
        Ok(jvm.array_length(&elements).await? as i32)
    }

    async fn get(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, index: i32) -> Result<ClassInstanceRef<Object>> {
        let elements: ClassInstanceRef<Array<Object>> = jvm.get_field(&this, "elements", "[Ljava/lang/Object;").await?;
        let length = jvm.array_length(&elements).await? as i32;
        if index < 0 || index >= length {
            return Err(jvm.exception("java/lang/IndexOutOfBoundsException", "index").await);
        }
        Ok(jvm
            .load_array::<ClassInstanceRef<Object>>(&elements, index as usize, 1)
            .await?
            .into_iter()
            .next()
            .unwrap())
    }

    async fn set(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        index: i32,
        element: ClassInstanceRef<Object>,
    ) -> Result<ClassInstanceRef<Object>> {
        let mut elements: ClassInstanceRef<Array<Object>> = jvm.get_field(&this, "elements", "[Ljava/lang/Object;").await?;
        let length = jvm.array_length(&elements).await? as i32;
        if index < 0 || index >= length {
            return Err(jvm.exception("java/lang/IndexOutOfBoundsException", "index").await);
        }
        let previous = jvm
            .load_array::<ClassInstanceRef<Object>>(&elements, index as usize, 1)
            .await?
            .into_iter()
            .next()
            .unwrap();
        jvm.store_array(&mut elements, index as usize, core::iter::once(element)).await?;
        let calls: i32 = jvm.get_field(&this, "setCalls", "I").await?;
        jvm.put_field(&mut this, "setCalls", "I", calls + 1).await?;
        Ok(previous)
    }

    async fn list_iterator(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let calls: i32 = jvm.get_field(&this, "listIteratorCalls", "I").await?;
        jvm.put_field(&mut this, "listIteratorCalls", "I", calls + 1).await?;
        jvm.invoke_special(&this, "java/util/AbstractList", "listIterator", "()Ljava/util/ListIterator;", ())
            .await
    }

    async fn list_iterator_at(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, index: i32) -> Result<ClassInstanceRef<Object>> {
        let calls: i32 = jvm.get_field(&this, "listIteratorCalls", "I").await?;
        jvm.put_field(&mut this, "listIteratorCalls", "I", calls + 1).await?;
        jvm.invoke_special(&this, "java/util/AbstractList", "listIterator", "(I)Ljava/util/ListIterator;", (index,))
            .await
    }
}

struct CollectionsInvalidRandom;

impl CollectionsInvalidRandom {
    fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "CollectionsInvalidRandom",
            parent_class: Some("java/util/Random"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(I)V", Self::init, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("nextInt", "(I)I", Self::next_int, MethodAccessFlags::PUBLIC),
            ],
            fields: vec![JavaFieldProto::new("result", "I", FieldAccessFlags::PRIVATE)],
            access_flags: ClassAccessFlags::empty(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, result: i32) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/util/Random", "<init>", "(J)V", (0i64,)).await?;
        jvm.put_field(&mut this, "result", "I", result).await
    }

    async fn next_int(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, _: i32) -> Result<i32> {
        jvm.get_field(&this, "result", "I").await
    }
}

struct CollectionsExceptionalSet;

impl CollectionsExceptionalSet {
    fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "CollectionsExceptionalSet",
            parent_class: Some("java/util/AbstractSet"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/lang/Object;I)V", Self::init, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("size", "()I", Self::size, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("iterator", "()Ljava/util/Iterator;", Self::iterator, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("contains", "(Ljava/lang/Object;)Z", Self::contains, MethodAccessFlags::PUBLIC),
            ],
            fields: vec![
                JavaFieldProto::new("element", "Ljava/lang/Object;", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("mode", "I", FieldAccessFlags::PRIVATE),
            ],
            access_flags: ClassAccessFlags::empty(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, element: ClassInstanceRef<Object>, mode: i32) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/util/AbstractSet", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "element", "Ljava/lang/Object;", element).await?;
        jvm.put_field(&mut this, "mode", "I", mode).await
    }

    async fn size(_: &Jvm, _: &mut RuntimeContext, _: ClassInstanceRef<Self>) -> Result<i32> {
        Ok(1)
    }

    async fn iterator(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let element: ClassInstanceRef<Object> = jvm.get_field(&this, "element", "Ljava/lang/Object;").await?;
        let list = jvm
            .new_class("java/util/Collections$CopiesList", "(ILjava/lang/Object;)V", (1, element))
            .await?;
        jvm.invoke_virtual(&list, &list.class_definition().name(), "iterator", "()Ljava/util/Iterator;", ())
            .await
    }

    async fn contains(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, target: ClassInstanceRef<Object>) -> Result<bool> {
        match jvm.get_field::<i32>(&this, "mode", "I").await? {
            0 => Err(jvm.exception("java/lang/ClassCastException", "incompatible element").await),
            1 if target.is_null() => Err(jvm.exception("java/lang/NullPointerException", "null element").await),
            2 => Err(jvm.exception("java/lang/IllegalStateException", "unexpected contains failure").await),
            _ => {
                let element: ClassInstanceRef<Object> = jvm.get_field(&this, "element", "Ljava/lang/Object;").await?;
                if element.is_null() {
                    Ok(target.is_null())
                } else if target.is_null() {
                    Ok(false)
                } else {
                    jvm.invoke_virtual(&element, &element.class_definition().name(), "equals", "(Ljava/lang/Object;)Z", (target,))
                        .await
                }
            }
        }
    }
}

struct CollectionsAsymmetricEquals;

impl CollectionsAsymmetricEquals {
    fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "CollectionsAsymmetricEquals",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(Z)V", Self::init, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("equals", "(Ljava/lang/Object;)Z", Self::equals, MethodAccessFlags::PUBLIC),
            ],
            fields: vec![JavaFieldProto::new("result", "Z", FieldAccessFlags::PRIVATE)],
            access_flags: ClassAccessFlags::empty(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, result: bool) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "result", "Z", result).await
    }

    async fn equals(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, _: ClassInstanceRef<Object>) -> Result<bool> {
        jvm.get_field(&this, "result", "Z").await
    }
}

struct CollectionsEqualsProbe;

impl CollectionsEqualsProbe {
    fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "CollectionsEqualsProbe",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(ZZ)V", Self::init, MethodAccessFlags::PUBLIC),
                JavaMethodProto::new("equals", "(Ljava/lang/Object;)Z", Self::equals, MethodAccessFlags::PUBLIC),
            ],
            fields: vec![
                JavaFieldProto::new("result", "Z", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("fail", "Z", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("equalsCalls", "I", FieldAccessFlags::PRIVATE),
            ],
            access_flags: ClassAccessFlags::empty(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, result: bool, fail: bool) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "result", "Z", result).await?;
        jvm.put_field(&mut this, "fail", "Z", fail).await?;
        jvm.put_field(&mut this, "equalsCalls", "I", 0).await
    }

    async fn equals(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, _: ClassInstanceRef<Object>) -> Result<bool> {
        let calls: i32 = jvm.get_field(&this, "equalsCalls", "I").await?;
        jvm.put_field(&mut this, "equalsCalls", "I", calls + 1).await?;
        if jvm.get_field::<bool>(&this, "fail", "Z").await? {
            return Err(jvm.exception("java/lang/IllegalStateException", "equals failure").await);
        }
        jvm.get_field(&this, "result", "Z").await
    }
}

struct CollectionsEntryProbe;

impl CollectionsEntryProbe {
    fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "CollectionsEntryProbe",
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["java/util/Map$Entry"],
            methods: vec![
                JavaMethodProto::new(
                    "<init>",
                    "(Ljava/lang/Object;Ljava/lang/Object;ZZ)V",
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
                JavaFieldProto::new("key", "Ljava/lang/Object;", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("value", "Ljava/lang/Object;", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("throwKey", "Z", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("throwValue", "Z", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("keyCalls", "I", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("valueCalls", "I", FieldAccessFlags::PRIVATE),
                JavaFieldProto::new("setValueCalls", "I", FieldAccessFlags::PRIVATE),
            ],
            access_flags: ClassAccessFlags::empty(),
        }
    }

    async fn init(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        key: ClassInstanceRef<Object>,
        value: ClassInstanceRef<Object>,
        throw_key: bool,
        throw_value: bool,
    ) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "key", "Ljava/lang/Object;", key).await?;
        jvm.put_field(&mut this, "value", "Ljava/lang/Object;", value).await?;
        jvm.put_field(&mut this, "throwKey", "Z", throw_key).await?;
        jvm.put_field(&mut this, "throwValue", "Z", throw_value).await?;
        jvm.put_field(&mut this, "keyCalls", "I", 0).await?;
        jvm.put_field(&mut this, "valueCalls", "I", 0).await?;
        jvm.put_field(&mut this, "setValueCalls", "I", 0).await
    }

    async fn get_key(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let calls: i32 = jvm.get_field(&this, "keyCalls", "I").await?;
        jvm.put_field(&mut this, "keyCalls", "I", calls + 1).await?;
        if jvm.get_field::<bool>(&this, "throwKey", "Z").await? {
            return Err(jvm.exception("java/lang/IllegalStateException", "getKey failure").await);
        }
        jvm.get_field(&this, "key", "Ljava/lang/Object;").await
    }

    async fn get_value(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<ClassInstanceRef<Object>> {
        let calls: i32 = jvm.get_field(&this, "valueCalls", "I").await?;
        jvm.put_field(&mut this, "valueCalls", "I", calls + 1).await?;
        if jvm.get_field::<bool>(&this, "throwValue", "Z").await? {
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
        let calls: i32 = jvm.get_field(&this, "setValueCalls", "I").await?;
        jvm.put_field(&mut this, "setValueCalls", "I", calls + 1).await?;
        let previous: ClassInstanceRef<Object> = jvm.get_field(&this, "value", "Ljava/lang/Object;").await?;
        jvm.put_field(&mut this, "value", "Ljava/lang/Object;", value).await?;
        Ok(previous)
    }
}

async fn collections_test_jvm() -> Result<Jvm> {
    let runtime = TestRuntime::new(BTreeMap::new());
    let jvm = create_test_jvm(runtime.clone()).await?;
    for proto in [
        CollectionsSortValue::as_proto(),
        CollectionsComparator::as_proto(),
        CollectionsProbeList::as_proto(),
        CollectionsInvalidRandom::as_proto(),
        CollectionsExceptionalSet::as_proto(),
        CollectionsAsymmetricEquals::as_proto(),
        CollectionsEqualsProbe::as_proto(),
        CollectionsEntryProbe::as_proto(),
    ] {
        jvm.register_class(
            Box::new(ClassDefinitionImpl::from_class_proto(proto, Box::new(runtime.clone()) as Box<_>)),
            None,
        )
        .await?;
    }
    Ok(jvm)
}

async fn integer_list(jvm: &Jvm, values: &[i32]) -> Result<ClassInstanceRef<Object>> {
    let list: ClassInstanceRef<Object> = jvm.new_class("java/util/ArrayList", "()V", ()).await?.into();
    for value in values {
        let element: ClassInstanceRef<Object> = jvm.new_class("java/lang/Integer", "(I)V", (*value,)).await?.into();
        let _: bool = jvm
            .invoke_virtual(&list, &list.class_definition().name(), "add", "(Ljava/lang/Object;)Z", (element,))
            .await?;
    }
    Ok(list)
}

async fn integer_values(jvm: &Jvm, list: &ClassInstanceRef<Object>) -> Result<Vec<i32>> {
    let size: i32 = jvm.invoke_virtual(list, &list.class_definition().name(), "size", "()I", ()).await?;
    let mut values = Vec::with_capacity(size as usize);
    for index in 0..size {
        let element: ClassInstanceRef<Object> = jvm
            .invoke_virtual(list, &list.class_definition().name(), "get", "(I)Ljava/lang/Object;", (index,))
            .await?;
        values.push(
            jvm.invoke_virtual(&element, &element.class_definition().name(), "intValue", "()I", ())
                .await?,
        );
    }
    Ok(values)
}

#[tokio::test]
async fn test_coll_01_exact_descriptors_access_and_singletons() -> Result<()> {
    let jvm = test_jvm().await?;
    let class = jvm.resolve_class("java/util/Collections").await?;
    assert!(class.definition.access_flags().contains(ClassAccessFlags::PUBLIC));
    assert!(
        class
            .definition
            .method("<init>", "()V", false)
            .expect("Collections private constructor")
            .access_flags()
            .contains(MethodAccessFlags::PRIVATE)
    );

    let descriptors = [
        ("sort", "(Ljava/util/List;)V"),
        ("sort", "(Ljava/util/List;Ljava/util/Comparator;)V"),
        ("binarySearch", "(Ljava/util/List;Ljava/lang/Object;)I"),
        ("binarySearch", "(Ljava/util/List;Ljava/lang/Object;Ljava/util/Comparator;)I"),
        ("reverse", "(Ljava/util/List;)V"),
        ("fill", "(Ljava/util/List;Ljava/lang/Object;)V"),
        ("copy", "(Ljava/util/List;Ljava/util/List;)V"),
        ("shuffle", "(Ljava/util/List;)V"),
        ("shuffle", "(Ljava/util/List;Ljava/util/Random;)V"),
        ("min", "(Ljava/util/Collection;)Ljava/lang/Object;"),
        ("min", "(Ljava/util/Collection;Ljava/util/Comparator;)Ljava/lang/Object;"),
        ("max", "(Ljava/util/Collection;)Ljava/lang/Object;"),
        ("max", "(Ljava/util/Collection;Ljava/util/Comparator;)Ljava/lang/Object;"),
        ("nCopies", "(ILjava/lang/Object;)Ljava/util/List;"),
        ("singleton", "(Ljava/lang/Object;)Ljava/util/Set;"),
    ];
    for (name, descriptor) in descriptors {
        let method = class
            .definition
            .method(name, descriptor, true)
            .unwrap_or_else(|| panic!("missing Collections.{name}{descriptor}"));
        assert!(
            method.access_flags().contains(MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC),
            "wrong access for Collections.{name}{descriptor}"
        );
    }

    for (name, descriptor) in [("EMPTY_LIST", "Ljava/util/List;"), ("EMPTY_SET", "Ljava/util/Set;")] {
        let field = class
            .definition
            .field(name, descriptor, true)
            .unwrap_or_else(|| panic!("missing Collections.{name}:{descriptor}"));
        assert!(
            field
                .access_flags()
                .contains(FieldAccessFlags::PUBLIC | FieldAccessFlags::STATIC | FieldAccessFlags::FINAL)
        );
    }

    let empty_list: ClassInstanceRef<Object> = jvm.get_static_field("java/util/Collections", "EMPTY_LIST", "Ljava/util/List;").await?;
    let same_empty_list: ClassInstanceRef<Object> = jvm.get_static_field("java/util/Collections", "EMPTY_LIST", "Ljava/util/List;").await?;
    let empty_set: ClassInstanceRef<Object> = jvm.get_static_field("java/util/Collections", "EMPTY_SET", "Ljava/util/Set;").await?;
    assert_eq!(empty_list.identity(), same_empty_list.identity());
    assert!(jvm.is_instance(empty_list.as_ref(), "java/util/List"));
    assert!(jvm.is_instance(empty_set.as_ref(), "java/util/Set"));
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&empty_list, &empty_list.class_definition().name(), "size", "()I", ())
            .await?,
        0
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&empty_set, &empty_set.class_definition().name(), "size", "()I", ())
            .await?,
        0
    );

    for (name, parent) in [
        ("java/util/Collections$EmptyList", "java/util/AbstractList"),
        ("java/util/Collections$CopiesList", "java/util/AbstractList"),
        ("java/util/Collections$EmptySet", "java/util/AbstractSet"),
        ("java/util/Collections$SingletonSet", "java/util/AbstractSet"),
    ] {
        let inner = jvm.resolve_class(name).await?;
        assert_eq!(inner.definition.super_class_name().as_deref(), Some(parent));
        assert!(inner.definition.interface_names().iter().any(|name| name == "java/io/Serializable"));
        assert!(!inner.definition.access_flags().contains(ClassAccessFlags::PUBLIC));
    }
    let copies_class = jvm.resolve_class("java/util/Collections$CopiesList").await?;
    assert!(
        copies_class
            .definition
            .field("n", "I", false)
            .expect("CopiesList.n")
            .access_flags()
            .contains(FieldAccessFlags::FINAL)
    );
    assert!(
        copies_class
            .definition
            .field("element", "Ljava/lang/Object;", false)
            .expect("CopiesList.element")
            .access_flags()
            .contains(FieldAccessFlags::FINAL)
    );

    for name in ["java/util/AbstractList", "java/util/AbstractSet"] {
        let abstract_class = jvm.resolve_class(name).await?;
        assert_eq!(
            abstract_class.definition.access_flags(),
            ClassAccessFlags::PUBLIC | ClassAccessFlags::ABSTRACT,
            "{name} must have exact public abstract access"
        );
        assert_eq!(
            abstract_class
                .definition
                .method("<init>", "()V", false)
                .expect("abstract collection constructor")
                .access_flags(),
            MethodAccessFlags::PROTECTED,
            "{name} constructor must have exact protected access"
        );
    }

    Ok(())
}

#[tokio::test]
async fn test_coll_02_stable_sort_and_comparator_failure_is_atomic() -> Result<()> {
    let jvm = collections_test_jvm().await?;
    let mut elements: ClassInstanceRef<Array<Object>> = jvm.instantiate_array("Ljava/lang/Object;", 4).await?.into();
    let original: [ClassInstanceRef<Object>; 4] = [
        jvm.new_class("CollectionsSortValue", "(IIZ)V", (2, 0, false)).await?.into(),
        jvm.new_class("CollectionsSortValue", "(IIZ)V", (1, 1, false)).await?.into(),
        jvm.new_class("CollectionsSortValue", "(IIZ)V", (2, 2, false)).await?.into(),
        jvm.new_class("CollectionsSortValue", "(IIZ)V", (1, 3, false)).await?.into(),
    ];
    jvm.store_array(&mut elements, 0, original.clone()).await?;
    let list: ClassInstanceRef<Object> = jvm.new_class("CollectionsProbeList", "([Ljava/lang/Object;)V", (elements,)).await?.into();

    jvm.invoke_static::<_, ()>("java/util/Collections", "sort", "(Ljava/util/List;)V", (list.clone(),))
        .await?;
    let sorted: ClassInstanceRef<Array<Object>> = jvm.get_field(&list, "elements", "[Ljava/lang/Object;").await?;
    let sorted = jvm.load_array::<ClassInstanceRef<Object>>(&sorted, 0, 4).await?;
    let mut keys_and_ids = vec![];
    for value in sorted {
        keys_and_ids.push((
            jvm.get_field::<i32>(&value, "key", "I").await?,
            jvm.get_field::<i32>(&value, "id", "I").await?,
        ));
    }
    assert_eq!(keys_and_ids, vec![(1, 1), (1, 3), (2, 0), (2, 2)]);
    assert_eq!(jvm.get_field::<i32>(&list, "listIteratorCalls", "I").await?, 1);
    assert_eq!(jvm.get_field::<i32>(&list, "setCalls", "I").await?, 4);

    let reverse: ClassInstanceRef<Object> = jvm.new_class("CollectionsComparator", "(ZZ)V", (true, false)).await?.into();
    jvm.invoke_static::<_, ()>(
        "java/util/Collections",
        "sort",
        "(Ljava/util/List;Ljava/util/Comparator;)V",
        (list.clone(), reverse),
    )
    .await?;
    let reverse_sorted: ClassInstanceRef<Array<Object>> = jvm.get_field(&list, "elements", "[Ljava/lang/Object;").await?;
    let reverse_sorted = jvm.load_array::<ClassInstanceRef<Object>>(&reverse_sorted, 0, 4).await?;
    let mut reverse_keys_and_ids = vec![];
    for value in reverse_sorted {
        reverse_keys_and_ids.push((
            jvm.get_field::<i32>(&value, "key", "I").await?,
            jvm.get_field::<i32>(&value, "id", "I").await?,
        ));
    }
    assert_eq!(reverse_keys_and_ids, vec![(2, 0), (2, 2), (1, 1), (1, 3)]);

    let mut failing_elements: ClassInstanceRef<Array<Object>> = jvm.instantiate_array("Ljava/lang/Object;", 3).await?.into();
    let failing_original: [ClassInstanceRef<Object>; 3] = [
        jvm.new_class("CollectionsSortValue", "(IIZ)V", (3, 10, false)).await?.into(),
        jvm.new_class("CollectionsSortValue", "(IIZ)V", (2, 11, false)).await?.into(),
        jvm.new_class("CollectionsSortValue", "(IIZ)V", (1, 12, false)).await?.into(),
    ];
    jvm.store_array(&mut failing_elements, 0, failing_original.clone()).await?;
    let failing_list: ClassInstanceRef<Object> = jvm
        .new_class("CollectionsProbeList", "([Ljava/lang/Object;)V", (failing_elements,))
        .await?
        .into();
    let comparator: ClassInstanceRef<Object> = jvm.new_class("CollectionsComparator", "(ZZ)V", (false, true)).await?.into();
    let result: Result<()> = jvm
        .invoke_static(
            "java/util/Collections",
            "sort",
            "(Ljava/util/List;Ljava/util/Comparator;)V",
            (failing_list.clone(), comparator),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("failing comparator must propagate");
    };
    assert!(jvm.is_instance(exception.as_ref(), "java/lang/IllegalStateException"));
    let after: ClassInstanceRef<Array<Object>> = jvm.get_field(&failing_list, "elements", "[Ljava/lang/Object;").await?;
    let after = jvm.load_array::<ClassInstanceRef<Object>>(&after, 0, 3).await?;
    assert_eq!(
        after.iter().map(|element| element.identity()).collect::<Vec<_>>(),
        failing_original.iter().map(|element| element.identity()).collect::<Vec<_>>()
    );
    assert_eq!(jvm.get_field::<i32>(&failing_list, "listIteratorCalls", "I").await?, 0);
    assert_eq!(jvm.get_field::<i32>(&failing_list, "setCalls", "I").await?, 0);

    Ok(())
}

#[tokio::test]
async fn test_coll_03_binary_search_insertion_duplicates_and_comparator() -> Result<()> {
    let jvm = collections_test_jvm().await?;
    let list = integer_list(&jvm, &[1, 2, 2, 2, 4]).await?;
    let duplicate: ClassInstanceRef<Object> = jvm.new_class("java/lang/Integer", "(I)V", (2,)).await?.into();
    let found: i32 = jvm
        .invoke_static(
            "java/util/Collections",
            "binarySearch",
            "(Ljava/util/List;Ljava/lang/Object;)I",
            (list.clone(), duplicate),
        )
        .await?;
    assert!((1..=3).contains(&found));

    for (key, expected) in [(0, -1), (3, -5), (5, -6)] {
        let key: ClassInstanceRef<Object> = jvm.new_class("java/lang/Integer", "(I)V", (key,)).await?.into();
        assert_eq!(
            jvm.invoke_static::<_, i32>(
                "java/util/Collections",
                "binarySearch",
                "(Ljava/util/List;Ljava/lang/Object;)I",
                (list.clone(), key),
            )
            .await?,
            expected
        );
    }

    let reverse_list: ClassInstanceRef<Object> = jvm.new_class("java/util/ArrayList", "()V", ()).await?.into();
    for key in [3, 2, 1] {
        let value: ClassInstanceRef<Object> = jvm.new_class("CollectionsSortValue", "(IIZ)V", (key, key, false)).await?.into();
        let _: bool = jvm
            .invoke_virtual(
                &reverse_list,
                &reverse_list.class_definition().name(),
                "add",
                "(Ljava/lang/Object;)Z",
                (value,),
            )
            .await?;
    }
    let comparator: ClassInstanceRef<Object> = jvm.new_class("CollectionsComparator", "(ZZ)V", (true, false)).await?.into();
    let key: ClassInstanceRef<Object> = jvm.new_class("CollectionsSortValue", "(IIZ)V", (2, 9, false)).await?.into();
    assert_eq!(
        jvm.invoke_static::<_, i32>(
            "java/util/Collections",
            "binarySearch",
            "(Ljava/util/List;Ljava/lang/Object;Ljava/util/Comparator;)I",
            (reverse_list.clone(), key, comparator),
        )
        .await?,
        1
    );
    let null_comparator: ClassInstanceRef<Object> = None.into();
    let natural_key: ClassInstanceRef<Object> = jvm.new_class("java/lang/Integer", "(I)V", (4,)).await?.into();
    assert_eq!(
        jvm.invoke_static::<_, i32>(
            "java/util/Collections",
            "binarySearch",
            "(Ljava/util/List;Ljava/lang/Object;Ljava/util/Comparator;)I",
            (list, natural_key, null_comparator),
        )
        .await?,
        4
    );

    Ok(())
}

#[tokio::test]
async fn test_coll_04_reverse_fill_copy_use_list_iterator_and_validate_destination() -> Result<()> {
    let jvm = collections_test_jvm().await?;
    let mut elements: ClassInstanceRef<Array<Object>> = jvm.instantiate_array("Ljava/lang/Object;", 4).await?.into();
    let mut values: Vec<ClassInstanceRef<Object>> = vec![];
    for value in [1, 2, 3, 4] {
        values.push(jvm.new_class("java/lang/Integer", "(I)V", (value,)).await?.into());
    }
    jvm.store_array(&mut elements, 0, values).await?;
    let list: ClassInstanceRef<Object> = jvm.new_class("CollectionsProbeList", "([Ljava/lang/Object;)V", (elements,)).await?.into();

    jvm.invoke_static::<_, ()>("java/util/Collections", "reverse", "(Ljava/util/List;)V", (list.clone(),))
        .await?;
    assert_eq!(integer_values(&jvm, &list).await?, vec![4, 3, 2, 1]);
    assert_eq!(jvm.get_field::<i32>(&list, "listIteratorCalls", "I").await?, 2);
    assert_eq!(jvm.get_field::<i32>(&list, "setCalls", "I").await?, 4);

    let fill_value: ClassInstanceRef<Object> = jvm.new_class("java/lang/Integer", "(I)V", (7,)).await?.into();
    jvm.invoke_static::<_, ()>(
        "java/util/Collections",
        "fill",
        "(Ljava/util/List;Ljava/lang/Object;)V",
        (list.clone(), fill_value),
    )
    .await?;
    assert_eq!(integer_values(&jvm, &list).await?, vec![7, 7, 7, 7]);
    assert_eq!(jvm.get_field::<i32>(&list, "listIteratorCalls", "I").await?, 3);

    let source = integer_list(&jvm, &[8, 9]).await?;
    jvm.invoke_static::<_, ()>(
        "java/util/Collections",
        "copy",
        "(Ljava/util/List;Ljava/util/List;)V",
        (list.clone(), source),
    )
    .await?;
    assert_eq!(integer_values(&jvm, &list).await?, vec![8, 9, 7, 7]);
    assert_eq!(jvm.get_field::<i32>(&list, "listIteratorCalls", "I").await?, 4);

    let odd = integer_list(&jvm, &[1, 2, 3, 4, 5]).await?;
    jvm.invoke_static::<_, ()>("java/util/Collections", "reverse", "(Ljava/util/List;)V", (odd.clone(),))
        .await?;
    assert_eq!(integer_values(&jvm, &odd).await?, vec![5, 4, 3, 2, 1]);

    let short = integer_list(&jvm, &[10]).await?;
    let source = integer_list(&jvm, &[1, 2]).await?;
    let result: Result<()> = jvm
        .invoke_static(
            "java/util/Collections",
            "copy",
            "(Ljava/util/List;Ljava/util/List;)V",
            (short.clone(), source),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("short destination must fail");
    };
    assert!(jvm.is_instance(exception.as_ref(), "java/lang/IndexOutOfBoundsException"));
    assert_eq!(integer_values(&jvm, &short).await?, vec![10]);

    Ok(())
}

#[tokio::test]
async fn test_coll_05_shuffle_is_a_reproducible_permutation() -> Result<()> {
    let jvm = test_jvm().await?;
    let first = integer_list(&jvm, &[0, 1, 2, 3, 4, 5, 6, 7]).await?;
    let second = integer_list(&jvm, &[0, 1, 2, 3, 4, 5, 6, 7]).await?;
    let first_random: ClassInstanceRef<Object> = jvm.new_class("java/util/Random", "(J)V", (12345i64,)).await?.into();
    let second_random: ClassInstanceRef<Object> = jvm.new_class("java/util/Random", "(J)V", (12345i64,)).await?.into();

    jvm.invoke_static::<_, ()>(
        "java/util/Collections",
        "shuffle",
        "(Ljava/util/List;Ljava/util/Random;)V",
        (first.clone(), first_random),
    )
    .await?;
    jvm.invoke_static::<_, ()>(
        "java/util/Collections",
        "shuffle",
        "(Ljava/util/List;Ljava/util/Random;)V",
        (second.clone(), second_random),
    )
    .await?;
    let first_values = integer_values(&jvm, &first).await?;
    assert_eq!(first_values, integer_values(&jvm, &second).await?);
    let mut permutation = first_values;
    permutation.sort();
    assert_eq!(permutation, vec![0, 1, 2, 3, 4, 5, 6, 7]);

    let default_shuffle = integer_list(&jvm, &[1, 2, 3, 4]).await?;
    jvm.invoke_static::<_, ()>("java/util/Collections", "shuffle", "(Ljava/util/List;)V", (default_shuffle.clone(),))
        .await?;
    let mut default_values = integer_values(&jvm, &default_shuffle).await?;
    default_values.sort();
    assert_eq!(default_values, vec![1, 2, 3, 4]);

    let null_random: ClassInstanceRef<Object> = None.into();
    for list in [integer_list(&jvm, &[]).await?, integer_list(&jvm, &[1]).await?] {
        jvm.invoke_static::<_, ()>(
            "java/util/Collections",
            "shuffle",
            "(Ljava/util/List;Ljava/util/Random;)V",
            (list, null_random.clone()),
        )
        .await?;
    }
    let list = integer_list(&jvm, &[1, 2]).await?;
    let result: Result<()> = jvm
        .invoke_static(
            "java/util/Collections",
            "shuffle",
            "(Ljava/util/List;Ljava/util/Random;)V",
            (list, null_random),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("shuffle must use a non-null random when a swap is required");
    };
    assert!(jvm.is_instance(exception.as_ref(), "java/lang/NullPointerException"));

    Ok(())
}

#[tokio::test]
async fn test_coll_05_invalid_random_indices_throw_java_exceptions_without_mutation() -> Result<()> {
    let jvm = collections_test_jvm().await?;
    for result in [-1, 3] {
        let list = integer_list(&jvm, &[0, 1, 2]).await?;
        let random: ClassInstanceRef<Object> = jvm.new_class("CollectionsInvalidRandom", "(I)V", (result,)).await?.into();
        let shuffle_result: Result<()> = jvm
            .invoke_static(
                "java/util/Collections",
                "shuffle",
                "(Ljava/util/List;Ljava/util/Random;)V",
                (list.clone(), random),
            )
            .await;
        let Err(JavaError::JavaException(exception)) = shuffle_result else {
            panic!("invalid Random result {result} must throw a Java exception");
        };
        assert!(jvm.is_instance(exception.as_ref(), "java/lang/ArrayIndexOutOfBoundsException"));
        assert_eq!(integer_values(&jvm, &list).await?, vec![0, 1, 2]);
    }

    Ok(())
}

#[tokio::test]
async fn test_coll_06_min_max_natural_comparator_empty_and_null() -> Result<()> {
    let jvm = collections_test_jvm().await?;
    let values = integer_list(&jvm, &[3, -2, 7, 1]).await?;
    let min: ClassInstanceRef<Object> = jvm
        .invoke_static(
            "java/util/Collections",
            "min",
            "(Ljava/util/Collection;)Ljava/lang/Object;",
            (values.clone(),),
        )
        .await?;
    let max: ClassInstanceRef<Object> = jvm
        .invoke_static(
            "java/util/Collections",
            "max",
            "(Ljava/util/Collection;)Ljava/lang/Object;",
            (values.clone(),),
        )
        .await?;
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&min, &min.class_definition().name(), "intValue", "()I", ())
            .await?,
        -2
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&max, &max.class_definition().name(), "intValue", "()I", ())
            .await?,
        7
    );

    let comparable_values: ClassInstanceRef<Object> = jvm.new_class("java/util/ArrayList", "()V", ()).await?.into();
    for key in [1, 3, 2] {
        let value: ClassInstanceRef<Object> = jvm.new_class("CollectionsSortValue", "(IIZ)V", (key, key, false)).await?.into();
        let _: bool = jvm
            .invoke_virtual(
                &comparable_values,
                &comparable_values.class_definition().name(),
                "add",
                "(Ljava/lang/Object;)Z",
                (value,),
            )
            .await?;
    }
    let reverse: ClassInstanceRef<Object> = jvm.new_class("CollectionsComparator", "(ZZ)V", (true, false)).await?.into();
    let comparator_min: ClassInstanceRef<Object> = jvm
        .invoke_static(
            "java/util/Collections",
            "min",
            "(Ljava/util/Collection;Ljava/util/Comparator;)Ljava/lang/Object;",
            (comparable_values.clone(), reverse.clone()),
        )
        .await?;
    let comparator_max: ClassInstanceRef<Object> = jvm
        .invoke_static(
            "java/util/Collections",
            "max",
            "(Ljava/util/Collection;Ljava/util/Comparator;)Ljava/lang/Object;",
            (comparable_values, reverse),
        )
        .await?;
    assert_eq!(jvm.get_field::<i32>(&comparator_min, "key", "I").await?, 3);
    assert_eq!(jvm.get_field::<i32>(&comparator_max, "key", "I").await?, 1);

    let empty = integer_list(&jvm, &[]).await?;
    for name in ["min", "max"] {
        let result: Result<ClassInstanceRef<Object>> = jvm
            .invoke_static(
                "java/util/Collections",
                name,
                "(Ljava/util/Collection;)Ljava/lang/Object;",
                (empty.clone(),),
            )
            .await;
        let Err(JavaError::JavaException(exception)) = result else {
            panic!("{name} on an empty collection must fail");
        };
        assert!(jvm.is_instance(exception.as_ref(), "java/util/NoSuchElementException"));
    }

    let nulls: ClassInstanceRef<Object> = jvm.new_class("java/util/ArrayList", "()V", ()).await?.into();
    let null: ClassInstanceRef<Object> = None.into();
    let value: ClassInstanceRef<Object> = jvm.new_class("java/lang/Integer", "(I)V", (1,)).await?.into();
    let _: bool = jvm
        .invoke_virtual(&nulls, &nulls.class_definition().name(), "add", "(Ljava/lang/Object;)Z", (null,))
        .await?;
    let _: bool = jvm
        .invoke_virtual(&nulls, &nulls.class_definition().name(), "add", "(Ljava/lang/Object;)Z", (value,))
        .await?;
    let result: Result<ClassInstanceRef<Object>> = jvm
        .invoke_static("java/util/Collections", "min", "(Ljava/util/Collection;)Ljava/lang/Object;", (nulls,))
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("natural min with null must fail");
    };
    assert!(jvm.is_instance(exception.as_ref(), "java/lang/NullPointerException"));

    Ok(())
}

#[tokio::test]
async fn test_coll_07_ncopies_singleton_and_empty_are_immutable() -> Result<()> {
    let jvm = test_jvm().await?;
    let element: ClassInstanceRef<Object> = jvm.new_class("java/lang/Integer", "(I)V", (11,)).await?.into();
    let copies: ClassInstanceRef<Object> = jvm
        .invoke_static(
            "java/util/Collections",
            "nCopies",
            "(ILjava/lang/Object;)Ljava/util/List;",
            (3, element.clone()),
        )
        .await?;
    assert_eq!(integer_values(&jvm, &copies).await?, vec![11, 11, 11]);
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(
            &copies,
            &copies.class_definition().name(),
            "indexOf",
            "(Ljava/lang/Object;)I",
            (element.clone(),)
        )
        .await?,
        0
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(
            &copies,
            &copies.class_definition().name(),
            "lastIndexOf",
            "(Ljava/lang/Object;)I",
            (element.clone(),)
        )
        .await?,
        2
    );

    let negative: Result<ClassInstanceRef<Object>> = jvm
        .invoke_static(
            "java/util/Collections",
            "nCopies",
            "(ILjava/lang/Object;)Ljava/util/List;",
            (-1, element.clone()),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = negative else {
        panic!("negative nCopies count must fail");
    };
    assert!(jvm.is_instance(exception.as_ref(), "java/lang/IllegalArgumentException"));

    let add_result: Result<bool> = jvm
        .invoke_virtual(
            &copies,
            &copies.class_definition().name(),
            "add",
            "(Ljava/lang/Object;)Z",
            (element.clone(),),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = add_result else {
        panic!("nCopies.add must fail");
    };
    assert!(jvm.is_instance(exception.as_ref(), "java/lang/UnsupportedOperationException"));

    let clear_result: Result<()> = jvm.invoke_virtual(&copies, &copies.class_definition().name(), "clear", "()V", ()).await;
    let Err(JavaError::JavaException(exception)) = clear_result else {
        panic!("non-empty nCopies.clear must fail");
    };
    assert!(jvm.is_instance(exception.as_ref(), "java/lang/UnsupportedOperationException"));
    let zero_copies: ClassInstanceRef<Object> = jvm
        .invoke_static(
            "java/util/Collections",
            "nCopies",
            "(ILjava/lang/Object;)Ljava/util/List;",
            (0, element.clone()),
        )
        .await?;
    let _: () = jvm
        .invoke_virtual(&zero_copies, &zero_copies.class_definition().name(), "clear", "()V", ())
        .await?;

    let iterator: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&copies, &copies.class_definition().name(), "iterator", "()Ljava/util/Iterator;", ())
        .await?;
    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&iterator, &iterator.class_definition().name(), "next", "()Ljava/lang/Object;", ())
        .await?;
    let remove_result: Result<()> = jvm
        .invoke_virtual(&iterator, &iterator.class_definition().name(), "remove", "()V", ())
        .await;
    let Err(JavaError::JavaException(exception)) = remove_result else {
        panic!("nCopies iterator.remove must fail");
    };
    assert!(jvm.is_instance(exception.as_ref(), "java/lang/UnsupportedOperationException"));

    let list_iterator: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &copies,
            &copies.class_definition().name(),
            "listIterator",
            "()Ljava/util/ListIterator;",
            (),
        )
        .await?;
    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &list_iterator,
            &list_iterator.class_definition().name(),
            "next",
            "()Ljava/lang/Object;",
            (),
        )
        .await?;
    for (name, descriptor) in [("set", "(Ljava/lang/Object;)V"), ("add", "(Ljava/lang/Object;)V")] {
        let result: Result<()> = jvm
            .invoke_virtual(
                &list_iterator,
                &list_iterator.class_definition().name(),
                name,
                descriptor,
                (element.clone(),),
            )
            .await;
        let Err(JavaError::JavaException(exception)) = result else {
            panic!("nCopies listIterator.{name} must fail");
        };
        assert!(jvm.is_instance(exception.as_ref(), "java/lang/UnsupportedOperationException"));
    }

    let singleton: ClassInstanceRef<Object> = jvm
        .invoke_static(
            "java/util/Collections",
            "singleton",
            "(Ljava/lang/Object;)Ljava/util/Set;",
            (element.clone(),),
        )
        .await?;
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&singleton, &singleton.class_definition().name(), "size", "()I", ())
            .await?,
        1
    );
    assert!(
        jvm.invoke_virtual::<_, bool>(
            &singleton,
            &singleton.class_definition().name(),
            "contains",
            "(Ljava/lang/Object;)Z",
            (element.clone(),)
        )
        .await?
    );
    let singleton_iterator: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&singleton, &singleton.class_definition().name(), "iterator", "()Ljava/util/Iterator;", ())
        .await?;
    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &singleton_iterator,
            &singleton_iterator.class_definition().name(),
            "next",
            "()Ljava/lang/Object;",
            (),
        )
        .await?;
    let remove_result: Result<()> = jvm
        .invoke_virtual(&singleton_iterator, &singleton_iterator.class_definition().name(), "remove", "()V", ())
        .await;
    let Err(JavaError::JavaException(exception)) = remove_result else {
        panic!("singleton iterator.remove must fail");
    };
    assert!(jvm.is_instance(exception.as_ref(), "java/lang/UnsupportedOperationException"));

    let singleton_clear: Result<()> = jvm
        .invoke_virtual(&singleton, &singleton.class_definition().name(), "clear", "()V", ())
        .await;
    let Err(JavaError::JavaException(exception)) = singleton_clear else {
        panic!("singleton.clear must fail");
    };
    assert!(jvm.is_instance(exception.as_ref(), "java/lang/UnsupportedOperationException"));
    let absent: ClassInstanceRef<Object> = jvm.new_class("java/lang/Integer", "(I)V", (99,)).await?.into();
    assert!(
        !jvm.invoke_virtual::<_, bool>(
            &singleton,
            &singleton.class_definition().name(),
            "remove",
            "(Ljava/lang/Object;)Z",
            (absent,)
        )
        .await?
    );

    let one_copy: ClassInstanceRef<Object> = jvm
        .invoke_static(
            "java/util/Collections",
            "nCopies",
            "(ILjava/lang/Object;)Ljava/util/List;",
            (1, element.clone()),
        )
        .await?;
    for target in [zero_copies.clone(), one_copy] {
        jvm.invoke_static::<_, ()>("java/util/Collections", "reverse", "(Ljava/util/List;)V", (target,))
            .await?;
    }

    let empty: ClassInstanceRef<Object> = jvm.get_static_field("java/util/Collections", "EMPTY_LIST", "Ljava/util/List;").await?;
    let _: () = jvm.invoke_virtual(&empty, &empty.class_definition().name(), "clear", "()V", ()).await?;
    assert!(
        !jvm.invoke_virtual::<_, bool>(
            &empty,
            &empty.class_definition().name(),
            "remove",
            "(Ljava/lang/Object;)Z",
            (element.clone(),)
        )
        .await?
    );
    let add_result: Result<bool> = jvm
        .invoke_virtual(
            &empty,
            &empty.class_definition().name(),
            "add",
            "(Ljava/lang/Object;)Z",
            (element.clone(),),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = add_result else {
        panic!("EMPTY_LIST.add must fail");
    };
    assert!(jvm.is_instance(exception.as_ref(), "java/lang/UnsupportedOperationException"));

    let empty_set: ClassInstanceRef<Object> = jvm.get_static_field("java/util/Collections", "EMPTY_SET", "Ljava/util/Set;").await?;
    let _: () = jvm
        .invoke_virtual(&empty_set, &empty_set.class_definition().name(), "clear", "()V", ())
        .await?;
    let add_result: Result<bool> = jvm
        .invoke_virtual(
            &empty_set,
            &empty_set.class_definition().name(),
            "add",
            "(Ljava/lang/Object;)Z",
            (element.clone(),),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = add_result else {
        panic!("EMPTY_SET.add must fail");
    };
    assert!(jvm.is_instance(exception.as_ref(), "java/lang/UnsupportedOperationException"));

    let removal_source = integer_list(&jvm, &[11]).await?;
    for (target, name, descriptor, argument) in [
        (copies.clone(), "addAll", "(Ljava/util/Collection;)Z", removal_source.clone()),
        (copies.clone(), "removeAll", "(Ljava/util/Collection;)Z", removal_source.clone()),
        (copies.clone(), "retainAll", "(Ljava/util/Collection;)Z", empty.clone()),
        (singleton.clone(), "removeAll", "(Ljava/util/Collection;)Z", removal_source),
    ] {
        let result: Result<bool> = jvm
            .invoke_virtual(&target, &target.class_definition().name(), name, descriptor, (argument,))
            .await;
        let Err(JavaError::JavaException(exception)) = result else {
            panic!("{name} requiring mutation must fail");
        };
        assert!(jvm.is_instance(exception.as_ref(), "java/lang/UnsupportedOperationException"));
    }

    assert!(
        !jvm.invoke_virtual::<_, bool>(
            &copies,
            &copies.class_definition().name(),
            "addAll",
            "(Ljava/util/Collection;)Z",
            (zero_copies.clone(),)
        )
        .await?
    );

    let text = JavaLangString::from_rust_string(&jvm, "text").await?;
    let text_copies: ClassInstanceRef<Object> = jvm
        .invoke_static(
            "java/util/Collections",
            "nCopies",
            "(ILjava/lang/Object;)Ljava/util/List;",
            (2, text.clone()),
        )
        .await?;
    let destination: ClassInstanceRef<Array<Object>> = jvm.instantiate_array("Ljava/lang/String;", 3).await?.into();
    let typed: ClassInstanceRef<Array<Object>> = jvm
        .invoke_virtual(
            &text_copies,
            &text_copies.class_definition().name(),
            "toArray",
            "([Ljava/lang/Object;)[Ljava/lang/Object;",
            (destination,),
        )
        .await?;
    let typed_values = jvm.load_array::<ClassInstanceRef<Object>>(&typed, 0, 3).await?;
    assert_eq!(typed_values[0].identity(), text.identity());
    assert_eq!(typed_values[1].identity(), text.identity());
    assert!(typed_values[2].is_null());

    let null: ClassInstanceRef<Object> = None.into();
    let null_copies: ClassInstanceRef<Object> = jvm
        .invoke_static(
            "java/util/Collections",
            "nCopies",
            "(ILjava/lang/Object;)Ljava/util/List;",
            (2, null.clone()),
        )
        .await?;
    assert!(
        jvm.invoke_virtual::<_, ClassInstanceRef<Object>>(&null_copies, &null_copies.class_definition().name(), "get", "(I)Ljava/lang/Object;", (1,))
            .await?
            .is_null()
    );
    let null_singleton: ClassInstanceRef<Object> = jvm
        .invoke_static(
            "java/util/Collections",
            "singleton",
            "(Ljava/lang/Object;)Ljava/util/Set;",
            (null.clone(),),
        )
        .await?;
    assert!(
        jvm.invoke_virtual::<_, bool>(
            &null_singleton,
            &null_singleton.class_definition().name(),
            "contains",
            "(Ljava/lang/Object;)Z",
            (null,)
        )
        .await?
    );

    Ok(())
}

#[tokio::test]
async fn test_coll_07_immutable_equals_and_hash_code_match_standard_collections() -> Result<()> {
    let jvm = test_jvm().await?;
    let null: ClassInstanceRef<Object> = None.into();
    let empty_list: ClassInstanceRef<Object> = jvm.get_static_field("java/util/Collections", "EMPTY_LIST", "Ljava/util/List;").await?;
    let empty_set: ClassInstanceRef<Object> = jvm.get_static_field("java/util/Collections", "EMPTY_SET", "Ljava/util/Set;").await?;
    let standard_empty_list = integer_list(&jvm, &[]).await?;
    let standard_empty_set: ClassInstanceRef<Object> = jvm.new_class("java/util/HashSet", "()V", ()).await?.into();

    for collection in [empty_list.clone(), empty_set.clone()] {
        assert!(
            !jvm.invoke_virtual::<_, bool>(
                &collection,
                &collection.class_definition().name(),
                "equals",
                "(Ljava/lang/Object;)Z",
                (null.clone(),)
            )
            .await?
        );
        let first_hash: i32 = jvm
            .invoke_virtual(&collection, &collection.class_definition().name(), "hashCode", "()I", ())
            .await?;
        assert_eq!(
            jvm.invoke_virtual::<_, i32>(&collection, &collection.class_definition().name(), "hashCode", "()I", ())
                .await?,
            first_hash
        );
    }
    assert!(
        jvm.invoke_virtual::<_, bool>(
            &empty_list,
            &empty_list.class_definition().name(),
            "equals",
            "(Ljava/lang/Object;)Z",
            (standard_empty_list.clone(),),
        )
        .await?
    );
    assert!(
        jvm.invoke_virtual::<_, bool>(
            &standard_empty_list,
            &standard_empty_list.class_definition().name(),
            "equals",
            "(Ljava/lang/Object;)Z",
            (empty_list.clone(),),
        )
        .await?
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&empty_list, &empty_list.class_definition().name(), "hashCode", "()I", ())
            .await?,
        1
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(
            &standard_empty_list,
            &standard_empty_list.class_definition().name(),
            "hashCode",
            "()I",
            ()
        )
        .await?,
        1
    );

    assert!(
        jvm.invoke_virtual::<_, bool>(
            &empty_set,
            &empty_set.class_definition().name(),
            "equals",
            "(Ljava/lang/Object;)Z",
            (standard_empty_set.clone(),),
        )
        .await?
    );
    assert!(
        jvm.invoke_virtual::<_, bool>(
            &standard_empty_set,
            &standard_empty_set.class_definition().name(),
            "equals",
            "(Ljava/lang/Object;)Z",
            (empty_set.clone(),),
        )
        .await?
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&empty_set, &empty_set.class_definition().name(), "hashCode", "()I", ())
            .await?,
        0
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&standard_empty_set, &standard_empty_set.class_definition().name(), "hashCode", "()I", ())
            .await?,
        0
    );

    let element: ClassInstanceRef<Object> = jvm.new_class("java/lang/Integer", "(I)V", (11,)).await?.into();
    let element_hash: i32 = jvm
        .invoke_virtual(&element, &element.class_definition().name(), "hashCode", "()I", ())
        .await?;
    let copies: ClassInstanceRef<Object> = jvm
        .invoke_static(
            "java/util/Collections",
            "nCopies",
            "(ILjava/lang/Object;)Ljava/util/List;",
            (2, element.clone()),
        )
        .await?;
    let peer_list = integer_list(&jvm, &[11, 11]).await?;
    assert!(
        !jvm.invoke_virtual::<_, bool>(
            &copies,
            &copies.class_definition().name(),
            "equals",
            "(Ljava/lang/Object;)Z",
            (null.clone(),)
        )
        .await?
    );
    assert!(
        jvm.invoke_virtual::<_, bool>(
            &copies,
            &copies.class_definition().name(),
            "equals",
            "(Ljava/lang/Object;)Z",
            (peer_list.clone(),)
        )
        .await?
    );
    assert!(
        jvm.invoke_virtual::<_, bool>(
            &peer_list,
            &peer_list.class_definition().name(),
            "equals",
            "(Ljava/lang/Object;)Z",
            (copies.clone(),)
        )
        .await?
    );
    let copies_hash: i32 = jvm
        .invoke_virtual(&copies, &copies.class_definition().name(), "hashCode", "()I", ())
        .await?;
    assert_eq!(copies_hash, 31i32.wrapping_mul(31 + element_hash).wrapping_add(element_hash));
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&copies, &copies.class_definition().name(), "hashCode", "()I", ())
            .await?,
        copies_hash
    );
    let one_copy: ClassInstanceRef<Object> = jvm
        .invoke_static(
            "java/util/Collections",
            "nCopies",
            "(ILjava/lang/Object;)Ljava/util/List;",
            (1, element.clone()),
        )
        .await?;
    assert!(
        !jvm.invoke_virtual::<_, bool>(
            &copies,
            &copies.class_definition().name(),
            "equals",
            "(Ljava/lang/Object;)Z",
            (one_copy.clone(),)
        )
        .await?
    );
    assert_ne!(
        copies_hash,
        jvm.invoke_virtual::<_, i32>(&one_copy, &one_copy.class_definition().name(), "hashCode", "()I", ())
            .await?
    );

    let singleton: ClassInstanceRef<Object> = jvm
        .invoke_static(
            "java/util/Collections",
            "singleton",
            "(Ljava/lang/Object;)Ljava/util/Set;",
            (element.clone(),),
        )
        .await?;
    let peer_set: ClassInstanceRef<Object> = jvm.new_class("java/util/HashSet", "()V", ()).await?.into();
    let peer_element: ClassInstanceRef<Object> = jvm.new_class("java/lang/Integer", "(I)V", (11,)).await?.into();
    let _: bool = jvm
        .invoke_virtual(
            &peer_set,
            &peer_set.class_definition().name(),
            "add",
            "(Ljava/lang/Object;)Z",
            (peer_element,),
        )
        .await?;
    assert!(
        !jvm.invoke_virtual::<_, bool>(
            &singleton,
            &singleton.class_definition().name(),
            "equals",
            "(Ljava/lang/Object;)Z",
            (null.clone(),)
        )
        .await?
    );
    assert!(
        jvm.invoke_virtual::<_, bool>(
            &singleton,
            &singleton.class_definition().name(),
            "equals",
            "(Ljava/lang/Object;)Z",
            (peer_set.clone(),)
        )
        .await?
    );
    assert!(
        jvm.invoke_virtual::<_, bool>(
            &peer_set,
            &peer_set.class_definition().name(),
            "equals",
            "(Ljava/lang/Object;)Z",
            (singleton.clone(),)
        )
        .await?
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&singleton, &singleton.class_definition().name(), "hashCode", "()I", ())
            .await?,
        element_hash
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&singleton, &singleton.class_definition().name(), "hashCode", "()I", ())
            .await?,
        jvm.invoke_virtual::<_, i32>(&peer_set, &peer_set.class_definition().name(), "hashCode", "()I", ())
            .await?
    );

    let null_copies: ClassInstanceRef<Object> = jvm
        .invoke_static(
            "java/util/Collections",
            "nCopies",
            "(ILjava/lang/Object;)Ljava/util/List;",
            (2, null.clone()),
        )
        .await?;
    let null_peer_list: ClassInstanceRef<Object> = jvm.new_class("java/util/ArrayList", "()V", ()).await?.into();
    for _ in 0..2 {
        let _: bool = jvm
            .invoke_virtual(
                &null_peer_list,
                &null_peer_list.class_definition().name(),
                "add",
                "(Ljava/lang/Object;)Z",
                (null.clone(),),
            )
            .await?;
    }
    assert!(
        jvm.invoke_virtual::<_, bool>(
            &null_copies,
            &null_copies.class_definition().name(),
            "equals",
            "(Ljava/lang/Object;)Z",
            (null_peer_list.clone(),),
        )
        .await?
    );
    assert!(
        jvm.invoke_virtual::<_, bool>(
            &null_peer_list,
            &null_peer_list.class_definition().name(),
            "equals",
            "(Ljava/lang/Object;)Z",
            (null_copies.clone(),),
        )
        .await?
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&null_copies, &null_copies.class_definition().name(), "hashCode", "()I", ())
            .await?,
        961
    );

    let null_singleton: ClassInstanceRef<Object> = jvm
        .invoke_static(
            "java/util/Collections",
            "singleton",
            "(Ljava/lang/Object;)Ljava/util/Set;",
            (null.clone(),),
        )
        .await?;
    let null_peer_set: ClassInstanceRef<Object> = jvm.new_class("java/util/HashSet", "()V", ()).await?.into();
    let _: bool = jvm
        .invoke_virtual(
            &null_peer_set,
            &null_peer_set.class_definition().name(),
            "add",
            "(Ljava/lang/Object;)Z",
            (null.clone(),),
        )
        .await?;
    assert!(
        jvm.invoke_virtual::<_, bool>(
            &null_singleton,
            &null_singleton.class_definition().name(),
            "equals",
            "(Ljava/lang/Object;)Z",
            (null_peer_set.clone(),),
        )
        .await?
    );
    assert!(
        jvm.invoke_virtual::<_, bool>(
            &null_peer_set,
            &null_peer_set.class_definition().name(),
            "equals",
            "(Ljava/lang/Object;)Z",
            (null_singleton.clone(),),
        )
        .await?
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&null_singleton, &null_singleton.class_definition().name(), "hashCode", "()I", ())
            .await?,
        0
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&null_peer_set, &null_peer_set.class_definition().name(), "hashCode", "()I", ())
            .await?,
        0
    );

    Ok(())
}

#[tokio::test]
async fn test_coll_07_abstract_set_equals_catches_only_jdk_compatibility_exceptions() -> Result<()> {
    let jvm = collections_test_jvm().await?;
    let value: ClassInstanceRef<Object> = jvm.new_class("java/lang/Integer", "(I)V", (7,)).await?.into();
    let peer: ClassInstanceRef<Object> = jvm
        .invoke_static(
            "java/util/Collections",
            "singleton",
            "(Ljava/lang/Object;)Ljava/util/Set;",
            (value.clone(),),
        )
        .await?;

    let incompatible: ClassInstanceRef<Object> = jvm
        .new_class("CollectionsExceptionalSet", "(Ljava/lang/Object;I)V", (value.clone(), 0))
        .await?
        .into();
    assert!(
        !jvm.invoke_virtual::<_, bool>(
            &incompatible,
            &incompatible.class_definition().name(),
            "equals",
            "(Ljava/lang/Object;)Z",
            (peer.clone(),)
        )
        .await?
    );
    assert!(
        jvm.invoke_virtual::<_, bool>(&peer, &peer.class_definition().name(), "equals", "(Ljava/lang/Object;)Z", (incompatible,))
            .await?
    );

    let null: ClassInstanceRef<Object> = None.into();
    let null_peer: ClassInstanceRef<Object> = jvm
        .invoke_static(
            "java/util/Collections",
            "singleton",
            "(Ljava/lang/Object;)Ljava/util/Set;",
            (null.clone(),),
        )
        .await?;
    let null_rejecting: ClassInstanceRef<Object> = jvm
        .new_class("CollectionsExceptionalSet", "(Ljava/lang/Object;I)V", (null.clone(), 1))
        .await?
        .into();
    assert!(
        !jvm.invoke_virtual::<_, bool>(
            &null_rejecting,
            &null_rejecting.class_definition().name(),
            "equals",
            "(Ljava/lang/Object;)Z",
            (null_peer.clone(),),
        )
        .await?
    );
    assert!(
        jvm.invoke_virtual::<_, bool>(
            &null_peer,
            &null_peer.class_definition().name(),
            "equals",
            "(Ljava/lang/Object;)Z",
            (null_rejecting,)
        )
        .await?
    );

    let unexpected: ClassInstanceRef<Object> = jvm
        .new_class("CollectionsExceptionalSet", "(Ljava/lang/Object;I)V", (value, 2))
        .await?
        .into();
    let result: Result<bool> = jvm
        .invoke_virtual(
            &unexpected,
            &unexpected.class_definition().name(),
            "equals",
            "(Ljava/lang/Object;)Z",
            (peer,),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("AbstractSet.equals must propagate non-compatibility exceptions");
    };
    assert!(jvm.is_instance(exception.as_ref(), "java/lang/IllegalStateException"));

    Ok(())
}

#[tokio::test]
async fn test_coll_07_singleton_set_equals_uses_other_element_direction() -> Result<()> {
    let jvm = collections_test_jvm().await?;
    let accepts: ClassInstanceRef<Object> = jvm.new_class("CollectionsAsymmetricEquals", "(Z)V", (true,)).await?.into();
    let rejects: ClassInstanceRef<Object> = jvm.new_class("CollectionsAsymmetricEquals", "(Z)V", (false,)).await?.into();

    let accepting_singleton: ClassInstanceRef<Object> = jvm
        .invoke_static(
            "java/util/Collections",
            "singleton",
            "(Ljava/lang/Object;)Ljava/util/Set;",
            (accepts.clone(),),
        )
        .await?;
    let rejecting_peer: ClassInstanceRef<Object> = jvm
        .new_class("CollectionsExceptionalSet", "(Ljava/lang/Object;I)V", (rejects.clone(), 3))
        .await?
        .into();
    assert!(
        !jvm.invoke_virtual::<_, bool>(
            &accepting_singleton,
            &accepting_singleton.class_definition().name(),
            "equals",
            "(Ljava/lang/Object;)Z",
            (rejecting_peer,),
        )
        .await?
    );

    let rejecting_singleton: ClassInstanceRef<Object> = jvm
        .invoke_static("java/util/Collections", "singleton", "(Ljava/lang/Object;)Ljava/util/Set;", (rejects,))
        .await?;
    let accepting_peer: ClassInstanceRef<Object> = jvm
        .new_class("CollectionsExceptionalSet", "(Ljava/lang/Object;I)V", (accepts, 3))
        .await?
        .into();
    assert!(
        jvm.invoke_virtual::<_, bool>(
            &rejecting_singleton,
            &rejecting_singleton.class_definition().name(),
            "equals",
            "(Ljava/lang/Object;)Z",
            (accepting_peer,),
        )
        .await?
    );

    Ok(())
}

#[tokio::test]
async fn test_coll_08_exact_descriptors_access_and_null_factories() -> Result<()> {
    let jvm = test_jvm().await?;
    let collections = jvm.resolve_class("java/util/Collections").await?;
    let factories = [
        ("unmodifiableCollection", "(Ljava/util/Collection;)Ljava/util/Collection;"),
        ("unmodifiableList", "(Ljava/util/List;)Ljava/util/List;"),
        ("unmodifiableSet", "(Ljava/util/Set;)Ljava/util/Set;"),
        ("unmodifiableMap", "(Ljava/util/Map;)Ljava/util/Map;"),
        ("unmodifiableSortedSet", "(Ljava/util/SortedSet;)Ljava/util/SortedSet;"),
        ("unmodifiableSortedMap", "(Ljava/util/SortedMap;)Ljava/util/SortedMap;"),
    ];
    for (name, descriptor) in factories {
        let method = collections
            .definition
            .method(name, descriptor, true)
            .unwrap_or_else(|| panic!("missing Collections.{name}{descriptor}"));
        assert_eq!(method.access_flags(), MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC);

        let null: ClassInstanceRef<Object> = None.into();
        let result: Result<ClassInstanceRef<Object>> = jvm.invoke_static("java/util/Collections", name, descriptor, (null,)).await;
        let Err(JavaError::JavaException(exception)) = result else {
            panic!("Collections.{name}(null) must fail");
        };
        assert!(jvm.is_instance(exception.as_ref(), "java/lang/NullPointerException"));
    }

    type WrapperShape<'a> = (
        &'a str,
        &'a str,
        &'a [&'a str],
        &'a str,
        &'a [(&'a str, &'a str, FieldAccessFlags)],
        &'a [(&'a str, &'a str)],
        bool,
    );
    let wrapper_shapes: &[WrapperShape<'_>] = &[
        (
            "java/util/Collections$UnmodifiableCollection",
            "java/lang/Object",
            &["java/util/Collection", "java/io/Serializable"],
            "(Ljava/util/Collection;)V",
            &[("c", "Ljava/util/Collection;", FieldAccessFlags::FINAL)],
            &[
                ("size", "()I"),
                ("isEmpty", "()Z"),
                ("contains", "(Ljava/lang/Object;)Z"),
                ("iterator", "()Ljava/util/Iterator;"),
                ("toArray", "()[Ljava/lang/Object;"),
                ("toArray", "([Ljava/lang/Object;)[Ljava/lang/Object;"),
                ("containsAll", "(Ljava/util/Collection;)Z"),
                ("toString", "()Ljava/lang/String;"),
                ("add", "(Ljava/lang/Object;)Z"),
                ("remove", "(Ljava/lang/Object;)Z"),
                ("addAll", "(Ljava/util/Collection;)Z"),
                ("removeAll", "(Ljava/util/Collection;)Z"),
                ("retainAll", "(Ljava/util/Collection;)Z"),
                ("clear", "()V"),
            ],
            true,
        ),
        (
            "java/util/Collections$UnmodifiableCollection$1",
            "java/lang/Object",
            &["java/util/Iterator"],
            "(Ljava/util/Iterator;)V",
            &[("i", "Ljava/util/Iterator;", FieldAccessFlags::PRIVATE | FieldAccessFlags::FINAL)],
            &[("hasNext", "()Z"), ("next", "()Ljava/lang/Object;"), ("remove", "()V")],
            false,
        ),
        (
            "java/util/Collections$UnmodifiableList",
            "java/util/Collections$UnmodifiableCollection",
            &["java/util/List"],
            "(Ljava/util/List;)V",
            &[("list", "Ljava/util/List;", FieldAccessFlags::FINAL)],
            &[
                ("equals", "(Ljava/lang/Object;)Z"),
                ("hashCode", "()I"),
                ("get", "(I)Ljava/lang/Object;"),
                ("indexOf", "(Ljava/lang/Object;)I"),
                ("lastIndexOf", "(Ljava/lang/Object;)I"),
                ("listIterator", "()Ljava/util/ListIterator;"),
                ("listIterator", "(I)Ljava/util/ListIterator;"),
                ("subList", "(II)Ljava/util/List;"),
                ("set", "(ILjava/lang/Object;)Ljava/lang/Object;"),
                ("add", "(ILjava/lang/Object;)V"),
                ("addAll", "(ILjava/util/Collection;)Z"),
                ("remove", "(I)Ljava/lang/Object;"),
            ],
            true,
        ),
        (
            "java/util/Collections$UnmodifiableList$1",
            "java/lang/Object",
            &["java/util/ListIterator"],
            "(Ljava/util/ListIterator;)V",
            &[("i", "Ljava/util/ListIterator;", FieldAccessFlags::PRIVATE | FieldAccessFlags::FINAL)],
            &[
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
            false,
        ),
        (
            "java/util/Collections$UnmodifiableSet",
            "java/util/Collections$UnmodifiableCollection",
            &["java/util/Set", "java/io/Serializable"],
            "(Ljava/util/Set;)V",
            &[],
            &[("equals", "(Ljava/lang/Object;)Z"), ("hashCode", "()I")],
            true,
        ),
        (
            "java/util/Collections$UnmodifiableSortedSet",
            "java/util/Collections$UnmodifiableSet",
            &["java/util/SortedSet", "java/io/Serializable"],
            "(Ljava/util/SortedSet;)V",
            &[("ss", "Ljava/util/SortedSet;", FieldAccessFlags::PRIVATE | FieldAccessFlags::FINAL)],
            &[
                ("comparator", "()Ljava/util/Comparator;"),
                ("first", "()Ljava/lang/Object;"),
                ("last", "()Ljava/lang/Object;"),
                ("subSet", "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/SortedSet;"),
                ("headSet", "(Ljava/lang/Object;)Ljava/util/SortedSet;"),
                ("tailSet", "(Ljava/lang/Object;)Ljava/util/SortedSet;"),
            ],
            true,
        ),
        (
            "java/util/Collections$UnmodifiableMap",
            "java/lang/Object",
            &["java/util/Map", "java/io/Serializable"],
            "(Ljava/util/Map;)V",
            &[
                ("m", "Ljava/util/Map;", FieldAccessFlags::PRIVATE | FieldAccessFlags::FINAL),
                ("keySet", "Ljava/util/Set;", FieldAccessFlags::PRIVATE | FieldAccessFlags::TRANSIENT),
                ("entrySet", "Ljava/util/Set;", FieldAccessFlags::PRIVATE | FieldAccessFlags::TRANSIENT),
                (
                    "values",
                    "Ljava/util/Collection;",
                    FieldAccessFlags::PRIVATE | FieldAccessFlags::TRANSIENT,
                ),
            ],
            &[
                ("size", "()I"),
                ("isEmpty", "()Z"),
                ("containsKey", "(Ljava/lang/Object;)Z"),
                ("containsValue", "(Ljava/lang/Object;)Z"),
                ("get", "(Ljava/lang/Object;)Ljava/lang/Object;"),
                ("keySet", "()Ljava/util/Set;"),
                ("values", "()Ljava/util/Collection;"),
                ("entrySet", "()Ljava/util/Set;"),
                ("equals", "(Ljava/lang/Object;)Z"),
                ("hashCode", "()I"),
                ("toString", "()Ljava/lang/String;"),
                ("put", "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"),
                ("remove", "(Ljava/lang/Object;)Ljava/lang/Object;"),
                ("putAll", "(Ljava/util/Map;)V"),
                ("clear", "()V"),
            ],
            true,
        ),
        (
            "java/util/Collections$UnmodifiableMap$UnmodifiableEntrySet",
            "java/util/Collections$UnmodifiableSet",
            &[],
            "(Ljava/util/Set;)V",
            &[],
            &[
                ("iterator", "()Ljava/util/Iterator;"),
                ("toArray", "()[Ljava/lang/Object;"),
                ("toArray", "([Ljava/lang/Object;)[Ljava/lang/Object;"),
                ("contains", "(Ljava/lang/Object;)Z"),
                ("containsAll", "(Ljava/util/Collection;)Z"),
                ("equals", "(Ljava/lang/Object;)Z"),
            ],
            true,
        ),
        (
            "java/util/Collections$UnmodifiableMap$UnmodifiableEntrySet$1",
            "java/lang/Object",
            &["java/util/Iterator"],
            "(Ljava/util/Iterator;)V",
            &[("i", "Ljava/util/Iterator;", FieldAccessFlags::PRIVATE | FieldAccessFlags::FINAL)],
            &[("hasNext", "()Z"), ("next", "()Ljava/lang/Object;"), ("remove", "()V")],
            false,
        ),
        (
            "java/util/Collections$UnmodifiableMap$UnmodifiableEntrySet$UnmodifiableEntry",
            "java/lang/Object",
            &["java/util/Map$Entry"],
            "(Ljava/util/Map$Entry;)V",
            &[("e", "Ljava/util/Map$Entry;", FieldAccessFlags::PRIVATE | FieldAccessFlags::FINAL)],
            &[
                ("getKey", "()Ljava/lang/Object;"),
                ("getValue", "()Ljava/lang/Object;"),
                ("setValue", "(Ljava/lang/Object;)Ljava/lang/Object;"),
                ("equals", "(Ljava/lang/Object;)Z"),
                ("hashCode", "()I"),
                ("toString", "()Ljava/lang/String;"),
            ],
            false,
        ),
        (
            "java/util/Collections$UnmodifiableSortedMap",
            "java/util/Collections$UnmodifiableMap",
            &["java/util/SortedMap", "java/io/Serializable"],
            "(Ljava/util/SortedMap;)V",
            &[("sm", "Ljava/util/SortedMap;", FieldAccessFlags::PRIVATE | FieldAccessFlags::FINAL)],
            &[
                ("comparator", "()Ljava/util/Comparator;"),
                ("firstKey", "()Ljava/lang/Object;"),
                ("lastKey", "()Ljava/lang/Object;"),
                ("subMap", "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/SortedMap;"),
                ("headMap", "(Ljava/lang/Object;)Ljava/util/SortedMap;"),
                ("tailMap", "(Ljava/lang/Object;)Ljava/util/SortedMap;"),
            ],
            true,
        ),
    ];
    let serializable = jvm.resolve_class("java/io/Serializable").await?.java_class();
    for (name, parent, interfaces, constructor, fields, methods, is_serializable) in wrapper_shapes {
        let proto = get_runtime_class_proto(name).unwrap_or_else(|| panic!("missing {name}"));
        assert_eq!(proto.parent_class, Some(*parent), "{name} superclass");
        assert_eq!(proto.interfaces.as_slice(), *interfaces, "{name} direct interfaces");
        assert_eq!(proto.access_flags, ClassAccessFlags::default(), "{name} access");
        assert_eq!(proto.fields.len(), fields.len(), "{name} field count");
        for (field_name, descriptor, access_flags) in *fields {
            let field = proto
                .fields
                .iter()
                .find(|field| field.name == *field_name && field.descriptor == *descriptor)
                .unwrap_or_else(|| panic!("missing {name}.{field_name}:{descriptor}"));
            assert_eq!(field.access_flags, *access_flags, "{name}.{field_name}:{descriptor}");
        }
        assert_eq!(proto.methods.len(), methods.len() + 1, "{name} method count");
        let constructor_method = proto
            .methods
            .iter()
            .find(|method| method.name == "<init>" && method.descriptor == *constructor)
            .unwrap_or_else(|| panic!("missing {name}.<init>{constructor}"));
        assert_eq!(
            constructor_method.access_flags,
            MethodAccessFlags::default(),
            "{name}.<init>{constructor}"
        );
        for (method_name, descriptor) in *methods {
            let method = proto
                .methods
                .iter()
                .find(|method| method.name == *method_name && method.descriptor == *descriptor)
                .unwrap_or_else(|| panic!("missing {name}.{method_name}{descriptor}"));
            assert_eq!(method.access_flags, MethodAccessFlags::PUBLIC, "{name}.{method_name}{descriptor}");
        }

        let wrapper_class = jvm.resolve_class(name).await?.java_class();
        assert_eq!(
            jvm.invoke_virtual::<_, bool>(
                &serializable,
                &serializable.class_definition().name(),
                "isAssignableFrom",
                "(Ljava/lang/Class;)Z",
                (wrapper_class,),
            )
            .await?,
            *is_serializable,
            "{name} Serializable"
        );
    }

    Ok(())
}

#[tokio::test]
async fn test_coll_08_unmodifiable_collection_is_live_identity_based_and_always_throws() -> Result<()> {
    let jvm = test_jvm().await?;
    let backing = integer_list(&jvm, &[1]).await?;
    let wrapper: ClassInstanceRef<Object> = jvm
        .invoke_static(
            "java/util/Collections",
            "unmodifiableCollection",
            "(Ljava/util/Collection;)Ljava/util/Collection;",
            (backing.clone(),),
        )
        .await?;
    let nested: ClassInstanceRef<Object> = jvm
        .invoke_static(
            "java/util/Collections",
            "unmodifiableCollection",
            "(Ljava/util/Collection;)Ljava/util/Collection;",
            (wrapper.clone(),),
        )
        .await?;
    assert_ne!(wrapper.identity(), nested.identity());
    assert!(
        jvm.invoke_virtual::<_, bool>(
            &wrapper,
            &wrapper.class_definition().name(),
            "equals",
            "(Ljava/lang/Object;)Z",
            (wrapper.clone(),)
        )
        .await?
    );
    assert!(
        !jvm.invoke_virtual::<_, bool>(
            &wrapper,
            &wrapper.class_definition().name(),
            "equals",
            "(Ljava/lang/Object;)Z",
            (backing.clone(),)
        )
        .await?
    );
    assert!(
        !jvm.invoke_virtual::<_, bool>(
            &wrapper,
            &wrapper.class_definition().name(),
            "equals",
            "(Ljava/lang/Object;)Z",
            (nested.clone(),)
        )
        .await?
    );
    let null: ClassInstanceRef<Object> = None.into();
    assert!(
        !jvm.invoke_virtual::<_, bool>(
            &wrapper,
            &wrapper.class_definition().name(),
            "equals",
            "(Ljava/lang/Object;)Z",
            (null.clone(),)
        )
        .await?
    );

    let value: ClassInstanceRef<Object> = jvm.new_class("java/lang/Integer", "(I)V", (2,)).await?.into();
    let _: bool = jvm
        .invoke_virtual(
            &backing,
            &backing.class_definition().name(),
            "add",
            "(Ljava/lang/Object;)Z",
            (value.clone(),),
        )
        .await?;
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&wrapper, &wrapper.class_definition().name(), "size", "()I", ())
            .await?,
        2
    );
    assert!(
        jvm.invoke_virtual::<_, bool>(
            &wrapper,
            &wrapper.class_definition().name(),
            "contains",
            "(Ljava/lang/Object;)Z",
            (value.clone(),)
        )
        .await?
    );
    let backing_text: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&backing, &backing.class_definition().name(), "toString", "()Ljava/lang/String;", ())
        .await?;
    let wrapper_text: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&wrapper, &wrapper.class_definition().name(), "toString", "()Ljava/lang/String;", ())
        .await?;
    assert!(
        jvm.invoke_virtual::<_, bool>(
            &wrapper_text,
            &wrapper_text.class_definition().name(),
            "equals",
            "(Ljava/lang/Object;)Z",
            (backing_text,)
        )
        .await?
    );

    let mut destination: ClassInstanceRef<Array<Object>> = jvm.instantiate_array("Ljava/lang/Object;", 3).await?.into();
    let sentinel: ClassInstanceRef<Object> = jvm.new_class("java/lang/Integer", "(I)V", (99,)).await?.into();
    jvm.store_array(&mut destination, 2, core::iter::once(sentinel)).await?;
    let destination_identity = destination.identity();
    let result: ClassInstanceRef<Array<Object>> = jvm
        .invoke_virtual(
            &wrapper,
            &wrapper.class_definition().name(),
            "toArray",
            "([Ljava/lang/Object;)[Ljava/lang/Object;",
            (destination,),
        )
        .await?;
    assert_eq!(result.identity(), destination_identity);
    assert!(jvm.load_array::<ClassInstanceRef<Object>>(&result, 2, 1).await?[0].is_null());

    let empty = integer_list(&jvm, &[]).await?;
    for (name, argument) in [
        ("add", null.clone()),
        ("remove", jvm.new_class("java/lang/Integer", "(I)V", (404,)).await?.into()),
    ] {
        let result: Result<bool> = jvm
            .invoke_virtual(&wrapper, &wrapper.class_definition().name(), name, "(Ljava/lang/Object;)Z", (argument,))
            .await;
        let Err(JavaError::JavaException(exception)) = result else {
            panic!("{name} must throw even when no change is possible");
        };
        assert!(jvm.is_instance(exception.as_ref(), "java/lang/UnsupportedOperationException"));
    }
    for (name, argument) in [
        ("addAll", empty.clone()),
        ("addAll", null.clone()),
        ("removeAll", empty.clone()),
        ("removeAll", null.clone()),
        ("retainAll", wrapper.clone()),
        ("retainAll", null.clone()),
    ] {
        let result: Result<bool> = jvm
            .invoke_virtual(
                &wrapper,
                &wrapper.class_definition().name(),
                name,
                "(Ljava/util/Collection;)Z",
                (argument,),
            )
            .await;
        let Err(JavaError::JavaException(exception)) = result else {
            panic!("{name} must throw before no-op or null handling");
        };
        assert!(jvm.is_instance(exception.as_ref(), "java/lang/UnsupportedOperationException"));
    }

    let empty_wrapper: ClassInstanceRef<Object> = jvm
        .invoke_static(
            "java/util/Collections",
            "unmodifiableCollection",
            "(Ljava/util/Collection;)Ljava/util/Collection;",
            (empty,),
        )
        .await?;
    let clear: Result<()> = jvm
        .invoke_virtual(&empty_wrapper, &empty_wrapper.class_definition().name(), "clear", "()V", ())
        .await;
    let Err(JavaError::JavaException(exception)) = clear else {
        panic!("clear on an empty wrapper must throw");
    };
    assert!(jvm.is_instance(exception.as_ref(), "java/lang/UnsupportedOperationException"));

    let iterator: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&wrapper, &wrapper.class_definition().name(), "iterator", "()Ljava/util/Iterator;", ())
        .await?;
    let remove: Result<()> = jvm
        .invoke_virtual(&iterator, &iterator.class_definition().name(), "remove", "()V", ())
        .await;
    let Err(JavaError::JavaException(exception)) = remove else {
        panic!("iterator.remove before next must throw UOE");
    };
    assert!(jvm.is_instance(exception.as_ref(), "java/lang/UnsupportedOperationException"));
    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&iterator, &iterator.class_definition().name(), "next", "()Ljava/lang/Object;", ())
        .await?;
    let remove: Result<()> = jvm
        .invoke_virtual(&iterator, &iterator.class_definition().name(), "remove", "()V", ())
        .await;
    let Err(JavaError::JavaException(exception)) = remove else {
        panic!("iterator.remove after next must throw UOE");
    };
    assert!(jvm.is_instance(exception.as_ref(), "java/lang/UnsupportedOperationException"));

    Ok(())
}

#[tokio::test]
async fn test_coll_08_unmodifiable_list_delegates_value_contract_and_wraps_sub_lists() -> Result<()> {
    let jvm = test_jvm().await?;
    let backing = integer_list(&jvm, &[1, 2, 3]).await?;
    let list: ClassInstanceRef<Object> = jvm
        .invoke_static(
            "java/util/Collections",
            "unmodifiableList",
            "(Ljava/util/List;)Ljava/util/List;",
            (backing.clone(),),
        )
        .await?;
    let nested: ClassInstanceRef<Object> = jvm
        .invoke_static(
            "java/util/Collections",
            "unmodifiableList",
            "(Ljava/util/List;)Ljava/util/List;",
            (list.clone(),),
        )
        .await?;
    assert_ne!(list.identity(), nested.identity());
    assert!(
        jvm.invoke_virtual::<_, bool>(
            &list,
            &list.class_definition().name(),
            "equals",
            "(Ljava/lang/Object;)Z",
            (backing.clone(),)
        )
        .await?
    );
    assert!(
        jvm.invoke_virtual::<_, bool>(
            &nested,
            &nested.class_definition().name(),
            "equals",
            "(Ljava/lang/Object;)Z",
            (list.clone(),)
        )
        .await?
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&list, &list.class_definition().name(), "hashCode", "()I", ())
            .await?,
        jvm.invoke_virtual::<_, i32>(&backing, &backing.class_definition().name(), "hashCode", "()I", ())
            .await?
    );
    let null: ClassInstanceRef<Object> = None.into();
    assert!(
        !jvm.invoke_virtual::<_, bool>(&list, &list.class_definition().name(), "equals", "(Ljava/lang/Object;)Z", (null.clone(),))
            .await?
    );
    let backing_text: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&backing, &backing.class_definition().name(), "toString", "()Ljava/lang/String;", ())
        .await?;
    let list_text: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&list, &list.class_definition().name(), "toString", "()Ljava/lang/String;", ())
        .await?;
    assert!(
        jvm.invoke_virtual::<_, bool>(
            &list_text,
            &list_text.class_definition().name(),
            "equals",
            "(Ljava/lang/Object;)Z",
            (backing_text,)
        )
        .await?
    );

    let replacement: ClassInstanceRef<Object> = jvm.new_class("java/lang/Integer", "(I)V", (20,)).await?.into();
    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &backing,
            &backing.class_definition().name(),
            "set",
            "(ILjava/lang/Object;)Ljava/lang/Object;",
            (1, replacement.clone()),
        )
        .await?;
    assert_eq!(integer_values(&jvm, &list).await?, vec![1, 20, 3]);

    let sub_list: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&list, &list.class_definition().name(), "subList", "(II)Ljava/util/List;", (0, 2))
        .await?;
    let nested_sub_list: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&sub_list, &sub_list.class_definition().name(), "subList", "(II)Ljava/util/List;", (1, 2))
        .await?;
    assert_eq!(integer_values(&jvm, &sub_list).await?, vec![1, 20]);
    assert_eq!(integer_values(&jvm, &nested_sub_list).await?, vec![20]);
    let changed: ClassInstanceRef<Object> = jvm.new_class("java/lang/Integer", "(I)V", (21,)).await?.into();
    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &backing,
            &backing.class_definition().name(),
            "set",
            "(ILjava/lang/Object;)Ljava/lang/Object;",
            (1, changed.clone()),
        )
        .await?;
    assert_eq!(integer_values(&jvm, &nested_sub_list).await?, vec![21]);

    let set: Result<ClassInstanceRef<Object>> = jvm
        .invoke_virtual(
            &list,
            &list.class_definition().name(),
            "set",
            "(ILjava/lang/Object;)Ljava/lang/Object;",
            (-1, changed.clone()),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = set else {
        panic!("set with invalid index must throw UOE first");
    };
    assert!(jvm.is_instance(exception.as_ref(), "java/lang/UnsupportedOperationException"));
    let add: Result<()> = jvm
        .invoke_virtual(
            &list,
            &list.class_definition().name(),
            "add",
            "(ILjava/lang/Object;)V",
            (-1, changed.clone()),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = add else {
        panic!("add with invalid index must throw UOE first");
    };
    assert!(jvm.is_instance(exception.as_ref(), "java/lang/UnsupportedOperationException"));
    let remove: Result<ClassInstanceRef<Object>> = jvm
        .invoke_virtual(&list, &list.class_definition().name(), "remove", "(I)Ljava/lang/Object;", (-1,))
        .await;
    let Err(JavaError::JavaException(exception)) = remove else {
        panic!("remove with invalid index must throw UOE first");
    };
    assert!(jvm.is_instance(exception.as_ref(), "java/lang/UnsupportedOperationException"));
    let add_all: Result<bool> = jvm
        .invoke_virtual(
            &list,
            &list.class_definition().name(),
            "addAll",
            "(ILjava/util/Collection;)Z",
            (-1, null.clone()),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = add_all else {
        panic!("indexed addAll must throw UOE before index and null validation");
    };
    assert!(jvm.is_instance(exception.as_ref(), "java/lang/UnsupportedOperationException"));
    let empty = integer_list(&jvm, &[]).await?;
    let add_all: Result<bool> = jvm
        .invoke_virtual(&list, &list.class_definition().name(), "addAll", "(ILjava/util/Collection;)Z", (0, empty))
        .await;
    let Err(JavaError::JavaException(exception)) = add_all else {
        panic!("indexed addAll of an empty collection must throw");
    };
    assert!(jvm.is_instance(exception.as_ref(), "java/lang/UnsupportedOperationException"));

    let sub_set: Result<ClassInstanceRef<Object>> = jvm
        .invoke_virtual(
            &sub_list,
            &sub_list.class_definition().name(),
            "set",
            "(ILjava/lang/Object;)Ljava/lang/Object;",
            (0, changed.clone()),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = sub_set else {
        panic!("subList must remain unmodifiable");
    };
    assert!(jvm.is_instance(exception.as_ref(), "java/lang/UnsupportedOperationException"));

    let iterator: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&list, &list.class_definition().name(), "listIterator", "()Ljava/util/ListIterator;", ())
        .await?;
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&iterator, &iterator.class_definition().name(), "nextIndex", "()I", ())
            .await?,
        0
    );
    for (name, descriptor, argument) in [
        ("set", "(Ljava/lang/Object;)V", changed.clone()),
        ("add", "(Ljava/lang/Object;)V", changed.clone()),
    ] {
        let result: Result<()> = jvm
            .invoke_virtual(&iterator, &iterator.class_definition().name(), name, descriptor, (argument,))
            .await;
        let Err(JavaError::JavaException(exception)) = result else {
            panic!("listIterator.{name} before next must throw UOE");
        };
        assert!(jvm.is_instance(exception.as_ref(), "java/lang/UnsupportedOperationException"));
    }
    let remove: Result<()> = jvm
        .invoke_virtual(&iterator, &iterator.class_definition().name(), "remove", "()V", ())
        .await;
    let Err(JavaError::JavaException(exception)) = remove else {
        panic!("listIterator.remove before next must throw UOE");
    };
    assert!(jvm.is_instance(exception.as_ref(), "java/lang/UnsupportedOperationException"));
    let first: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&iterator, &iterator.class_definition().name(), "next", "()Ljava/lang/Object;", ())
        .await?;
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&first, &first.class_definition().name(), "intValue", "()I", ())
            .await?,
        1
    );
    assert!(
        jvm.invoke_virtual::<_, bool>(&iterator, &iterator.class_definition().name(), "hasPrevious", "()Z", ())
            .await?
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&iterator, &iterator.class_definition().name(), "previousIndex", "()I", ())
            .await?,
        0
    );
    for (name, descriptor, argument) in [
        ("set", "(Ljava/lang/Object;)V", changed.clone()),
        ("add", "(Ljava/lang/Object;)V", changed),
    ] {
        let result: Result<()> = jvm
            .invoke_virtual(&iterator, &iterator.class_definition().name(), name, descriptor, (argument,))
            .await;
        let Err(JavaError::JavaException(exception)) = result else {
            panic!("listIterator.{name} after next must throw UOE");
        };
        assert!(jvm.is_instance(exception.as_ref(), "java/lang/UnsupportedOperationException"));
    }
    let remove: Result<()> = jvm
        .invoke_virtual(&iterator, &iterator.class_definition().name(), "remove", "()V", ())
        .await;
    let Err(JavaError::JavaException(exception)) = remove else {
        panic!("listIterator.remove after next must throw UOE");
    };
    assert!(jvm.is_instance(exception.as_ref(), "java/lang/UnsupportedOperationException"));

    Ok(())
}

#[tokio::test]
async fn test_coll_08_unmodifiable_set_and_map_views_are_live_and_always_unmodifiable() -> Result<()> {
    let jvm = test_jvm().await?;
    let set: ClassInstanceRef<Object> = jvm.new_class("java/util/HashSet", "()V", ()).await?.into();
    let first: ClassInstanceRef<Object> = jvm.new_class("java/lang/Integer", "(I)V", (1,)).await?.into();
    let second: ClassInstanceRef<Object> = jvm.new_class("java/lang/Integer", "(I)V", (2,)).await?.into();
    let _: bool = jvm
        .invoke_virtual(&set, &set.class_definition().name(), "add", "(Ljava/lang/Object;)Z", (first.clone(),))
        .await?;
    let wrapped_set: ClassInstanceRef<Object> = jvm
        .invoke_static(
            "java/util/Collections",
            "unmodifiableSet",
            "(Ljava/util/Set;)Ljava/util/Set;",
            (set.clone(),),
        )
        .await?;
    let nested_set: ClassInstanceRef<Object> = jvm
        .invoke_static(
            "java/util/Collections",
            "unmodifiableSet",
            "(Ljava/util/Set;)Ljava/util/Set;",
            (wrapped_set.clone(),),
        )
        .await?;
    assert_ne!(wrapped_set.identity(), nested_set.identity());
    assert!(
        jvm.invoke_virtual::<_, bool>(
            &wrapped_set,
            &wrapped_set.class_definition().name(),
            "equals",
            "(Ljava/lang/Object;)Z",
            (set.clone(),)
        )
        .await?
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&wrapped_set, &wrapped_set.class_definition().name(), "hashCode", "()I", ())
            .await?,
        jvm.invoke_virtual::<_, i32>(&set, &set.class_definition().name(), "hashCode", "()I", ())
            .await?
    );
    let null: ClassInstanceRef<Object> = None.into();
    assert!(
        !jvm.invoke_virtual::<_, bool>(
            &wrapped_set,
            &wrapped_set.class_definition().name(),
            "equals",
            "(Ljava/lang/Object;)Z",
            (null.clone(),)
        )
        .await?
    );
    let _: bool = jvm
        .invoke_virtual(&set, &set.class_definition().name(), "add", "(Ljava/lang/Object;)Z", (second.clone(),))
        .await?;
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&wrapped_set, &wrapped_set.class_definition().name(), "size", "()I", ())
            .await?,
        2
    );
    let absent: ClassInstanceRef<Object> = jvm.new_class("java/lang/Integer", "(I)V", (404,)).await?.into();
    let remove: Result<bool> = jvm
        .invoke_virtual(
            &wrapped_set,
            &wrapped_set.class_definition().name(),
            "remove",
            "(Ljava/lang/Object;)Z",
            (absent.clone(),),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = remove else {
        panic!("set removal of an absent element must throw");
    };
    assert!(jvm.is_instance(exception.as_ref(), "java/lang/UnsupportedOperationException"));

    let map: ClassInstanceRef<Object> = jvm.new_class("java/util/HashMap", "()V", ()).await?.into();
    let value = JavaLangString::from_rust_string(&jvm, "one").await?;
    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &map,
            &map.class_definition().name(),
            "put",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            (first.clone(), value.clone()),
        )
        .await?;
    let wrapped_map: ClassInstanceRef<Object> = jvm
        .invoke_static(
            "java/util/Collections",
            "unmodifiableMap",
            "(Ljava/util/Map;)Ljava/util/Map;",
            (map.clone(),),
        )
        .await?;
    let nested_map: ClassInstanceRef<Object> = jvm
        .invoke_static(
            "java/util/Collections",
            "unmodifiableMap",
            "(Ljava/util/Map;)Ljava/util/Map;",
            (wrapped_map.clone(),),
        )
        .await?;
    assert_ne!(wrapped_map.identity(), nested_map.identity());
    assert!(
        jvm.invoke_virtual::<_, bool>(
            &wrapped_map,
            &wrapped_map.class_definition().name(),
            "equals",
            "(Ljava/lang/Object;)Z",
            (map.clone(),)
        )
        .await?
    );
    assert!(
        jvm.invoke_virtual::<_, bool>(
            &nested_map,
            &nested_map.class_definition().name(),
            "equals",
            "(Ljava/lang/Object;)Z",
            (wrapped_map.clone(),)
        )
        .await?
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&wrapped_map, &wrapped_map.class_definition().name(), "hashCode", "()I", ())
            .await?,
        jvm.invoke_virtual::<_, i32>(&map, &map.class_definition().name(), "hashCode", "()I", ())
            .await?
    );
    assert!(
        !jvm.invoke_virtual::<_, bool>(
            &wrapped_map,
            &wrapped_map.class_definition().name(),
            "equals",
            "(Ljava/lang/Object;)Z",
            (null.clone(),)
        )
        .await?
    );
    let map_text: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&map, &map.class_definition().name(), "toString", "()Ljava/lang/String;", ())
        .await?;
    let wrapped_text: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &wrapped_map,
            &wrapped_map.class_definition().name(),
            "toString",
            "()Ljava/lang/String;",
            (),
        )
        .await?;
    assert!(
        jvm.invoke_virtual::<_, bool>(
            &wrapped_text,
            &wrapped_text.class_definition().name(),
            "equals",
            "(Ljava/lang/Object;)Z",
            (map_text,)
        )
        .await?
    );

    let keys: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&wrapped_map, &wrapped_map.class_definition().name(), "keySet", "()Ljava/util/Set;", ())
        .await?;
    let same_keys: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&wrapped_map, &wrapped_map.class_definition().name(), "keySet", "()Ljava/util/Set;", ())
        .await?;
    let values: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &wrapped_map,
            &wrapped_map.class_definition().name(),
            "values",
            "()Ljava/util/Collection;",
            (),
        )
        .await?;
    let same_values: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &wrapped_map,
            &wrapped_map.class_definition().name(),
            "values",
            "()Ljava/util/Collection;",
            (),
        )
        .await?;
    let entries: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&wrapped_map, &wrapped_map.class_definition().name(), "entrySet", "()Ljava/util/Set;", ())
        .await?;
    let same_entries: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&wrapped_map, &wrapped_map.class_definition().name(), "entrySet", "()Ljava/util/Set;", ())
        .await?;
    assert_eq!(keys.identity(), same_keys.identity());
    assert_eq!(values.identity(), same_values.identity());
    assert_eq!(entries.identity(), same_entries.identity());
    let second_value = JavaLangString::from_rust_string(&jvm, "two").await?;
    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &map,
            &map.class_definition().name(),
            "put",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            (second.clone(), second_value.clone()),
        )
        .await?;
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&keys, &keys.class_definition().name(), "size", "()I", ())
            .await?,
        2
    );
    assert!(
        jvm.invoke_virtual::<_, bool>(
            &values,
            &values.class_definition().name(),
            "contains",
            "(Ljava/lang/Object;)Z",
            (second_value,)
        )
        .await?
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&entries, &entries.class_definition().name(), "size", "()I", ())
            .await?,
        2
    );

    let put: Result<ClassInstanceRef<Object>> = jvm
        .invoke_virtual(
            &wrapped_map,
            &wrapped_map.class_definition().name(),
            "put",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            (null.clone(), null.clone()),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = put else {
        panic!("map.put must throw UOE before key/value validation");
    };
    assert!(jvm.is_instance(exception.as_ref(), "java/lang/UnsupportedOperationException"));
    let remove: Result<ClassInstanceRef<Object>> = jvm
        .invoke_virtual(
            &wrapped_map,
            &wrapped_map.class_definition().name(),
            "remove",
            "(Ljava/lang/Object;)Ljava/lang/Object;",
            (absent.clone(),),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = remove else {
        panic!("map.remove of absent key must throw");
    };
    assert!(jvm.is_instance(exception.as_ref(), "java/lang/UnsupportedOperationException"));
    for source in [
        jvm.new_class("java/util/HashMap", "()V", ()).await?.into(),
        wrapped_map.clone(),
        null.clone(),
    ] {
        let put_all: Result<()> = jvm
            .invoke_virtual(
                &wrapped_map,
                &wrapped_map.class_definition().name(),
                "putAll",
                "(Ljava/util/Map;)V",
                (source,),
            )
            .await;
        let Err(JavaError::JavaException(exception)) = put_all else {
            panic!("map.putAll must throw for empty, self, and null inputs");
        };
        assert!(jvm.is_instance(exception.as_ref(), "java/lang/UnsupportedOperationException"));
    }
    let empty_map: ClassInstanceRef<Object> = jvm.new_class("java/util/HashMap", "()V", ()).await?.into();
    let empty_wrapper: ClassInstanceRef<Object> = jvm
        .invoke_static(
            "java/util/Collections",
            "unmodifiableMap",
            "(Ljava/util/Map;)Ljava/util/Map;",
            (empty_map,),
        )
        .await?;
    let clear: Result<()> = jvm
        .invoke_virtual(&empty_wrapper, &empty_wrapper.class_definition().name(), "clear", "()V", ())
        .await;
    let Err(JavaError::JavaException(exception)) = clear else {
        panic!("map.clear on empty map must throw");
    };
    assert!(jvm.is_instance(exception.as_ref(), "java/lang/UnsupportedOperationException"));

    let empty_collection = integer_list(&jvm, &[]).await?;
    for view in [keys, values, entries] {
        let add: Result<bool> = jvm
            .invoke_virtual(&view, &view.class_definition().name(), "add", "(Ljava/lang/Object;)Z", (null.clone(),))
            .await;
        let Err(JavaError::JavaException(exception)) = add else {
            panic!("map view add must throw");
        };
        assert!(jvm.is_instance(exception.as_ref(), "java/lang/UnsupportedOperationException"));
        let remove: Result<bool> = jvm
            .invoke_virtual(
                &view,
                &view.class_definition().name(),
                "remove",
                "(Ljava/lang/Object;)Z",
                (absent.clone(),),
            )
            .await;
        let Err(JavaError::JavaException(exception)) = remove else {
            panic!("map view remove of absent value must throw");
        };
        assert!(jvm.is_instance(exception.as_ref(), "java/lang/UnsupportedOperationException"));
        for name in ["addAll", "removeAll", "retainAll"] {
            let bulk: Result<bool> = jvm
                .invoke_virtual(
                    &view,
                    &view.class_definition().name(),
                    name,
                    "(Ljava/util/Collection;)Z",
                    (empty_collection.clone(),),
                )
                .await;
            let Err(JavaError::JavaException(exception)) = bulk else {
                panic!("map view {name} must throw for an empty input");
            };
            assert!(jvm.is_instance(exception.as_ref(), "java/lang/UnsupportedOperationException"));
        }
        let clear: Result<()> = jvm.invoke_virtual(&view, &view.class_definition().name(), "clear", "()V", ()).await;
        let Err(JavaError::JavaException(exception)) = clear else {
            panic!("map view clear must throw");
        };
        assert!(jvm.is_instance(exception.as_ref(), "java/lang/UnsupportedOperationException"));
        let iterator: ClassInstanceRef<Object> = jvm
            .invoke_virtual(&view, &view.class_definition().name(), "iterator", "()Ljava/util/Iterator;", ())
            .await?;
        let remove: Result<()> = jvm
            .invoke_virtual(&iterator, &iterator.class_definition().name(), "remove", "()V", ())
            .await;
        let Err(JavaError::JavaException(exception)) = remove else {
            panic!("map view iterator.remove must throw before next");
        };
        assert!(jvm.is_instance(exception.as_ref(), "java/lang/UnsupportedOperationException"));
    }

    Ok(())
}

#[tokio::test]
async fn test_coll_08_unmodifiable_entry_set_wraps_iterator_and_all_array_results() -> Result<()> {
    let jvm = test_jvm().await?;
    let map: ClassInstanceRef<Object> = jvm.new_class("java/util/HashMap", "()V", ()).await?.into();
    let key: ClassInstanceRef<Object> = jvm.new_class("java/lang/Integer", "(I)V", (7,)).await?.into();
    let original = JavaLangString::from_rust_string(&jvm, "before").await?;
    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &map,
            &map.class_definition().name(),
            "put",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            (key.clone(), original),
        )
        .await?;
    let raw_entries: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&map, &map.class_definition().name(), "entrySet", "()Ljava/util/Set;", ())
        .await?;
    let raw_iterator: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &raw_entries,
            &raw_entries.class_definition().name(),
            "iterator",
            "()Ljava/util/Iterator;",
            (),
        )
        .await?;
    let raw_entry: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&raw_iterator, &raw_iterator.class_definition().name(), "next", "()Ljava/lang/Object;", ())
        .await?;

    let wrapped_map: ClassInstanceRef<Object> = jvm
        .invoke_static(
            "java/util/Collections",
            "unmodifiableMap",
            "(Ljava/util/Map;)Ljava/util/Map;",
            (map.clone(),),
        )
        .await?;
    let entries: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&wrapped_map, &wrapped_map.class_definition().name(), "entrySet", "()Ljava/util/Set;", ())
        .await?;
    assert!(
        jvm.invoke_virtual::<_, bool>(
            &entries,
            &entries.class_definition().name(),
            "contains",
            "(Ljava/lang/Object;)Z",
            (raw_entry.clone(),)
        )
        .await?
    );
    assert!(
        jvm.invoke_virtual::<_, bool>(
            &entries,
            &entries.class_definition().name(),
            "containsAll",
            "(Ljava/util/Collection;)Z",
            (raw_entries.clone(),)
        )
        .await?
    );
    assert!(
        jvm.invoke_virtual::<_, bool>(
            &entries,
            &entries.class_definition().name(),
            "equals",
            "(Ljava/lang/Object;)Z",
            (raw_entries.clone(),)
        )
        .await?
    );

    let iterator: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&entries, &entries.class_definition().name(), "iterator", "()Ljava/util/Iterator;", ())
        .await?;
    let wrapped_entry: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&iterator, &iterator.class_definition().name(), "next", "()Ljava/lang/Object;", ())
        .await?;
    assert_eq!(
        wrapped_entry.class_definition().name(),
        "java/util/Collections$UnmodifiableMap$UnmodifiableEntrySet$UnmodifiableEntry"
    );
    assert!(
        jvm.invoke_virtual::<_, bool>(
            &wrapped_entry,
            &wrapped_entry.class_definition().name(),
            "equals",
            "(Ljava/lang/Object;)Z",
            (raw_entry.clone(),)
        )
        .await?
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&wrapped_entry, &wrapped_entry.class_definition().name(), "hashCode", "()I", ())
            .await?,
        jvm.invoke_virtual::<_, i32>(&raw_entry, &raw_entry.class_definition().name(), "hashCode", "()I", ())
            .await?
    );
    let null_entry: ClassInstanceRef<Object> = None.into();
    assert!(
        !jvm.invoke_virtual::<_, bool>(
            &wrapped_entry,
            &wrapped_entry.class_definition().name(),
            "equals",
            "(Ljava/lang/Object;)Z",
            (null_entry,),
        )
        .await?
    );
    let raw_text: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&raw_entry, &raw_entry.class_definition().name(), "toString", "()Ljava/lang/String;", ())
        .await?;
    let wrapped_text: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &wrapped_entry,
            &wrapped_entry.class_definition().name(),
            "toString",
            "()Ljava/lang/String;",
            (),
        )
        .await?;
    assert!(
        jvm.invoke_virtual::<_, bool>(
            &wrapped_text,
            &wrapped_text.class_definition().name(),
            "equals",
            "(Ljava/lang/Object;)Z",
            (raw_text,)
        )
        .await?
    );
    let replacement = JavaLangString::from_rust_string(&jvm, "after").await?;
    let set_value: Result<ClassInstanceRef<Object>> = jvm
        .invoke_virtual(
            &wrapped_entry,
            &wrapped_entry.class_definition().name(),
            "setValue",
            "(Ljava/lang/Object;)Ljava/lang/Object;",
            (replacement.clone(),),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = set_value else {
        panic!("wrapped entry.setValue must throw");
    };
    assert!(jvm.is_instance(exception.as_ref(), "java/lang/UnsupportedOperationException"));

    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &map,
            &map.class_definition().name(),
            "put",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            (key, replacement.clone()),
        )
        .await?;
    let live_value: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &wrapped_entry,
            &wrapped_entry.class_definition().name(),
            "getValue",
            "()Ljava/lang/Object;",
            (),
        )
        .await?;
    assert!(
        jvm.invoke_virtual::<_, bool>(
            &live_value,
            &live_value.class_definition().name(),
            "equals",
            "(Ljava/lang/Object;)Z",
            (replacement,)
        )
        .await?
    );

    let array: ClassInstanceRef<Array<Object>> = jvm
        .invoke_virtual(&entries, &entries.class_definition().name(), "toArray", "()[Ljava/lang/Object;", ())
        .await?;
    let array_entry = jvm.load_array::<ClassInstanceRef<Object>>(&array, 0, 1).await?[0].clone();
    assert_eq!(
        array_entry.class_definition().name(),
        "java/util/Collections$UnmodifiableMap$UnmodifiableEntrySet$UnmodifiableEntry"
    );
    let null_value: ClassInstanceRef<Object> = None.into();
    let set_value: Result<ClassInstanceRef<Object>> = jvm
        .invoke_virtual(
            &array_entry,
            &array_entry.class_definition().name(),
            "setValue",
            "(Ljava/lang/Object;)Ljava/lang/Object;",
            (null_value,),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = set_value else {
        panic!("entry returned by toArray must be unmodifiable");
    };
    assert!(jvm.is_instance(exception.as_ref(), "java/lang/UnsupportedOperationException"));

    let mut typed_destination: ClassInstanceRef<Array<Object>> = jvm.instantiate_array("Ljava/util/Map$Entry;", 2).await?.into();
    jvm.store_array(&mut typed_destination, 1, core::iter::once(raw_entry.clone())).await?;
    let destination_identity = typed_destination.identity();
    let typed: ClassInstanceRef<Array<Object>> = jvm
        .invoke_virtual(
            &entries,
            &entries.class_definition().name(),
            "toArray",
            "([Ljava/lang/Object;)[Ljava/lang/Object;",
            (typed_destination,),
        )
        .await?;
    assert_eq!(typed.identity(), destination_identity);
    let typed_values = jvm.load_array::<ClassInstanceRef<Object>>(&typed, 0, 2).await?;
    assert_eq!(
        typed_values[0].class_definition().name(),
        "java/util/Collections$UnmodifiableMap$UnmodifiableEntrySet$UnmodifiableEntry"
    );
    assert!(typed_values[1].is_null());

    let small: ClassInstanceRef<Array<Object>> = jvm.instantiate_array("Ljava/util/Map$Entry;", 0).await?.into();
    let grown: ClassInstanceRef<Array<Object>> = jvm
        .invoke_virtual(
            &entries,
            &entries.class_definition().name(),
            "toArray",
            "([Ljava/lang/Object;)[Ljava/lang/Object;",
            (small,),
        )
        .await?;
    assert_eq!(jvm.array_length(&grown).await?, 1);
    assert_eq!(grown.class_definition().name(), "[Ljava/util/Map$Entry;");
    assert_eq!(
        jvm.load_array::<ClassInstanceRef<Object>>(&grown, 0, 1).await?[0]
            .class_definition()
            .name(),
        "java/util/Collections$UnmodifiableMap$UnmodifiableEntrySet$UnmodifiableEntry"
    );

    let null_array: ClassInstanceRef<Array<Object>> = None.into();
    let result: Result<ClassInstanceRef<Array<Object>>> = jvm
        .invoke_virtual(
            &entries,
            &entries.class_definition().name(),
            "toArray",
            "([Ljava/lang/Object;)[Ljava/lang/Object;",
            (null_array,),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("typed entry toArray(null) must throw");
    };
    assert!(jvm.is_instance(exception.as_ref(), "java/lang/NullPointerException"));

    let incompatible: ClassInstanceRef<Array<Object>> = jvm.instantiate_array("Ljava/util/HashMap$Entry;", 2).await?.into();
    let result: Result<ClassInstanceRef<Array<Object>>> = jvm
        .invoke_virtual(
            &entries,
            &entries.class_definition().name(),
            "toArray",
            "([Ljava/lang/Object;)[Ljava/lang/Object;",
            (incompatible.clone(),),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("concrete mutable-entry array must reject wrapped entries");
    };
    assert!(jvm.is_instance(exception.as_ref(), "java/lang/ArrayStoreException"));
    for element in jvm.load_array::<ClassInstanceRef<Object>>(&incompatible, 0, 2).await? {
        assert!(element.is_null(), "typed array failure must not leak raw entries");
    }

    Ok(())
}

#[tokio::test]
async fn test_coll_08_unmodifiable_entry_equals_and_contains_preserve_jdk_ordering() -> Result<()> {
    let jvm = collections_test_jvm().await?;

    let rejecting_key: ClassInstanceRef<Object> = jvm.new_class("CollectionsEqualsProbe", "(ZZ)V", (false, false)).await?.into();
    let unused_value: ClassInstanceRef<Object> = jvm.new_class("CollectionsEqualsProbe", "(ZZ)V", (true, false)).await?.into();
    let backing_entry: ClassInstanceRef<Object> = jvm
        .new_class(
            "CollectionsEntryProbe",
            "(Ljava/lang/Object;Ljava/lang/Object;ZZ)V",
            (rejecting_key.clone(), unused_value, false, true),
        )
        .await?
        .into();
    let candidate_key: ClassInstanceRef<Object> = jvm.new_class("CollectionsEqualsProbe", "(ZZ)V", (true, true)).await?.into();
    let candidate_value: ClassInstanceRef<Object> = jvm.new_class("CollectionsEqualsProbe", "(ZZ)V", (true, true)).await?.into();
    let candidate_entry: ClassInstanceRef<Object> = jvm
        .new_class(
            "CollectionsEntryProbe",
            "(Ljava/lang/Object;Ljava/lang/Object;ZZ)V",
            (candidate_key.clone(), candidate_value, false, true),
        )
        .await?
        .into();
    let wrapped_entry: ClassInstanceRef<Object> = jvm
        .new_class(
            "java/util/Collections$UnmodifiableMap$UnmodifiableEntrySet$UnmodifiableEntry",
            "(Ljava/util/Map$Entry;)V",
            (backing_entry.clone(),),
        )
        .await?
        .into();
    assert!(
        !jvm.invoke_virtual::<_, bool>(
            &wrapped_entry,
            &wrapped_entry.class_definition().name(),
            "equals",
            "(Ljava/lang/Object;)Z",
            (candidate_entry.clone(),)
        )
        .await?
    );
    assert_eq!(jvm.get_field::<i32>(&backing_entry, "keyCalls", "I").await?, 1);
    assert_eq!(jvm.get_field::<i32>(&candidate_entry, "keyCalls", "I").await?, 1);
    assert_eq!(jvm.get_field::<i32>(&backing_entry, "valueCalls", "I").await?, 0);
    assert_eq!(jvm.get_field::<i32>(&candidate_entry, "valueCalls", "I").await?, 0);
    assert_eq!(jvm.get_field::<i32>(&rejecting_key, "equalsCalls", "I").await?, 1);
    assert_eq!(jvm.get_field::<i32>(&candidate_key, "equalsCalls", "I").await?, 0);

    let accepting_key: ClassInstanceRef<Object> = jvm.new_class("CollectionsEqualsProbe", "(ZZ)V", (true, false)).await?.into();
    let accepting_value: ClassInstanceRef<Object> = jvm.new_class("CollectionsEqualsProbe", "(ZZ)V", (true, false)).await?.into();
    let backing_entry: ClassInstanceRef<Object> = jvm
        .new_class(
            "CollectionsEntryProbe",
            "(Ljava/lang/Object;Ljava/lang/Object;ZZ)V",
            (accepting_key.clone(), accepting_value.clone(), false, false),
        )
        .await?
        .into();
    let reverse_direction_key: ClassInstanceRef<Object> = jvm.new_class("CollectionsEqualsProbe", "(ZZ)V", (false, true)).await?.into();
    let reverse_direction_value: ClassInstanceRef<Object> = jvm.new_class("CollectionsEqualsProbe", "(ZZ)V", (false, true)).await?.into();
    let candidate_entry: ClassInstanceRef<Object> = jvm
        .new_class(
            "CollectionsEntryProbe",
            "(Ljava/lang/Object;Ljava/lang/Object;ZZ)V",
            (reverse_direction_key.clone(), reverse_direction_value.clone(), false, false),
        )
        .await?
        .into();
    let wrapped_entry: ClassInstanceRef<Object> = jvm
        .new_class(
            "java/util/Collections$UnmodifiableMap$UnmodifiableEntrySet$UnmodifiableEntry",
            "(Ljava/util/Map$Entry;)V",
            (backing_entry.clone(),),
        )
        .await?
        .into();
    assert!(
        jvm.invoke_virtual::<_, bool>(
            &wrapped_entry,
            &wrapped_entry.class_definition().name(),
            "equals",
            "(Ljava/lang/Object;)Z",
            (candidate_entry.clone(),)
        )
        .await?
    );
    assert_eq!(jvm.get_field::<i32>(&backing_entry, "keyCalls", "I").await?, 1);
    assert_eq!(jvm.get_field::<i32>(&backing_entry, "valueCalls", "I").await?, 1);
    assert_eq!(jvm.get_field::<i32>(&candidate_entry, "keyCalls", "I").await?, 1);
    assert_eq!(jvm.get_field::<i32>(&candidate_entry, "valueCalls", "I").await?, 1);
    assert_eq!(jvm.get_field::<i32>(&accepting_key, "equalsCalls", "I").await?, 1);
    assert_eq!(jvm.get_field::<i32>(&accepting_value, "equalsCalls", "I").await?, 1);
    assert_eq!(jvm.get_field::<i32>(&reverse_direction_key, "equalsCalls", "I").await?, 0);
    assert_eq!(jvm.get_field::<i32>(&reverse_direction_value, "equalsCalls", "I").await?, 0);

    let replacement: ClassInstanceRef<Object> = jvm.new_class("java/lang/Object", "()V", ()).await?.into();
    let result: Result<ClassInstanceRef<Object>> = jvm
        .invoke_virtual(
            &wrapped_entry,
            &wrapped_entry.class_definition().name(),
            "setValue",
            "(Ljava/lang/Object;)Ljava/lang/Object;",
            (replacement,),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("wrapped custom entry.setValue must throw");
    };
    assert!(jvm.is_instance(exception.as_ref(), "java/lang/UnsupportedOperationException"));
    assert_eq!(jvm.get_field::<i32>(&backing_entry, "setValueCalls", "I").await?, 0);

    let throwing_key_entry: ClassInstanceRef<Object> = jvm
        .new_class(
            "CollectionsEntryProbe",
            "(Ljava/lang/Object;Ljava/lang/Object;ZZ)V",
            (accepting_key.clone(), accepting_value.clone(), true, false),
        )
        .await?
        .into();
    let throwing_key_wrapper: ClassInstanceRef<Object> = jvm
        .new_class(
            "java/util/Collections$UnmodifiableMap$UnmodifiableEntrySet$UnmodifiableEntry",
            "(Ljava/util/Map$Entry;)V",
            (throwing_key_entry.clone(),),
        )
        .await?
        .into();
    let untouched_candidate: ClassInstanceRef<Object> = jvm
        .new_class(
            "CollectionsEntryProbe",
            "(Ljava/lang/Object;Ljava/lang/Object;ZZ)V",
            (accepting_key.clone(), accepting_value.clone(), false, false),
        )
        .await?
        .into();
    let result: Result<bool> = jvm
        .invoke_virtual(
            &throwing_key_wrapper,
            &throwing_key_wrapper.class_definition().name(),
            "equals",
            "(Ljava/lang/Object;)Z",
            (untouched_candidate.clone(),),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("backing getKey exception must propagate");
    };
    assert!(jvm.is_instance(exception.as_ref(), "java/lang/IllegalStateException"));
    assert_eq!(jvm.get_field::<i32>(&throwing_key_entry, "keyCalls", "I").await?, 1);
    assert_eq!(jvm.get_field::<i32>(&untouched_candidate, "keyCalls", "I").await?, 0);

    let throwing_equals_key: ClassInstanceRef<Object> = jvm.new_class("CollectionsEqualsProbe", "(ZZ)V", (true, true)).await?.into();
    let throwing_equals_entry: ClassInstanceRef<Object> = jvm
        .new_class(
            "CollectionsEntryProbe",
            "(Ljava/lang/Object;Ljava/lang/Object;ZZ)V",
            (throwing_equals_key.clone(), accepting_value.clone(), false, false),
        )
        .await?
        .into();
    let throwing_equals_wrapper: ClassInstanceRef<Object> = jvm
        .new_class(
            "java/util/Collections$UnmodifiableMap$UnmodifiableEntrySet$UnmodifiableEntry",
            "(Ljava/util/Map$Entry;)V",
            (throwing_equals_entry.clone(),),
        )
        .await?
        .into();
    let result: Result<bool> = jvm
        .invoke_virtual(
            &throwing_equals_wrapper,
            &throwing_equals_wrapper.class_definition().name(),
            "equals",
            "(Ljava/lang/Object;)Z",
            (candidate_entry.clone(),),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("backing key.equals exception must propagate");
    };
    assert!(jvm.is_instance(exception.as_ref(), "java/lang/IllegalStateException"));
    assert_eq!(jvm.get_field::<i32>(&throwing_equals_key, "equalsCalls", "I").await?, 1);
    assert_eq!(jvm.get_field::<i32>(&throwing_equals_entry, "valueCalls", "I").await?, 0);

    let required_throwing_value_entry: ClassInstanceRef<Object> = jvm
        .new_class(
            "CollectionsEntryProbe",
            "(Ljava/lang/Object;Ljava/lang/Object;ZZ)V",
            (reverse_direction_key, reverse_direction_value, false, true),
        )
        .await?
        .into();
    let result: Result<bool> = jvm
        .invoke_virtual(
            &wrapped_entry,
            &wrapped_entry.class_definition().name(),
            "equals",
            "(Ljava/lang/Object;)Z",
            (required_throwing_value_entry.clone(),),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("candidate getValue exception must propagate after equal keys");
    };
    assert!(jvm.is_instance(exception.as_ref(), "java/lang/IllegalStateException"));
    assert_eq!(jvm.get_field::<i32>(&required_throwing_value_entry, "keyCalls", "I").await?, 1);
    assert_eq!(jvm.get_field::<i32>(&required_throwing_value_entry, "valueCalls", "I").await?, 1);

    let map: ClassInstanceRef<Object> = jvm.new_class("java/util/HashMap", "()V", ()).await?.into();
    let map_key: ClassInstanceRef<Object> = jvm.new_class("java/lang/Integer", "(I)V", (7,)).await?.into();
    let map_value: ClassInstanceRef<Object> = JavaLangString::from_rust_string(&jvm, "seven").await?.into();
    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &map,
            &map.class_definition().name(),
            "put",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            (map_key.clone(), map_value.clone()),
        )
        .await?;
    let wrapped_map: ClassInstanceRef<Object> = jvm
        .invoke_static("java/util/Collections", "unmodifiableMap", "(Ljava/util/Map;)Ljava/util/Map;", (map,))
        .await?;
    let entries: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&wrapped_map, &wrapped_map.class_definition().name(), "entrySet", "()Ljava/util/Set;", ())
        .await?;
    let matching: ClassInstanceRef<Object> = jvm
        .new_class(
            "CollectionsEntryProbe",
            "(Ljava/lang/Object;Ljava/lang/Object;ZZ)V",
            (map_key, map_value, false, false),
        )
        .await?
        .into();
    assert!(
        jvm.invoke_virtual::<_, bool>(
            &entries,
            &entries.class_definition().name(),
            "contains",
            "(Ljava/lang/Object;)Z",
            (matching.clone(),)
        )
        .await?
    );
    assert_eq!(jvm.get_field::<i32>(&matching, "keyCalls", "I").await?, 1);
    assert_eq!(jvm.get_field::<i32>(&matching, "valueCalls", "I").await?, 1);

    let missing_key: ClassInstanceRef<Object> = jvm.new_class("java/lang/Integer", "(I)V", (8,)).await?.into();
    let null: ClassInstanceRef<Object> = None.into();
    let missing: ClassInstanceRef<Object> = jvm
        .new_class(
            "CollectionsEntryProbe",
            "(Ljava/lang/Object;Ljava/lang/Object;ZZ)V",
            (missing_key, null.clone(), false, true),
        )
        .await?
        .into();
    assert!(
        !jvm.invoke_virtual::<_, bool>(
            &entries,
            &entries.class_definition().name(),
            "contains",
            "(Ljava/lang/Object;)Z",
            (missing.clone(),)
        )
        .await?
    );
    assert_eq!(jvm.get_field::<i32>(&missing, "keyCalls", "I").await?, 1);
    assert_eq!(jvm.get_field::<i32>(&missing, "valueCalls", "I").await?, 0);

    let existing_key: ClassInstanceRef<Object> = jvm.new_class("java/lang/Integer", "(I)V", (7,)).await?.into();
    let required_throwing_value: ClassInstanceRef<Object> = jvm
        .new_class(
            "CollectionsEntryProbe",
            "(Ljava/lang/Object;Ljava/lang/Object;ZZ)V",
            (existing_key, null.clone(), false, true),
        )
        .await?
        .into();
    let result: Result<bool> = jvm
        .invoke_virtual(
            &entries,
            &entries.class_definition().name(),
            "contains",
            "(Ljava/lang/Object;)Z",
            (required_throwing_value.clone(),),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("contains must propagate getValue failure after finding the key");
    };
    assert!(jvm.is_instance(exception.as_ref(), "java/lang/IllegalStateException"));
    assert_eq!(jvm.get_field::<i32>(&required_throwing_value, "keyCalls", "I").await?, 1);
    assert_eq!(jvm.get_field::<i32>(&required_throwing_value, "valueCalls", "I").await?, 1);

    let throwing_key: ClassInstanceRef<Object> = jvm
        .new_class(
            "CollectionsEntryProbe",
            "(Ljava/lang/Object;Ljava/lang/Object;ZZ)V",
            (null.clone(), null.clone(), true, true),
        )
        .await?
        .into();
    let result: Result<bool> = jvm
        .invoke_virtual(
            &entries,
            &entries.class_definition().name(),
            "contains",
            "(Ljava/lang/Object;)Z",
            (throwing_key.clone(),),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("contains must propagate getKey failure");
    };
    assert!(jvm.is_instance(exception.as_ref(), "java/lang/IllegalStateException"));
    assert_eq!(jvm.get_field::<i32>(&throwing_key, "keyCalls", "I").await?, 1);
    assert_eq!(jvm.get_field::<i32>(&throwing_key, "valueCalls", "I").await?, 0);

    let never_reached: ClassInstanceRef<Object> = jvm
        .new_class(
            "CollectionsEntryProbe",
            "(Ljava/lang/Object;Ljava/lang/Object;ZZ)V",
            (null.clone(), null, true, true),
        )
        .await?
        .into();
    let candidates: ClassInstanceRef<Object> = jvm.new_class("java/util/ArrayList", "()V", ()).await?.into();
    let _: bool = jvm
        .invoke_virtual(
            &candidates,
            &candidates.class_definition().name(),
            "add",
            "(Ljava/lang/Object;)Z",
            (matching.clone(),),
        )
        .await?;
    let _: bool = jvm
        .invoke_virtual(
            &candidates,
            &candidates.class_definition().name(),
            "add",
            "(Ljava/lang/Object;)Z",
            (missing.clone(),),
        )
        .await?;
    let _: bool = jvm
        .invoke_virtual(
            &candidates,
            &candidates.class_definition().name(),
            "add",
            "(Ljava/lang/Object;)Z",
            (never_reached.clone(),),
        )
        .await?;
    assert!(
        !jvm.invoke_virtual::<_, bool>(
            &entries,
            &entries.class_definition().name(),
            "containsAll",
            "(Ljava/util/Collection;)Z",
            (candidates,)
        )
        .await?
    );
    assert_eq!(jvm.get_field::<i32>(&never_reached, "keyCalls", "I").await?, 0);
    assert_eq!(jvm.get_field::<i32>(&never_reached, "valueCalls", "I").await?, 0);

    Ok(())
}

#[tokio::test]
async fn test_coll_08_unmodifiable_entry_arrays_preserve_runtime_type_and_partial_failure_state() -> Result<()> {
    let jvm = test_jvm().await?;
    let map: ClassInstanceRef<Object> = jvm.new_class("java/util/HashMap", "()V", ()).await?.into();
    for value in [1, 2] {
        let key: ClassInstanceRef<Object> = jvm.new_class("java/lang/Integer", "(I)V", (value,)).await?.into();
        let mapped: ClassInstanceRef<Object> = jvm.new_class("java/lang/Integer", "(I)V", (value * 10,)).await?.into();
        let _: ClassInstanceRef<Object> = jvm
            .invoke_virtual(
                &map,
                &map.class_definition().name(),
                "put",
                "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
                (key, mapped),
            )
            .await?;
    }
    let raw_entries: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&map, &map.class_definition().name(), "entrySet", "()Ljava/util/Set;", ())
        .await?;
    let raw_array: ClassInstanceRef<Array<Object>> = jvm
        .invoke_virtual(
            &raw_entries,
            &raw_entries.class_definition().name(),
            "toArray",
            "()[Ljava/lang/Object;",
            (),
        )
        .await?;
    let raw_values = jvm.load_array::<ClassInstanceRef<Object>>(&raw_array, 0, 2).await?;
    let wrapped_map: ClassInstanceRef<Object> = jvm
        .invoke_static("java/util/Collections", "unmodifiableMap", "(Ljava/util/Map;)Ljava/util/Map;", (map,))
        .await?;
    let entries: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&wrapped_map, &wrapped_map.class_definition().name(), "entrySet", "()Ljava/util/Set;", ())
        .await?;

    let untyped: ClassInstanceRef<Array<Object>> = jvm
        .invoke_virtual(&entries, &entries.class_definition().name(), "toArray", "()[Ljava/lang/Object;", ())
        .await?;
    for entry in jvm.load_array::<ClassInstanceRef<Object>>(&untyped, 0, 2).await? {
        assert_eq!(
            entry.class_definition().name(),
            "java/util/Collections$UnmodifiableMap$UnmodifiableEntrySet$UnmodifiableEntry"
        );
        let null: ClassInstanceRef<Object> = None.into();
        let result: Result<ClassInstanceRef<Object>> = jvm
            .invoke_virtual(
                &entry,
                &entry.class_definition().name(),
                "setValue",
                "(Ljava/lang/Object;)Ljava/lang/Object;",
                (null,),
            )
            .await;
        let Err(JavaError::JavaException(exception)) = result else {
            panic!("untyped entry array must contain only unmodifiable entries");
        };
        assert!(jvm.is_instance(exception.as_ref(), "java/lang/UnsupportedOperationException"));
    }

    let mut oversized: ClassInstanceRef<Array<Object>> = jvm.instantiate_array("Ljava/util/Map$Entry;", 4).await?.into();
    jvm.store_array(&mut oversized, 2, [raw_values[0].clone(), raw_values[1].clone()]).await?;
    let oversized_identity = oversized.identity();
    let reused: ClassInstanceRef<Array<Object>> = jvm
        .invoke_virtual(
            &entries,
            &entries.class_definition().name(),
            "toArray",
            "([Ljava/lang/Object;)[Ljava/lang/Object;",
            (oversized,),
        )
        .await?;
    assert_eq!(reused.identity(), oversized_identity);
    let reused_values = jvm.load_array::<ClassInstanceRef<Object>>(&reused, 0, 4).await?;
    for entry in &reused_values[..2] {
        assert_eq!(
            entry.class_definition().name(),
            "java/util/Collections$UnmodifiableMap$UnmodifiableEntrySet$UnmodifiableEntry"
        );
    }
    assert!(reused_values[2].is_null(), "oversized typed array needs a null terminator");
    assert_eq!(
        reused_values[3].identity(),
        raw_values[1].identity(),
        "elements after the terminator remain untouched"
    );

    let small: ClassInstanceRef<Array<Object>> = jvm.instantiate_array("Ljava/util/Map$Entry;", 0).await?.into();
    let grown: ClassInstanceRef<Array<Object>> = jvm
        .invoke_virtual(
            &entries,
            &entries.class_definition().name(),
            "toArray",
            "([Ljava/lang/Object;)[Ljava/lang/Object;",
            (small,),
        )
        .await?;
    assert_eq!(grown.class_definition().name(), "[Ljava/util/Map$Entry;");
    assert_eq!(jvm.array_length(&grown).await?, 2);
    for entry in jvm.load_array::<ClassInstanceRef<Object>>(&grown, 0, 2).await? {
        assert_eq!(
            entry.class_definition().name(),
            "java/util/Collections$UnmodifiableMap$UnmodifiableEntrySet$UnmodifiableEntry"
        );
    }

    let mut concrete: ClassInstanceRef<Array<Object>> = jvm.instantiate_array("Ljava/util/HashMap$Entry;", 2).await?.into();
    jvm.store_array(&mut concrete, 0, [raw_values[0].clone(), raw_values[1].clone()]).await?;
    let result: Result<ClassInstanceRef<Array<Object>>> = jvm
        .invoke_virtual(
            &entries,
            &entries.class_definition().name(),
            "toArray",
            "([Ljava/lang/Object;)[Ljava/lang/Object;",
            (concrete.clone(),),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("HashMap.Entry[] must reject wrapped entries");
    };
    assert!(jvm.is_instance(exception.as_ref(), "java/lang/ArrayStoreException"));
    let concrete_values = jvm.load_array::<ClassInstanceRef<Object>>(&concrete, 0, 2).await?;
    assert_eq!(concrete_values[0].identity(), raw_values[0].identity());
    assert_eq!(concrete_values[1].identity(), raw_values[1].identity());

    let sentinel_row: ClassInstanceRef<Object> = jvm.instantiate_array("Ljava/util/Map$Entry;", 1).await?.into();
    let mut matrix: ClassInstanceRef<Array<Object>> = jvm.instantiate_array("[Ljava/util/Map$Entry;", 3).await?.into();
    jvm.store_array(&mut matrix, 0, [sentinel_row.clone(), sentinel_row.clone(), sentinel_row.clone()])
        .await?;
    let result: Result<ClassInstanceRef<Array<Object>>> = jvm
        .invoke_virtual(
            &entries,
            &entries.class_definition().name(),
            "toArray",
            "([Ljava/lang/Object;)[Ljava/lang/Object;",
            (matrix.clone(),),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("Map.Entry[][] must reject scalar wrapped entries");
    };
    assert!(jvm.is_instance(exception.as_ref(), "java/lang/ArrayStoreException"));
    for row in jvm.load_array::<ClassInstanceRef<Object>>(&matrix, 0, 3).await? {
        assert_eq!(row.identity(), sentinel_row.identity(), "ASE must preserve every not-yet-written slot");
    }

    Ok(())
}

#[tokio::test]
async fn test_coll_08_unmodifiable_sorted_ranges_remain_live_and_unmodifiable() -> Result<()> {
    let jvm = test_jvm().await?;
    let map: ClassInstanceRef<Object> = jvm.new_class("java/util/TreeMap", "()V", ()).await?.into();
    let mut keys = Vec::new();
    for key in [1, 3, 5] {
        let key: ClassInstanceRef<Object> = jvm.new_class("java/lang/Integer", "(I)V", (key,)).await?.into();
        let value = JavaLangString::from_rust_string(&jvm, "value").await?;
        let _: ClassInstanceRef<Object> = jvm
            .invoke_virtual(
                &map,
                &map.class_definition().name(),
                "put",
                "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
                (key.clone(), value),
            )
            .await?;
        keys.push(key);
    }
    let sorted_map: ClassInstanceRef<Object> = jvm
        .invoke_static(
            "java/util/Collections",
            "unmodifiableSortedMap",
            "(Ljava/util/SortedMap;)Ljava/util/SortedMap;",
            (map.clone(),),
        )
        .await?;
    let nested_sorted_map: ClassInstanceRef<Object> = jvm
        .invoke_static(
            "java/util/Collections",
            "unmodifiableSortedMap",
            "(Ljava/util/SortedMap;)Ljava/util/SortedMap;",
            (sorted_map.clone(),),
        )
        .await?;
    assert_ne!(sorted_map.identity(), nested_sorted_map.identity());
    assert!(
        jvm.invoke_virtual::<_, ClassInstanceRef<Object>>(
            &sorted_map,
            &sorted_map.class_definition().name(),
            "comparator",
            "()Ljava/util/Comparator;",
            ()
        )
        .await?
        .is_null()
    );
    let first: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&sorted_map, &sorted_map.class_definition().name(), "firstKey", "()Ljava/lang/Object;", ())
        .await?;
    let last: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&sorted_map, &sorted_map.class_definition().name(), "lastKey", "()Ljava/lang/Object;", ())
        .await?;
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&first, &first.class_definition().name(), "intValue", "()I", ())
            .await?,
        1
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&last, &last.class_definition().name(), "intValue", "()I", ())
            .await?,
        5
    );

    let range: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &sorted_map,
            &sorted_map.class_definition().name(),
            "subMap",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/SortedMap;",
            (keys[0].clone(), keys[2].clone()),
        )
        .await?;
    let nested_range: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &range,
            &range.class_definition().name(),
            "headMap",
            "(Ljava/lang/Object;)Ljava/util/SortedMap;",
            (keys[1].clone(),),
        )
        .await?;
    let head_range: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &sorted_map,
            &sorted_map.class_definition().name(),
            "headMap",
            "(Ljava/lang/Object;)Ljava/util/SortedMap;",
            (keys[2].clone(),),
        )
        .await?;
    let tail_range: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &sorted_map,
            &sorted_map.class_definition().name(),
            "tailMap",
            "(Ljava/lang/Object;)Ljava/util/SortedMap;",
            (keys[0].clone(),),
        )
        .await?;
    for wrapped_range in [&range, &nested_range, &head_range, &tail_range] {
        assert_eq!(wrapped_range.class_definition().name(), "java/util/Collections$UnmodifiableSortedMap");
    }
    let inserted_key: ClassInstanceRef<Object> = jvm.new_class("java/lang/Integer", "(I)V", (2,)).await?.into();
    let inserted_value = JavaLangString::from_rust_string(&jvm, "inserted").await?;
    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &map,
            &map.class_definition().name(),
            "put",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            (inserted_key.clone(), inserted_value.clone()),
        )
        .await?;
    assert!(
        jvm.invoke_virtual::<_, bool>(
            &range,
            &range.class_definition().name(),
            "containsKey",
            "(Ljava/lang/Object;)Z",
            (inserted_key.clone(),)
        )
        .await?
    );
    assert!(
        jvm.invoke_virtual::<_, bool>(
            &nested_range,
            &nested_range.class_definition().name(),
            "containsKey",
            "(Ljava/lang/Object;)Z",
            (inserted_key.clone(),),
        )
        .await?
    );
    assert!(
        jvm.invoke_virtual::<_, bool>(
            &head_range,
            &head_range.class_definition().name(),
            "containsKey",
            "(Ljava/lang/Object;)Z",
            (inserted_key.clone(),)
        )
        .await?
    );
    assert!(
        jvm.invoke_virtual::<_, bool>(
            &tail_range,
            &tail_range.class_definition().name(),
            "containsKey",
            "(Ljava/lang/Object;)Z",
            (inserted_key.clone(),)
        )
        .await?
    );
    let put: Result<ClassInstanceRef<Object>> = jvm
        .invoke_virtual(
            &range,
            &range.class_definition().name(),
            "put",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            (inserted_key.clone(), inserted_value),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = put else {
        panic!("sorted map range must remain unmodifiable");
    };
    assert!(jvm.is_instance(exception.as_ref(), "java/lang/UnsupportedOperationException"));

    let set: ClassInstanceRef<Object> = jvm.new_class("java/util/TreeSet", "()V", ()).await?.into();
    for key in &keys {
        let _: bool = jvm
            .invoke_virtual(&set, &set.class_definition().name(), "add", "(Ljava/lang/Object;)Z", (key.clone(),))
            .await?;
    }
    let sorted_set: ClassInstanceRef<Object> = jvm
        .invoke_static(
            "java/util/Collections",
            "unmodifiableSortedSet",
            "(Ljava/util/SortedSet;)Ljava/util/SortedSet;",
            (set.clone(),),
        )
        .await?;
    let nested_sorted_set: ClassInstanceRef<Object> = jvm
        .invoke_static(
            "java/util/Collections",
            "unmodifiableSortedSet",
            "(Ljava/util/SortedSet;)Ljava/util/SortedSet;",
            (sorted_set.clone(),),
        )
        .await?;
    assert_ne!(sorted_set.identity(), nested_sorted_set.identity());
    assert!(
        jvm.invoke_virtual::<_, ClassInstanceRef<Object>>(
            &sorted_set,
            &sorted_set.class_definition().name(),
            "comparator",
            "()Ljava/util/Comparator;",
            ()
        )
        .await?
        .is_null()
    );
    let first: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&sorted_set, &sorted_set.class_definition().name(), "first", "()Ljava/lang/Object;", ())
        .await?;
    let last: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&sorted_set, &sorted_set.class_definition().name(), "last", "()Ljava/lang/Object;", ())
        .await?;
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&first, &first.class_definition().name(), "intValue", "()I", ())
            .await?,
        1
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&last, &last.class_definition().name(), "intValue", "()I", ())
            .await?,
        5
    );
    let set_range: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &sorted_set,
            &sorted_set.class_definition().name(),
            "subSet",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/SortedSet;",
            (keys[0].clone(), keys[2].clone()),
        )
        .await?;
    let nested_set_range: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &set_range,
            &set_range.class_definition().name(),
            "tailSet",
            "(Ljava/lang/Object;)Ljava/util/SortedSet;",
            (inserted_key.clone(),),
        )
        .await?;
    let head_set_range: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &sorted_set,
            &sorted_set.class_definition().name(),
            "headSet",
            "(Ljava/lang/Object;)Ljava/util/SortedSet;",
            (keys[2].clone(),),
        )
        .await?;
    let tail_set_range: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &sorted_set,
            &sorted_set.class_definition().name(),
            "tailSet",
            "(Ljava/lang/Object;)Ljava/util/SortedSet;",
            (keys[0].clone(),),
        )
        .await?;
    for wrapped_range in [&set_range, &nested_set_range, &head_set_range, &tail_set_range] {
        assert_eq!(wrapped_range.class_definition().name(), "java/util/Collections$UnmodifiableSortedSet");
    }
    let _: bool = jvm
        .invoke_virtual(
            &set,
            &set.class_definition().name(),
            "add",
            "(Ljava/lang/Object;)Z",
            (inserted_key.clone(),),
        )
        .await?;
    assert!(
        jvm.invoke_virtual::<_, bool>(
            &set_range,
            &set_range.class_definition().name(),
            "contains",
            "(Ljava/lang/Object;)Z",
            (inserted_key.clone(),)
        )
        .await?
    );
    assert!(
        jvm.invoke_virtual::<_, bool>(
            &nested_set_range,
            &nested_set_range.class_definition().name(),
            "contains",
            "(Ljava/lang/Object;)Z",
            (inserted_key.clone(),),
        )
        .await?
    );
    assert!(
        jvm.invoke_virtual::<_, bool>(
            &head_set_range,
            &head_set_range.class_definition().name(),
            "contains",
            "(Ljava/lang/Object;)Z",
            (inserted_key.clone(),),
        )
        .await?
    );
    assert!(
        jvm.invoke_virtual::<_, bool>(
            &tail_set_range,
            &tail_set_range.class_definition().name(),
            "contains",
            "(Ljava/lang/Object;)Z",
            (inserted_key.clone(),),
        )
        .await?
    );
    let add: Result<bool> = jvm
        .invoke_virtual(
            &set_range,
            &set_range.class_definition().name(),
            "add",
            "(Ljava/lang/Object;)Z",
            (inserted_key,),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = add else {
        panic!("sorted set range must remain unmodifiable");
    };
    assert!(jvm.is_instance(exception.as_ref(), "java/lang/UnsupportedOperationException"));

    Ok(())
}

#[tokio::test]
async fn test_coll_08_unmodifiable_sorted_ranges_preserve_custom_comparator_boundaries() -> Result<()> {
    let jvm = collections_test_jvm().await?;
    let comparator: ClassInstanceRef<Object> = jvm.new_class("CollectionsComparator", "(ZZZ)V", (true, false, true)).await?.into();
    let high: ClassInstanceRef<Object> = jvm.new_class("CollectionsSortValue", "(IIZ)V", (30, 30, false)).await?.into();
    let middle: ClassInstanceRef<Object> = jvm.new_class("CollectionsSortValue", "(IIZ)V", (20, 20, false)).await?.into();
    let low: ClassInstanceRef<Object> = jvm.new_class("CollectionsSortValue", "(IIZ)V", (10, 10, false)).await?.into();
    let live_key: ClassInstanceRef<Object> = jvm.new_class("CollectionsSortValue", "(IIZ)V", (25, 25, false)).await?.into();
    let outside: ClassInstanceRef<Object> = jvm.new_class("CollectionsSortValue", "(IIZ)V", (40, 40, false)).await?.into();
    let null: ClassInstanceRef<Object> = None.into();

    let map: ClassInstanceRef<Object> = jvm
        .new_class("java/util/TreeMap", "(Ljava/util/Comparator;)V", (comparator.clone(),))
        .await?
        .into();
    for key in [high.clone(), middle.clone(), low.clone(), null.clone()] {
        let value = JavaLangString::from_rust_string(&jvm, "value").await?;
        let _: ClassInstanceRef<Object> = jvm
            .invoke_virtual(
                &map,
                &map.class_definition().name(),
                "put",
                "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
                (key, value),
            )
            .await?;
    }
    let sorted_map: ClassInstanceRef<Object> = jvm
        .invoke_static(
            "java/util/Collections",
            "unmodifiableSortedMap",
            "(Ljava/util/SortedMap;)Ljava/util/SortedMap;",
            (map.clone(),),
        )
        .await?;
    let returned_comparator: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &sorted_map,
            &sorted_map.class_definition().name(),
            "comparator",
            "()Ljava/util/Comparator;",
            (),
        )
        .await?;
    assert_eq!(returned_comparator.identity(), comparator.identity());
    assert!(
        jvm.invoke_virtual::<_, bool>(
            &sorted_map,
            &sorted_map.class_definition().name(),
            "containsKey",
            "(Ljava/lang/Object;)Z",
            (null.clone(),)
        )
        .await?
    );
    let first_key: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&sorted_map, &sorted_map.class_definition().name(), "firstKey", "()Ljava/lang/Object;", ())
        .await?;
    let last_key: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&sorted_map, &sorted_map.class_definition().name(), "lastKey", "()Ljava/lang/Object;", ())
        .await?;
    assert_eq!(first_key.identity(), high.identity());
    assert!(last_key.is_null());

    let map_range: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &sorted_map,
            &sorted_map.class_definition().name(),
            "subMap",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/SortedMap;",
            (high.clone(), null.clone()),
        )
        .await?;
    let map_head: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &sorted_map,
            &sorted_map.class_definition().name(),
            "headMap",
            "(Ljava/lang/Object;)Ljava/util/SortedMap;",
            (null.clone(),),
        )
        .await?;
    let map_tail: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &sorted_map,
            &sorted_map.class_definition().name(),
            "tailMap",
            "(Ljava/lang/Object;)Ljava/util/SortedMap;",
            (null.clone(),),
        )
        .await?;
    let nested_map: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &map_range,
            &map_range.class_definition().name(),
            "subMap",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/SortedMap;",
            (middle.clone(), low.clone()),
        )
        .await?;
    let equal_map: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &map_range,
            &map_range.class_definition().name(),
            "subMap",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/SortedMap;",
            (middle.clone(), middle.clone()),
        )
        .await?;
    for range in [&map_range, &map_head, &map_tail, &nested_map, &equal_map] {
        assert_eq!(range.class_definition().name(), "java/util/Collections$UnmodifiableSortedMap");
        let range_comparator: ClassInstanceRef<Object> = jvm
            .invoke_virtual(range, &range.class_definition().name(), "comparator", "()Ljava/util/Comparator;", ())
            .await?;
        assert_eq!(range_comparator.identity(), comparator.identity());
    }
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&equal_map, &equal_map.class_definition().name(), "size", "()I", ())
            .await?,
        0
    );
    assert!(
        !jvm.invoke_virtual::<_, bool>(
            &map_head,
            &map_head.class_definition().name(),
            "containsKey",
            "(Ljava/lang/Object;)Z",
            (null.clone(),)
        )
        .await?
    );
    assert!(
        jvm.invoke_virtual::<_, bool>(
            &map_tail,
            &map_tail.class_definition().name(),
            "containsKey",
            "(Ljava/lang/Object;)Z",
            (null.clone(),)
        )
        .await?
    );

    let result: Result<ClassInstanceRef<Object>> = jvm
        .invoke_virtual(
            &sorted_map,
            &sorted_map.class_definition().name(),
            "subMap",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/SortedMap;",
            (low.clone(), high.clone()),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("reverse comparator must reject reversed map boundaries");
    };
    assert!(jvm.is_instance(exception.as_ref(), "java/lang/IllegalArgumentException"));
    let result: Result<ClassInstanceRef<Object>> = jvm
        .invoke_virtual(
            &map_range,
            &map_range.class_definition().name(),
            "tailMap",
            "(Ljava/lang/Object;)Ljava/util/SortedMap;",
            (outside.clone(),),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("nested map range must reject an endpoint outside its lower boundary");
    };
    assert!(jvm.is_instance(exception.as_ref(), "java/lang/IllegalArgumentException"));

    let live_value = JavaLangString::from_rust_string(&jvm, "live").await?;
    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &map,
            &map.class_definition().name(),
            "put",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            (live_key.clone(), live_value.clone()),
        )
        .await?;
    assert!(
        jvm.invoke_virtual::<_, bool>(
            &map_range,
            &map_range.class_definition().name(),
            "containsKey",
            "(Ljava/lang/Object;)Z",
            (live_key.clone(),)
        )
        .await?
    );

    jvm.put_field(&mut comparator.clone(), "fail", "Z", true).await?;
    let result: Result<ClassInstanceRef<Object>> = jvm
        .invoke_virtual(
            &map_range,
            &map_range.class_definition().name(),
            "headMap",
            "(Ljava/lang/Object;)Ljava/util/SortedMap;",
            (middle.clone(),),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("range factory must propagate the backing comparator exception");
    };
    assert!(jvm.is_instance(exception.as_ref(), "java/lang/IllegalStateException"));
    let result: Result<ClassInstanceRef<Object>> = jvm
        .invoke_virtual(
            &map_range,
            &map_range.class_definition().name(),
            "put",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            (outside.clone(), live_value),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("unmodifiable map mutation must throw before comparator range validation");
    };
    assert!(jvm.is_instance(exception.as_ref(), "java/lang/UnsupportedOperationException"));
    jvm.put_field(&mut comparator.clone(), "fail", "Z", false).await?;

    let set: ClassInstanceRef<Object> = jvm
        .new_class("java/util/TreeSet", "(Ljava/util/Comparator;)V", (comparator.clone(),))
        .await?
        .into();
    for key in [high.clone(), middle.clone(), low.clone(), null.clone()] {
        let _: bool = jvm
            .invoke_virtual(&set, &set.class_definition().name(), "add", "(Ljava/lang/Object;)Z", (key,))
            .await?;
    }
    let sorted_set: ClassInstanceRef<Object> = jvm
        .invoke_static(
            "java/util/Collections",
            "unmodifiableSortedSet",
            "(Ljava/util/SortedSet;)Ljava/util/SortedSet;",
            (set.clone(),),
        )
        .await?;
    let returned_comparator: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &sorted_set,
            &sorted_set.class_definition().name(),
            "comparator",
            "()Ljava/util/Comparator;",
            (),
        )
        .await?;
    assert_eq!(returned_comparator.identity(), comparator.identity());
    assert!(
        jvm.invoke_virtual::<_, bool>(
            &sorted_set,
            &sorted_set.class_definition().name(),
            "contains",
            "(Ljava/lang/Object;)Z",
            (null.clone(),)
        )
        .await?
    );
    let first: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&sorted_set, &sorted_set.class_definition().name(), "first", "()Ljava/lang/Object;", ())
        .await?;
    let last: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&sorted_set, &sorted_set.class_definition().name(), "last", "()Ljava/lang/Object;", ())
        .await?;
    assert_eq!(first.identity(), high.identity());
    assert!(last.is_null());

    let set_range: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &sorted_set,
            &sorted_set.class_definition().name(),
            "subSet",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/SortedSet;",
            (high.clone(), null.clone()),
        )
        .await?;
    let set_head: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &sorted_set,
            &sorted_set.class_definition().name(),
            "headSet",
            "(Ljava/lang/Object;)Ljava/util/SortedSet;",
            (null.clone(),),
        )
        .await?;
    let set_tail: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &sorted_set,
            &sorted_set.class_definition().name(),
            "tailSet",
            "(Ljava/lang/Object;)Ljava/util/SortedSet;",
            (null.clone(),),
        )
        .await?;
    let nested_set: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &set_range,
            &set_range.class_definition().name(),
            "subSet",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/SortedSet;",
            (middle.clone(), low.clone()),
        )
        .await?;
    let equal_set: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &set_range,
            &set_range.class_definition().name(),
            "subSet",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/SortedSet;",
            (middle.clone(), middle.clone()),
        )
        .await?;
    for range in [&set_range, &set_head, &set_tail, &nested_set, &equal_set] {
        assert_eq!(range.class_definition().name(), "java/util/Collections$UnmodifiableSortedSet");
        let range_comparator: ClassInstanceRef<Object> = jvm
            .invoke_virtual(range, &range.class_definition().name(), "comparator", "()Ljava/util/Comparator;", ())
            .await?;
        assert_eq!(range_comparator.identity(), comparator.identity());
    }
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&equal_set, &equal_set.class_definition().name(), "size", "()I", ())
            .await?,
        0
    );
    assert!(
        !jvm.invoke_virtual::<_, bool>(
            &set_head,
            &set_head.class_definition().name(),
            "contains",
            "(Ljava/lang/Object;)Z",
            (null.clone(),)
        )
        .await?
    );
    assert!(
        jvm.invoke_virtual::<_, bool>(
            &set_tail,
            &set_tail.class_definition().name(),
            "contains",
            "(Ljava/lang/Object;)Z",
            (null.clone(),)
        )
        .await?
    );

    let result: Result<ClassInstanceRef<Object>> = jvm
        .invoke_virtual(
            &sorted_set,
            &sorted_set.class_definition().name(),
            "subSet",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/util/SortedSet;",
            (low, high),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("reverse comparator must reject reversed set boundaries");
    };
    assert!(jvm.is_instance(exception.as_ref(), "java/lang/IllegalArgumentException"));
    let result: Result<ClassInstanceRef<Object>> = jvm
        .invoke_virtual(
            &set_range,
            &set_range.class_definition().name(),
            "tailSet",
            "(Ljava/lang/Object;)Ljava/util/SortedSet;",
            (outside.clone(),),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("nested set range must reject an endpoint outside its lower boundary");
    };
    assert!(jvm.is_instance(exception.as_ref(), "java/lang/IllegalArgumentException"));

    let _: bool = jvm
        .invoke_virtual(&set, &set.class_definition().name(), "add", "(Ljava/lang/Object;)Z", (live_key.clone(),))
        .await?;
    assert!(
        jvm.invoke_virtual::<_, bool>(
            &set_range,
            &set_range.class_definition().name(),
            "contains",
            "(Ljava/lang/Object;)Z",
            (live_key,)
        )
        .await?
    );

    jvm.put_field(&mut comparator.clone(), "fail", "Z", true).await?;
    let result: Result<ClassInstanceRef<Object>> = jvm
        .invoke_virtual(
            &set_range,
            &set_range.class_definition().name(),
            "headSet",
            "(Ljava/lang/Object;)Ljava/util/SortedSet;",
            (middle,),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("set range factory must propagate the backing comparator exception");
    };
    assert!(jvm.is_instance(exception.as_ref(), "java/lang/IllegalStateException"));
    let result: Result<bool> = jvm
        .invoke_virtual(
            &set_range,
            &set_range.class_definition().name(),
            "add",
            "(Ljava/lang/Object;)Z",
            (outside,),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("unmodifiable set mutation must throw before comparator range validation");
    };
    assert!(jvm.is_instance(exception.as_ref(), "java/lang/UnsupportedOperationException"));

    Ok(())
}
