use alloc::{boxed::Box, collections::BTreeMap, vec};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use java_runtime::{RuntimeClassProto, RuntimeContext, classes::java::lang::Object};
use jvm::{Array, ClassInstanceRef, JavaChar, JavaError, Jvm, Result, runtime::JavaLangString};
use jvm_rust::ClassDefinitionImpl;

use test_utils::{TestRuntime, create_test_jvm, test_jvm};

struct ArraysSortValue;

impl ArraysSortValue {
    fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "ArraysSortValue",
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
        if !jvm.is_instance(other.as_ref(), "ArraysSortValue") {
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

struct ArraysComparator;

impl ArraysComparator {
    fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "ArraysComparator",
            parent_class: Some("java/lang/Object"),
            interfaces: vec!["java/util/Comparator"],
            methods: vec![
                JavaMethodProto::new("<init>", "(ZZ)V", Self::init, MethodAccessFlags::PUBLIC),
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
            ],
            access_flags: ClassAccessFlags::empty(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, reverse: bool, fail: bool) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "reverse", "Z", reverse).await?;
        jvm.put_field(&mut this, "fail", "Z", fail).await
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
        if left.is_null() || right.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "value").await);
        }
        let comparison = jvm
            .get_field::<i32>(&left, "key", "I")
            .await?
            .cmp(&jvm.get_field::<i32>(&right, "key", "I").await?) as i32;
        Ok(if jvm.get_field::<bool>(&this, "reverse", "Z").await? {
            -comparison
        } else {
            comparison
        })
    }
}

struct ArraysEqualsProbe;

impl ArraysEqualsProbe {
    fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "ArraysEqualsProbe",
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

async fn arrays_object_test_jvm() -> Result<Jvm> {
    let runtime = TestRuntime::new(BTreeMap::new());
    let jvm = create_test_jvm(runtime.clone()).await?;
    for proto in [ArraysSortValue::as_proto(), ArraysComparator::as_proto(), ArraysEqualsProbe::as_proto()] {
        jvm.register_class(
            Box::new(ClassDefinitionImpl::from_class_proto(proto, Box::new(runtime.clone()) as Box<_>)),
            None,
        )
        .await?;
    }
    Ok(jvm)
}

#[tokio::test]
async fn test_arr_00_exact_descriptors_access_and_registration() -> Result<()> {
    let jvm = test_jvm().await?;
    let class = jvm.resolve_class("java/util/Arrays").await?;
    assert!(
        class
            .definition
            .access_flags()
            .contains(ClassAccessFlags::PUBLIC | ClassAccessFlags::FINAL)
    );
    assert!(
        class
            .definition
            .method("<init>", "()V", false)
            .expect("Arrays private constructor")
            .access_flags()
            .contains(MethodAccessFlags::PRIVATE)
    );

    let descriptors = [
        ("sort", "([B)V"),
        ("sort", "([BII)V"),
        ("sort", "([C)V"),
        ("sort", "([CII)V"),
        ("sort", "([S)V"),
        ("sort", "([SII)V"),
        ("sort", "([I)V"),
        ("sort", "([III)V"),
        ("sort", "([J)V"),
        ("sort", "([JII)V"),
        ("sort", "([F)V"),
        ("sort", "([FII)V"),
        ("sort", "([D)V"),
        ("sort", "([DII)V"),
        ("sort", "([Ljava/lang/Object;)V"),
        ("sort", "([Ljava/lang/Object;II)V"),
        ("sort", "([Ljava/lang/Object;Ljava/util/Comparator;)V"),
        ("sort", "([Ljava/lang/Object;IILjava/util/Comparator;)V"),
        ("binarySearch", "([BB)I"),
        ("binarySearch", "([CC)I"),
        ("binarySearch", "([SS)I"),
        ("binarySearch", "([II)I"),
        ("binarySearch", "([JJ)I"),
        ("binarySearch", "([FF)I"),
        ("binarySearch", "([DD)I"),
        ("binarySearch", "([Ljava/lang/Object;Ljava/lang/Object;)I"),
        ("binarySearch", "([Ljava/lang/Object;Ljava/lang/Object;Ljava/util/Comparator;)I"),
        ("equals", "([Z[Z)Z"),
        ("equals", "([B[B)Z"),
        ("equals", "([C[C)Z"),
        ("equals", "([S[S)Z"),
        ("equals", "([I[I)Z"),
        ("equals", "([J[J)Z"),
        ("equals", "([F[F)Z"),
        ("equals", "([D[D)Z"),
        ("equals", "([Ljava/lang/Object;[Ljava/lang/Object;)Z"),
        ("fill", "([ZZ)V"),
        ("fill", "([ZIIZ)V"),
        ("fill", "([BB)V"),
        ("fill", "([BIIB)V"),
        ("fill", "([CC)V"),
        ("fill", "([CIIC)V"),
        ("fill", "([SS)V"),
        ("fill", "([SIIS)V"),
        ("fill", "([II)V"),
        ("fill", "([IIII)V"),
        ("fill", "([JJ)V"),
        ("fill", "([JIIJ)V"),
        ("fill", "([FF)V"),
        ("fill", "([FIIF)V"),
        ("fill", "([DD)V"),
        ("fill", "([DIID)V"),
        ("fill", "([Ljava/lang/Object;Ljava/lang/Object;)V"),
        ("fill", "([Ljava/lang/Object;IILjava/lang/Object;)V"),
        ("asList", "([Ljava/lang/Object;)Ljava/util/List;"),
    ];
    for (name, descriptor) in descriptors {
        let method = class
            .definition
            .method(name, descriptor, true)
            .unwrap_or_else(|| panic!("missing Arrays.{name}{descriptor}"));
        assert!(
            method.access_flags().contains(MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC),
            "wrong access for Arrays.{name}{descriptor}"
        );
    }
    assert!(class.definition.method("sort", "([Z)V", true).is_none());
    assert!(class.definition.method("sort", "([ZII)V", true).is_none());
    assert!(class.definition.method("binarySearch", "([ZZ)I", true).is_none());

    let array_list = jvm.resolve_class("java/util/Arrays$ArrayList").await?;
    assert_eq!(array_list.definition.super_class_name().as_deref(), Some("java/util/AbstractList"));
    assert!(array_list.definition.interface_names().iter().any(|name| name == "java/io/Serializable"));
    assert!(!array_list.definition.access_flags().contains(ClassAccessFlags::PUBLIC));
    let field = array_list
        .definition
        .field("a", "[Ljava/lang/Object;", false)
        .expect("Arrays$ArrayList.a");
    assert!(field.access_flags().contains(FieldAccessFlags::PRIVATE | FieldAccessFlags::FINAL));

    Ok(())
}

#[tokio::test]
async fn test_arr_01_all_primitive_sort_overloads_and_ranges() -> Result<()> {
    let jvm = test_jvm().await?;

    let mut bytes: ClassInstanceRef<Array<i8>> = jvm.instantiate_array("B", 3).await?.into();
    jvm.store_array(&mut bytes, 0, [3i8, -1, 2]).await?;
    jvm.invoke_static::<_, ()>("java/util/Arrays", "sort", "([B)V", (bytes.clone(),)).await?;
    assert_eq!(jvm.load_array::<i8>(&bytes, 0, 3).await?, [-1, 2, 3]);

    let mut chars: ClassInstanceRef<Array<JavaChar>> = jvm.instantiate_array("C", 4).await?.into();
    jvm.store_array(&mut chars, 0, [9 as JavaChar, 4 as JavaChar, 2 as JavaChar, 8 as JavaChar])
        .await?;
    jvm.invoke_static::<_, ()>("java/util/Arrays", "sort", "([CII)V", (chars.clone(), 1, 3))
        .await?;
    assert_eq!(jvm.load_array::<JavaChar>(&chars, 0, 4).await?, [9, 2, 4, 8]);

    let mut shorts: ClassInstanceRef<Array<i16>> = jvm.instantiate_array("S", 3).await?.into();
    jvm.store_array(&mut shorts, 0, [2i16, -4, 1]).await?;
    jvm.invoke_static::<_, ()>("java/util/Arrays", "sort", "([S)V", (shorts.clone(),)).await?;
    assert_eq!(jvm.load_array::<i16>(&shorts, 0, 3).await?, [-4, 1, 2]);

    let mut ints: ClassInstanceRef<Array<i32>> = jvm.instantiate_array("I", 5).await?.into();
    jvm.store_array(&mut ints, 0, [9, 4, 3, 2, 8]).await?;
    jvm.invoke_static::<_, ()>("java/util/Arrays", "sort", "([III)V", (ints.clone(), 1, 4))
        .await?;
    assert_eq!(jvm.load_array::<i32>(&ints, 0, 5).await?, [9, 2, 3, 4, 8]);

    let mut longs: ClassInstanceRef<Array<i64>> = jvm.instantiate_array("J", 3).await?.into();
    jvm.store_array(&mut longs, 0, [i64::MAX, i64::MIN, 0]).await?;
    jvm.invoke_static::<_, ()>("java/util/Arrays", "sort", "([J)V", (longs.clone(),)).await?;
    assert_eq!(jvm.load_array::<i64>(&longs, 0, 3).await?, [i64::MIN, 0, i64::MAX]);

    let mut floats: ClassInstanceRef<Array<f32>> = jvm.instantiate_array("F", 3).await?.into();
    jvm.store_array(&mut floats, 0, [2.0f32, -1.0f32, 1.0f32]).await?;
    jvm.invoke_static::<_, ()>("java/util/Arrays", "sort", "([F)V", (floats.clone(),)).await?;
    assert_eq!(jvm.load_array::<f32>(&floats, 0, 3).await?, [-1.0, 1.0, 2.0]);

    let mut doubles: ClassInstanceRef<Array<f64>> = jvm.instantiate_array("D", 4).await?.into();
    jvm.store_array(&mut doubles, 0, [9.0, 2.0, 1.0, 8.0]).await?;
    jvm.invoke_static::<_, ()>("java/util/Arrays", "sort", "([DII)V", (doubles.clone(), 1, 3))
        .await?;
    assert_eq!(jvm.load_array::<f64>(&doubles, 0, 4).await?, [9.0, 1.0, 2.0, 8.0]);

    let empty: ClassInstanceRef<Array<i32>> = jvm.instantiate_array("I", 0).await?.into();
    jvm.invoke_static::<_, ()>("java/util/Arrays", "sort", "([I)V", (empty,)).await?;
    let single: ClassInstanceRef<Array<i32>> = jvm.instantiate_array("I", 1).await?.into();
    jvm.invoke_static::<_, ()>("java/util/Arrays", "sort", "([III)V", (single.clone(), 0, 1))
        .await?;
    assert_eq!(jvm.load_array::<i32>(&single, 0, 1).await?, [0]);

    Ok(())
}

#[tokio::test]
async fn test_arr_01_range_exception_order() -> Result<()> {
    let jvm = test_jvm().await?;
    let array: ClassInstanceRef<Array<i32>> = jvm.instantiate_array("I", 2).await?.into();

    let result: Result<()> = jvm.invoke_static("java/util/Arrays", "sort", "([III)V", (array.clone(), 4, 3)).await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("fromIndex > toIndex must fail");
    };
    assert!(jvm.is_instance(exception.as_ref(), "java/lang/IllegalArgumentException"));

