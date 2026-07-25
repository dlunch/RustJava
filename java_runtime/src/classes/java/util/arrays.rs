use alloc::vec;
use core::cmp::Ordering;

use java_class_proto::JavaMethodProto;
use java_constants::{ClassAccessFlags, MethodAccessFlags};
use jvm::{Array, ClassInstanceRef, JavaChar, JavaValue, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::Object};

macro_rules! primitive_sort_methods {
    ($whole:ident, $range:ident, $ty:ty, $compare:expr) => {
        async fn $whole(jvm: &Jvm, _: &mut RuntimeContext, array: ClassInstanceRef<Array<$ty>>) -> Result<()> {
            if array.is_null() {
                return Err(jvm.exception("java/lang/NullPointerException", "array").await);
            }
            let length = jvm.array_length(&array).await?;
            Self::sort_primitive(jvm, array, 0, length, $compare).await
        }

        async fn $range(jvm: &Jvm, _: &mut RuntimeContext, array: ClassInstanceRef<Array<$ty>>, from_index: i32, to_index: i32) -> Result<()> {
            let (from_index, to_index) = Self::checked_range(jvm, &array, from_index, to_index).await?;
            Self::sort_primitive(jvm, array, from_index, to_index, $compare).await
        }
    };
}

macro_rules! primitive_binary_search_method {
    ($name:ident, $ty:ty, $compare:expr) => {
        async fn $name(jvm: &Jvm, _: &mut RuntimeContext, array: ClassInstanceRef<Array<$ty>>, key: $ty) -> Result<i32> {
            if array.is_null() {
                return Err(jvm.exception("java/lang/NullPointerException", "array").await);
            }
            let length = jvm.array_length(&array).await?;
            let values = jvm.load_array::<$ty>(&array, 0, length).await?;
            Ok(Self::binary_search_primitive(&values, &key, $compare))
        }
    };
}

macro_rules! primitive_equals_method {
    ($name:ident, $ty:ty) => {
        async fn $name(jvm: &Jvm, _: &mut RuntimeContext, first: ClassInstanceRef<Array<$ty>>, second: ClassInstanceRef<Array<$ty>>) -> Result<bool> {
            if first.is_null() || second.is_null() {
                return Ok(first.is_null() && second.is_null());
            }
            let first_length = jvm.array_length(&first).await?;
            if first_length != jvm.array_length(&second).await? {
                return Ok(false);
            }
            Ok(jvm.load_array::<$ty>(&first, 0, first_length).await? == jvm.load_array::<$ty>(&second, 0, first_length).await?)
        }
    };
}

macro_rules! primitive_fill_methods {
    ($whole:ident, $range:ident, $ty:ty) => {
        async fn $whole(jvm: &Jvm, _: &mut RuntimeContext, mut array: ClassInstanceRef<Array<$ty>>, value: $ty) -> Result<()> {
            if array.is_null() {
                return Err(jvm.exception("java/lang/NullPointerException", "array").await);
            }
            let length = jvm.array_length(&array).await?;
            if length != 0 {
                jvm.store_array(&mut array, 0, vec![value; length]).await?;
            }
            Ok(())
        }

        async fn $range(
            jvm: &Jvm,
            _: &mut RuntimeContext,
            mut array: ClassInstanceRef<Array<$ty>>,
            from_index: i32,
            to_index: i32,
            value: $ty,
        ) -> Result<()> {
            let (from_index, to_index) = Self::checked_range(jvm, &array, from_index, to_index).await?;
            if from_index != to_index {
                jvm.store_array(&mut array, from_index, vec![value; to_index - from_index]).await?;
            }
            Ok(())
        }
    };
}

// public final class java.util.Arrays
pub struct Arrays;

