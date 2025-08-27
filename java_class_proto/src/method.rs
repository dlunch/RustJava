use alloc::boxed::Box;
use core::{future::Future, marker::PhantomData};

use jvm::{ClassInstanceRef, JavaChar, JavaValue, Jvm};

macro_rules! __impl_fn_helper {
    ($($arg: ident),*) => {
        impl<'a, C, E, R, F, Fut, $($arg),*> FnHelper<'a, C, E, R, ($($arg,)*)> for F
        where
            F: Fn(&'a Jvm, &'a mut C, $($arg),*) -> Fut,
            C: ?Sized + 'a + Send,
            Fut: Future<Output = Result<R, E>> + 'a + Send,
            $($arg: TypeConverter<$arg> + 'a),*
        {
            type Output = Fut;
            #[allow(unused_assignments, non_snake_case, unused_mut, unused_variables)]
            fn do_call(&self, jvm: &'a Jvm, context: &'a mut C, args: Box<[JavaValue]>) -> Fut {
                let mut args = alloc::vec::Vec::from(args).into_iter();
                $(
                    let $arg = $arg::to_rust(&jvm, args.next().unwrap());
                )*
                self(jvm, context, $($arg),*)
            }
        }
    };
}

#[macro_export]
macro_rules! __impl_method_body {
    ($($arg: ident),*) => {
        #[async_trait::async_trait]
        impl<F, C, R, E, $($arg),*> MethodBody<E, C> for MethodHolder<F, R, ($($arg,)*)>
        where
            F: for<'a> FnHelper<'a, C, E, R, ($($arg,)*)> + Sync + Send,
            C: ?Sized + Send,
            R: TypeConverter<R> + Sync + Send,
            $($arg: Sync + Send),*
        {
            async fn call(&self, jvm: &Jvm, context: &mut C, args: Box<[JavaValue]>) -> Result<JavaValue, E> {
                let result = self.0.do_call(jvm, context, args).await?;

                Ok(R::from_rust(&jvm, result))
            }
        }
    };
}

macro_rules! __impl_method_impl {
    ($($arg: ident),*) => {
        impl<F, C, R, E, $($arg),*> MethodImpl<F, C, R, E, ($($arg,)*)> for F
        where
            F: for<'a> FnHelper<'a, C, E, R, ($($arg,)*)> + 'static + Sync + Send,
            C: ?Sized + Send,
            R: TypeConverter<R> + 'static + Sync + Send,
            $($arg: 'static + Sync + Send),*
        {
            fn into_body(self) -> Box<dyn MethodBody<E, C>> {
                Box::new(MethodHolder(self, PhantomData))
            }
        }
    };
}

macro_rules! __generate {
    ($($arg: ident),*) => {
        __impl_fn_helper!($($arg),*);
        __impl_method_body!($($arg),*);
        __impl_method_impl!($($arg),*);
    };
}

#[async_trait::async_trait]
pub trait MethodBody<E, C>: Sync + Send
where
    C: ?Sized + Send,
{
    async fn call(&self, jvm: &Jvm, context: &mut C, args: Box<[JavaValue]>) -> Result<JavaValue, E>;
}

trait FnHelper<'a, C, E, R, P>
where
    C: ?Sized + 'a + Send,
{
    type Output: Future<Output = Result<R, E>> + 'a + Send;
    fn do_call(&self, jvm: &'a Jvm, context: &'a mut C, args: Box<[JavaValue]>) -> Self::Output;
}

struct MethodHolder<F, R, P>(pub F, PhantomData<(R, P)>);

pub trait TypeConverter<T> {
    fn to_rust(jvm: &Jvm, raw: JavaValue) -> T;
    fn from_rust(jvm: &Jvm, rust: T) -> JavaValue;
}

pub trait MethodImpl<F, C, R, E, P>
where
    C: ?Sized + Send,
{
    fn into_body(self) -> Box<dyn MethodBody<E, C>>;
}

__generate!();
__generate!(P0);
__generate!(P0, P1);
__generate!(P0, P1, P2);
__generate!(P0, P1, P2, P3);
__generate!(P0, P1, P2, P3, P4);
__generate!(P0, P1, P2, P3, P4, P5);
__generate!(P0, P1, P2, P3, P4, P5, P6);
__generate!(P0, P1, P2, P3, P4, P5, P6, P7);
__generate!(P0, P1, P2, P3, P4, P5, P6, P7, P8);
__generate!(P0, P1, P2, P3, P4, P5, P6, P7, P8, P9);
__generate!(P0, P1, P2, P3, P4, P5, P6, P7, P8);

impl TypeConverter<i8> for i8 {
    fn to_rust(_: &Jvm, raw: JavaValue) -> i8 {
        raw.into()
    }

    fn from_rust(_: &Jvm, rust: i8) -> JavaValue {
        rust.into()
    }
}

impl TypeConverter<i16> for i16 {
    fn to_rust(_: &Jvm, raw: JavaValue) -> i16 {
        raw.into()
    }

    fn from_rust(_: &Jvm, rust: i16) -> JavaValue {
        rust.into()
    }
}

impl TypeConverter<i32> for i32 {
    fn to_rust(_: &Jvm, raw: JavaValue) -> i32 {
        raw.into()
    }

    fn from_rust(_: &Jvm, rust: i32) -> JavaValue {
        rust.into()
    }
}

impl TypeConverter<JavaChar> for JavaChar {
    fn to_rust(_: &Jvm, raw: JavaValue) -> JavaChar {
        raw.into()
    }

    fn from_rust(_: &Jvm, rust: JavaChar) -> JavaValue {
        rust.into()
    }
}

impl TypeConverter<i64> for i64 {
    fn to_rust(_: &Jvm, raw: JavaValue) -> i64 {
        raw.into()
    }

    fn from_rust(_: &Jvm, rust: i64) -> JavaValue {
        rust.into()
    }
}

impl TypeConverter<bool> for bool {
    fn to_rust(_: &Jvm, raw: JavaValue) -> bool {
        raw.into()
    }

    fn from_rust(_: &Jvm, rust: bool) -> JavaValue {
        rust.into()
    }
}

impl TypeConverter<f32> for f32 {
    fn to_rust(_: &Jvm, raw: JavaValue) -> f32 {
        raw.into()
    }

    fn from_rust(_: &Jvm, rust: f32) -> JavaValue {
        rust.into()
    }
}

impl TypeConverter<f64> for f64 {
    fn to_rust(_: &Jvm, raw: JavaValue) -> f64 {
        raw.into()
    }

    fn from_rust(_: &Jvm, rust: f64) -> JavaValue {
        rust.into()
    }
}

impl TypeConverter<()> for () {
    fn to_rust(_: &Jvm, _: JavaValue) {}

    fn from_rust(_: &Jvm, _: ()) -> JavaValue {
        JavaValue::Void
    }
}

impl<T> TypeConverter<ClassInstanceRef<T>> for ClassInstanceRef<T>
where
    T: Sync + Send,
{
    fn to_rust(_: &Jvm, raw: JavaValue) -> Self {
        Self::new(raw.into())
    }

    fn from_rust(_: &Jvm, value: Self) -> JavaValue {
        value.instance.into()
    }
}
