use alloc::boxed::Box;
use core::{future::Future, marker::PhantomData};

use jvm::{JavaValue, Jvm};

macro_rules! __impl_fn_helper {
    ($($arg: ident),*) => {
        impl<'a, C, E, R, F, Fut, $($arg),*> FnHelper<'a, C, E, R, ($($arg,)*)> for F
        where
            F: Fn(&'a Jvm, &'a mut C, $($arg),*) -> Fut,
            C: ?Sized + 'a,
            Fut: Future<Output = Result<R, E>> + 'a,
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
        #[async_trait::async_trait(?Send)]
        impl<F, C, R, E, $($arg),*> MethodBody<E, C> for MethodHolder<F, R, ($($arg,)*)>
        where
            F: for<'a> FnHelper<'a, C, E, R, ($($arg,)*)>,
            C: ?Sized,
            R: TypeConverter<R>,
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
            F: for<'a> FnHelper<'a, C, E, R, ($($arg,)*)> + 'static,
            C: ?Sized,
            R: TypeConverter<R> + 'static,
            $($arg: 'static),*
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

#[async_trait::async_trait(?Send)]
pub trait MethodBody<E, C>
where
    C: ?Sized,
{
    async fn call(&self, jvm: &Jvm, context: &mut C, args: Box<[JavaValue]>) -> Result<JavaValue, E>;
}

trait FnHelper<'a, C, E, R, P>
where
    C: ?Sized + 'a,
{
    type Output: Future<Output = Result<R, E>> + 'a;
    fn do_call(&self, jvm: &'a Jvm, context: &'a mut C, args: Box<[JavaValue]>) -> Self::Output;
}

struct MethodHolder<F, R, P>(pub F, PhantomData<(R, P)>);

pub trait TypeConverter<T> {
    fn to_rust(jvm: &Jvm, raw: JavaValue) -> T;
    fn from_rust(jvm: &Jvm, rust: T) -> JavaValue;
}

pub trait MethodImpl<F, C, R, E, P>
where
    C: ?Sized,
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
