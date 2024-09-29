use alloc::{
    boxed::Box,
    vec::{self, Vec},
};
use core::{array, iter};

use crate::value::JavaValue;

pub trait InvokeArg: Sync + Send {
    type IntoIter: Iterator<Item = JavaValue>;

    fn into_arg(self) -> Box<[JavaValue]>;
    fn into_iter(self) -> Self::IntoIter;
}

impl InvokeArg for Vec<JavaValue> {
    type IntoIter = vec::IntoIter<JavaValue>;
    fn into_arg(self) -> Box<[JavaValue]> {
        self.into_boxed_slice()
    }

    fn into_iter(self) -> Self::IntoIter {
        iter::IntoIterator::into_iter(self)
    }
}

impl<const N: usize> InvokeArg for [JavaValue; N] {
    type IntoIter = array::IntoIter<JavaValue, N>;

    fn into_arg(self) -> Box<[JavaValue]> {
        self.into()
    }

    fn into_iter(self) -> Self::IntoIter {
        iter::IntoIterator::into_iter(self)
    }
}

impl InvokeArg for () {
    type IntoIter = array::IntoIter<JavaValue, 0>;

    fn into_arg(self) -> Box<[JavaValue]> {
        Box::new([])
    }

    fn into_iter(self) -> Self::IntoIter {
        iter::IntoIterator::into_iter([])
    }
}

impl<T1> InvokeArg for (T1,)
where
    T1: Into<JavaValue> + Sync + Send,
{
    type IntoIter = array::IntoIter<JavaValue, 1>;

    fn into_arg(self) -> Box<[JavaValue]> {
        Box::new([self.0.into()])
    }

    fn into_iter(self) -> Self::IntoIter {
        iter::IntoIterator::into_iter([self.0.into()])
    }
}

impl<T1, T2> InvokeArg for (T1, T2)
where
    T1: Into<JavaValue> + Sync + Send,
    T2: Into<JavaValue> + Sync + Send,
{
    type IntoIter = array::IntoIter<JavaValue, 2>;

    fn into_arg(self) -> Box<[JavaValue]> {
        Box::new([self.0.into(), self.1.into()])
    }

    fn into_iter(self) -> Self::IntoIter {
        iter::IntoIterator::into_iter([self.0.into(), self.1.into()])
    }
}

impl<T1, T2, T3> InvokeArg for (T1, T2, T3)
where
    T1: Into<JavaValue> + Sync + Send,
    T2: Into<JavaValue> + Sync + Send,
    T3: Into<JavaValue> + Sync + Send,
{
    type IntoIter = array::IntoIter<JavaValue, 3>;

    fn into_arg(self) -> Box<[JavaValue]> {
        Box::new([self.0.into(), self.1.into(), self.2.into()])
    }

    fn into_iter(self) -> Self::IntoIter {
        iter::IntoIterator::into_iter([self.0.into(), self.1.into(), self.2.into()])
    }
}

impl<T1, T2, T3, T4> InvokeArg for (T1, T2, T3, T4)
where
    T1: Into<JavaValue> + Sync + Send,
    T2: Into<JavaValue> + Sync + Send,
    T3: Into<JavaValue> + Sync + Send,
    T4: Into<JavaValue> + Sync + Send,
{
    type IntoIter = array::IntoIter<JavaValue, 4>;

    fn into_arg(self) -> Box<[JavaValue]> {
        Box::new([self.0.into(), self.1.into(), self.2.into(), self.3.into()])
    }

    fn into_iter(self) -> Self::IntoIter {
        iter::IntoIterator::into_iter([self.0.into(), self.1.into(), self.2.into(), self.3.into()])
    }
}