    for (from, to) in [(-1, 1), (0, 3)] {
        let result: Result<()> = jvm.invoke_static("java/util/Arrays", "sort", "([III)V", (array.clone(), from, to)).await;
        let Err(JavaError::JavaException(exception)) = result else {
            panic!("out-of-bounds range must fail");
        };
        assert!(jvm.is_instance(exception.as_ref(), "java/lang/ArrayIndexOutOfBoundsException"));
    }

    Ok(())
}

#[tokio::test]
async fn test_arr_01_float_double_total_order() -> Result<()> {
    let jvm = test_jvm().await?;
    let nan_a = f32::from_bits(0x7fc0_0001);
    let nan_b = f32::from_bits(0xffc0_1234);
    let mut floats: ClassInstanceRef<Array<f32>> = jvm.instantiate_array("F", 9).await?.into();
    jvm.store_array(
        &mut floats,
        0,
        [nan_a, 0.0, f32::INFINITY, -0.0, -1.0, nan_b, f32::NEG_INFINITY, 1.0, 4.0],
    )
    .await?;
    jvm.invoke_static::<_, ()>("java/util/Arrays", "sort", "([F)V", (floats.clone(),)).await?;
    let sorted = jvm.load_array::<f32>(&floats, 0, 9).await?;
    assert_eq!(sorted[..7], [f32::NEG_INFINITY, -1.0, -0.0, 0.0, 1.0, 4.0, f32::INFINITY]);
    assert_eq!(sorted[2].to_bits(), (-0.0f32).to_bits());
    assert_eq!(sorted[3].to_bits(), 0.0f32.to_bits());
    assert!(sorted[7].is_nan() && sorted[8].is_nan());

    let nan_a = f64::from_bits(0x7ff8_0000_0000_0001);
    let nan_b = f64::from_bits(0xfff8_0000_0000_1234);
    let mut doubles: ClassInstanceRef<Array<f64>> = jvm.instantiate_array("D", 7).await?.into();
    jvm.store_array(&mut doubles, 0, [nan_a, 0.0, f64::INFINITY, -0.0, nan_b, f64::NEG_INFINITY, 1.0])
        .await?;
    jvm.invoke_static::<_, ()>("java/util/Arrays", "sort", "([D)V", (doubles.clone(),))
        .await?;
    let sorted = jvm.load_array::<f64>(&doubles, 0, 7).await?;
    assert_eq!(sorted[..5], [f64::NEG_INFINITY, -0.0, 0.0, 1.0, f64::INFINITY]);
    assert_eq!(sorted[1].to_bits(), (-0.0f64).to_bits());
    assert_eq!(sorted[2].to_bits(), 0.0f64.to_bits());
    assert!(sorted[5].is_nan() && sorted[6].is_nan());

    Ok(())
}

#[tokio::test]
async fn test_arr_02_object_sort_is_stable_and_supports_all_modes() -> Result<()> {
    let jvm = arrays_object_test_jvm().await?;
    let values = [
        jvm.new_class("ArraysSortValue", "(IIZ)V", (2, 0, false)).await?,
        jvm.new_class("ArraysSortValue", "(IIZ)V", (1, 1, false)).await?,
        jvm.new_class("ArraysSortValue", "(IIZ)V", (2, 2, false)).await?,
        jvm.new_class("ArraysSortValue", "(IIZ)V", (1, 3, false)).await?,
    ];
    let mut array: ClassInstanceRef<Array<Object>> = jvm.instantiate_array("Ljava/lang/Object;", 4).await?.into();
    jvm.store_array(&mut array, 0, values).await?;
    jvm.invoke_static::<_, ()>("java/util/Arrays", "sort", "([Ljava/lang/Object;)V", (array.clone(),))
        .await?;
    let sorted = jvm.load_array::<ClassInstanceRef<Object>>(&array, 0, 4).await?;
    let mut ids = vec![];
    for value in sorted {
        ids.push(jvm.get_field::<i32>(&value, "id", "I").await?);
    }
    assert_eq!(ids, [1, 3, 0, 2]);

    let outside_left = jvm.new_class("ArraysSortValue", "(IIZ)V", (99, 10, false)).await?;
    let low = jvm.new_class("ArraysSortValue", "(IIZ)V", (1, 11, false)).await?;
    let high = jvm.new_class("ArraysSortValue", "(IIZ)V", (3, 12, false)).await?;
    let outside_right = jvm.new_class("ArraysSortValue", "(IIZ)V", (-99, 13, false)).await?;
    let mut range: ClassInstanceRef<Array<Object>> = jvm.instantiate_array("Ljava/lang/Object;", 4).await?.into();
    jvm.store_array(&mut range, 0, [outside_left.clone(), low, high, outside_right.clone()])
        .await?;
    let reverse = jvm.new_class("ArraysComparator", "(ZZ)V", (true, false)).await?;
    jvm.invoke_static::<_, ()>(
        "java/util/Arrays",
        "sort",
        "([Ljava/lang/Object;IILjava/util/Comparator;)V",
        (range.clone(), 1, 3, reverse),
    )
    .await?;
    let sorted = jvm.load_array::<ClassInstanceRef<Object>>(&range, 0, 4).await?;
    assert_eq!(sorted[0].identity(), outside_left.identity());
    assert_eq!(jvm.get_field::<i32>(&sorted[1], "key", "I").await?, 3);
    assert_eq!(jvm.get_field::<i32>(&sorted[2], "key", "I").await?, 1);
    assert_eq!(sorted[3].identity(), outside_right.identity());

    let null_comparator: ClassInstanceRef<Object> = None.into();
    jvm.invoke_static::<_, ()>(
        "java/util/Arrays",
        "sort",
        "([Ljava/lang/Object;Ljava/util/Comparator;)V",
        (range.clone(), null_comparator),
    )
    .await?;
    let sorted = jvm.load_array::<ClassInstanceRef<Object>>(&range, 0, 4).await?;
    let keys = [
        jvm.get_field::<i32>(&sorted[0], "key", "I").await?,
        jvm.get_field::<i32>(&sorted[1], "key", "I").await?,
        jvm.get_field::<i32>(&sorted[2], "key", "I").await?,
        jvm.get_field::<i32>(&sorted[3], "key", "I").await?,
    ];
    assert_eq!(keys, [-99, 1, 3, 99]);

    Ok(())
}

#[tokio::test]
async fn test_arr_02_comparison_exceptions_do_not_partially_write() -> Result<()> {
    let jvm = arrays_object_test_jvm().await?;
    let first = jvm.new_class("ArraysSortValue", "(IIZ)V", (2, 1, false)).await?;
    let failing = jvm.new_class("ArraysSortValue", "(IIZ)V", (1, 2, true)).await?;
    let last = jvm.new_class("ArraysSortValue", "(IIZ)V", (0, 3, false)).await?;
    let mut natural: ClassInstanceRef<Array<Object>> = jvm.instantiate_array("Ljava/lang/Object;", 3).await?.into();
    jvm.store_array(&mut natural, 0, [first.clone(), failing.clone(), last.clone()]).await?;
    let result: Result<()> = jvm
        .invoke_static("java/util/Arrays", "sort", "([Ljava/lang/Object;)V", (natural.clone(),))
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("natural comparison failure must propagate");
    };
    assert!(jvm.is_instance(exception.as_ref(), "java/lang/IllegalStateException"));
    let after = jvm.load_array::<ClassInstanceRef<Object>>(&natural, 0, 3).await?;
    assert_eq!(
        after.iter().map(|value| value.identity()).collect::<Vec<_>>(),
        [first.identity(), failing.identity(), last.identity()]
    );

    let mut compared: ClassInstanceRef<Array<Object>> = jvm.instantiate_array("Ljava/lang/Object;", 3).await?.into();
    jvm.store_array(&mut compared, 0, [last.clone(), first.clone(), failing.clone()]).await?;
    let comparator = jvm.new_class("ArraysComparator", "(ZZ)V", (false, true)).await?;
    let result: Result<()> = jvm
        .invoke_static(
            "java/util/Arrays",
            "sort",
            "([Ljava/lang/Object;Ljava/util/Comparator;)V",
            (compared.clone(), comparator),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("comparator failure must propagate");
    };
    assert!(jvm.is_instance(exception.as_ref(), "java/lang/IllegalStateException"));
    let after = jvm.load_array::<ClassInstanceRef<Object>>(&compared, 0, 3).await?;
    assert_eq!(
        after.iter().map(|value| value.identity()).collect::<Vec<_>>(),
        [last.identity(), first.identity(), failing.identity()]
    );

    let first_object = jvm.new_class("java/lang/Object", "()V", ()).await?;
    let second_object = jvm.new_class("java/lang/Object", "()V", ()).await?;
    let mut non_comparable: ClassInstanceRef<Array<Object>> = jvm.instantiate_array("Ljava/lang/Object;", 2).await?.into();
    jvm.store_array(&mut non_comparable, 0, [first_object.clone(), second_object.clone()])
        .await?;
    let result: Result<()> = jvm
        .invoke_static("java/util/Arrays", "sort", "([Ljava/lang/Object;)V", (non_comparable.clone(),))
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("natural sort must reject non-Comparable values");
    };
    assert!(jvm.is_instance(exception.as_ref(), "java/lang/ClassCastException"));
    let after = jvm.load_array::<ClassInstanceRef<Object>>(&non_comparable, 0, 2).await?;
    assert_eq!(after[0].identity(), first_object.identity());
    assert_eq!(after[1].identity(), second_object.identity());

    Ok(())
}

