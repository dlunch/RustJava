use java_constants::{ClassAccessFlags, MethodAccessFlags};
use java_runtime::{classes::java::lang::Object, get_runtime_class_proto};
use jvm::{Array, ClassInstanceRef, JavaError, Result, runtime::JavaLangString};

use test_utils::test_jvm;

#[tokio::test]
async fn list_01_iterators_are_live_and_support_every_state_transition() -> Result<()> {
    let jvm = test_jvm().await?;

    for class_name in ["java/util/ArrayList", "java/util/Vector"] {
        let list = jvm.new_class(class_name, "()V", ()).await?;
        for value in ["a", "b", "c"] {
            let value = JavaLangString::from_rust_string(&jvm, value).await?;
            let _: bool = jvm
                .invoke_virtual(&list, &list.class_definition().name(), "add", "(Ljava/lang/Object;)Z", (value,))
                .await?;
        }

        let iterator: ClassInstanceRef<Object> = jvm
            .invoke_virtual(
                &list,
                &list.class_definition().name(),
                "listIterator",
                "(I)Ljava/util/ListIterator;",
                (1,),
            )
            .await?;
        assert!(jvm.is_instance(&**iterator, "java/util/ListIterator"));
        assert_eq!(
            jvm.invoke_virtual::<_, i32>(&iterator, &iterator.class_definition().name(), "nextIndex", "()I", ())
                .await?,
            1
        );
        assert_eq!(
            jvm.invoke_virtual::<_, i32>(&iterator, &iterator.class_definition().name(), "previousIndex", "()I", ())
                .await?,
            0
        );

        let previous: ClassInstanceRef<Object> = jvm
            .invoke_virtual(&iterator, &iterator.class_definition().name(), "previous", "()Ljava/lang/Object;", ())
            .await?;
        assert_eq!(JavaLangString::to_rust_string(&jvm, &previous).await?, "a");

        let replacement = JavaLangString::from_rust_string(&jvm, "A").await?;
        let _: () = jvm
            .invoke_virtual(
                &iterator,
                &iterator.class_definition().name(),
                "set",
                "(Ljava/lang/Object;)V",
                (replacement,),
            )
            .await?;
        let inserted = JavaLangString::from_rust_string(&jvm, "x").await?;
        let _: () = jvm
            .invoke_virtual(
                &iterator,
                &iterator.class_definition().name(),
                "add",
                "(Ljava/lang/Object;)V",
                (inserted,),
            )
            .await?;

        let result: Result<()> = jvm
            .invoke_virtual(&iterator, &iterator.class_definition().name(), "remove", "()V", ())
            .await;
        let Err(JavaError::JavaException(exception)) = result else {
            panic!("remove after add must fail");
        };
        assert!(jvm.is_instance(&*exception, "java/lang/IllegalStateException"));
        let result: Result<()> = jvm
            .invoke_virtual(
                &iterator,
                &iterator.class_definition().name(),
                "set",
                "(Ljava/lang/Object;)V",
                (JavaLangString::from_rust_string(&jvm, "invalid").await?,),
            )
            .await;
        let Err(JavaError::JavaException(exception)) = result else {
            panic!("set after add must fail");
        };
        assert!(jvm.is_instance(&*exception, "java/lang/IllegalStateException"));

        let next: ClassInstanceRef<Object> = jvm
            .invoke_virtual(&iterator, &iterator.class_definition().name(), "next", "()Ljava/lang/Object;", ())
            .await?;
        assert_eq!(JavaLangString::to_rust_string(&jvm, &next).await?, "A");
        let _: () = jvm
            .invoke_virtual(&iterator, &iterator.class_definition().name(), "remove", "()V", ())
            .await?;

        let tail = JavaLangString::from_rust_string(&jvm, "d").await?;
        let _: bool = jvm
            .invoke_virtual(&list, &list.class_definition().name(), "add", "(Ljava/lang/Object;)Z", (tail,))
            .await?;
        while jvm
            .invoke_virtual::<_, bool>(&iterator, &iterator.class_definition().name(), "hasNext", "()Z", ())
            .await?
        {
            let _: ClassInstanceRef<Object> = jvm
                .invoke_virtual(&iterator, &iterator.class_definition().name(), "next", "()Ljava/lang/Object;", ())
                .await?;
        }
        assert_eq!(
            jvm.invoke_virtual::<_, i32>(&iterator, &iterator.class_definition().name(), "nextIndex", "()I", ())
                .await?,
            4
        );

        let result: Result<ClassInstanceRef<Object>> = jvm
            .invoke_virtual(&iterator, &iterator.class_definition().name(), "next", "()Ljava/lang/Object;", ())
            .await;
        let Err(JavaError::JavaException(exception)) = result else {
            panic!("exhausted iterator must fail");
        };
        assert!(jvm.is_instance(&*exception, "java/util/NoSuchElementException"));

        for invalid_index in [-1, 5] {
            let result: Result<ClassInstanceRef<Object>> = jvm
                .invoke_virtual(
                    &list,
                    &list.class_definition().name(),
                    "listIterator",
                    "(I)Ljava/util/ListIterator;",
                    (invalid_index,),
                )
                .await;
            let Err(JavaError::JavaException(exception)) = result else {
                panic!("invalid list iterator index must fail");
            };
            assert!(jvm.is_instance(&*exception, "java/lang/IndexOutOfBoundsException"));
        }
    }

    Ok(())
}