impl<T1, T2, T3, T4, T5> InvokeArg for (T1, T2, T3, T4, T5)
where
    T1: Into<JavaValue> + Sync + Send,
    T2: Into<JavaValue> + Sync + Send,
    T3: Into<JavaValue> + Sync + Send,
    T4: Into<JavaValue> + Sync + Send,
    T5: Into<JavaValue> + Sync + Send,
{
    type IntoIter = array::IntoIter<JavaValue, 5>;

    fn into_arg(self) -> Box<[JavaValue]> {
        Box::new([self.0.into(), self.1.into(), self.2.into(), self.3.into(), self.4.into()])
    }

    fn into_iter(self) -> Self::IntoIter {
        iter::IntoIterator::into_iter([self.0.into(), self.1.into(), self.2.into(), self.3.into(), self.4.into()])
    }
}

impl<T1, T2, T3, T4, T5, T6> InvokeArg for (T1, T2, T3, T4, T5, T6)
where
    T1: Into<JavaValue> + Sync + Send,
    T2: Into<JavaValue> + Sync + Send,
    T3: Into<JavaValue> + Sync + Send,
    T4: Into<JavaValue> + Sync + Send,
    T5: Into<JavaValue> + Sync + Send,
    T6: Into<JavaValue> + Sync + Send,
{
    type IntoIter = array::IntoIter<JavaValue, 6>;

    fn into_arg(self) -> Box<[JavaValue]> {
        Box::new([self.0.into(), self.1.into(), self.2.into(), self.3.into(), self.4.into(), self.5.into()])
    }

    fn into_iter(self) -> Self::IntoIter {
        iter::IntoIterator::into_iter([self.0.into(), self.1.into(), self.2.into(), self.3.into(), self.4.into(), self.5.into()])
    }
}

impl<T1, T2, T3, T4, T5, T6, T7> InvokeArg for (T1, T2, T3, T4, T5, T6, T7)
where
    T1: Into<JavaValue> + Sync + Send,
    T2: Into<JavaValue> + Sync + Send,
    T3: Into<JavaValue> + Sync + Send,
    T4: Into<JavaValue> + Sync + Send,
    T5: Into<JavaValue> + Sync + Send,
    T6: Into<JavaValue> + Sync + Send,
    T7: Into<JavaValue> + Sync + Send,
{
    type IntoIter = array::IntoIter<JavaValue, 7>;

    fn into_arg(self) -> Box<[JavaValue]> {
        Box::new([
            self.0.into(),
            self.1.into(),
            self.2.into(),
            self.3.into(),
            self.4.into(),
            self.5.into(),
            self.6.into(),
        ])
    }

    fn into_iter(self) -> Self::IntoIter {
        iter::IntoIterator::into_iter([
            self.0.into(),
            self.1.into(),
            self.2.into(),
            self.3.into(),
            self.4.into(),
            self.5.into(),
            self.6.into(),
        ])
    }
}

impl<T1, T2, T3, T4, T5, T6, T7, T8> InvokeArg for (T1, T2, T3, T4, T5, T6, T7, T8)
where
    T1: Into<JavaValue> + Sync + Send,
    T2: Into<JavaValue> + Sync + Send,
    T3: Into<JavaValue> + Sync + Send,
    T4: Into<JavaValue> + Sync + Send,
    T5: Into<JavaValue> + Sync + Send,
    T6: Into<JavaValue> + Sync + Send,
    T7: Into<JavaValue> + Sync + Send,
    T8: Into<JavaValue> + Sync + Send,
{
    type IntoIter = array::IntoIter<JavaValue, 8>;

    fn into_arg(self) -> Box<[JavaValue]> {
        Box::new([
            self.0.into(),
            self.1.into(),
            self.2.into(),
            self.3.into(),
            self.4.into(),
            self.5.into(),
            self.6.into(),
            self.7.into(),
        ])
    }

    fn into_iter(self) -> Self::IntoIter {
        iter::IntoIterator::into_iter([
            self.0.into(),
            self.1.into(),
            self.2.into(),
            self.3.into(),
            self.4.into(),
            self.5.into(),
            self.6.into(),
            self.7.into(),
        ])
    }
}