#[tokio::test]
async fn test_arr_03_04_binary_search_insertion_points_and_object_modes() -> Result<()> {
    let jvm = arrays_object_test_jvm().await?;

    let mut bytes: ClassInstanceRef<Array<i8>> = jvm.instantiate_array("B", 3).await?.into();
    jvm.store_array(&mut bytes, 0, [-2i8, 0, 5]).await?;
    assert_eq!(
        jvm.invoke_static::<_, i32>("java/util/Arrays", "binarySearch", "([BB)I", (bytes, 5i8))
            .await?,
        2
    );
    let mut chars: ClassInstanceRef<Array<JavaChar>> = jvm.instantiate_array("C", 2).await?.into();
    jvm.store_array(&mut chars, 0, [1 as JavaChar, 3 as JavaChar]).await?;
    assert_eq!(
        jvm.invoke_static::<_, i32>("java/util/Arrays", "binarySearch", "([CC)I", (chars, 2 as JavaChar))
            .await?,
        -2
    );
    let mut shorts: ClassInstanceRef<Array<i16>> = jvm.instantiate_array("S", 2).await?.into();
    jvm.store_array(&mut shorts, 0, [-1i16, 7]).await?;
    assert_eq!(
        jvm.invoke_static::<_, i32>("java/util/Arrays", "binarySearch", "([SS)I", (shorts, -1i16))
            .await?,
        0
    );
    let mut ints: ClassInstanceRef<Array<i32>> = jvm.instantiate_array("I", 3).await?.into();
    jvm.store_array(&mut ints, 0, [1, 3, 5]).await?;
    for (key, expected) in [(0, -1), (1, 0), (2, -2), (5, 2), (6, -4)] {
        assert_eq!(
            jvm.invoke_static::<_, i32>("java/util/Arrays", "binarySearch", "([II)I", (ints.clone(), key))
                .await?,
            expected
        );
    }
    let mut longs: ClassInstanceRef<Array<i64>> = jvm.instantiate_array("J", 2).await?.into();
    jvm.store_array(&mut longs, 0, [i64::MIN, i64::MAX]).await?;
    assert_eq!(
        jvm.invoke_static::<_, i32>("java/util/Arrays", "binarySearch", "([JJ)I", (longs, i64::MAX))
            .await?,
        1
    );
    let mut floats: ClassInstanceRef<Array<f32>> = jvm.instantiate_array("F", 3).await?.into();
    jvm.store_array(&mut floats, 0, [-0.0, 0.0, f32::NAN]).await?;
    assert_eq!(
        jvm.invoke_static::<_, i32>("java/util/Arrays", "binarySearch", "([FF)I", (floats.clone(), -0.0f32))
            .await?,
        0
    );
    assert_eq!(
        jvm.invoke_static::<_, i32>("java/util/Arrays", "binarySearch", "([FF)I", (floats, f32::from_bits(0x7fc0_1234)))
            .await?,
        2
    );
    let mut doubles: ClassInstanceRef<Array<f64>> = jvm.instantiate_array("D", 2).await?.into();
    jvm.store_array(&mut doubles, 0, [0.0, f64::INFINITY]).await?;
    assert_eq!(
        jvm.invoke_static::<_, i32>("java/util/Arrays", "binarySearch", "([DD)I", (doubles, -0.0f64))
            .await?,
        -1
    );

    let low = jvm.new_class("ArraysSortValue", "(IIZ)V", (1, 1, false)).await?;
    let high = jvm.new_class("ArraysSortValue", "(IIZ)V", (3, 2, false)).await?;
    let key = jvm.new_class("ArraysSortValue", "(IIZ)V", (2, 3, false)).await?;
    let mut objects: ClassInstanceRef<Array<Object>> = jvm.instantiate_array("Ljava/lang/Object;", 2).await?.into();
    jvm.store_array(&mut objects, 0, [low, high]).await?;
    assert_eq!(
        jvm.invoke_static::<_, i32>(
            "java/util/Arrays",
            "binarySearch",
            "([Ljava/lang/Object;Ljava/lang/Object;)I",
            (objects.clone(), key.clone()),
        )
        .await?,
        -2
    );

    let high = jvm.new_class("ArraysSortValue", "(IIZ)V", (3, 4, false)).await?;
    let low = jvm.new_class("ArraysSortValue", "(IIZ)V", (1, 5, false)).await?;
    let mut descending: ClassInstanceRef<Array<Object>> = jvm.instantiate_array("Ljava/lang/Object;", 2).await?.into();
    jvm.store_array(&mut descending, 0, [high, low]).await?;
    let reverse = jvm.new_class("ArraysComparator", "(ZZ)V", (true, false)).await?;
    assert_eq!(
        jvm.invoke_static::<_, i32>(
            "java/util/Arrays",
            "binarySearch",
            "([Ljava/lang/Object;Ljava/lang/Object;Ljava/util/Comparator;)I",
            (descending, key, reverse),
        )
        .await?,
        -2
    );

    let failing_key = jvm.new_class("ArraysSortValue", "(IIZ)V", (2, 6, true)).await?;
    let result: Result<i32> = jvm
        .invoke_static(
            "java/util/Arrays",
            "binarySearch",
            "([Ljava/lang/Object;Ljava/lang/Object;)I",
            (objects, failing_key),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("natural binarySearch comparison failure must propagate");
    };
    assert!(jvm.is_instance(exception.as_ref(), "java/lang/IllegalStateException"));

    let first_object = jvm.new_class("java/lang/Object", "()V", ()).await?;
    let second_object = jvm.new_class("java/lang/Object", "()V", ()).await?;
    let mut non_comparable: ClassInstanceRef<Array<Object>> = jvm.instantiate_array("Ljava/lang/Object;", 1).await?.into();
    jvm.store_array(&mut non_comparable, 0, core::iter::once(first_object)).await?;
    let result: Result<i32> = jvm
        .invoke_static(
            "java/util/Arrays",
            "binarySearch",
            "([Ljava/lang/Object;Ljava/lang/Object;)I",
            (non_comparable, second_object),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("natural binarySearch must reject non-Comparable values");
    };
    assert!(jvm.is_instance(exception.as_ref(), "java/lang/ClassCastException"));

    Ok(())
}

#[tokio::test]
async fn test_arr_05_equals_null_float_bits_and_query_direction() -> Result<()> {
    let jvm = arrays_object_test_jvm().await?;
    let null_ints: ClassInstanceRef<Array<i32>> = None.into();
    let values: ClassInstanceRef<Array<i32>> = jvm.instantiate_array("I", 0).await?.into();
    assert!(
        jvm.invoke_static::<_, bool>("java/util/Arrays", "equals", "([I[I)Z", (null_ints.clone(), null_ints.clone()),)
            .await?
    );
    assert!(
        !jvm.invoke_static::<_, bool>("java/util/Arrays", "equals", "([I[I)Z", (null_ints, values))
            .await?
    );

    let mut first_float: ClassInstanceRef<Array<f32>> = jvm.instantiate_array("F", 1).await?.into();
    let mut second_float: ClassInstanceRef<Array<f32>> = jvm.instantiate_array("F", 1).await?.into();
    jvm.store_array(&mut first_float, 0, [f32::from_bits(0x7fc0_0001)]).await?;
    jvm.store_array(&mut second_float, 0, [f32::from_bits(0xffc0_1234)]).await?;
    assert!(
        jvm.invoke_static::<_, bool>("java/util/Arrays", "equals", "([F[F)Z", (first_float.clone(), second_float.clone()),)
            .await?
    );
    jvm.store_array(&mut first_float, 0, [-0.0f32]).await?;
    jvm.store_array(&mut second_float, 0, [0.0f32]).await?;
    assert!(
        !jvm.invoke_static::<_, bool>("java/util/Arrays", "equals", "([F[F)Z", (first_float, second_float))
            .await?
    );

    let mut first_double: ClassInstanceRef<Array<f64>> = jvm.instantiate_array("D", 1).await?.into();
    let mut second_double: ClassInstanceRef<Array<f64>> = jvm.instantiate_array("D", 1).await?.into();
    jvm.store_array(&mut first_double, 0, [f64::from_bits(0x7ff8_0000_0000_0001)]).await?;
    jvm.store_array(&mut second_double, 0, [f64::from_bits(0xfff8_0000_0000_1234)]).await?;
    assert!(
        jvm.invoke_static::<_, bool>("java/util/Arrays", "equals", "([D[D)Z", (first_double.clone(), second_double.clone()),)
            .await?
    );
    jvm.store_array(&mut first_double, 0, [-0.0f64]).await?;
    jvm.store_array(&mut second_double, 0, [0.0f64]).await?;
    assert!(
        !jvm.invoke_static::<_, bool>("java/util/Arrays", "equals", "([D[D)Z", (first_double, second_double))
            .await?
    );

    let query: ClassInstanceRef<Object> = jvm.new_class("ArraysEqualsProbe", "(Z)V", (true,)).await?.into();
    let stored: ClassInstanceRef<Object> = jvm.new_class("ArraysEqualsProbe", "(Z)V", (false,)).await?.into();
    let mut first: ClassInstanceRef<Array<Object>> = jvm.instantiate_array("Ljava/lang/Object;", 2).await?.into();
    let mut second: ClassInstanceRef<Array<Object>> = jvm.instantiate_array("Ljava/lang/Object;", 2).await?.into();
    let null: ClassInstanceRef<Object> = None.into();
    jvm.store_array(&mut first, 0, [query, null.clone()]).await?;
    jvm.store_array(&mut second, 0, [stored, null]).await?;
    assert!(
        jvm.invoke_static::<_, bool>("java/util/Arrays", "equals", "([Ljava/lang/Object;[Ljava/lang/Object;)Z", (first, second),)
            .await?
    );

    Ok(())
}

#[tokio::test]
async fn test_arr_06_07_fill_ranges_and_object_store_failure() -> Result<()> {
    let jvm = test_jvm().await?;
    let mut ints: ClassInstanceRef<Array<i32>> = jvm.instantiate_array("I", 5).await?.into();
    jvm.store_array(&mut ints, 0, [1, 2, 3, 4, 5]).await?;
    jvm.invoke_static::<_, ()>("java/util/Arrays", "fill", "([IIII)V", (ints.clone(), 1, 4, 9))
        .await?;
    assert_eq!(jvm.load_array::<i32>(&ints, 0, 5).await?, [1, 9, 9, 9, 5]);
    jvm.invoke_static::<_, ()>("java/util/Arrays", "fill", "([II)V", (ints.clone(), -2))
        .await?;
    assert_eq!(jvm.load_array::<i32>(&ints, 0, 5).await?, [-2; 5]);

    let result: Result<()> = jvm.invoke_static("java/util/Arrays", "fill", "([IIII)V", (ints.clone(), 8, 7, 0)).await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("fill fromIndex > toIndex must fail");
    };
    assert!(jvm.is_instance(exception.as_ref(), "java/lang/IllegalArgumentException"));
    let result: Result<()> = jvm.invoke_static("java/util/Arrays", "fill", "([IIII)V", (ints, -1, 2, 0)).await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("fill bounds must fail");
    };
    assert!(jvm.is_instance(exception.as_ref(), "java/lang/ArrayIndexOutOfBoundsException"));

    let left = JavaLangString::from_rust_string(&jvm, "left").await?;
    let right = JavaLangString::from_rust_string(&jvm, "right").await?;
    let mut strings: ClassInstanceRef<Array<Object>> = jvm.instantiate_array("Ljava/lang/String;", 3).await?.into();
    jvm.store_array(&mut strings, 0, [left.clone(), left.clone(), right.clone()]).await?;
    jvm.invoke_static::<_, ()>(
        "java/util/Arrays",
        "fill",
        "([Ljava/lang/Object;IILjava/lang/Object;)V",
        (strings.clone(), 1, 3, right.clone()),
    )
    .await?;
    let after = jvm.load_array::<ClassInstanceRef<Object>>(&strings, 0, 3).await?;
    assert_eq!(after[0].identity(), left.identity());
    assert_eq!(after[1].identity(), right.identity());
    assert_eq!(after[2].identity(), right.identity());

    let incompatible = jvm.new_class("java/lang/Object", "()V", ()).await?;
    let result: Result<()> = jvm
        .invoke_static(
            "java/util/Arrays",
            "fill",
            "([Ljava/lang/Object;IILjava/lang/Object;)V",
            (strings.clone(), 1, 3, incompatible),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("incompatible object fill must fail");
    };
    assert!(jvm.is_instance(exception.as_ref(), "java/lang/ArrayStoreException"));
    let after_failure = jvm.load_array::<ClassInstanceRef<Object>>(&strings, 0, 3).await?;
    assert_eq!(
        after_failure.iter().map(|value| value.identity()).collect::<Vec<_>>(),
        after.iter().map(|value| value.identity()).collect::<Vec<_>>()
    );

    Ok(())
}