#[tokio::test]
async fn list_01_descriptors_flags_and_iterator_state_fields_match_contract() -> Result<()> {
    let array_list_iterator = get_runtime_class_proto("java/util/ArrayList$Itr").unwrap();
    assert_eq!(array_list_iterator.methods.iter().filter(|method| method.name == "<init>").count(), 1);
    assert!(
        array_list_iterator
            .methods
            .iter()
            .any(|method| method.name == "<init>" && method.descriptor == "(Ljava/util/List;I)V")
    );

    for class_name in ["java/util/ArrayList", "java/util/Vector"] {
        let proto = get_runtime_class_proto(class_name).unwrap();
        for descriptor in ["()Ljava/util/ListIterator;", "(I)Ljava/util/ListIterator;"] {
            let method = proto
                .methods
                .iter()
                .find(|method| method.name == "listIterator" && method.descriptor == descriptor)
                .unwrap();
            let expected = if class_name == "java/util/Vector" {
                MethodAccessFlags::PUBLIC | MethodAccessFlags::SYNCHRONIZED
            } else {
                MethodAccessFlags::PUBLIC
            };
            assert_eq!(method.access_flags, expected);
        }
    }

    for (class_name, list_descriptor) in [
        ("java/util/AbstractList$ListItr", "Ljava/util/List;"),
        ("java/util/ArrayList$ListItr", "Ljava/util/List;"),
        ("java/util/Vector$ListItr", "Ljava/util/Vector;"),
        ("java/util/LinkedList$ListItr", "Ljava/util/LinkedList;"),
    ] {
        let proto = get_runtime_class_proto(class_name).unwrap();
        assert_eq!(proto.interfaces, vec!["java/util/ListIterator"]);
        let field_proto = if class_name == "java/util/LinkedList$ListItr" {
            get_runtime_class_proto(class_name).unwrap()
        } else {
            get_runtime_class_proto(proto.parent_class.unwrap()).unwrap()
        };
        for (name, descriptor) in [("list", list_descriptor), ("cursor", "I"), ("lastReturned", "I")] {
            assert!(
                field_proto
                    .fields
                    .iter()
                    .any(|field| field.name == name && field.descriptor == descriptor),
                "missing {class_name}.{name}:{descriptor}"
            );
        }
        for (name, descriptor) in [
            ("hasPrevious", "()Z"),
            ("previous", "()Ljava/lang/Object;"),
            ("nextIndex", "()I"),
            ("previousIndex", "()I"),
            ("set", "(Ljava/lang/Object;)V"),
            ("add", "(Ljava/lang/Object;)V"),
        ] {
            let method = proto
                .methods
                .iter()
                .find(|method| method.name == name && method.descriptor == descriptor)
                .unwrap();
            assert_eq!(method.access_flags, MethodAccessFlags::PUBLIC);
        }
    }

    let vector = get_runtime_class_proto("java/util/Vector").unwrap();
    for (name, descriptor) in [
        ("size", "()I"),
        ("get", "(I)Ljava/lang/Object;"),
        ("set", "(ILjava/lang/Object;)Ljava/lang/Object;"),
        ("add", "(Ljava/lang/Object;)Z"),
        ("add", "(ILjava/lang/Object;)V"),
        ("remove", "(I)Ljava/lang/Object;"),
    ] {
        let method = vector
            .methods
            .iter()
            .find(|method| method.name == name && method.descriptor == descriptor)
            .unwrap();
        assert_eq!(method.access_flags, MethodAccessFlags::PUBLIC | MethodAccessFlags::SYNCHRONIZED);
    }

    Ok(())
}