impl Arrays {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/Arrays",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, MethodAccessFlags::PRIVATE),
                JavaMethodProto::new("sort", "([B)V", Self::sort_byte, MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC),
                JavaMethodProto::new(
                    "sort",
                    "([BII)V",
                    Self::sort_byte_range,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new("sort", "([C)V", Self::sort_char, MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC),
                JavaMethodProto::new(
                    "sort",
                    "([CII)V",
                    Self::sort_char_range,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new("sort", "([S)V", Self::sort_short, MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC),
                JavaMethodProto::new(
                    "sort",
                    "([SII)V",
                    Self::sort_short_range,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new("sort", "([I)V", Self::sort_int, MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC),
                JavaMethodProto::new(
                    "sort",
                    "([III)V",
                    Self::sort_int_range,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new("sort", "([J)V", Self::sort_long, MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC),
                JavaMethodProto::new(
                    "sort",
                    "([JII)V",
                    Self::sort_long_range,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new("sort", "([F)V", Self::sort_float, MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC),
                JavaMethodProto::new(
                    "sort",
                    "([FII)V",
                    Self::sort_float_range,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new("sort", "([D)V", Self::sort_double, MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC),
                JavaMethodProto::new(
                    "sort",
                    "([DII)V",
                    Self::sort_double_range,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "sort",
                    "([Ljava/lang/Object;)V",
                    Self::sort_object,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "sort",
                    "([Ljava/lang/Object;II)V",
                    Self::sort_object_range,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "sort",
                    "([Ljava/lang/Object;Ljava/util/Comparator;)V",
                    Self::sort_object_comparator,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "sort",
                    "([Ljava/lang/Object;IILjava/util/Comparator;)V",
                    Self::sort_object_range_comparator,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "binarySearch",
                    "([BB)I",
                    Self::binary_search_byte,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "binarySearch",
                    "([CC)I",
                    Self::binary_search_char,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "binarySearch",
                    "([SS)I",
                    Self::binary_search_short,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "binarySearch",
                    "([II)I",
                    Self::binary_search_int,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "binarySearch",
                    "([JJ)I",
                    Self::binary_search_long,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "binarySearch",
                    "([FF)I",
                    Self::binary_search_float,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "binarySearch",
                    "([DD)I",
                    Self::binary_search_double,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "binarySearch",
                    "([Ljava/lang/Object;Ljava/lang/Object;)I",
                    Self::binary_search_object,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "binarySearch",
                    "([Ljava/lang/Object;Ljava/lang/Object;Ljava/util/Comparator;)I",
                    Self::binary_search_object_comparator,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "equals",
                    "([Z[Z)Z",
                    Self::equals_boolean,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "equals",
                    "([B[B)Z",
                    Self::equals_byte,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "equals",
                    "([C[C)Z",
                    Self::equals_char,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "equals",
                    "([S[S)Z",
                    Self::equals_short,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "equals",
                    "([I[I)Z",
                    Self::equals_int,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "equals",
                    "([J[J)Z",
                    Self::equals_long,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "equals",
                    "([F[F)Z",
                    Self::equals_float,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "equals",
                    "([D[D)Z",
                    Self::equals_double,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "equals",
                    "([Ljava/lang/Object;[Ljava/lang/Object;)Z",
                    Self::equals_object,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "fill",
                    "([ZZ)V",
                    Self::fill_boolean,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "fill",
                    "([ZIIZ)V",
                    Self::fill_boolean_range,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new("fill", "([BB)V", Self::fill_byte, MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC),
                JavaMethodProto::new(
                    "fill",
                    "([BIIB)V",
                    Self::fill_byte_range,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new("fill", "([CC)V", Self::fill_char, MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC),
                JavaMethodProto::new(
                    "fill",
                    "([CIIC)V",
                    Self::fill_char_range,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new("fill", "([SS)V", Self::fill_short, MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC),
                JavaMethodProto::new(
                    "fill",
                    "([SIIS)V",
                    Self::fill_short_range,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new("fill", "([II)V", Self::fill_int, MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC),
                JavaMethodProto::new(
                    "fill",
                    "([IIII)V",
                    Self::fill_int_range,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new("fill", "([JJ)V", Self::fill_long, MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC),
                JavaMethodProto::new(
                    "fill",
                    "([JIIJ)V",
                    Self::fill_long_range,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new("fill", "([FF)V", Self::fill_float, MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC),
                JavaMethodProto::new(
                    "fill",
                    "([FIIF)V",
                    Self::fill_float_range,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new("fill", "([DD)V", Self::fill_double, MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC),
                JavaMethodProto::new(
                    "fill",
                    "([DIID)V",
                    Self::fill_double_range,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "fill",
                    "([Ljava/lang/Object;Ljava/lang/Object;)V",
                    Self::fill_object,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "fill",
                    "([Ljava/lang/Object;IILjava/lang/Object;)V",
                    Self::fill_object_range,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "asList",
                    "([Ljava/lang/Object;)Ljava/util/List;",
                    Self::as_list,
                    MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
                ),
            ],
            fields: vec![],
            access_flags: ClassAccessFlags::PUBLIC | ClassAccessFlags::FINAL,
        }
    }

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
        let length = jvm.array_length(array).await?;
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
        let mut values = jvm.load_array::<T>(&array, from_index, to_index - from_index).await?;
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

    primitive_sort_methods!(sort_byte, sort_byte_range, i8, Ord::cmp);
    primitive_sort_methods!(sort_char, sort_char_range, JavaChar, Ord::cmp);
    primitive_sort_methods!(sort_short, sort_short_range, i16, Ord::cmp);
    primitive_sort_methods!(sort_int, sort_int_range, i32, Ord::cmp);
    primitive_sort_methods!(sort_long, sort_long_range, i64, Ord::cmp);
    primitive_sort_methods!(sort_float, sort_float_range, f32, Self::float_order);
    primitive_sort_methods!(sort_double, sort_double_range, f64, Self::double_order);

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
            jvm.invoke_virtual(left, "compareTo", "(Ljava/lang/Object;)I", (right.clone(),)).await
        } else {
            jvm.invoke_virtual(
                comparator,
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

    async fn sort_object(jvm: &Jvm, _: &mut RuntimeContext, array: ClassInstanceRef<Array<Object>>) -> Result<()> {
        if array.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "array").await);
        }
        let length = jvm.array_length(&array).await?;
        Self::sort_objects(jvm, array, 0, length, None.into()).await
    }

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

    primitive_binary_search_method!(binary_search_byte, i8, Ord::cmp);
    primitive_binary_search_method!(binary_search_char, JavaChar, Ord::cmp);
    primitive_binary_search_method!(binary_search_short, i16, Ord::cmp);
    primitive_binary_search_method!(binary_search_int, i32, Ord::cmp);
    primitive_binary_search_method!(binary_search_long, i64, Ord::cmp);
    primitive_binary_search_method!(binary_search_float, f32, Self::float_order);
    primitive_binary_search_method!(binary_search_double, f64, Self::double_order);

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

    async fn binary_search_object(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        array: ClassInstanceRef<Array<Object>>,
        key: ClassInstanceRef<Object>,
    ) -> Result<i32> {
        Self::binary_search_objects(jvm, array, key, None.into()).await
    }

    async fn binary_search_object_comparator(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        array: ClassInstanceRef<Array<Object>>,
        key: ClassInstanceRef<Object>,
        comparator: ClassInstanceRef<Object>,
    ) -> Result<i32> {
        Self::binary_search_objects(jvm, array, key, comparator).await
    }

    primitive_equals_method!(equals_boolean, bool);
    primitive_equals_method!(equals_byte, i8);
    primitive_equals_method!(equals_char, JavaChar);
    primitive_equals_method!(equals_short, i16);
    primitive_equals_method!(equals_int, i32);
    primitive_equals_method!(equals_long, i64);

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
            } else if !jvm.invoke_virtual::<_, bool>(&left, "equals", "(Ljava/lang/Object;)Z", (right,)).await? {
                return Ok(false);
            }
        }
        Ok(true)
    }

    primitive_fill_methods!(fill_boolean, fill_boolean_range, bool);
    primitive_fill_methods!(fill_byte, fill_byte_range, i8);
    primitive_fill_methods!(fill_char, fill_char_range, JavaChar);
    primitive_fill_methods!(fill_short, fill_short_range, i16);
    primitive_fill_methods!(fill_int, fill_int_range, i32);
    primitive_fill_methods!(fill_long, fill_long_range, i64);
    primitive_fill_methods!(fill_float, fill_float_range, f32);
    primitive_fill_methods!(fill_double, fill_double_range, f64);

    async fn fill_object(jvm: &Jvm, _: &mut RuntimeContext, array: ClassInstanceRef<Array<Object>>, value: ClassInstanceRef<Object>) -> Result<()> {
        if array.is_null() {
            return Err(jvm.exception("java/lang/NullPointerException", "array").await);
        }
        let length = jvm.array_length(&array).await?;
        Self::fill_objects(jvm, array, 0, length, value).await
    }

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
