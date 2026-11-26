use jvm::{Array, ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::{
    RuntimeContext,
    classes::java::{
        lang::Object,
        util::{List, Random, Set},
    },
};

// class java.util.Collections
#[java_class(name = "java/util/Collections", public)]
pub struct Collections {
    #[field(public, static, final)]
    EMPTY_LIST: ClassInstanceRef<List>,
    #[field(public, static, final)]
    EMPTY_SET: ClassInstanceRef<Set>,
    #[field(private, static)]
    r: ClassInstanceRef<Random>,
}

#[java_methods]
impl Collections {
    #[method(static_initializer)]
    async fn clinit(jvm: &Jvm, _: &mut RuntimeContext) -> Result<()> {
        let empty_list = jvm.new_class("java/util/Collections$EmptyList", "()V", ()).await?;
        jvm.put_static_field("java/util/Collections", "EMPTY_LIST", "Ljava/util/List;", empty_list)
            .await?;

        let empty_set = jvm.new_class("java/util/Collections$EmptySet", "()V", ()).await?;
        jvm.put_static_field("java/util/Collections", "EMPTY_SET", "Ljava/util/Set;", empty_set)
            .await?;

        let random = jvm.new_class("java/util/Random", "()V", ()).await?;
        jvm.put_static_field("java/util/Collections", "r", "Ljava/util/Random;", random).await
    }

    #[method(constructor, private)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await
    }

    async fn compare(
        jvm: &Jvm,
        comparator: &ClassInstanceRef<Object>,
        left: &ClassInstanceRef<Object>,
        right: &ClassInstanceRef<Object>,
    ) -> Result<i32> {
        if comparator.is_null() {
            if left.is_null() {
                return Err(jvm.exception("java/lang/NullPointerException", "collection element").await);
            }
            if !jvm.is_instance(left.as_ref(), "java/lang/Comparable") {
                return Err(jvm.exception("java/lang/ClassCastException", &left.class_definition().name()).await);
            }
            jvm.invoke_virtual(
                left,
                &left.class_definition().name(),
                "compareTo",
                "(Ljava/lang/Object;)I",
                (right.clone(),),
            )
            .await
        } else {
            jvm.invoke_virtual(
                comparator,
                &comparator.class_definition().name(),
                "compare",
                "(Ljava/lang/Object;Ljava/lang/Object;)I",
                (left.clone(), right.clone()),
            )
            .await
        }
    }

    async fn write_list(jvm: &Jvm, list: &ClassInstanceRef<Object>, elements: ClassInstanceRef<Array<Object>>) -> Result<()> {
        let length = jvm.array_length(&elements).await?;
        let iterator: ClassInstanceRef<Object> = jvm
            .invoke_virtual(list, &list.class_definition().name(), "listIterator", "()Ljava/util/ListIterator;", ())
            .await?;
        for element in jvm.load_array::<ClassInstanceRef<Object>>(&elements, 0, length).await? {
            let _: ClassInstanceRef<Object> = jvm
                .invoke_virtual(&iterator, &iterator.class_definition().name(), "next", "()Ljava/lang/Object;", ())
                .await?;
            let _: () = jvm
                .invoke_virtual(&iterator, &iterator.class_definition().name(), "set", "(Ljava/lang/Object;)V", (element,))
                .await?;
        }
        Ok(())
    }

    async fn sort_values(jvm: &Jvm, list: ClassInstanceRef<Object>, comparator: ClassInstanceRef<Object>) -> Result<()> {
        if list.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "list").await);
        }

        let elements: ClassInstanceRef<Array<Object>> = jvm
            .invoke_virtual(&list, &list.class_definition().name(), "toArray", "()[Ljava/lang/Object;", ())
            .await?;
        if comparator.is_null() {
            let _: () = jvm
                .invoke_static("java/util/Arrays", "sort", "([Ljava/lang/Object;)V", (elements.clone(),))
                .await?;
        } else {
            let _: () = jvm
                .invoke_static(
                    "java/util/Arrays",
                    "sort",
                    "([Ljava/lang/Object;Ljava/util/Comparator;)V",
                    (elements.clone(), comparator),
                )
                .await?;
        }

        Self::write_list(jvm, &list, elements).await
    }

    #[method(descriptor = "(Ljava/util/List;)V", public, static)]
    async fn sort(jvm: &Jvm, _: &mut RuntimeContext, list: ClassInstanceRef<Object>) -> Result<()> {
        Self::sort_values(jvm, list, None.into()).await
    }

    #[method(name = "sort", descriptor = "(Ljava/util/List;Ljava/util/Comparator;)V", public, static)]
    async fn sort_comparator(jvm: &Jvm, _: &mut RuntimeContext, list: ClassInstanceRef<Object>, comparator: ClassInstanceRef<Object>) -> Result<()> {
        Self::sort_values(jvm, list, comparator).await
    }

    async fn search(jvm: &Jvm, list: ClassInstanceRef<Object>, key: ClassInstanceRef<Object>, comparator: ClassInstanceRef<Object>) -> Result<i32> {
        if list.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "list").await);
        }

        let mut low = 0i32;
        let mut high: i32 = jvm.invoke_virtual(&list, &list.class_definition().name(), "size", "()I", ()).await?;
        while low < high {
            let middle = low + (high - low) / 2;
            let value: ClassInstanceRef<Object> = jvm
                .invoke_virtual(&list, &list.class_definition().name(), "get", "(I)Ljava/lang/Object;", (middle,))
                .await?;
            let comparison = Self::compare(jvm, &comparator, &value, &key).await?;
            if comparison < 0 {
                low = middle + 1;
            } else if comparison > 0 {
                high = middle;
            } else {
                return Ok(middle);
            }
        }
        Ok(-low - 1)
    }

    #[method(descriptor = "(Ljava/util/List;Ljava/lang/Object;)I", public, static)]
    async fn binary_search(jvm: &Jvm, _: &mut RuntimeContext, list: ClassInstanceRef<Object>, key: ClassInstanceRef<Object>) -> Result<i32> {
        Self::search(jvm, list, key, None.into()).await
    }

    #[method(
        name = "binarySearch",
        descriptor = "(Ljava/util/List;Ljava/lang/Object;Ljava/util/Comparator;)I",
        public,
        static
    )]
    async fn binary_search_comparator(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        list: ClassInstanceRef<Object>,
        key: ClassInstanceRef<Object>,
        comparator: ClassInstanceRef<Object>,
    ) -> Result<i32> {
        Self::search(jvm, list, key, comparator).await
    }

    #[method(descriptor = "(Ljava/util/List;)V", public, static)]
    async fn reverse(jvm: &Jvm, _: &mut RuntimeContext, list: ClassInstanceRef<Object>) -> Result<()> {
        if list.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "list").await);
        }

        let size: i32 = jvm.invoke_virtual(&list, &list.class_definition().name(), "size", "()I", ()).await?;
        if size < 2 {
            return Ok(());
        }
        let forward: ClassInstanceRef<Object> = jvm
            .invoke_virtual(&list, &list.class_definition().name(), "listIterator", "()Ljava/util/ListIterator;", ())
            .await?;
        let backward: ClassInstanceRef<Object> = jvm
            .invoke_virtual(
                &list,
                &list.class_definition().name(),
                "listIterator",
                "(I)Ljava/util/ListIterator;",
                (size,),
            )
            .await?;
        for _ in 0..size / 2 {
            let left: ClassInstanceRef<Object> = jvm
                .invoke_virtual(&forward, &forward.class_definition().name(), "next", "()Ljava/lang/Object;", ())
                .await?;
            let right: ClassInstanceRef<Object> = jvm
                .invoke_virtual(&backward, &backward.class_definition().name(), "previous", "()Ljava/lang/Object;", ())
                .await?;
            let _: () = jvm
                .invoke_virtual(&forward, &forward.class_definition().name(), "set", "(Ljava/lang/Object;)V", (right,))
                .await?;
            let _: () = jvm
                .invoke_virtual(&backward, &backward.class_definition().name(), "set", "(Ljava/lang/Object;)V", (left,))
                .await?;
        }
        Ok(())
    }

    #[method(descriptor = "(Ljava/util/List;Ljava/lang/Object;)V", public, static)]
    async fn fill(jvm: &Jvm, _: &mut RuntimeContext, list: ClassInstanceRef<Object>, element: ClassInstanceRef<Object>) -> Result<()> {
        if list.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "list").await);
        }

        let iterator: ClassInstanceRef<Object> = jvm
            .invoke_virtual(&list, &list.class_definition().name(), "listIterator", "()Ljava/util/ListIterator;", ())
            .await?;
        while jvm
            .invoke_virtual::<_, bool>(&iterator, &iterator.class_definition().name(), "hasNext", "()Z", ())
            .await?
        {
            let _: ClassInstanceRef<Object> = jvm
                .invoke_virtual(&iterator, &iterator.class_definition().name(), "next", "()Ljava/lang/Object;", ())
                .await?;
            let _: () = jvm
                .invoke_virtual(
                    &iterator,
                    &iterator.class_definition().name(),
                    "set",
                    "(Ljava/lang/Object;)V",
                    (element.clone(),),
                )
                .await?;
        }
        Ok(())
    }

    #[method(descriptor = "(Ljava/util/List;Ljava/util/List;)V", public, static)]
    async fn copy(jvm: &Jvm, _: &mut RuntimeContext, destination: ClassInstanceRef<Object>, source: ClassInstanceRef<Object>) -> Result<()> {
        if source.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "source").await);
        }
        let source_size: i32 = jvm.invoke_virtual(&source, &source.class_definition().name(), "size", "()I", ()).await?;
        if destination.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "destination").await);
        }
        let destination_size: i32 = jvm
            .invoke_virtual(&destination, &destination.class_definition().name(), "size", "()I", ())
            .await?;
        if source_size > destination_size {
            return Err(jvm
                .exception("java/lang/IndexOutOfBoundsException", "source does not fit in destination")
                .await);
        }

        let elements: ClassInstanceRef<Array<Object>> = jvm
            .invoke_virtual(&source, &source.class_definition().name(), "toArray", "()[Ljava/lang/Object;", ())
            .await?;
        let iterator: ClassInstanceRef<Object> = jvm
            .invoke_virtual(
                &destination,
                &destination.class_definition().name(),
                "listIterator",
                "()Ljava/util/ListIterator;",
                (),
            )
            .await?;
        for element in jvm.load_array::<ClassInstanceRef<Object>>(&elements, 0, source_size as usize).await? {
            let _: ClassInstanceRef<Object> = jvm
                .invoke_virtual(&iterator, &iterator.class_definition().name(), "next", "()Ljava/lang/Object;", ())
                .await?;
            let _: () = jvm
                .invoke_virtual(&iterator, &iterator.class_definition().name(), "set", "(Ljava/lang/Object;)V", (element,))
                .await?;
        }
        Ok(())
    }

    async fn shuffle_values(jvm: &Jvm, list: ClassInstanceRef<Object>, random: ClassInstanceRef<Object>) -> Result<()> {
        if list.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "list").await);
        }
        let elements: ClassInstanceRef<Array<Object>> = jvm
            .invoke_virtual(&list, &list.class_definition().name(), "toArray", "()[Ljava/lang/Object;", ())
            .await?;
        let length = jvm.array_length(&elements).await?;
        if length < 2 {
            return Ok(());
        }
        if random.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "random").await);
        }
        let mut values = jvm.load_array::<ClassInstanceRef<Object>>(&elements, 0, length).await?;
        for index in (1..length).rev() {
            let swap_index: i32 = jvm
                .invoke_virtual(&random, "java/util/Random", "nextInt", "(I)I", ((index + 1) as i32,))
                .await?;
            if swap_index < 0 || swap_index as usize > index {
                return Err(jvm.exception("java/lang/ArrayIndexOutOfBoundsException", "random index").await);
            }
            values.swap(index, swap_index as usize);
        }

        let mut shuffled: ClassInstanceRef<Array<Object>> = jvm.instantiate_array("Ljava/lang/Object;", length).await?.into();
        if !values.is_empty() {
            jvm.store_array(&mut shuffled, 0, values).await?;
        }
        Self::write_list(jvm, &list, shuffled).await
    }

    #[method(descriptor = "(Ljava/util/List;)V", public, static)]
    async fn shuffle(jvm: &Jvm, _: &mut RuntimeContext, list: ClassInstanceRef<Object>) -> Result<()> {
        let random: ClassInstanceRef<Object> = jvm.get_static_field("java/util/Collections", "r", "Ljava/util/Random;").await?;
        Self::shuffle_values(jvm, list, random).await
    }

    #[method(name = "shuffle", descriptor = "(Ljava/util/List;Ljava/util/Random;)V", public, static)]
    async fn shuffle_random(jvm: &Jvm, _: &mut RuntimeContext, list: ClassInstanceRef<Object>, random: ClassInstanceRef<Object>) -> Result<()> {
        Self::shuffle_values(jvm, list, random).await
    }

    async fn extreme(
        jvm: &Jvm,
        collection: ClassInstanceRef<Object>,
        comparator: ClassInstanceRef<Object>,
        find_minimum: bool,
    ) -> Result<ClassInstanceRef<Object>> {
        if collection.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "collection").await);
        }

        let iterator: ClassInstanceRef<Object> = jvm
            .invoke_virtual(
                &collection,
                &collection.class_definition().name(),
                "iterator",
                "()Ljava/util/Iterator;",
                (),
            )
            .await?;
        if !jvm
            .invoke_virtual::<_, bool>(&iterator, &iterator.class_definition().name(), "hasNext", "()Z", ())
            .await?
        {
            return Err(jvm.exception("java/util/NoSuchElementException", "empty collection").await);
        }
        let mut candidate: ClassInstanceRef<Object> = jvm
            .invoke_virtual(&iterator, &iterator.class_definition().name(), "next", "()Ljava/lang/Object;", ())
            .await?;

        while jvm
            .invoke_virtual::<_, bool>(&iterator, &iterator.class_definition().name(), "hasNext", "()Z", ())
            .await?
        {
            let next: ClassInstanceRef<Object> = jvm
                .invoke_virtual(&iterator, &iterator.class_definition().name(), "next", "()Ljava/lang/Object;", ())
                .await?;
            let comparison = Self::compare(jvm, &comparator, &next, &candidate).await?;
            if (find_minimum && comparison < 0) || (!find_minimum && comparison > 0) {
                candidate = next;
            }
        }
        Ok(candidate)
    }

    #[method(descriptor = "(Ljava/util/Collection;)Ljava/lang/Object;", public, static)]
    async fn min(jvm: &Jvm, _: &mut RuntimeContext, collection: ClassInstanceRef<Object>) -> Result<ClassInstanceRef<Object>> {
        Self::extreme(jvm, collection, None.into(), true).await
    }

    #[method(
        name = "min",
        descriptor = "(Ljava/util/Collection;Ljava/util/Comparator;)Ljava/lang/Object;",
        public,
        static
    )]
    async fn min_comparator(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        collection: ClassInstanceRef<Object>,
        comparator: ClassInstanceRef<Object>,
    ) -> Result<ClassInstanceRef<Object>> {
        Self::extreme(jvm, collection, comparator, true).await
    }

    #[method(descriptor = "(Ljava/util/Collection;)Ljava/lang/Object;", public, static)]
    async fn max(jvm: &Jvm, _: &mut RuntimeContext, collection: ClassInstanceRef<Object>) -> Result<ClassInstanceRef<Object>> {
        Self::extreme(jvm, collection, None.into(), false).await
    }

    #[method(
        name = "max",
        descriptor = "(Ljava/util/Collection;Ljava/util/Comparator;)Ljava/lang/Object;",
        public,
        static
    )]
    async fn max_comparator(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        collection: ClassInstanceRef<Object>,
        comparator: ClassInstanceRef<Object>,
    ) -> Result<ClassInstanceRef<Object>> {
        Self::extreme(jvm, collection, comparator, false).await
    }

    #[method(descriptor = "(ILjava/lang/Object;)Ljava/util/List;", public, static)]
    async fn n_copies(jvm: &Jvm, _: &mut RuntimeContext, count: i32, element: ClassInstanceRef<Object>) -> Result<ClassInstanceRef<Object>> {
        if count < 0 {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "List length = negative").await);
        }
        Ok(jvm
            .new_class("java/util/Collections$CopiesList", "(ILjava/lang/Object;)V", (count, element))
            .await?
            .into())
    }

    #[method(descriptor = "(Ljava/lang/Object;)Ljava/util/Set;", public, static)]
    async fn singleton(jvm: &Jvm, _: &mut RuntimeContext, element: ClassInstanceRef<Object>) -> Result<ClassInstanceRef<Object>> {
        Ok(jvm
            .new_class("java/util/Collections$SingletonSet", "(Ljava/lang/Object;)V", (element,))
            .await?
            .into())
    }

    #[method(descriptor = "(Ljava/util/Collection;)Ljava/util/Collection;", public, static)]
    async fn unmodifiable_collection(jvm: &Jvm, _: &mut RuntimeContext, collection: ClassInstanceRef<Object>) -> Result<ClassInstanceRef<Object>> {
        Ok(jvm
            .new_class("java/util/Collections$UnmodifiableCollection", "(Ljava/util/Collection;)V", (collection,))
            .await?
            .into())
    }

    #[method(descriptor = "(Ljava/util/List;)Ljava/util/List;", public, static)]
    async fn unmodifiable_list(jvm: &Jvm, _: &mut RuntimeContext, list: ClassInstanceRef<Object>) -> Result<ClassInstanceRef<Object>> {
        Ok(jvm
            .new_class("java/util/Collections$UnmodifiableList", "(Ljava/util/List;)V", (list,))
            .await?
            .into())
    }

    #[method(descriptor = "(Ljava/util/Set;)Ljava/util/Set;", public, static)]
    async fn unmodifiable_set(jvm: &Jvm, _: &mut RuntimeContext, set: ClassInstanceRef<Object>) -> Result<ClassInstanceRef<Object>> {
        Ok(jvm
            .new_class("java/util/Collections$UnmodifiableSet", "(Ljava/util/Set;)V", (set,))
            .await?
            .into())
    }

    #[method(descriptor = "(Ljava/util/Map;)Ljava/util/Map;", public, static)]
    async fn unmodifiable_map(jvm: &Jvm, _: &mut RuntimeContext, map: ClassInstanceRef<Object>) -> Result<ClassInstanceRef<Object>> {
        Ok(jvm
            .new_class("java/util/Collections$UnmodifiableMap", "(Ljava/util/Map;)V", (map,))
            .await?
            .into())
    }

    #[method(descriptor = "(Ljava/util/SortedSet;)Ljava/util/SortedSet;", public, static)]
    async fn unmodifiable_sorted_set(jvm: &Jvm, _: &mut RuntimeContext, set: ClassInstanceRef<Object>) -> Result<ClassInstanceRef<Object>> {
        Ok(jvm
            .new_class("java/util/Collections$UnmodifiableSortedSet", "(Ljava/util/SortedSet;)V", (set,))
            .await?
            .into())
    }

    #[method(descriptor = "(Ljava/util/SortedMap;)Ljava/util/SortedMap;", public, static)]
    async fn unmodifiable_sorted_map(jvm: &Jvm, _: &mut RuntimeContext, map: ClassInstanceRef<Object>) -> Result<ClassInstanceRef<Object>> {
        Ok(jvm
            .new_class("java/util/Collections$UnmodifiableSortedMap", "(Ljava/util/SortedMap;)V", (map,))
            .await?
            .into())
    }
}