#[tokio::test]
async fn test_arr_08_as_list_is_live_fixed_size_and_uses_abstract_list_contracts() -> Result<()> {
    let jvm = test_jvm().await?;
    let first = JavaLangString::from_rust_string(&jvm, "first").await?;
    let second = JavaLangString::from_rust_string(&jvm, "second").await?;
    let third = JavaLangString::from_rust_string(&jvm, "third").await?;
    let mut array: ClassInstanceRef<Array<Object>> = jvm.instantiate_array("Ljava/lang/String;", 2).await?.into();
    jvm.store_array(&mut array, 0, [first.clone(), second.clone()]).await?;
    let list: ClassInstanceRef<Object> = jvm
        .invoke_static("java/util/Arrays", "asList", "([Ljava/lang/Object;)Ljava/util/List;", (array.clone(),))
        .await?;
    assert!(jvm.is_instance(list.as_ref(), "java/util/List"));
    assert!(jvm.is_instance(list.as_ref(), "java/io/Serializable"));
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&list, &list.class_definition().name(), "size", "()I", ())
            .await?,
        2
    );

    jvm.store_array(&mut array, 0, core::iter::once(third.clone())).await?;
    let from_list: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&list, &list.class_definition().name(), "get", "(I)Ljava/lang/Object;", (0,))
        .await?;
    assert_eq!(from_list.identity(), third.identity());
    let previous: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &list,
            &list.class_definition().name(),
            "set",
            "(ILjava/lang/Object;)Ljava/lang/Object;",
            (1, first.clone()),
        )
        .await?;
    assert_eq!(previous.identity(), second.identity());
    let from_array = jvm.load_array::<ClassInstanceRef<Object>>(&array, 1, 1).await?.remove(0);
    assert_eq!(from_array.identity(), first.identity());
    let incompatible = jvm.new_class("java/lang/Object", "()V", ()).await?;
    let result: Result<ClassInstanceRef<Object>> = jvm
        .invoke_virtual(
            &list,
            &list.class_definition().name(),
            "set",
            "(ILjava/lang/Object;)Ljava/lang/Object;",
            (1, incompatible),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("array-backed list set must preserve the component type");
    };
    assert!(jvm.is_instance(exception.as_ref(), "java/lang/ArrayStoreException"));
    assert_eq!(
        jvm.load_array::<ClassInstanceRef<Object>>(&array, 1, 1).await?.remove(0).identity(),
        first.identity()
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(
            &list,
            &list.class_definition().name(),
            "indexOf",
            "(Ljava/lang/Object;)I",
            (first.clone(),)
        )
        .await?,
        1
    );
    assert!(
        jvm.invoke_virtual::<_, bool>(
            &list,
            &list.class_definition().name(),
            "contains",
            "(Ljava/lang/Object;)Z",
            (third.clone(),)
        )
        .await?
    );

    let iterator: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&list, &list.class_definition().name(), "listIterator", "()Ljava/util/ListIterator;", ())
        .await?;
    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&iterator, &iterator.class_definition().name(), "next", "()Ljava/lang/Object;", ())
        .await?;
    let _: () = jvm
        .invoke_virtual(
            &iterator,
            &iterator.class_definition().name(),
            "set",
            "(Ljava/lang/Object;)V",
            (second.clone(),),
        )
        .await?;
    assert_eq!(
        jvm.load_array::<ClassInstanceRef<Object>>(&array, 0, 1).await?.remove(0).identity(),
        second.identity()
    );

    let result: Result<()> = jvm
        .invoke_virtual(
            &iterator,
            &iterator.class_definition().name(),
            "add",
            "(Ljava/lang/Object;)V",
            (third.clone(),),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("list iterator add must fail");
    };
    assert!(jvm.is_instance(exception.as_ref(), "java/lang/UnsupportedOperationException"));
    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&iterator, &iterator.class_definition().name(), "next", "()Ljava/lang/Object;", ())
        .await?;
    let result: Result<()> = jvm
        .invoke_virtual(&iterator, &iterator.class_definition().name(), "remove", "()V", ())
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("list iterator remove must fail");
    };
    assert!(jvm.is_instance(exception.as_ref(), "java/lang/UnsupportedOperationException"));

    let result: Result<bool> = jvm
        .invoke_virtual(&list, &list.class_definition().name(), "add", "(Ljava/lang/Object;)Z", (third.clone(),))
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("fixed list add must fail");
    };
    assert!(jvm.is_instance(exception.as_ref(), "java/lang/UnsupportedOperationException"));
    let result: Result<bool> = jvm
        .invoke_virtual(&list, &list.class_definition().name(), "remove", "(Ljava/lang/Object;)Z", (first,))
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("fixed list remove must fail");
    };
    assert!(jvm.is_instance(exception.as_ref(), "java/lang/UnsupportedOperationException"));
    let result: Result<()> = jvm.invoke_virtual(&list, &list.class_definition().name(), "clear", "()V", ()).await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("fixed list clear must fail");
    };
    assert!(jvm.is_instance(exception.as_ref(), "java/lang/UnsupportedOperationException"));

    let typed: ClassInstanceRef<Array<Object>> = jvm.instantiate_array("Ljava/lang/String;", 0).await?.into();
    let copied: ClassInstanceRef<Array<Object>> = jvm
        .invoke_virtual(
            &list,
            &list.class_definition().name(),
            "toArray",
            "([Ljava/lang/Object;)[Ljava/lang/Object;",
            (typed,),
        )
        .await?;
    assert_eq!(copied.class_definition().name(), "[Ljava/lang/String;");
    assert_eq!(jvm.array_length(&copied).await?, 2);

    let null_array: ClassInstanceRef<Array<Object>> = None.into();
    let result: Result<ClassInstanceRef<Object>> = jvm
        .invoke_static("java/util/Arrays", "asList", "([Ljava/lang/Object;)Ljava/util/List;", (null_array,))
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("Arrays.asList(null) must fail");
    };
    assert!(jvm.is_instance(exception.as_ref(), "java/lang/NullPointerException"));

    Ok(())
}

#[tokio::test]
async fn test_arr_01_every_primitive_sort_whole_and_range_overload_executes() -> Result<()> {
    let jvm = test_jvm().await?;

    let mut bytes: ClassInstanceRef<Array<i8>> = jvm.instantiate_array("B", 4).await?.into();
    jvm.store_array(&mut bytes, 0, [4i8, 3, 2, 1]).await?;
    jvm.invoke_static::<_, ()>("java/util/Arrays", "sort", "([B)V", (bytes.clone(),)).await?;
    assert_eq!(jvm.load_array::<i8>(&bytes, 0, 4).await?, [1, 2, 3, 4]);
    jvm.store_array(&mut bytes, 0, [9i8, 4, 2, 8]).await?;
    jvm.invoke_static::<_, ()>("java/util/Arrays", "sort", "([BII)V", (bytes.clone(), 1, 3))
        .await?;
    assert_eq!(jvm.load_array::<i8>(&bytes, 0, 4).await?, [9, 2, 4, 8]);

    let mut chars: ClassInstanceRef<Array<JavaChar>> = jvm.instantiate_array("C", 4).await?.into();
    jvm.store_array(&mut chars, 0, [4 as JavaChar, 3 as JavaChar, 2 as JavaChar, 1 as JavaChar])
        .await?;
    jvm.invoke_static::<_, ()>("java/util/Arrays", "sort", "([C)V", (chars.clone(),)).await?;
    assert_eq!(jvm.load_array::<JavaChar>(&chars, 0, 4).await?, [1, 2, 3, 4]);
    jvm.store_array(&mut chars, 0, [9 as JavaChar, 4 as JavaChar, 2 as JavaChar, 8 as JavaChar])
        .await?;
    jvm.invoke_static::<_, ()>("java/util/Arrays", "sort", "([CII)V", (chars.clone(), 1, 3))
        .await?;
    assert_eq!(jvm.load_array::<JavaChar>(&chars, 0, 4).await?, [9, 2, 4, 8]);

    let mut shorts: ClassInstanceRef<Array<i16>> = jvm.instantiate_array("S", 4).await?.into();
    jvm.store_array(&mut shorts, 0, [4i16, 3, 2, 1]).await?;
    jvm.invoke_static::<_, ()>("java/util/Arrays", "sort", "([S)V", (shorts.clone(),)).await?;
    assert_eq!(jvm.load_array::<i16>(&shorts, 0, 4).await?, [1, 2, 3, 4]);
    jvm.store_array(&mut shorts, 0, [9i16, 4, 2, 8]).await?;
    jvm.invoke_static::<_, ()>("java/util/Arrays", "sort", "([SII)V", (shorts.clone(), 1, 3))
        .await?;
    assert_eq!(jvm.load_array::<i16>(&shorts, 0, 4).await?, [9, 2, 4, 8]);

    let mut ints: ClassInstanceRef<Array<i32>> = jvm.instantiate_array("I", 4).await?.into();
    jvm.store_array(&mut ints, 0, [4, 3, 2, 1]).await?;
    jvm.invoke_static::<_, ()>("java/util/Arrays", "sort", "([I)V", (ints.clone(),)).await?;
    assert_eq!(jvm.load_array::<i32>(&ints, 0, 4).await?, [1, 2, 3, 4]);
    jvm.store_array(&mut ints, 0, [9, 4, 2, 8]).await?;
    jvm.invoke_static::<_, ()>("java/util/Arrays", "sort", "([III)V", (ints.clone(), 1, 3))
        .await?;
    assert_eq!(jvm.load_array::<i32>(&ints, 0, 4).await?, [9, 2, 4, 8]);

    let mut longs: ClassInstanceRef<Array<i64>> = jvm.instantiate_array("J", 4).await?.into();
    jvm.store_array(&mut longs, 0, [4i64, 3, 2, 1]).await?;
    jvm.invoke_static::<_, ()>("java/util/Arrays", "sort", "([J)V", (longs.clone(),)).await?;
    assert_eq!(jvm.load_array::<i64>(&longs, 0, 4).await?, [1, 2, 3, 4]);
    jvm.store_array(&mut longs, 0, [9i64, 4, 2, 8]).await?;
    jvm.invoke_static::<_, ()>("java/util/Arrays", "sort", "([JII)V", (longs.clone(), 1, 3))
        .await?;
    assert_eq!(jvm.load_array::<i64>(&longs, 0, 4).await?, [9, 2, 4, 8]);

    let mut floats: ClassInstanceRef<Array<f32>> = jvm.instantiate_array("F", 4).await?.into();
    jvm.store_array(&mut floats, 0, [4.0f32, 3.0, 2.0, 1.0]).await?;
    jvm.invoke_static::<_, ()>("java/util/Arrays", "sort", "([F)V", (floats.clone(),)).await?;
    assert_eq!(jvm.load_array::<f32>(&floats, 0, 4).await?, [1.0, 2.0, 3.0, 4.0]);
    jvm.store_array(&mut floats, 0, [9.0f32, 4.0, 2.0, 8.0]).await?;
    jvm.invoke_static::<_, ()>("java/util/Arrays", "sort", "([FII)V", (floats.clone(), 1, 3))
        .await?;
    assert_eq!(jvm.load_array::<f32>(&floats, 0, 4).await?, [9.0, 2.0, 4.0, 8.0]);

    let mut doubles: ClassInstanceRef<Array<f64>> = jvm.instantiate_array("D", 4).await?.into();
    jvm.store_array(&mut doubles, 0, [4.0f64, 3.0, 2.0, 1.0]).await?;
    jvm.invoke_static::<_, ()>("java/util/Arrays", "sort", "([D)V", (doubles.clone(),))
        .await?;
    assert_eq!(jvm.load_array::<f64>(&doubles, 0, 4).await?, [1.0, 2.0, 3.0, 4.0]);
    jvm.store_array(&mut doubles, 0, [9.0f64, 4.0, 2.0, 8.0]).await?;
    jvm.invoke_static::<_, ()>("java/util/Arrays", "sort", "([DII)V", (doubles.clone(), 1, 3))
        .await?;
    assert_eq!(jvm.load_array::<f64>(&doubles, 0, 4).await?, [9.0, 2.0, 4.0, 8.0]);

    Ok(())
}