#[tokio::test]
async fn linked_list_list_iterator_covers_boundaries_and_every_legal_transition() -> Result<()> {
    let jvm = test_jvm().await?;
    let list = jvm.new_class("java/util/LinkedList", "()V", ()).await?;
    let iterator: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&list, &list.class_definition().name(), "listIterator", "()Ljava/util/ListIterator;", ())
        .await?;
    assert!(jvm.is_instance(&**iterator, "java/util/ListIterator"));
    assert!(jvm.is_instance(&**iterator, "java/util/Iterator"));
    assert!(
        !jvm.invoke_virtual::<_, bool>(&iterator, &iterator.class_definition().name(), "hasNext", "()Z", ())
            .await?
    );
    assert!(
        !jvm.invoke_virtual::<_, bool>(&iterator, &iterator.class_definition().name(), "hasPrevious", "()Z", ())
            .await?
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&iterator, &iterator.class_definition().name(), "nextIndex", "()I", ())
            .await?,
        0
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&iterator, &iterator.class_definition().name(), "previousIndex", "()I", ())
            .await?,
        -1
    );

    for method in ["next", "previous"] {
        let result: Result<ClassInstanceRef<Object>> = jvm
            .invoke_virtual(&iterator, &iterator.class_definition().name(), method, "()Ljava/lang/Object;", ())
            .await;
        let Err(JavaError::JavaException(exception)) = result else {
            panic!("{method} on an empty iterator must fail");
        };
        assert!(jvm.is_instance(&*exception, "java/util/NoSuchElementException"));
    }
    let result: Result<()> = jvm
        .invoke_virtual(&iterator, &iterator.class_definition().name(), "remove", "()V", ())
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("remove before traversal must fail");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/IllegalStateException"));
    let result: Result<()> = jvm
        .invoke_virtual(
            &iterator,
            &iterator.class_definition().name(),
            "set",
            "(Ljava/lang/Object;)V",
            (JavaLangString::from_rust_string(&jvm, "invalid").await?,),
        )
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("set before traversal must fail");
    };
    assert!(jvm.is_instance(&*exception, "java/lang/IllegalStateException"));

    for value in ["a", "b", "c"] {
        let value = JavaLangString::from_rust_string(&jvm, value).await?;
        let _: bool = jvm
            .invoke_virtual(&list, &list.class_definition().name(), "add", "(Ljava/lang/Object;)Z", (value,))
            .await?;
    }

    let iterator: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &list,
            &list.class_definition().name(),
            "listIterator",
            "(I)Ljava/util/ListIterator;",
            (0,),
        )
        .await?;
    let value: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&iterator, &iterator.class_definition().name(), "next", "()Ljava/lang/Object;", ())
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &value).await?, "a");
    let _: () = jvm
        .invoke_virtual(
            &iterator,
            &iterator.class_definition().name(),
            "set",
            "(Ljava/lang/Object;)V",
            (JavaLangString::from_rust_string(&jvm, "A").await?,),
        )
        .await?;
    let _: () = jvm
        .invoke_virtual(&iterator, &iterator.class_definition().name(), "remove", "()V", ())
        .await?;
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&iterator, &iterator.class_definition().name(), "nextIndex", "()I", ())
            .await?,
        0
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&iterator, &iterator.class_definition().name(), "previousIndex", "()I", ())
            .await?,
        -1
    );

    for method in ["remove", "set"] {
        let result: Result<()> = if method == "remove" {
            jvm.invoke_virtual(&iterator, &iterator.class_definition().name(), method, "()V", ())
                .await
        } else {
            jvm.invoke_virtual(
                &iterator,
                &iterator.class_definition().name(),
                method,
                "(Ljava/lang/Object;)V",
                (JavaLangString::from_rust_string(&jvm, "invalid").await?,),
            )
            .await
        };
        let Err(JavaError::JavaException(exception)) = result else {
            panic!("{method} after remove must fail");
        };
        assert!(jvm.is_instance(&*exception, "java/lang/IllegalStateException"));
    }

    let _: () = jvm
        .invoke_virtual(
            &iterator,
            &iterator.class_definition().name(),
            "add",
            "(Ljava/lang/Object;)V",
            (JavaLangString::from_rust_string(&jvm, "x").await?,),
        )
        .await?;
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&iterator, &iterator.class_definition().name(), "nextIndex", "()I", ())
            .await?,
        1
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&iterator, &iterator.class_definition().name(), "previousIndex", "()I", ())
            .await?,
        0
    );
    for method in ["remove", "set"] {
        let result: Result<()> = if method == "remove" {
            jvm.invoke_virtual(&iterator, &iterator.class_definition().name(), method, "()V", ())
                .await
        } else {
            jvm.invoke_virtual(
                &iterator,
                &iterator.class_definition().name(),
                method,
                "(Ljava/lang/Object;)V",
                (JavaLangString::from_rust_string(&jvm, "invalid").await?,),
            )
            .await
        };
        let Err(JavaError::JavaException(exception)) = result else {
            panic!("{method} after add must fail");
        };
        assert!(jvm.is_instance(&*exception, "java/lang/IllegalStateException"));
    }

    let value: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&iterator, &iterator.class_definition().name(), "previous", "()Ljava/lang/Object;", ())
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &value).await?, "x");
    let _: () = jvm
        .invoke_virtual(
            &iterator,
            &iterator.class_definition().name(),
            "set",
            "(Ljava/lang/Object;)V",
            (JavaLangString::from_rust_string(&jvm, "X").await?,),
        )
        .await?;
    let _: () = jvm
        .invoke_virtual(&iterator, &iterator.class_definition().name(), "remove", "()V", ())
        .await?;
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&iterator, &iterator.class_definition().name(), "nextIndex", "()I", ())
            .await?,
        0
    );

    let result: Result<ClassInstanceRef<Object>> = jvm
        .invoke_virtual(&iterator, &iterator.class_definition().name(), "previous", "()Ljava/lang/Object;", ())
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("previous at the start boundary must fail");
    };
    assert!(jvm.is_instance(&*exception, "java/util/NoSuchElementException"));

    let middle: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &list,
            &list.class_definition().name(),
            "listIterator",
            "(I)Ljava/util/ListIterator;",
            (1,),
        )
        .await?;
    let value: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&middle, &middle.class_definition().name(), "previous", "()Ljava/lang/Object;", ())
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &value).await?, "b");
    let value: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&middle, &middle.class_definition().name(), "next", "()Ljava/lang/Object;", ())
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &value).await?, "b");
    let _: () = jvm
        .invoke_virtual(
            &middle,
            &middle.class_definition().name(),
            "set",
            "(Ljava/lang/Object;)V",
            (JavaLangString::from_rust_string(&jvm, "B").await?,),
        )
        .await?;
    let value: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&middle, &middle.class_definition().name(), "next", "()Ljava/lang/Object;", ())
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &value).await?, "c");
    assert!(
        !jvm.invoke_virtual::<_, bool>(&middle, &middle.class_definition().name(), "hasNext", "()Z", ())
            .await?
    );

    let result: Result<ClassInstanceRef<Object>> = jvm
        .invoke_virtual(&middle, &middle.class_definition().name(), "next", "()Ljava/lang/Object;", ())
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("next at the end boundary must fail");
    };
    assert!(jvm.is_instance(&*exception, "java/util/NoSuchElementException"));

    let value: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&middle, &middle.class_definition().name(), "previous", "()Ljava/lang/Object;", ())
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &value).await?, "c");
    let _: () = jvm
        .invoke_virtual(&middle, &middle.class_definition().name(), "remove", "()V", ())
        .await?;
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&middle, &middle.class_definition().name(), "nextIndex", "()I", ())
            .await?,
        1
    );
    let value: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&middle, &middle.class_definition().name(), "previous", "()Ljava/lang/Object;", ())
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &value).await?, "B");
    let _: () = jvm
        .invoke_virtual(&middle, &middle.class_definition().name(), "remove", "()V", ())
        .await?;
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&list, &list.class_definition().name(), "size", "()I", ())
            .await?,
        0
    );

    let header: ClassInstanceRef<Object> = jvm.get_field(&list, "header", "Ljava/util/LinkedList$Entry;").await?;
    let next: ClassInstanceRef<Object> = jvm.get_field(&header, "next", "Ljava/util/LinkedList$Entry;").await?;
    let previous: ClassInstanceRef<Object> = jvm.get_field(&header, "previous", "Ljava/util/LinkedList$Entry;").await?;
    assert!(
        jvm.invoke_virtual::<_, bool>(&header, &header.class_definition().name(), "equals", "(Ljava/lang/Object;)Z", (next,))
            .await?
    );
    assert!(
        jvm.invoke_virtual::<_, bool>(&header, &header.class_definition().name(), "equals", "(Ljava/lang/Object;)Z", (previous,))
            .await?
    );

    Ok(())
}

