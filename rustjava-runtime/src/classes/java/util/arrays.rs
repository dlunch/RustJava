use alloc::vec;
use core::cmp::Ordering;

use jvm::{Array, AsClassInstance, ClassInstanceRef, JavaChar, JavaValue, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::{RuntimeContext, classes::java::lang::Object};

// public final class java.util.Arrays
#[java_class(name = "java/util/Arrays", extends = "java/lang/Object", public, final)]
pub struct Arrays;

#[java_methods]
impl Arrays {
    #[method(constructor, private)]
    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await
    }

    async fn checked_range<T>(jvm: &Jvm, array: &ClassInstanceRef<Array<T>>, from_index: i32, to_index: i32) -> Result<(usize, usize)> {
        if array.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "array").await);
        }
        if from_index > to_index {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "fromIndex > toIndex").await);
        }
        let length = jvm.array_length(array.as_class_instance()).await?;
        if from_index < 0 || to_index < 0 || to_index as usize > length {
            return Err(jvm.exception("java/lang/ArrayIndexOutOfBoundsException", "array range").await);
        }
        Ok((from_index as usize, to_index as usize))
    }

    async fn sort_primitive<T, F>(jvm: &Jvm, mut array: ClassInstanceRef<Array<T>>, from_index: usize, to_index: usize, mut compare: F) -> Result<()>
    where
        T: From<JavaValue> + Into<JavaValue> + Send,
        F: FnMut(&T, &T) -> Ordering + Send,
    {
        let mut values = jvm.load_array::<T>(array.as_class_instance(), from_index, to_index - from_index).await?;
        values.sort_by(|left, right| compare(left, right));
        if !values.is_empty() {
            jvm.store_array(&mut array, from_index, values).await?;
        }
        Ok(())
    }

    fn float_order(left: &f32, right: &f32) -> Ordering {
        if left < right {
            Ordering::Less
        } else if left > right {
            Ordering::Greater
        } else {
            let left_bits = if left.is_nan() { 0x7fc0_0000 } else { left.to_bits() } as i32;
            let right_bits = if right.is_nan() { 0x7fc0_0000 } else { right.to_bits() } as i32;
            left_bits.cmp(&right_bits)
        }
    }

    fn double_order(left: &f64, right: &f64) -> Ordering {
        if left < right {
            Ordering::Less
        } else if left > right {
            Ordering::Greater
        } else {
            let left_bits = if left.is_nan() { 0x7ff8_0000_0000_0000 } else { left.to_bits() } as i64;
            let right_bits = if right.is_nan() { 0x7ff8_0000_0000_0000 } else { right.to_bits() } as i64;
            left_bits.cmp(&right_bits)
        }
    }

    async fn sort_primitive_array<T, F>(jvm: &Jvm, array: ClassInstanceRef<Array<T>>, compare: F) -> Result<()>
    where
        T: From<JavaValue> + Into<JavaValue> + Send,
        F: FnMut(&T, &T) -> Ordering + Send,
    {
        if array.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "array").await);
        }
        let length = jvm.array_length(array.as_class_instance()).await?;
        Self::sort_primitive(jvm, array, 0, length, compare).await
    }

    async fn sort_primitive_range<T, F>(jvm: &Jvm, array: ClassInstanceRef<Array<T>>, from_index: i32, to_index: i32, compare: F) -> Result<()>
    where
        T: From<JavaValue> + Into<JavaValue> + Send,
        F: FnMut(&T, &T) -> Ordering + Send,
    {
        let (from_index, to_index) = Self::checked_range(jvm, &array, from_index, to_index).await?;
        Self::sort_primitive(jvm, array, from_index, to_index, compare).await
    }

    #[method(name = "sort", public, static)]
    async fn sort_byte(jvm: &Jvm, _: &mut RuntimeContext, array: ClassInstanceRef<Array<i8>>) -> Result<()> {
        Self::sort_primitive_array(jvm, array, Ord::cmp).await
    }

    #[method(name = "sort", public, static)]
    async fn sort_byte_range(jvm: &Jvm, _: &mut RuntimeContext, array: ClassInstanceRef<Array<i8>>, from_index: i32, to_index: i32) -> Result<()> {
        Self::sort_primitive_range(jvm, array, from_index, to_index, Ord::cmp).await
    }

    #[method(name = "sort", public, static)]
    async fn sort_char(jvm: &Jvm, _: &mut RuntimeContext, array: ClassInstanceRef<Array<JavaChar>>) -> Result<()> {
        Self::sort_primitive_array(jvm, array, Ord::cmp).await
    }

    #[method(name = "sort", public, static)]
    async fn sort_char_range(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        array: ClassInstanceRef<Array<JavaChar>>,
        from_index: i32,
        to_index: i32,
    ) -> Result<()> {
        Self::sort_primitive_range(jvm, array, from_index, to_index, Ord::cmp).await
    }

    #[method(name = "sort", public, static)]
    async fn sort_short(jvm: &Jvm, _: &mut RuntimeContext, array: ClassInstanceRef<Array<i16>>) -> Result<()> {
        Self::sort_primitive_array(jvm, array, Ord::cmp).await
    }

    #[method(name = "sort", public, static)]
    async fn sort_short_range(jvm: &Jvm, _: &mut RuntimeContext, array: ClassInstanceRef<Array<i16>>, from_index: i32, to_index: i32) -> Result<()> {
        Self::sort_primitive_range(jvm, array, from_index, to_index, Ord::cmp).await
    }

    #[method(name = "sort", public, static)]
    async fn sort_int(jvm: &Jvm, _: &mut RuntimeContext, array: ClassInstanceRef<Array<i32>>) -> Result<()> {
        Self::sort_primitive_array(jvm, array, Ord::cmp).await
    }

    #[method(name = "sort", public, static)]
    async fn sort_int_range(jvm: &Jvm, _: &mut RuntimeContext, array: ClassInstanceRef<Array<i32>>, from_index: i32, to_index: i32) -> Result<()> {
        Self::sort_primitive_range(jvm, array, from_index, to_index, Ord::cmp).await
    }

    #[method(name = "sort", public, static)]
    async fn sort_long(jvm: &Jvm, _: &mut RuntimeContext, array: ClassInstanceRef<Array<i64>>) -> Result<()> {
        Self::sort_primitive_array(jvm, array, Ord::cmp).await
    }

    #[method(name = "sort", public, static)]
    async fn sort_long_range(jvm: &Jvm, _: &mut RuntimeContext, array: ClassInstanceRef<Array<i64>>, from_index: i32, to_index: i32) -> Result<()> {
        Self::sort_primitive_range(jvm, array, from_index, to_index, Ord::cmp).await
    }

    #[method(name = "sort", public, static)]
    async fn sort_float(jvm: &Jvm, _: &mut RuntimeContext, array: ClassInstanceRef<Array<f32>>) -> Result<()> {
        Self::sort_primitive_array(jvm, array, Self::float_order).await
    }

    #[method(name = "sort", public, static)]
    async fn sort_float_range(jvm: &Jvm, _: &mut RuntimeContext, array: ClassInstanceRef<Array<f32>>, from_index: i32, to_index: i32) -> Result<()> {
        Self::sort_primitive_range(jvm, array, from_index, to_index, Self::float_order).await
    }

    #[method(name = "sort", public, static)]
    async fn sort_double(jvm: &Jvm, _: &mut RuntimeContext, array: ClassInstanceRef<Array<f64>>) -> Result<()> {
        Self::sort_primitive_array(jvm, array, Self::double_order).await
    }

    #[method(name = "sort", public, static)]
    async fn sort_double_range(jvm: &Jvm, _: &mut RuntimeContext, array: ClassInstanceRef<Array<f64>>, from_index: i32, to_index: i32) -> Result<()> {
        Self::sort_primitive_range(jvm, array, from_index, to_index, Self::double_order).await
    }

    async fn compare_objects(
        jvm: &Jvm,
        comparator: &ClassInstanceRef<Object>,
        left: &ClassInstanceRef<Object>,
        right: &ClassInstanceRef<Object>,
    ) -> Result<i32> {
        if comparator.is_null() {
            if left.is_null() {
                return Err(jvm.exception("java/lang/NullPointerException", "array element").await);
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

    async fn sort_objects(
        jvm: &Jvm,
        mut array: ClassInstanceRef<Array<Object>>,
        from_index: usize,
        to_index: usize,
        comparator: ClassInstanceRef<Object>,
    ) -> Result<()> {
        let mut source = jvm
            .load_array::<ClassInstanceRef<Object>>(&array, from_index, to_index - from_index)
            .await?;
        let length = source.len();
        let mut target = source.clone();
        let mut width = 1usize;

        while width < length {
            let mut start = 0usize;
            while start < length {
                let middle = core::cmp::min(start + width, length);
                let end = core::cmp::min(start + width.saturating_mul(2), length);
                let mut left = start;
                let mut right = middle;
                let mut output = start;

                while left < middle && right < end {
                    if Self::compare_objects(jvm, &comparator, &source[left], &source[right]).await? <= 0 {
                        target[output] = source[left].clone();
                        left += 1;
                    } else {
                        target[output] = source[right].clone();
                        right += 1;
                    }
                    output += 1;
                }
                while left < middle {
                    target[output] = source[left].clone();
                    left += 1;
                    output += 1;
                }
                while right < end {
                    target[output] = source[right].clone();
                    right += 1;
                    output += 1;
                }
                start = end;
            }
            core::mem::swap(&mut source, &mut target);
            width = width.saturating_mul(2);
        }

        if !source.is_empty() {
            jvm.store_array(&mut array, from_index, source).await?;
        }
        Ok(())
    }

    #[method(name = "sort", public, static)]
    async fn sort_object(jvm: &Jvm, _: &mut RuntimeContext, array: ClassInstanceRef<Array<Object>>) -> Result<()> {
        if array.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "array").await);
        }
        let length = jvm.array_length(&array).await?;
        Self::sort_objects(jvm, array, 0, length, None.into()).await
    }

    #[method(name = "sort", public, static)]
    async fn sort_object_range(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        array: ClassInstanceRef<Array<Object>>,
        from_index: i32,
        to_index: i32,
    ) -> Result<()> {
        let (from_index, to_index) = Self::checked_range(jvm, &array, from_index, to_index).await?;
        Self::sort_objects(jvm, array, from_index, to_index, None.into()).await
    }

    #[method(name = "sort", descriptor = "([Ljava/lang/Object;Ljava/util/Comparator;)V", public, static)]
    async fn sort_object_comparator(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        array: ClassInstanceRef<Array<Object>>,
        comparator: ClassInstanceRef<Object>,
    ) -> Result<()> {
        if array.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "array").await);
        }
        let length = jvm.array_length(&array).await?;
        Self::sort_objects(jvm, array, 0, length, comparator).await
    }

    #[method(name = "sort", descriptor = "([Ljava/lang/Object;IILjava/util/Comparator;)V", public, static)]
    async fn sort_object_range_comparator(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        array: ClassInstanceRef<Array<Object>>,
        from_index: i32,
        to_index: i32,
        comparator: ClassInstanceRef<Object>,
    ) -> Result<()> {
        let (from_index, to_index) = Self::checked_range(jvm, &array, from_index, to_index).await?;
        Self::sort_objects(jvm, array, from_index, to_index, comparator).await
    }

    fn binary_search_primitive<T, F>(values: &[T], key: &T, mut compare: F) -> i32
    where
        F: FnMut(&T, &T) -> Ordering,
    {
        let mut low = 0usize;
        let mut high = values.len();
        while low < high {
            let middle = low + (high - low) / 2;
            match compare(&values[middle], key) {
                Ordering::Less => low = middle + 1,
                Ordering::Greater => high = middle,
                Ordering::Equal => return middle as i32,
            }
        }
        -(low as i32) - 1
    }

    async fn binary_search_primitive_array<T, F>(jvm: &Jvm, array: ClassInstanceRef<Array<T>>, key: T, compare: F) -> Result<i32>
    where
        T: From<JavaValue> + Send,
        F: FnMut(&T, &T) -> Ordering + Send,
    {
        if array.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "array").await);
        }
        let length = jvm.array_length(array.as_class_instance()).await?;
        let values = jvm.load_array::<T>(array.as_class_instance(), 0, length).await?;
        Ok(Self::binary_search_primitive(&values, &key, compare))
    }

    #[method(name = "binarySearch", public, static)]
    async fn binary_search_byte(jvm: &Jvm, _: &mut RuntimeContext, array: ClassInstanceRef<Array<i8>>, key: i8) -> Result<i32> {
        Self::binary_search_primitive_array(jvm, array, key, Ord::cmp).await
    }

    #[method(name = "binarySearch", public, static)]
    async fn binary_search_char(jvm: &Jvm, _: &mut RuntimeContext, array: ClassInstanceRef<Array<JavaChar>>, key: JavaChar) -> Result<i32> {
        Self::binary_search_primitive_array(jvm, array, key, Ord::cmp).await
    }

    #[method(name = "binarySearch", public, static)]
    async fn binary_search_short(jvm: &Jvm, _: &mut RuntimeContext, array: ClassInstanceRef<Array<i16>>, key: i16) -> Result<i32> {
        Self::binary_search_primitive_array(jvm, array, key, Ord::cmp).await
    }

    #[method(name = "binarySearch", public, static)]
    async fn binary_search_int(jvm: &Jvm, _: &mut RuntimeContext, array: ClassInstanceRef<Array<i32>>, key: i32) -> Result<i32> {
        Self::binary_search_primitive_array(jvm, array, key, Ord::cmp).await
    }

    #[method(name = "binarySearch", public, static)]
    async fn binary_search_long(jvm: &Jvm, _: &mut RuntimeContext, array: ClassInstanceRef<Array<i64>>, key: i64) -> Result<i32> {
        Self::binary_search_primitive_array(jvm, array, key, Ord::cmp).await
    }

    #[method(name = "binarySearch", public, static)]
    async fn binary_search_float(jvm: &Jvm, _: &mut RuntimeContext, array: ClassInstanceRef<Array<f32>>, key: f32) -> Result<i32> {
        Self::binary_search_primitive_array(jvm, array, key, Self::float_order).await
    }

    #[method(name = "binarySearch", public, static)]
    async fn binary_search_double(jvm: &Jvm, _: &mut RuntimeContext, array: ClassInstanceRef<Array<f64>>, key: f64) -> Result<i32> {
        Self::binary_search_primitive_array(jvm, array, key, Self::double_order).await
    }

    async fn binary_search_objects(
        jvm: &Jvm,
        array: ClassInstanceRef<Array<Object>>,
        key: ClassInstanceRef<Object>,
        comparator: ClassInstanceRef<Object>,
    ) -> Result<i32> {
        if array.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "array").await);
        }
        let length = jvm.array_length(&array).await?;
        let values = jvm.load_array::<ClassInstanceRef<Object>>(&array, 0, length).await?;
        let mut low = 0usize;
        let mut high = length;
        while low < high {
            let middle = low + (high - low) / 2;
            let comparison = Self::compare_objects(jvm, &comparator, &values[middle], &key).await?;
            if comparison < 0 {
                low = middle + 1;
            } else if comparison > 0 {
                high = middle;
            } else {
                return Ok(middle as i32);
            }
        }
        Ok(-(low as i32) - 1)
    }

    #[method(name = "binarySearch", public, static)]
    async fn binary_search_object(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        array: ClassInstanceRef<Array<Object>>,
        key: ClassInstanceRef<Object>,
    ) -> Result<i32> {
        Self::binary_search_objects(jvm, array, key, None.into()).await
    }

    #[method(
        name = "binarySearch",
        descriptor = "([Ljava/lang/Object;Ljava/lang/Object;Ljava/util/Comparator;)I",
        public,
        static
    )]
    async fn binary_search_object_comparator(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        array: ClassInstanceRef<Array<Object>>,
        key: ClassInstanceRef<Object>,
        comparator: ClassInstanceRef<Object>,
    ) -> Result<i32> {
        Self::binary_search_objects(jvm, array, key, comparator).await
    }

    async fn equals_primitive<T>(jvm: &Jvm, first: ClassInstanceRef<Array<T>>, second: ClassInstanceRef<Array<T>>) -> Result<bool>
    where
        T: From<JavaValue> + PartialEq + Send,
    {
        if first.is_null() || second.is_null() {
            return Ok(first.is_null() && second.is_null());
        }
        let first_length = jvm.array_length(first.as_class_instance()).await?;
        if first_length != jvm.array_length(second.as_class_instance()).await? {
            return Ok(false);
        }
        Ok(jvm.load_array::<T>(first.as_class_instance(), 0, first_length).await?
            == jvm.load_array::<T>(second.as_class_instance(), 0, first_length).await?)
    }

    #[method(name = "equals", public, static)]
    async fn equals_boolean(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        first: ClassInstanceRef<Array<bool>>,
        second: ClassInstanceRef<Array<bool>>,
    ) -> Result<bool> {
        Self::equals_primitive(jvm, first, second).await
    }

    #[method(name = "equals", public, static)]
    async fn equals_byte(jvm: &Jvm, _: &mut RuntimeContext, first: ClassInstanceRef<Array<i8>>, second: ClassInstanceRef<Array<i8>>) -> Result<bool> {
        Self::equals_primitive(jvm, first, second).await
    }

    #[method(name = "equals", public, static)]
    async fn equals_char(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        first: ClassInstanceRef<Array<JavaChar>>,
        second: ClassInstanceRef<Array<JavaChar>>,
    ) -> Result<bool> {
        Self::equals_primitive(jvm, first, second).await
    }

    #[method(name = "equals", public, static)]
    async fn equals_short(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        first: ClassInstanceRef<Array<i16>>,
        second: ClassInstanceRef<Array<i16>>,
    ) -> Result<bool> {
        Self::equals_primitive(jvm, first, second).await
    }

    #[method(name = "equals", public, static)]
    async fn equals_int(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        first: ClassInstanceRef<Array<i32>>,
        second: ClassInstanceRef<Array<i32>>,
    ) -> Result<bool> {
        Self::equals_primitive(jvm, first, second).await
    }

    #[method(name = "equals", public, static)]
    async fn equals_long(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        first: ClassInstanceRef<Array<i64>>,
        second: ClassInstanceRef<Array<i64>>,
    ) -> Result<bool> {
        Self::equals_primitive(jvm, first, second).await
    }

    #[method(name = "equals", public, static)]
    async fn equals_float(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        first: ClassInstanceRef<Array<f32>>,
        second: ClassInstanceRef<Array<f32>>,
    ) -> Result<bool> {
        if first.is_null() || second.is_null() {
            return Ok(first.is_null() && second.is_null());
        }
        let length = jvm.array_length(&first).await?;
        if length != jvm.array_length(&second).await? {
            return Ok(false);
        }
        let first_values = jvm.load_array::<f32>(&first, 0, length).await?;
        let second_values = jvm.load_array::<f32>(&second, 0, length).await?;
        Ok(first_values.iter().zip(second_values).all(|(left, right)| {
            let left_bits = if left.is_nan() { 0x7fc0_0000 } else { left.to_bits() };
            let right_bits = if right.is_nan() { 0x7fc0_0000 } else { right.to_bits() };
            left_bits == right_bits
        }))
    }

    #[method(name = "equals", public, static)]
    async fn equals_double(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        first: ClassInstanceRef<Array<f64>>,
        second: ClassInstanceRef<Array<f64>>,
    ) -> Result<bool> {
        if first.is_null() || second.is_null() {
            return Ok(first.is_null() && second.is_null());
        }
        let length = jvm.array_length(&first).await?;
        if length != jvm.array_length(&second).await? {
            return Ok(false);
        }
        let first_values = jvm.load_array::<f64>(&first, 0, length).await?;
        let second_values = jvm.load_array::<f64>(&second, 0, length).await?;
        Ok(first_values.iter().zip(second_values).all(|(left, right)| {
            let left_bits = if left.is_nan() { 0x7ff8_0000_0000_0000 } else { left.to_bits() };
            let right_bits = if right.is_nan() { 0x7ff8_0000_0000_0000 } else { right.to_bits() };
            left_bits == right_bits
        }))
    }

    #[method(name = "equals", public, static)]
    async fn equals_object(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        first: ClassInstanceRef<Array<Object>>,
        second: ClassInstanceRef<Array<Object>>,
    ) -> Result<bool> {
        if first.is_null() || second.is_null() {
            return Ok(first.is_null() && second.is_null());
        }
        let length = jvm.array_length(&first).await?;
        if length != jvm.array_length(&second).await? {
            return Ok(false);
        }
        let first_values = jvm.load_array::<ClassInstanceRef<Object>>(&first, 0, length).await?;
        let second_values = jvm.load_array::<ClassInstanceRef<Object>>(&second, 0, length).await?;
        for (left, right) in first_values.into_iter().zip(second_values) {
            if left.is_null() {
                if !right.is_null() {
                    return Ok(false);
                }
            } else if !jvm
                .invoke_virtual::<_, bool>(&left, "java/lang/Object", "equals", "(Ljava/lang/Object;)Z", (right,))
                .await?
            {
                return Ok(false);
            }
        }
        Ok(true)
    }

    async fn fill_primitive_array<T>(jvm: &Jvm, mut array: ClassInstanceRef<Array<T>>, value: T) -> Result<()>
    where
        T: Into<JavaValue> + Clone + Send,
    {
        if array.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "array").await);
        }
        let length = jvm.array_length(array.as_class_instance()).await?;
        if length != 0 {
            jvm.store_array(&mut array, 0, vec![value; length]).await?;
        }
        Ok(())
    }

    async fn fill_primitive_range<T>(jvm: &Jvm, mut array: ClassInstanceRef<Array<T>>, from_index: i32, to_index: i32, value: T) -> Result<()>
    where
        T: Into<JavaValue> + Clone + Send,
    {
        let (from_index, to_index) = Self::checked_range(jvm, &array, from_index, to_index).await?;
        if from_index != to_index {
            jvm.store_array(&mut array, from_index, vec![value; to_index - from_index]).await?;
        }
        Ok(())
    }

    #[method(name = "fill", public, static)]
    async fn fill_boolean(jvm: &Jvm, _: &mut RuntimeContext, array: ClassInstanceRef<Array<bool>>, value: bool) -> Result<()> {
        Self::fill_primitive_array(jvm, array, value).await
    }

    #[method(name = "fill", public, static)]
    async fn fill_boolean_range(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        array: ClassInstanceRef<Array<bool>>,
        from_index: i32,
        to_index: i32,
        value: bool,
    ) -> Result<()> {
        Self::fill_primitive_range(jvm, array, from_index, to_index, value).await
    }

    #[method(name = "fill", public, static)]
    async fn fill_byte(jvm: &Jvm, _: &mut RuntimeContext, array: ClassInstanceRef<Array<i8>>, value: i8) -> Result<()> {
        Self::fill_primitive_array(jvm, array, value).await
    }

    #[method(name = "fill", public, static)]
    async fn fill_byte_range(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        array: ClassInstanceRef<Array<i8>>,
        from_index: i32,
        to_index: i32,
        value: i8,
    ) -> Result<()> {
        Self::fill_primitive_range(jvm, array, from_index, to_index, value).await
    }

    #[method(name = "fill", public, static)]
    async fn fill_char(jvm: &Jvm, _: &mut RuntimeContext, array: ClassInstanceRef<Array<JavaChar>>, value: JavaChar) -> Result<()> {
        Self::fill_primitive_array(jvm, array, value).await
    }

    #[method(name = "fill", public, static)]
    async fn fill_char_range(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        array: ClassInstanceRef<Array<JavaChar>>,
        from_index: i32,
        to_index: i32,
        value: JavaChar,
    ) -> Result<()> {
        Self::fill_primitive_range(jvm, array, from_index, to_index, value).await
    }

    #[method(name = "fill", public, static)]
    async fn fill_short(jvm: &Jvm, _: &mut RuntimeContext, array: ClassInstanceRef<Array<i16>>, value: i16) -> Result<()> {
        Self::fill_primitive_array(jvm, array, value).await
    }

    #[method(name = "fill", public, static)]
    async fn fill_short_range(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        array: ClassInstanceRef<Array<i16>>,
        from_index: i32,
        to_index: i32,
        value: i16,
    ) -> Result<()> {
        Self::fill_primitive_range(jvm, array, from_index, to_index, value).await
    }

    #[method(name = "fill", public, static)]
    async fn fill_int(jvm: &Jvm, _: &mut RuntimeContext, array: ClassInstanceRef<Array<i32>>, value: i32) -> Result<()> {
        Self::fill_primitive_array(jvm, array, value).await
    }

    #[method(name = "fill", public, static)]
    async fn fill_int_range(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        array: ClassInstanceRef<Array<i32>>,
        from_index: i32,
        to_index: i32,
        value: i32,
    ) -> Result<()> {
        Self::fill_primitive_range(jvm, array, from_index, to_index, value).await
    }

    #[method(name = "fill", public, static)]
    async fn fill_long(jvm: &Jvm, _: &mut RuntimeContext, array: ClassInstanceRef<Array<i64>>, value: i64) -> Result<()> {
        Self::fill_primitive_array(jvm, array, value).await
    }

    #[method(name = "fill", public, static)]
    async fn fill_long_range(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        array: ClassInstanceRef<Array<i64>>,
        from_index: i32,
        to_index: i32,
        value: i64,
    ) -> Result<()> {
        Self::fill_primitive_range(jvm, array, from_index, to_index, value).await
    }

    #[method(name = "fill", public, static)]
    async fn fill_float(jvm: &Jvm, _: &mut RuntimeContext, array: ClassInstanceRef<Array<f32>>, value: f32) -> Result<()> {
        Self::fill_primitive_array(jvm, array, value).await
    }

    #[method(name = "fill", public, static)]
    async fn fill_float_range(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        array: ClassInstanceRef<Array<f32>>,
        from_index: i32,
        to_index: i32,
        value: f32,
    ) -> Result<()> {
        Self::fill_primitive_range(jvm, array, from_index, to_index, value).await
    }

    #[method(name = "fill", public, static)]
    async fn fill_double(jvm: &Jvm, _: &mut RuntimeContext, array: ClassInstanceRef<Array<f64>>, value: f64) -> Result<()> {
        Self::fill_primitive_array(jvm, array, value).await
    }

    #[method(name = "fill", public, static)]
    async fn fill_double_range(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        array: ClassInstanceRef<Array<f64>>,
        from_index: i32,
        to_index: i32,
        value: f64,
    ) -> Result<()> {
        Self::fill_primitive_range(jvm, array, from_index, to_index, value).await
    }

    #[method(name = "fill", public, static)]
    async fn fill_object(jvm: &Jvm, _: &mut RuntimeContext, array: ClassInstanceRef<Array<Object>>, value: ClassInstanceRef<Object>) -> Result<()> {
        if array.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "array").await);
        }
        let length = jvm.array_length(&array).await?;
        Self::fill_objects(jvm, array, 0, length, value).await
    }

    #[method(name = "fill", public, static)]
    async fn fill_object_range(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        array: ClassInstanceRef<Array<Object>>,
        from_index: i32,
        to_index: i32,
        value: ClassInstanceRef<Object>,
    ) -> Result<()> {
        let (from_index, to_index) = Self::checked_range(jvm, &array, from_index, to_index).await?;
        Self::fill_objects(jvm, array, from_index, to_index, value).await
    }

    async fn fill_objects(
        jvm: &Jvm,
        mut array: ClassInstanceRef<Array<Object>>,
        from_index: usize,
        to_index: usize,
        value: ClassInstanceRef<Object>,
    ) -> Result<()> {
        for index in from_index..to_index {
            if !value.is_null() && !jvm.array_store_allowed(array.as_ref(), value.as_ref()) {
                return Err(jvm.exception("java/lang/ArrayStoreException", &value.class_definition().name()).await);
            }
            jvm.store_array(&mut array, index, core::iter::once(value.clone())).await?;
        }
        Ok(())
    }

    #[method(descriptor = "([Ljava/lang/Object;)Ljava/util/List;", public, static)]
    async fn as_list(jvm: &Jvm, _: &mut RuntimeContext, array: ClassInstanceRef<Array<Object>>) -> Result<ClassInstanceRef<Object>> {
        if array.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "array").await);
        }
        Ok(jvm
            .new_class("java/util/Arrays$ArrayList", "([Ljava/lang/Object;)V", (array,))
            .await?
            .into())
    }
}