#[tokio::test]
async fn test_arr_03_04_binary_search_duplicates_null_comparator_and_exception_priority() -> Result<()> {
    let jvm = arrays_object_test_jvm().await?;

    let mut duplicates: ClassInstanceRef<Array<i32>> = jvm.instantiate_array("I", 5).await?.into();
    jvm.store_array(&mut duplicates, 0, [1, 2, 2, 2, 3]).await?;
    let hit = jvm
        .invoke_static::<_, i32>("java/util/Arrays", "binarySearch", "([II)I", (duplicates.clone(), 2))
        .await?;
    assert!((1..=3).contains(&hit));
    assert_eq!(jvm.load_array::<i32>(&duplicates, hit as usize, 1).await?, [2]);
    for (key, expected) in [(0, -1), (4, -6)] {
        assert_eq!(
            jvm.invoke_static::<_, i32>("java/util/Arrays", "binarySearch", "([II)I", (duplicates.clone(), key))
                .await?,
            expected
        );
    }

    let first = jvm.new_class("ArraysSortValue", "(IIZ)V", (1, 1, false)).await?;
    let duplicate_a = jvm.new_class("ArraysSortValue", "(IIZ)V", (2, 2, false)).await?;
    let duplicate_b = jvm.new_class("ArraysSortValue", "(IIZ)V", (2, 3, false)).await?;
    let last = jvm.new_class("ArraysSortValue", "(IIZ)V", (3, 4, false)).await?;
    let key = jvm.new_class("ArraysSortValue", "(IIZ)V", (2, 5, false)).await?;
    let mut objects: ClassInstanceRef<Array<Object>> = jvm.instantiate_array("Ljava/lang/Object;", 4).await?.into();
    jvm.store_array(&mut objects, 0, [first, duplicate_a, duplicate_b, last]).await?;
    let natural_hit = jvm
        .invoke_static::<_, i32>(
            "java/util/Arrays",
            "binarySearch",
            "([Ljava/lang/Object;Ljava/lang/Object;)I",
            (objects.clone(), key.clone()),
        )
        .await?;
    assert!((1..=2).contains(&natural_hit));

    let null_comparator: ClassInstanceRef<Object> = None.into();
    let null_comparator_hit = jvm
        .invoke_static::<_, i32>(
            "java/util/Arrays",
            "binarySearch",
            "([Ljava/lang/Object;Ljava/lang/Object;Ljava/util/Comparator;)I",
            (objects.clone(), key.clone(), null_comparator),
        )
        .await?;
    assert!((1..=2).contains(&null_comparator_hit));

    let comparator = jvm.new_class("ArraysComparator", "(ZZ)V", (false, false)).await?;
    let comparator_hit = jvm
        .invoke_static::<_, i32>(
            "java/util/Arrays",
            "binarySearch",
            "([Ljava/lang/Object;Ljava/lang/Object;Ljava/util/Comparator;)I",
            (objects.clone(), key.clone(), comparator),
        )
        .await?;
    assert!((1..=2).contains(&comparator_hit));

    let failing_comparator = jvm.new_class("ArraysComparator", "(ZZ)V", (false, true)).await?;
    let result: Result<i32> = jvm
        .invoke_static(
            "java/util/Arrays",
            "binarySearch",
            "([Ljava/lang/Object;Ljava/lang/Object;Ljava/util/Comparator;)I",
            (objects.clone(), key.clone(), failing_comparator),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("comparator binarySearch failure must propagate");
    };
    assert!(jvm.is_instance(exception.as_ref(), "java/lang/IllegalStateException"));

    let null_objects: ClassInstanceRef<Array<Object>> = None.into();
    let failing_comparator = jvm.new_class("ArraysComparator", "(ZZ)V", (false, true)).await?;
    let result: Result<i32> = jvm
        .invoke_static(
            "java/util/Arrays",
            "binarySearch",
            "([Ljava/lang/Object;Ljava/lang/Object;Ljava/util/Comparator;)I",
            (null_objects, key.clone(), failing_comparator),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("null array must fail before comparator invocation");
    };
    assert!(jvm.is_instance(exception.as_ref(), "java/lang/NullPointerException"));

    let empty: ClassInstanceRef<Array<Object>> = jvm.instantiate_array("Ljava/lang/Object;", 0).await?.into();
    let failing_comparator = jvm.new_class("ArraysComparator", "(ZZ)V", (false, true)).await?;
    assert_eq!(
        jvm.invoke_static::<_, i32>(
            "java/util/Arrays",
            "binarySearch",
            "([Ljava/lang/Object;Ljava/lang/Object;Ljava/util/Comparator;)I",
            (empty, key, failing_comparator),
        )
        .await?,
        -1
    );

    Ok(())
}