#[tokio::test]
async fn linked_list_sentinel_unlinks_middle_single_and_last_entries() -> Result<()> {
    let jvm = test_jvm().await?;
    let list = jvm.new_class("java/util/LinkedList", "()V", ()).await?;
    for value in ["first", "middle", "last"] {
        let value = JavaLangString::from_rust_string(&jvm, value).await?;
        let _: bool = jvm
            .invoke_virtual(&list, &list.class_definition().name(), "add", "(Ljava/lang/Object;)Z", (value,))
            .await?;
    }

    let removed: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&list, &list.class_definition().name(), "remove", "(I)Ljava/lang/Object;", (1,))
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &removed).await?, "middle");
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&list, &list.class_definition().name(), "size", "()I", ())
            .await?,
        2
    );
    for (index, expected) in ["first", "last"].into_iter().enumerate() {
        let value: ClassInstanceRef<Object> = jvm
            .invoke_virtual(&list, &list.class_definition().name(), "get", "(I)Ljava/lang/Object;", (index as i32,))
            .await?;
        assert_eq!(JavaLangString::to_rust_string(&jvm, &value).await?, expected);
    }

    let header: ClassInstanceRef<Object> = jvm.get_field(&list, "header", "Ljava/util/LinkedList$Entry;").await?;
    let first: ClassInstanceRef<Object> = jvm.get_field(&header, "next", "Ljava/util/LinkedList$Entry;").await?;
    let last: ClassInstanceRef<Object> = jvm.get_field(&header, "previous", "Ljava/util/LinkedList$Entry;").await?;
    let first_next: ClassInstanceRef<Object> = jvm.get_field(&first, "next", "Ljava/util/LinkedList$Entry;").await?;
    let last_previous: ClassInstanceRef<Object> = jvm.get_field(&last, "previous", "Ljava/util/LinkedList$Entry;").await?;
    assert!(
        jvm.invoke_virtual::<_, bool>(&last, &last.class_definition().name(), "equals", "(Ljava/lang/Object;)Z", (first_next,))
            .await?
    );
    assert!(
        jvm.invoke_virtual::<_, bool>(
            &first,
            &first.class_definition().name(),
            "equals",
            "(Ljava/lang/Object;)Z",
            (last_previous,)
        )
        .await?
    );

    let removed: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&list, &list.class_definition().name(), "removeFirst", "()Ljava/lang/Object;", ())
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &removed).await?, "first");
    let header: ClassInstanceRef<Object> = jvm.get_field(&list, "header", "Ljava/util/LinkedList$Entry;").await?;
    let only_next: ClassInstanceRef<Object> = jvm.get_field(&header, "next", "Ljava/util/LinkedList$Entry;").await?;
    let only_previous: ClassInstanceRef<Object> = jvm.get_field(&header, "previous", "Ljava/util/LinkedList$Entry;").await?;
    assert!(
        jvm.invoke_virtual::<_, bool>(
            &only_next,
            &only_next.class_definition().name(),
            "equals",
            "(Ljava/lang/Object;)Z",
            (only_previous,)
        )
        .await?
    );

    let removed: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&list, &list.class_definition().name(), "removeLast", "()Ljava/lang/Object;", ())
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &removed).await?, "last");
    let next: ClassInstanceRef<Object> = jvm.get_field(&header, "next", "Ljava/util/LinkedList$Entry;").await?;
    let previous: ClassInstanceRef<Object> = jvm.get_field(&header, "previous", "Ljava/util/LinkedList$Entry;").await?;
    assert!(
        jvm.invoke_virtual::<_, bool>(&header, &header.class_definition().name(), "equals", "(Ljava/lang/Object;)Z", (next,))
            .await?
    );
    assert!(
        jvm.invoke_virtual::<_, bool>(&header, &header.class_definition().name(), "equals", "(Ljava/lang/Object;)Z", (previous,))
            .await?
    );

    Ok(())
}

