use alloc::{boxed::Box, vec::Vec};
use core::{future::Future, marker::PhantomData};

use jvm::{JavaValue, Jvm, JvmCallback};

macro_rules! __impl_fn_helper {
    ($($arg: ident),*) => {
        impl<'a, E, R, F, Fut, $($arg),*> FnHelper<'a, E, R, ($($arg,)*)> for F
        where
            F: Fn(&'a mut Jvm, $($arg),*) -> Fut,
            Fut: Future<Output = Result<R, E>> + 'a,
            $($arg: TypeConverter<$arg> + 'a),*
        {
            type Output = Fut;
            #[allow(unused_assignments, non_snake_case, unused_mut, unused_variables)]
            fn do_call(&self, jvm: &'a mut Jvm, args: Box<[JavaValue]>) -> Fut {
                let mut args = Vec::from(args).into_iter();
                $(
                    let $arg = $arg::to_rust(jvm, args.next().unwrap());
                )*
                self(jvm, $($arg),*)
            }
        }
    };
}

#[macro_export]
macro_rules! __impl_method_body {
    ($($arg: ident),*) => {
        #[async_trait::async_trait(?Send)]
        impl<F, R, $($arg),*> JvmCallback for MethodHolder<F, R, ($($arg,)*)>
        where
            F: for<'a> FnHelper<'a, anyhow::Error, R, ($($arg,)*)>,
            R: TypeConverter<R>,
        {
            async fn call(&self, jvm: &mut Jvm, args: Box<[JavaValue]>) -> Result<JavaValue, anyhow::Error> {
                let result = self.0.do_call(jvm, args).await?;

                Ok(R::from_rust(jvm, result))
            }
        }
    };
}

macro_rules! __impl_method_impl {
    ($($arg: ident),*) => {
        impl<F, R,  $($arg),*> MethodImpl<F, R, ($($arg,)*)> for F
        where
            F: for<'a> FnHelper<'a, anyhow::Error, R, ($($arg,)*)> + 'static,
            R: TypeConverter<R> + 'static,
            $($arg: 'static),*
        {
            fn into_body(self) -> Box<dyn JvmCallback> {
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

trait FnHelper<'a, E, R, P> {
    type Output: Future<Output = Result<R, E>> + 'a;
    fn do_call(&self, jvm: &'a mut Jvm, args: Box<[JavaValue]>) -> Self::Output;
}

struct MethodHolder<F, R, P>(pub F, PhantomData<(R, P)>);

pub trait TypeConverter<T> {
    fn to_rust(jvm: &mut Jvm, raw: JavaValue) -> T;
    fn from_rust(jvm: &mut Jvm, rust: T) -> JavaValue;
}

pub trait MethodImpl<F, R, P> {
    fn into_body(self) -> Box<dyn JvmCallback>;
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