#[tokio::test]
async fn test_arr_05_every_equals_overload_values_lengths_and_nulls() -> Result<()> {
    let jvm = test_jvm().await?;

    let mut booleans_a: ClassInstanceRef<Array<bool>> = jvm.instantiate_array("Z", 2).await?.into();
    let mut booleans_b: ClassInstanceRef<Array<bool>> = jvm.instantiate_array("Z", 2).await?.into();
    jvm.store_array(&mut booleans_a, 0, [true, false]).await?;
    jvm.store_array(&mut booleans_b, 0, [true, false]).await?;
    assert!(
        jvm.invoke_static::<_, bool>("java/util/Arrays", "equals", "([Z[Z)Z", (booleans_a.clone(), booleans_b.clone()))
            .await?
    );
    jvm.store_array(&mut booleans_b, 1, [true]).await?;
    assert!(
        !jvm.invoke_static::<_, bool>("java/util/Arrays", "equals", "([Z[Z)Z", (booleans_a, booleans_b))
            .await?
    );

    let mut bytes_a: ClassInstanceRef<Array<i8>> = jvm.instantiate_array("B", 2).await?.into();
    let mut bytes_b: ClassInstanceRef<Array<i8>> = jvm.instantiate_array("B", 2).await?.into();
    jvm.store_array(&mut bytes_a, 0, [i8::MIN, i8::MAX]).await?;
    jvm.store_array(&mut bytes_b, 0, [i8::MIN, i8::MAX]).await?;
    assert!(
        jvm.invoke_static::<_, bool>("java/util/Arrays", "equals", "([B[B)Z", (bytes_a.clone(), bytes_b.clone()))
            .await?
    );
    jvm.store_array(&mut bytes_b, 1, [0i8]).await?;
    assert!(
        !jvm.invoke_static::<_, bool>("java/util/Arrays", "equals", "([B[B)Z", (bytes_a, bytes_b))
            .await?
    );

    let mut chars_a: ClassInstanceRef<Array<JavaChar>> = jvm.instantiate_array("C", 2).await?.into();
    let mut chars_b: ClassInstanceRef<Array<JavaChar>> = jvm.instantiate_array("C", 2).await?.into();
    jvm.store_array(&mut chars_a, 0, [0 as JavaChar, u16::MAX]).await?;
    jvm.store_array(&mut chars_b, 0, [0 as JavaChar, u16::MAX]).await?;
    assert!(
        jvm.invoke_static::<_, bool>("java/util/Arrays", "equals", "([C[C)Z", (chars_a.clone(), chars_b.clone()))
            .await?
    );
    jvm.store_array(&mut chars_b, 1, [1 as JavaChar]).await?;
    assert!(
        !jvm.invoke_static::<_, bool>("java/util/Arrays", "equals", "([C[C)Z", (chars_a, chars_b))
            .await?
    );

    let mut shorts_a: ClassInstanceRef<Array<i16>> = jvm.instantiate_array("S", 2).await?.into();
    let mut shorts_b: ClassInstanceRef<Array<i16>> = jvm.instantiate_array("S", 2).await?.into();
    jvm.store_array(&mut shorts_a, 0, [i16::MIN, i16::MAX]).await?;
    jvm.store_array(&mut shorts_b, 0, [i16::MIN, i16::MAX]).await?;
    assert!(
        jvm.invoke_static::<_, bool>("java/util/Arrays", "equals", "([S[S)Z", (shorts_a.clone(), shorts_b.clone()))
            .await?
    );
    jvm.store_array(&mut shorts_b, 1, [0i16]).await?;
    assert!(
        !jvm.invoke_static::<_, bool>("java/util/Arrays", "equals", "([S[S)Z", (shorts_a, shorts_b))
            .await?
    );

    let mut ints_a: ClassInstanceRef<Array<i32>> = jvm.instantiate_array("I", 2).await?.into();
    let mut ints_b: ClassInstanceRef<Array<i32>> = jvm.instantiate_array("I", 2).await?.into();
    jvm.store_array(&mut ints_a, 0, [i32::MIN, i32::MAX]).await?;
    jvm.store_array(&mut ints_b, 0, [i32::MIN, i32::MAX]).await?;
    assert!(
        jvm.invoke_static::<_, bool>("java/util/Arrays", "equals", "([I[I)Z", (ints_a.clone(), ints_b.clone()))
            .await?
    );
    let shorter: ClassInstanceRef<Array<i32>> = jvm.instantiate_array("I", 1).await?.into();
    assert!(
        !jvm.invoke_static::<_, bool>("java/util/Arrays", "equals", "([I[I)Z", (ints_a, shorter))
            .await?
    );

    let mut longs_a: ClassInstanceRef<Array<i64>> = jvm.instantiate_array("J", 2).await?.into();
    let mut longs_b: ClassInstanceRef<Array<i64>> = jvm.instantiate_array("J", 2).await?.into();
    jvm.store_array(&mut longs_a, 0, [i64::MIN, i64::MAX]).await?;
    jvm.store_array(&mut longs_b, 0, [i64::MIN, i64::MAX]).await?;
    assert!(
        jvm.invoke_static::<_, bool>("java/util/Arrays", "equals", "([J[J)Z", (longs_a.clone(), longs_b.clone()))
            .await?
    );
    jvm.store_array(&mut longs_b, 1, [0i64]).await?;
    assert!(
        !jvm.invoke_static::<_, bool>("java/util/Arrays", "equals", "([J[J)Z", (longs_a, longs_b))
            .await?
    );

    let mut floats_a: ClassInstanceRef<Array<f32>> = jvm.instantiate_array("F", 2).await?.into();
    let mut floats_b: ClassInstanceRef<Array<f32>> = jvm.instantiate_array("F", 2).await?.into();
    jvm.store_array(&mut floats_a, 0, [f32::from_bits(0x7fc0_0001), -0.0]).await?;
    jvm.store_array(&mut floats_b, 0, [f32::from_bits(0xffc0_1234), -0.0]).await?;
    assert!(
        jvm.invoke_static::<_, bool>("java/util/Arrays", "equals", "([F[F)Z", (floats_a.clone(), floats_b.clone()))
            .await?
    );
    jvm.store_array(&mut floats_b, 1, [0.0f32]).await?;
    assert!(
        !jvm.invoke_static::<_, bool>("java/util/Arrays", "equals", "([F[F)Z", (floats_a, floats_b))
            .await?
    );

    let mut doubles_a: ClassInstanceRef<Array<f64>> = jvm.instantiate_array("D", 2).await?.into();
    let mut doubles_b: ClassInstanceRef<Array<f64>> = jvm.instantiate_array("D", 2).await?.into();
    jvm.store_array(&mut doubles_a, 0, [f64::from_bits(0x7ff8_0000_0000_0001), -0.0]).await?;
    jvm.store_array(&mut doubles_b, 0, [f64::from_bits(0xfff8_0000_0000_1234), -0.0]).await?;
    assert!(
        jvm.invoke_static::<_, bool>("java/util/Arrays", "equals", "([D[D)Z", (doubles_a.clone(), doubles_b.clone()))
            .await?
    );
    jvm.store_array(&mut doubles_b, 1, [0.0f64]).await?;
    assert!(
        !jvm.invoke_static::<_, bool>("java/util/Arrays", "equals", "([D[D)Z", (doubles_a, doubles_b))
            .await?
    );

    let first: ClassInstanceRef<Object> = JavaLangString::from_rust_string(&jvm, "first").await?.into();
    let equal: ClassInstanceRef<Object> = JavaLangString::from_rust_string(&jvm, "first").await?.into();
    let different: ClassInstanceRef<Object> = JavaLangString::from_rust_string(&jvm, "different").await?.into();
    let null: ClassInstanceRef<Object> = None.into();
    let mut objects_a: ClassInstanceRef<Array<Object>> = jvm.instantiate_array("Ljava/lang/Object;", 2).await?.into();
    let mut objects_b: ClassInstanceRef<Array<Object>> = jvm.instantiate_array("Ljava/lang/Object;", 2).await?.into();
    jvm.store_array(&mut objects_a, 0, [first, null.clone()]).await?;
    jvm.store_array(&mut objects_b, 0, [equal, null.clone()]).await?;
    assert!(
        jvm.invoke_static::<_, bool>(
            "java/util/Arrays",
            "equals",
            "([Ljava/lang/Object;[Ljava/lang/Object;)Z",
            (objects_a.clone(), objects_b.clone()),
        )
        .await?
    );
    jvm.store_array(&mut objects_b, 0, [different, null]).await?;
    assert!(
        !jvm.invoke_static::<_, bool>(
            "java/util/Arrays",
            "equals",
            "([Ljava/lang/Object;[Ljava/lang/Object;)Z",
            (objects_a, objects_b),
        )
        .await?
    );

    let null_booleans: ClassInstanceRef<Array<bool>> = None.into();
    let empty_booleans: ClassInstanceRef<Array<bool>> = jvm.instantiate_array("Z", 0).await?.into();
    assert!(
        jvm.invoke_static::<_, bool>("java/util/Arrays", "equals", "([Z[Z)Z", (null_booleans.clone(), null_booleans),)
            .await?
    );
    assert!(
        !jvm.invoke_static::<_, bool>(
            "java/util/Arrays",
            "equals",
            "([Z[Z)Z",
            (ClassInstanceRef::<Array<bool>>::from(None), empty_booleans),
        )
        .await?
    );

    let null_bytes: ClassInstanceRef<Array<i8>> = None.into();
    let empty_bytes: ClassInstanceRef<Array<i8>> = jvm.instantiate_array("B", 0).await?.into();
    assert!(
        jvm.invoke_static::<_, bool>("java/util/Arrays", "equals", "([B[B)Z", (null_bytes.clone(), null_bytes.clone()),)
            .await?
    );
    assert!(
        !jvm.invoke_static::<_, bool>("java/util/Arrays", "equals", "([B[B)Z", (null_bytes, empty_bytes))
            .await?
    );

    let null_chars: ClassInstanceRef<Array<JavaChar>> = None.into();
    let empty_chars: ClassInstanceRef<Array<JavaChar>> = jvm.instantiate_array("C", 0).await?.into();
    assert!(
        jvm.invoke_static::<_, bool>("java/util/Arrays", "equals", "([C[C)Z", (null_chars.clone(), null_chars.clone()),)
            .await?
    );
    assert!(
        !jvm.invoke_static::<_, bool>("java/util/Arrays", "equals", "([C[C)Z", (null_chars, empty_chars))
            .await?
    );

    let null_shorts: ClassInstanceRef<Array<i16>> = None.into();
    let empty_shorts: ClassInstanceRef<Array<i16>> = jvm.instantiate_array("S", 0).await?.into();
    assert!(
        jvm.invoke_static::<_, bool>("java/util/Arrays", "equals", "([S[S)Z", (null_shorts.clone(), null_shorts.clone()),)
            .await?
    );
    assert!(
        !jvm.invoke_static::<_, bool>("java/util/Arrays", "equals", "([S[S)Z", (null_shorts, empty_shorts),)
            .await?
    );

    let null_ints: ClassInstanceRef<Array<i32>> = None.into();
    let empty_ints: ClassInstanceRef<Array<i32>> = jvm.instantiate_array("I", 0).await?.into();
    assert!(
        jvm.invoke_static::<_, bool>("java/util/Arrays", "equals", "([I[I)Z", (null_ints.clone(), null_ints.clone()),)
            .await?
    );
    assert!(
        !jvm.invoke_static::<_, bool>("java/util/Arrays", "equals", "([I[I)Z", (null_ints, empty_ints))
            .await?
    );

    let null_longs: ClassInstanceRef<Array<i64>> = None.into();
    let empty_longs: ClassInstanceRef<Array<i64>> = jvm.instantiate_array("J", 0).await?.into();
    assert!(
        jvm.invoke_static::<_, bool>("java/util/Arrays", "equals", "([J[J)Z", (null_longs.clone(), null_longs.clone()),)
            .await?
    );
    assert!(
        !jvm.invoke_static::<_, bool>("java/util/Arrays", "equals", "([J[J)Z", (null_longs, empty_longs))
            .await?
    );

    let null_floats: ClassInstanceRef<Array<f32>> = None.into();
    let empty_floats: ClassInstanceRef<Array<f32>> = jvm.instantiate_array("F", 0).await?.into();
    assert!(
        jvm.invoke_static::<_, bool>("java/util/Arrays", "equals", "([F[F)Z", (null_floats.clone(), null_floats.clone()),)
            .await?
    );
    assert!(
        !jvm.invoke_static::<_, bool>("java/util/Arrays", "equals", "([F[F)Z", (null_floats, empty_floats),)
            .await?
    );

    let null_doubles: ClassInstanceRef<Array<f64>> = None.into();
    let empty_doubles: ClassInstanceRef<Array<f64>> = jvm.instantiate_array("D", 0).await?.into();
    assert!(
        jvm.invoke_static::<_, bool>("java/util/Arrays", "equals", "([D[D)Z", (null_doubles.clone(), null_doubles.clone()),)
            .await?
    );
    assert!(
        !jvm.invoke_static::<_, bool>("java/util/Arrays", "equals", "([D[D)Z", (null_doubles, empty_doubles),)
            .await?
    );

    let null_objects: ClassInstanceRef<Array<Object>> = None.into();
    let empty_objects: ClassInstanceRef<Array<Object>> = jvm.instantiate_array("Ljava/lang/Object;", 0).await?.into();
    assert!(
        jvm.invoke_static::<_, bool>(
            "java/util/Arrays",
            "equals",
            "([Ljava/lang/Object;[Ljava/lang/Object;)Z",
            (null_objects.clone(), null_objects),
        )
        .await?
    );
    assert!(
        !jvm.invoke_static::<_, bool>(
            "java/util/Arrays",
            "equals",
            "([Ljava/lang/Object;[Ljava/lang/Object;)Z",
            (ClassInstanceRef::<Array<Object>>::from(None), empty_objects),
        )
        .await?
    );

    Ok(())
}