#[tokio::test]
async fn list_02_to_04_linked_list_uses_sentinel_and_live_list_iterator() -> Result<()> {
    let proto = get_runtime_class_proto("java/util/LinkedList").expect("LinkedList registration");
    assert_eq!(proto.access_flags, ClassAccessFlags::PUBLIC);
    assert_eq!(proto.interfaces, vec!["java/util/List", "java/lang/Cloneable", "java/io/Serializable"]);
    for (name, descriptor) in [
        ("<init>", "()V"),
        ("<init>", "(Ljava/util/Collection;)V"),
        ("addFirst", "(Ljava/lang/Object;)V"),
        ("addLast", "(Ljava/lang/Object;)V"),
        ("getFirst", "()Ljava/lang/Object;"),
        ("getLast", "()Ljava/lang/Object;"),
        ("removeFirst", "()Ljava/lang/Object;"),
        ("removeLast", "()Ljava/lang/Object;"),
        ("listIterator", "(I)Ljava/util/ListIterator;"),
    ] {
        let method = proto
            .methods
            .iter()
            .find(|method| method.name == name && method.descriptor == descriptor)
            .unwrap_or_else(|| panic!("missing LinkedList.{name}{descriptor}"));
        assert_eq!(method.access_flags, MethodAccessFlags::PUBLIC);
    }

    let jvm = test_jvm().await?;
    let list = jvm.new_class("java/util/LinkedList", "()V", ()).await?;
    let header: ClassInstanceRef<Object> = jvm.get_field(&list, "header", "Ljava/util/LinkedList$Entry;").await?;
    let next: ClassInstanceRef<Object> = jvm.get_field(&header, "next", "Ljava/util/LinkedList$Entry;").await?;
    let previous: ClassInstanceRef<Object> = jvm.get_field(&header, "previous", "Ljava/util/LinkedList$Entry;").await?;
    assert!(
        jvm.invoke_virtual::<_, bool>(&header, &header.class_definition().name(), "equals", "(Ljava/lang/Object;)Z", (next,))
            .await?
    );
    assert!(
        jvm.invoke_virtual::<_, bool>(&header, &header.class_definition().name(), "equals", "(Ljava/lang/Object;)Z", (previous,))
            .await?
    );

    let result: Result<ClassInstanceRef<Object>> = jvm
        .invoke_virtual(&list, &list.class_definition().name(), "getFirst", "()Ljava/lang/Object;", ())
        .await;
    let Err(JavaError::JavaException(exception)) = result else {
        panic!("empty LinkedList.getFirst must fail");
    };
    assert!(jvm.is_instance(&*exception, "java/util/NoSuchElementException"));

    let null: ClassInstanceRef<Object> = None.into();
    let first = JavaLangString::from_rust_string(&jvm, "first").await?;
    let last = JavaLangString::from_rust_string(&jvm, "last").await?;
    let _: () = jvm
        .invoke_virtual(&list, &list.class_definition().name(), "addFirst", "(Ljava/lang/Object;)V", (first,))
        .await?;
    let _: bool = jvm
        .invoke_virtual(&list, &list.class_definition().name(), "add", "(Ljava/lang/Object;)Z", (null,))
        .await?;
    let _: () = jvm
        .invoke_virtual(&list, &list.class_definition().name(), "addLast", "(Ljava/lang/Object;)V", (last,))
        .await?;
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&list, &list.class_definition().name(), "size", "()I", ())
            .await?,
        3
    );

    let iterator: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &list,
            &list.class_definition().name(),
            "listIterator",
            "(I)Ljava/util/ListIterator;",
            (3,),
        )
        .await?;
    let value: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&iterator, &iterator.class_definition().name(), "previous", "()Ljava/lang/Object;", ())
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &value).await?, "last");
    let _: () = jvm
        .invoke_virtual(&iterator, &iterator.class_definition().name(), "remove", "()V", ())
        .await?;
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&list, &list.class_definition().name(), "size", "()I", ())
            .await?,
        2
    );

    let duplicate = JavaLangString::from_rust_string(&jvm, "first").await?;
    let _: bool = jvm
        .invoke_virtual(&list, &list.class_definition().name(), "add", "(Ljava/lang/Object;)Z", (duplicate,))
        .await?;
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(
            &list,
            &list.class_definition().name(),
            "indexOf",
            "(Ljava/lang/Object;)I",
            (JavaLangString::from_rust_string(&jvm, "first").await?,)
        )
        .await?,
        0
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(
            &list,
            &list.class_definition().name(),
            "lastIndexOf",
            "(Ljava/lang/Object;)I",
            (JavaLangString::from_rust_string(&jvm, "first").await?,),
        )
        .await?,
        2
    );

    let copy = jvm
        .new_class("java/util/LinkedList", "(Ljava/util/Collection;)V", (list.clone(),))
        .await?;
    let destination: ClassInstanceRef<Array<Object>> = jvm.instantiate_array("Ljava/lang/String;", 4).await?.into();
    let array: ClassInstanceRef<Array<Object>> = jvm
        .invoke_virtual(
            &copy,
            &copy.class_definition().name(),
            "toArray",
            "([Ljava/lang/Object;)[Ljava/lang/Object;",
            (destination,),
        )
        .await?;
    assert_eq!(jvm.array_length(&array).await?, 4);

    let _: () = jvm.invoke_virtual(&list, &list.class_definition().name(), "clear", "()V", ()).await?;
    let header: ClassInstanceRef<Object> = jvm.get_field(&list, "header", "Ljava/util/LinkedList$Entry;").await?;
    let next: ClassInstanceRef<Object> = jvm.get_field(&header, "next", "Ljava/util/LinkedList$Entry;").await?;
    let previous: ClassInstanceRef<Object> = jvm.get_field(&header, "previous", "Ljava/util/LinkedList$Entry;").await?;
    assert!(
        jvm.invoke_virtual::<_, bool>(&header, &header.class_definition().name(), "equals", "(Ljava/lang/Object;)Z", (next,))
            .await?
    );
    assert!(
        jvm.invoke_virtual::<_, bool>(&header, &header.class_definition().name(), "equals", "(Ljava/lang/Object;)Z", (previous,))
            .await?
    );

    Ok(())
}