#[tokio::test]
async fn test_arr_06_07_every_fill_overload_ranges_and_exception_priority() -> Result<()> {
    let jvm = test_jvm().await?;

    let booleans: ClassInstanceRef<Array<bool>> = jvm.instantiate_array("Z", 3).await?.into();
    jvm.invoke_static::<_, ()>("java/util/Arrays", "fill", "([ZZ)V", (booleans.clone(), true))
        .await?;
    assert_eq!(jvm.load_array::<bool>(&booleans, 0, 3).await?, [true, true, true]);
    jvm.invoke_static::<_, ()>("java/util/Arrays", "fill", "([ZIIZ)V", (booleans.clone(), 1, 2, false))
        .await?;
    assert_eq!(jvm.load_array::<bool>(&booleans, 0, 3).await?, [true, false, true]);

    let bytes: ClassInstanceRef<Array<i8>> = jvm.instantiate_array("B", 3).await?.into();
    jvm.invoke_static::<_, ()>("java/util/Arrays", "fill", "([BB)V", (bytes.clone(), 7i8))
        .await?;
    assert_eq!(jvm.load_array::<i8>(&bytes, 0, 3).await?, [7, 7, 7]);
    jvm.invoke_static::<_, ()>("java/util/Arrays", "fill", "([BIIB)V", (bytes.clone(), 1, 2, -1i8))
        .await?;
    assert_eq!(jvm.load_array::<i8>(&bytes, 0, 3).await?, [7, -1, 7]);

    let chars: ClassInstanceRef<Array<JavaChar>> = jvm.instantiate_array("C", 3).await?.into();
    jvm.invoke_static::<_, ()>("java/util/Arrays", "fill", "([CC)V", (chars.clone(), 7 as JavaChar))
        .await?;
    assert_eq!(jvm.load_array::<JavaChar>(&chars, 0, 3).await?, [7, 7, 7]);
    jvm.invoke_static::<_, ()>("java/util/Arrays", "fill", "([CIIC)V", (chars.clone(), 1, 2, 1 as JavaChar))
        .await?;
    assert_eq!(jvm.load_array::<JavaChar>(&chars, 0, 3).await?, [7, 1, 7]);

    let shorts: ClassInstanceRef<Array<i16>> = jvm.instantiate_array("S", 3).await?.into();
    jvm.invoke_static::<_, ()>("java/util/Arrays", "fill", "([SS)V", (shorts.clone(), 7i16))
        .await?;
    assert_eq!(jvm.load_array::<i16>(&shorts, 0, 3).await?, [7, 7, 7]);
    jvm.invoke_static::<_, ()>("java/util/Arrays", "fill", "([SIIS)V", (shorts.clone(), 1, 2, -1i16))
        .await?;
    assert_eq!(jvm.load_array::<i16>(&shorts, 0, 3).await?, [7, -1, 7]);

    let ints: ClassInstanceRef<Array<i32>> = jvm.instantiate_array("I", 3).await?.into();
    jvm.invoke_static::<_, ()>("java/util/Arrays", "fill", "([II)V", (ints.clone(), 7))
        .await?;
    assert_eq!(jvm.load_array::<i32>(&ints, 0, 3).await?, [7, 7, 7]);
    jvm.invoke_static::<_, ()>("java/util/Arrays", "fill", "([IIII)V", (ints.clone(), 1, 2, -1))
        .await?;
    assert_eq!(jvm.load_array::<i32>(&ints, 0, 3).await?, [7, -1, 7]);

    let longs: ClassInstanceRef<Array<i64>> = jvm.instantiate_array("J", 3).await?.into();
    jvm.invoke_static::<_, ()>("java/util/Arrays", "fill", "([JJ)V", (longs.clone(), 7i64))
        .await?;
    assert_eq!(jvm.load_array::<i64>(&longs, 0, 3).await?, [7, 7, 7]);
    jvm.invoke_static::<_, ()>("java/util/Arrays", "fill", "([JIIJ)V", (longs.clone(), 1, 2, -1i64))
        .await?;
    assert_eq!(jvm.load_array::<i64>(&longs, 0, 3).await?, [7, -1, 7]);

    let floats: ClassInstanceRef<Array<f32>> = jvm.instantiate_array("F", 3).await?.into();
    jvm.invoke_static::<_, ()>("java/util/Arrays", "fill", "([FF)V", (floats.clone(), 7.0f32))
        .await?;
    assert_eq!(jvm.load_array::<f32>(&floats, 0, 3).await?, [7.0, 7.0, 7.0]);
    jvm.invoke_static::<_, ()>("java/util/Arrays", "fill", "([FIIF)V", (floats.clone(), 1, 2, -0.0f32))
        .await?;
    let float_values = jvm.load_array::<f32>(&floats, 0, 3).await?;
    assert_eq!(float_values[0], 7.0);
    assert_eq!(float_values[1].to_bits(), (-0.0f32).to_bits());
    assert_eq!(float_values[2], 7.0);

    let doubles: ClassInstanceRef<Array<f64>> = jvm.instantiate_array("D", 3).await?.into();
    jvm.invoke_static::<_, ()>("java/util/Arrays", "fill", "([DD)V", (doubles.clone(), 7.0f64))
        .await?;
    assert_eq!(jvm.load_array::<f64>(&doubles, 0, 3).await?, [7.0, 7.0, 7.0]);
    jvm.invoke_static::<_, ()>("java/util/Arrays", "fill", "([DIID)V", (doubles.clone(), 1, 2, -0.0f64))
        .await?;
    let double_values = jvm.load_array::<f64>(&doubles, 0, 3).await?;
    assert_eq!(double_values[0], 7.0);
    assert_eq!(double_values[1].to_bits(), (-0.0f64).to_bits());
    assert_eq!(double_values[2], 7.0);

    let first = JavaLangString::from_rust_string(&jvm, "first").await?;
    let second = JavaLangString::from_rust_string(&jvm, "second").await?;
    let objects: ClassInstanceRef<Array<Object>> = jvm.instantiate_array("Ljava/lang/Object;", 3).await?.into();
    jvm.invoke_static::<_, ()>(
        "java/util/Arrays",
        "fill",
        "([Ljava/lang/Object;Ljava/lang/Object;)V",
        (objects.clone(), first.clone()),
    )
    .await?;
    let values = jvm.load_array::<ClassInstanceRef<Object>>(&objects, 0, 3).await?;
    assert!(values.iter().all(|value| value.identity() == first.identity()));
    jvm.invoke_static::<_, ()>(
        "java/util/Arrays",
        "fill",
        "([Ljava/lang/Object;IILjava/lang/Object;)V",
        (objects.clone(), 1, 2, second.clone()),
    )
    .await?;
    let values = jvm.load_array::<ClassInstanceRef<Object>>(&objects, 0, 3).await?;
    assert_eq!(values[0].identity(), first.identity());
    assert_eq!(values[1].identity(), second.identity());
    assert_eq!(values[2].identity(), first.identity());

    jvm.invoke_static::<_, ()>("java/util/Arrays", "fill", "([IIII)V", (ints.clone(), 0, 0, 99))
        .await?;
    jvm.invoke_static::<_, ()>("java/util/Arrays", "fill", "([IIII)V", (ints.clone(), 3, 3, 99))
        .await?;
    assert_eq!(jvm.load_array::<i32>(&ints, 0, 3).await?, [7, -1, 7]);

    let result: Result<()> = jvm.invoke_static("java/util/Arrays", "fill", "([IIII)V", (ints.clone(), 5, 4, 99)).await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("fromIndex > toIndex must win over bounds");
    };
    assert!(jvm.is_instance(exception.as_ref(), "java/lang/IllegalArgumentException"));
    for (from, to) in [(-1, 0), (0, 4)] {
        let result: Result<()> = jvm
            .invoke_static("java/util/Arrays", "fill", "([IIII)V", (ints.clone(), from, to, 99))
            .await;
        let Err(JavaError::JavaException(exception)) = result else {
            panic!("fill range outside the array must fail");
        };
        assert!(jvm.is_instance(exception.as_ref(), "java/lang/ArrayIndexOutOfBoundsException"));
    }
    let null_ints: ClassInstanceRef<Array<i32>> = None.into();
    let result: Result<()> = jvm.invoke_static("java/util/Arrays", "fill", "([IIII)V", (null_ints, 5, 4, 99)).await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("null array must fail before range validation");
    };
    assert!(jvm.is_instance(exception.as_ref(), "java/lang/NullPointerException"));

    Ok(())
}

#[tokio::test]
async fn test_arr_07_multidimensional_fill_store_compatibility() -> Result<()> {
    let jvm = test_jvm().await?;
    let first_row: ClassInstanceRef<Object> = jvm.instantiate_array("Ljava/lang/String;", 1).await?.into();
    let second_row: ClassInstanceRef<Object> = jvm.instantiate_array("Ljava/lang/String;", 2).await?.into();
    let incompatible_row: ClassInstanceRef<Object> = jvm.instantiate_array("Ljava/lang/Object;", 1).await?.into();
    let matrix: ClassInstanceRef<Array<Object>> = jvm.instantiate_array("[Ljava/lang/String;", 3).await?.into();

    jvm.invoke_static::<_, ()>(
        "java/util/Arrays",
        "fill",
        "([Ljava/lang/Object;Ljava/lang/Object;)V",
        (matrix.clone(), first_row.clone()),
    )
    .await?;
    let values = jvm.load_array::<ClassInstanceRef<Object>>(&matrix, 0, 3).await?;
    assert!(values.iter().all(|value| value.identity() == first_row.identity()));

    jvm.invoke_static::<_, ()>(
        "java/util/Arrays",
        "fill",
        "([Ljava/lang/Object;IILjava/lang/Object;)V",
        (matrix.clone(), 1, 3, second_row.clone()),
    )
    .await?;
    let before_failure = jvm.load_array::<ClassInstanceRef<Object>>(&matrix, 0, 3).await?;
    assert_eq!(before_failure[0].identity(), first_row.identity());
    assert_eq!(before_failure[1].identity(), second_row.identity());
    assert_eq!(before_failure[2].identity(), second_row.identity());

    let result: Result<()> = jvm
        .invoke_static(
            "java/util/Arrays",
            "fill",
            "([Ljava/lang/Object;IILjava/lang/Object;)V",
            (matrix.clone(), 1, 3, incompatible_row),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("String[][] must reject an Object[] fill value");
    };
    assert!(jvm.is_instance(exception.as_ref(), "java/lang/ArrayStoreException"));
    let after_failure = jvm.load_array::<ClassInstanceRef<Object>>(&matrix, 0, 3).await?;
    assert_eq!(
        after_failure.iter().map(|value| value.identity()).collect::<Vec<_>>(),
        before_failure.iter().map(|value| value.identity()).collect::<Vec<_>>()
    );

    let incompatible_row: ClassInstanceRef<Object> = jvm.instantiate_array("Ljava/lang/Object;", 1).await?.into();
    jvm.invoke_static::<_, ()>(
        "java/util/Arrays",
        "fill",
        "([Ljava/lang/Object;IILjava/lang/Object;)V",
        (matrix.clone(), 1, 1, incompatible_row.clone()),
    )
    .await?;
    let result: Result<()> = jvm
        .invoke_static(
            "java/util/Arrays",
            "fill",
            "([Ljava/lang/Object;IILjava/lang/Object;)V",
            (matrix.clone(), 5, 4, incompatible_row.clone()),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("invalid range must fail before store compatibility");
    };
    assert!(jvm.is_instance(exception.as_ref(), "java/lang/IllegalArgumentException"));
    let result: Result<()> = jvm
        .invoke_static(
            "java/util/Arrays",
            "fill",
            "([Ljava/lang/Object;IILjava/lang/Object;)V",
            (matrix.clone(), 0, 4, incompatible_row),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("array bounds must fail before store compatibility");
    };
    assert!(jvm.is_instance(exception.as_ref(), "java/lang/ArrayIndexOutOfBoundsException"));

    let null: ClassInstanceRef<Object> = None.into();
    jvm.invoke_static::<_, ()>(
        "java/util/Arrays",
        "fill",
        "([Ljava/lang/Object;IILjava/lang/Object;)V",
        (matrix.clone(), 2, 3, null),
    )
    .await?;
    assert!(jvm.load_array::<ClassInstanceRef<Object>>(&matrix, 2, 1).await?.remove(0).is_null());

    Ok(())
}

#[tokio::test]
async fn test_arr_08_array_list_bulk_mutation_noop_and_unsupported_paths() -> Result<()> {
    let jvm = test_jvm().await?;
    let first = JavaLangString::from_rust_string(&jvm, "first").await?;
    let second = JavaLangString::from_rust_string(&jvm, "second").await?;
    let absent = JavaLangString::from_rust_string(&jvm, "absent").await?;
    let mut array: ClassInstanceRef<Array<Object>> = jvm.instantiate_array("Ljava/lang/Object;", 2).await?.into();
    jvm.store_array(&mut array, 0, [first.clone(), second.clone()]).await?;
    let list: ClassInstanceRef<Object> = jvm
        .invoke_static("java/util/Arrays", "asList", "([Ljava/lang/Object;)Ljava/util/List;", (array.clone(),))
        .await?;

    let empty = jvm.new_class("java/util/ArrayList", "()V", ()).await?;
    assert!(
        !jvm.invoke_virtual::<_, bool>(
            &list,
            &list.class_definition().name(),
            "addAll",
            "(Ljava/util/Collection;)Z",
            (empty.clone(),)
        )
        .await?
    );
    assert!(
        !jvm.invoke_virtual::<_, bool>(
            &list,
            &list.class_definition().name(),
            "addAll",
            "(ILjava/util/Collection;)Z",
            (1, empty.clone())
        )
        .await?
    );
    assert!(
        !jvm.invoke_virtual::<_, bool>(
            &list,
            &list.class_definition().name(),
            "removeAll",
            "(Ljava/util/Collection;)Z",
            (empty.clone(),)
        )
        .await?
    );
    assert!(
        !jvm.invoke_virtual::<_, bool>(
            &list,
            &list.class_definition().name(),
            "retainAll",
            "(Ljava/util/Collection;)Z",
            (list.clone(),)
        )
        .await?
    );

    let absent_collection = jvm.new_class("java/util/ArrayList", "()V", ()).await?;
    let _: bool = jvm
        .invoke_virtual(
            &absent_collection,
            &absent_collection.class_definition().name(),
            "add",
            "(Ljava/lang/Object;)Z",
            (absent,),
        )
        .await?;
    assert!(
        !jvm.invoke_virtual::<_, bool>(
            &list,
            &list.class_definition().name(),
            "removeAll",
            "(Ljava/util/Collection;)Z",
            (absent_collection,),
        )
        .await?
    );

    let all = jvm.new_class("java/util/ArrayList", "()V", ()).await?;
    let _: bool = jvm
        .invoke_virtual(&all, &all.class_definition().name(), "add", "(Ljava/lang/Object;)Z", (first.clone(),))
        .await?;
    let _: bool = jvm
        .invoke_virtual(&all, &all.class_definition().name(), "add", "(Ljava/lang/Object;)Z", (second.clone(),))
        .await?;
    assert!(
        !jvm.invoke_virtual::<_, bool>(&list, &list.class_definition().name(), "retainAll", "(Ljava/util/Collection;)Z", (all,))
            .await?
    );

    let non_empty = jvm.new_class("java/util/ArrayList", "()V", ()).await?;
    let _: bool = jvm
        .invoke_virtual(
            &non_empty,
            &non_empty.class_definition().name(),
            "add",
            "(Ljava/lang/Object;)Z",
            (first.clone(),),
        )
        .await?;
    for result in [
        jvm.invoke_virtual::<_, bool>(
            &list,
            &list.class_definition().name(),
            "addAll",
            "(Ljava/util/Collection;)Z",
            (non_empty.clone(),),
        )
        .await,
        jvm.invoke_virtual::<_, bool>(
            &list,
            &list.class_definition().name(),
            "addAll",
            "(ILjava/util/Collection;)Z",
            (1, non_empty.clone()),
        )
        .await,
        jvm.invoke_virtual::<_, bool>(
            &list,
            &list.class_definition().name(),
            "removeAll",
            "(Ljava/util/Collection;)Z",
            (non_empty.clone(),),
        )
        .await,
        jvm.invoke_virtual::<_, bool>(&list, &list.class_definition().name(), "retainAll", "(Ljava/util/Collection;)Z", (empty,))
            .await,
    ] {
        let Err(JavaError::JavaException(exception)) = result else {
            panic!("bulk operation requiring a size change must fail");
        };
        assert!(jvm.is_instance(exception.as_ref(), "java/lang/UnsupportedOperationException"));
        let values = jvm.load_array::<ClassInstanceRef<Object>>(&array, 0, 2).await?;
        assert_eq!(values[0].identity(), first.identity());
        assert_eq!(values[1].identity(), second.identity());
    }

    let null_collection: ClassInstanceRef<Object> = None.into();
    let result: Result<bool> = jvm
        .invoke_virtual(
            &list,
            &list.class_definition().name(),
            "removeAll",
            "(Ljava/util/Collection;)Z",
            (null_collection,),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("bulk operation must reject a null collection");
    };
    assert!(jvm.is_instance(exception.as_ref(), "java/lang/NullPointerException"));

    Ok(())
}

#[tokio::test]
async fn test_arr_08_array_list_typed_to_array_reuse_grow_termination_and_sequential_ase() -> Result<()> {
    let jvm = test_jvm().await?;
    let first = JavaLangString::from_rust_string(&jvm, "first").await?;
    let second = JavaLangString::from_rust_string(&jvm, "second").await?;
    let sentinel = JavaLangString::from_rust_string(&jvm, "sentinel").await?;
    let mut array: ClassInstanceRef<Array<Object>> = jvm.instantiate_array("Ljava/lang/Object;", 2).await?.into();
    jvm.store_array(&mut array, 0, [first.clone(), second.clone()]).await?;
    let list: ClassInstanceRef<Object> = jvm
        .invoke_static("java/util/Arrays", "asList", "([Ljava/lang/Object;)Ljava/util/List;", (array,))
        .await?;

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
    let reused_values = jvm.load_array::<ClassInstanceRef<Object>>(&reused, 0, 4).await?;
    assert_eq!(reused_values[0].identity(), first.identity());
    assert_eq!(reused_values[1].identity(), second.identity());
    assert!(reused_values[2].is_null());
    assert_eq!(reused_values[3].identity(), sentinel.identity());

    let mut exact: ClassInstanceRef<Array<Object>> = jvm.instantiate_array("Ljava/lang/String;", 2).await?.into();
    jvm.store_array(&mut exact, 0, [sentinel.clone(), sentinel.clone()]).await?;
    let exact_identity = exact.identity();
    let exact_result: ClassInstanceRef<Array<Object>> = jvm
        .invoke_virtual(
            &list,
            &list.class_definition().name(),
            "toArray",
            "([Ljava/lang/Object;)[Ljava/lang/Object;",
            (exact,),
        )
        .await?;
    assert_eq!(exact_result.identity(), exact_identity);
    let exact_values = jvm.load_array::<ClassInstanceRef<Object>>(&exact_result, 0, 2).await?;
    assert_eq!(exact_values[0].identity(), first.identity());
    assert_eq!(exact_values[1].identity(), second.identity());

    let mut small: ClassInstanceRef<Array<Object>> = jvm.instantiate_array("Ljava/lang/String;", 1).await?.into();
    jvm.store_array(&mut small, 0, core::iter::once(sentinel.clone())).await?;
    let small_identity = small.identity();
    let grown: ClassInstanceRef<Array<Object>> = jvm
        .invoke_virtual(
            &list,
            &list.class_definition().name(),
            "toArray",
            "([Ljava/lang/Object;)[Ljava/lang/Object;",
            (small.clone(),),
        )
        .await?;
    assert_ne!(grown.identity(), small_identity);
    assert_eq!(grown.class_definition().name(), "[Ljava/lang/String;");
    assert_eq!(jvm.array_length(&grown).await?, 2);
    assert_eq!(
        jvm.load_array::<ClassInstanceRef<Object>>(&small, 0, 1).await?.remove(0).identity(),
        sentinel.identity()
    );

    let null_destination: ClassInstanceRef<Array<Object>> = None.into();
    let result: Result<ClassInstanceRef<Array<Object>>> = jvm
        .invoke_virtual(
            &list,
            &list.class_definition().name(),
            "toArray",
            "([Ljava/lang/Object;)[Ljava/lang/Object;",
            (null_destination,),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("typed toArray must reject null");
    };
    assert!(jvm.is_instance(exception.as_ref(), "java/lang/NullPointerException"));

    let compatible_row: ClassInstanceRef<Object> = jvm.instantiate_array("Ljava/lang/String;", 1).await?.into();
    let incompatible_row: ClassInstanceRef<Object> = jvm.instantiate_array("Ljava/lang/Object;", 1).await?.into();
    let trailing_row: ClassInstanceRef<Object> = jvm.instantiate_array("Ljava/lang/String;", 2).await?.into();
    let sentinel_row: ClassInstanceRef<Object> = jvm.instantiate_array("Ljava/lang/String;", 3).await?.into();
    let mut rows: ClassInstanceRef<Array<Object>> = jvm.instantiate_array("Ljava/lang/Object;", 3).await?.into();
    jvm.store_array(&mut rows, 0, [compatible_row.clone(), incompatible_row, trailing_row])
        .await?;
    let row_list: ClassInstanceRef<Object> = jvm
        .invoke_static("java/util/Arrays", "asList", "([Ljava/lang/Object;)Ljava/util/List;", (rows,))
        .await?;
    let mut destination: ClassInstanceRef<Array<Object>> = jvm.instantiate_array("[Ljava/lang/String;", 4).await?.into();
    jvm.store_array(
        &mut destination,
        0,
        [sentinel_row.clone(), sentinel_row.clone(), sentinel_row.clone(), sentinel_row.clone()],
    )
    .await?;
    let result: Result<ClassInstanceRef<Array<Object>>> = jvm
        .invoke_virtual(
            &row_list,
            &row_list.class_definition().name(),
            "toArray",
            "([Ljava/lang/Object;)[Ljava/lang/Object;",
            (destination.clone(),),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("typed multidimensional toArray must reject an Object[] row");
    };
    assert!(jvm.is_instance(exception.as_ref(), "java/lang/ArrayStoreException"));
    let partial = jvm.load_array::<ClassInstanceRef<Object>>(&destination, 0, 4).await?;
    assert_eq!(partial[0].identity(), compatible_row.identity());
    assert_eq!(partial[1].identity(), sentinel_row.identity());
    assert_eq!(partial[2].identity(), sentinel_row.identity());
    assert_eq!(partial[3].identity(), sentinel_row.identity());

    Ok(())
}