#[tokio::test]
async fn nested_sub_lists_propagate_every_mutation_through_three_ancestors() -> Result<()> {
    let jvm = test_jvm().await?;
    let root: ClassInstanceRef<Object> = jvm.new_class("java/util/ArrayList", "()V", ()).await?.into();
    for value in ["a", "b", "c", "d", "e", "f", "g", "h"] {
        let value = JavaLangString::from_rust_string(&jvm, value).await?;
        let _: bool = jvm
            .invoke_virtual(&root, &root.class_definition().name(), "add", "(Ljava/lang/Object;)Z", (value,))
            .await?;
    }

    let first: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&root, &root.class_definition().name(), "subList", "(II)Ljava/util/List;", (1, 7))
        .await?;
    let second: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&first, &first.class_definition().name(), "subList", "(II)Ljava/util/List;", (1, 5))
        .await?;
    let third: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&second, &second.class_definition().name(), "subList", "(II)Ljava/util/List;", (1, 3))
        .await?;
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&root, &root.class_definition().name(), "size", "()I", ())
            .await?,
        8
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&first, &first.class_definition().name(), "size", "()I", ())
            .await?,
        6
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&second, &second.class_definition().name(), "size", "()I", ())
            .await?,
        4
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&third, &third.class_definition().name(), "size", "()I", ())
            .await?,
        2
    );

    let source = jvm.new_class("java/util/ArrayList", "()V", ()).await?;
    for value in ["x", "y"] {
        let value = JavaLangString::from_rust_string(&jvm, value).await?;
        let _: bool = jvm
            .invoke_virtual(&source, &source.class_definition().name(), "add", "(Ljava/lang/Object;)Z", (value,))
            .await?;
    }
    assert!(
        jvm.invoke_virtual::<_, bool>(
            &third,
            &third.class_definition().name(),
            "addAll",
            "(ILjava/util/Collection;)Z",
            (1, source)
        )
        .await?
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&root, &root.class_definition().name(), "size", "()I", ())
            .await?,
        10
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&first, &first.class_definition().name(), "size", "()I", ())
            .await?,
        8
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&second, &second.class_definition().name(), "size", "()I", ())
            .await?,
        6
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&third, &third.class_definition().name(), "size", "()I", ())
            .await?,
        4
    );
    for (target, expected) in [
        (root.clone(), &["a", "b", "c", "d", "x", "y", "e", "f", "g", "h"][..]),
        (first.clone(), &["b", "c", "d", "x", "y", "e", "f", "g"][..]),
        (second.clone(), &["c", "d", "x", "y", "e", "f"][..]),
        (third.clone(), &["d", "x", "y", "e"][..]),
    ] {
        for (index, expected) in expected.iter().enumerate() {
            let value: ClassInstanceRef<Object> = jvm
                .invoke_virtual(
                    &target,
                    &target.class_definition().name(),
                    "get",
                    "(I)Ljava/lang/Object;",
                    (index as i32,),
                )
                .await?;
            assert_eq!(JavaLangString::to_rust_string(&jvm, &value).await?, *expected);
        }
    }

    let removed: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&third, &third.class_definition().name(), "remove", "(I)Ljava/lang/Object;", (0,))
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &removed).await?, "d");
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&root, &root.class_definition().name(), "size", "()I", ())
            .await?,
        9
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&first, &first.class_definition().name(), "size", "()I", ())
            .await?,
        7
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&second, &second.class_definition().name(), "size", "()I", ())
            .await?,
        5
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&third, &third.class_definition().name(), "size", "()I", ())
            .await?,
        3
    );
    for (target, expected) in [
        (root.clone(), &["a", "b", "c", "x", "y", "e", "f", "g", "h"][..]),
        (first.clone(), &["b", "c", "x", "y", "e", "f", "g"][..]),
        (second.clone(), &["c", "x", "y", "e", "f"][..]),
        (third.clone(), &["x", "y", "e"][..]),
    ] {
        for (index, expected) in expected.iter().enumerate() {
            let value: ClassInstanceRef<Object> = jvm
                .invoke_virtual(
                    &target,
                    &target.class_definition().name(),
                    "get",
                    "(I)Ljava/lang/Object;",
                    (index as i32,),
                )
                .await?;
            assert_eq!(JavaLangString::to_rust_string(&jvm, &value).await?, *expected);
        }
    }

    let iterator: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &third,
            &third.class_definition().name(),
            "listIterator",
            "(I)Ljava/util/ListIterator;",
            (1,),
        )
        .await?;
    let value: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&iterator, &iterator.class_definition().name(), "next", "()Ljava/lang/Object;", ())
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &value).await?, "y");
    let _: () = jvm
        .invoke_virtual(
            &iterator,
            &iterator.class_definition().name(),
            "set",
            "(Ljava/lang/Object;)V",
            (JavaLangString::from_rust_string(&jvm, "Y").await?,),
        )
        .await?;
    let _: () = jvm
        .invoke_virtual(
            &iterator,
            &iterator.class_definition().name(),
            "add",
            "(Ljava/lang/Object;)V",
            (JavaLangString::from_rust_string(&jvm, "z").await?,),
        )
        .await?;
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&root, &root.class_definition().name(), "size", "()I", ())
            .await?,
        10
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&first, &first.class_definition().name(), "size", "()I", ())
            .await?,
        8
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&second, &second.class_definition().name(), "size", "()I", ())
            .await?,
        6
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&third, &third.class_definition().name(), "size", "()I", ())
            .await?,
        4
    );
    for (target, expected) in [
        (root.clone(), &["a", "b", "c", "x", "Y", "z", "e", "f", "g", "h"][..]),
        (first.clone(), &["b", "c", "x", "Y", "z", "e", "f", "g"][..]),
        (second.clone(), &["c", "x", "Y", "z", "e", "f"][..]),
        (third.clone(), &["x", "Y", "z", "e"][..]),
    ] {
        for (index, expected) in expected.iter().enumerate() {
            let value: ClassInstanceRef<Object> = jvm
                .invoke_virtual(
                    &target,
                    &target.class_definition().name(),
                    "get",
                    "(I)Ljava/lang/Object;",
                    (index as i32,),
                )
                .await?;
            assert_eq!(JavaLangString::to_rust_string(&jvm, &value).await?, *expected);
        }
    }

    let value: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&iterator, &iterator.class_definition().name(), "previous", "()Ljava/lang/Object;", ())
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &value).await?, "z");
    let _: () = jvm
        .invoke_virtual(&iterator, &iterator.class_definition().name(), "remove", "()V", ())
        .await?;
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&root, &root.class_definition().name(), "size", "()I", ())
            .await?,
        9
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&first, &first.class_definition().name(), "size", "()I", ())
            .await?,
        7
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&second, &second.class_definition().name(), "size", "()I", ())
            .await?,
        5
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&third, &third.class_definition().name(), "size", "()I", ())
            .await?,
        3
    );
    for (target, expected) in [
        (root.clone(), &["a", "b", "c", "x", "Y", "e", "f", "g", "h"][..]),
        (first.clone(), &["b", "c", "x", "Y", "e", "f", "g"][..]),
        (second.clone(), &["c", "x", "Y", "e", "f"][..]),
        (third.clone(), &["x", "Y", "e"][..]),
    ] {
        for (index, expected) in expected.iter().enumerate() {
            let value: ClassInstanceRef<Object> = jvm
                .invoke_virtual(
                    &target,
                    &target.class_definition().name(),
                    "get",
                    "(I)Ljava/lang/Object;",
                    (index as i32,),
                )
                .await?;
            assert_eq!(JavaLangString::to_rust_string(&jvm, &value).await?, *expected);
        }
    }

    let value: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&iterator, &iterator.class_definition().name(), "previous", "()Ljava/lang/Object;", ())
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &value).await?, "Y");
    let _: () = jvm
        .invoke_virtual(&iterator, &iterator.class_definition().name(), "remove", "()V", ())
        .await?;
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&root, &root.class_definition().name(), "size", "()I", ())
            .await?,
        8
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&first, &first.class_definition().name(), "size", "()I", ())
            .await?,
        6
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&second, &second.class_definition().name(), "size", "()I", ())
            .await?,
        4
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&third, &third.class_definition().name(), "size", "()I", ())
            .await?,
        2
    );

    for (target, expected) in [
        (root.clone(), &["a", "b", "c", "x", "e", "f", "g", "h"][..]),
        (first.clone(), &["b", "c", "x", "e", "f", "g"][..]),
        (second.clone(), &["c", "x", "e", "f"][..]),
        (third.clone(), &["x", "e"][..]),
    ] {
        for (index, expected) in expected.iter().enumerate() {
            let value: ClassInstanceRef<Object> = jvm
                .invoke_virtual(
                    &target,
                    &target.class_definition().name(),
                    "get",
                    "(I)Ljava/lang/Object;",
                    (index as i32,),
                )
                .await?;
            assert_eq!(JavaLangString::to_rust_string(&jvm, &value).await?, *expected);
        }
    }

    let sentinel = JavaLangString::from_rust_string(&jvm, "sentinel").await?;
    let mut destination: ClassInstanceRef<Array<Object>> = jvm.instantiate_array("Ljava/lang/String;", 4).await?.into();
    jvm.store_array(
        &mut destination,
        0,
        [sentinel.clone(), sentinel.clone(), sentinel.clone(), sentinel.clone()],
    )
    .await?;
    let typed: ClassInstanceRef<Array<Object>> = jvm
        .invoke_virtual(
            &third,
            &third.class_definition().name(),
            "toArray",
            "([Ljava/lang/Object;)[Ljava/lang/Object;",
            (destination,),
        )
        .await?;
    let values: Vec<ClassInstanceRef<Object>> = jvm.load_array(&typed, 0, 4).await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &values[0]).await?, "x");
    assert_eq!(JavaLangString::to_rust_string(&jvm, &values[1]).await?, "e");
    assert!(values[2].is_null());
    assert_eq!(JavaLangString::to_rust_string(&jvm, &values[3]).await?, "sentinel");

    let _: () = jvm.invoke_virtual(&third, &third.class_definition().name(), "clear", "()V", ()).await?;
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&root, &root.class_definition().name(), "size", "()I", ())
            .await?,
        6
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&first, &first.class_definition().name(), "size", "()I", ())
            .await?,
        4
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&second, &second.class_definition().name(), "size", "()I", ())
            .await?,
        2
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&third, &third.class_definition().name(), "size", "()I", ())
            .await?,
        0
    );
    for (target, expected) in [
        (root.clone(), &["a", "b", "c", "f", "g", "h"][..]),
        (first.clone(), &["b", "c", "f", "g"][..]),
        (second.clone(), &["c", "f"][..]),
        (third.clone(), &[] as &[&str]),
    ] {
        for (index, expected) in expected.iter().enumerate() {
            let value: ClassInstanceRef<Object> = jvm
                .invoke_virtual(
                    &target,
                    &target.class_definition().name(),
                    "get",
                    "(I)Ljava/lang/Object;",
                    (index as i32,),
                )
                .await?;
            assert_eq!(JavaLangString::to_rust_string(&jvm, &value).await?, *expected);
        }
    }

    Ok(())
}

#[tokio::test]
async fn sl_01_to_02_nested_sub_lists_are_live_in_both_directions() -> Result<()> {
    let jvm = test_jvm().await?;
    let root = jvm.new_class("java/util/ArrayList", "()V", ()).await?;
    for value in ["a", "b", "c", "d", "e"] {
        let value = JavaLangString::from_rust_string(&jvm, value).await?;
        let _: bool = jvm
            .invoke_virtual(&root, &root.class_definition().name(), "add", "(Ljava/lang/Object;)Z", (value,))
            .await?;
    }

    let view: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&root, &root.class_definition().name(), "subList", "(II)Ljava/util/List;", (1, 5))
        .await?;
    let nested: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&view, &view.class_definition().name(), "subList", "(II)Ljava/util/List;", (1, 3))
        .await?;
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&view, &view.class_definition().name(), "size", "()I", ())
            .await?,
        4
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&nested, &nested.class_definition().name(), "size", "()I", ())
            .await?,
        2
    );

    let inserted = JavaLangString::from_rust_string(&jvm, "x").await?;
    let _: () = jvm
        .invoke_virtual(&nested, &nested.class_definition().name(), "add", "(ILjava/lang/Object;)V", (1, inserted))
        .await?;
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&root, &root.class_definition().name(), "size", "()I", ())
            .await?,
        6
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&view, &view.class_definition().name(), "size", "()I", ())
            .await?,
        5
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&nested, &nested.class_definition().name(), "size", "()I", ())
            .await?,
        3
    );

    let root_value: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&root, &root.class_definition().name(), "get", "(I)Ljava/lang/Object;", (3,))
        .await?;
    assert_eq!(JavaLangString::to_rust_string(&jvm, &root_value).await?, "x");

    let iterator: ClassInstanceRef<Object> = jvm
        .invoke_virtual(
            &nested,
            &nested.class_definition().name(),
            "listIterator",
            "(I)Ljava/util/ListIterator;",
            (0,),
        )
        .await?;
    let _: ClassInstanceRef<Object> = jvm
        .invoke_virtual(&iterator, &iterator.class_definition().name(), "next", "()Ljava/lang/Object;", ())
        .await?;
    let replacement = JavaLangString::from_rust_string(&jvm, "C").await?;
    let _: () = jvm
        .invoke_virtual(
            &iterator,
            &iterator.class_definition().name(),
            "set",
            "(Ljava/lang/Object;)V",
            (replacement,),
        )
        .await?;
    let _: () = jvm
        .invoke_virtual(&iterator, &iterator.class_definition().name(), "remove", "()V", ())
        .await?;

    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&root, &root.class_definition().name(), "size", "()I", ())
            .await?,
        5
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&view, &view.class_definition().name(), "size", "()I", ())
            .await?,
        4
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&nested, &nested.class_definition().name(), "size", "()I", ())
            .await?,
        2
    );

    let _: () = jvm.invoke_virtual(&nested, &nested.class_definition().name(), "clear", "()V", ()).await?;
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&root, &root.class_definition().name(), "size", "()I", ())
            .await?,
        3
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&view, &view.class_definition().name(), "size", "()I", ())
            .await?,
        2
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&nested, &nested.class_definition().name(), "size", "()I", ())
            .await?,
        0
    );

    let source = jvm.new_class("java/util/ArrayList", "()V", ()).await?;
    for value in ["y", "z"] {
        let value = JavaLangString::from_rust_string(&jvm, value).await?;
        let _: bool = jvm
            .invoke_virtual(&source, &source.class_definition().name(), "add", "(Ljava/lang/Object;)Z", (value,))
            .await?;
    }
    let modified: bool = jvm
        .invoke_virtual(
            &view,
            &view.class_definition().name(),
            "addAll",
            "(ILjava/util/Collection;)Z",
            (1, source),
        )
        .await?;
    assert!(modified);
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&root, &root.class_definition().name(), "size", "()I", ())
            .await?,
        5
    );
    assert_eq!(
        jvm.invoke_virtual::<_, i32>(&view, &view.class_definition().name(), "size", "()I", ())
            .await?,
        4
    );

    let typed: ClassInstanceRef<Array<Object>> = jvm.instantiate_array("Ljava/lang/String;", 0).await?.into();
    let typed: ClassInstanceRef<Array<Object>> = jvm
        .invoke_virtual(
            &view,
            &view.class_definition().name(),
            "toArray",
            "([Ljava/lang/Object;)[Ljava/lang/Object;",
            (typed,),
        )
        .await?;
    assert_eq!(jvm.array_length(&typed).await?, 4);

    for (from, to, expected) in [
        (-1, 0, "java/lang/IndexOutOfBoundsException"),
        (0, 5, "java/lang/IndexOutOfBoundsException"),
        (3, 2, "java/lang/IllegalArgumentException"),
    ] {
        let result: Result<ClassInstanceRef<Object>> = jvm
            .invoke_virtual(&view, &view.class_definition().name(), "subList", "(II)Ljava/util/List;", (from, to))
            .await;
        let Err(JavaError::JavaException(exception)) = result else {
            panic!("invalid subList range must fail");
        };
        assert!(jvm.is_instance(&*exception, expected));
    }

    Ok(())
}
