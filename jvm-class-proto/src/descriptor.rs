use alloc::string::String;

use jvm::{Array, ClassInstanceRef, JavaChar};

/// Java type descriptor of a Rust type that appears in a method signature.
pub trait JavaDescriptor {
    fn descriptor() -> String;
}

macro_rules! __impl_primitive_descriptor {
    ($($type: ty => $descriptor: literal),*) => {
        $(
            impl JavaDescriptor for $type {
                fn descriptor() -> String {
                    $descriptor.into()
                }
            }
        )*
    };
}

__impl_primitive_descriptor!(bool => "Z", i8 => "B", JavaChar => "C", i16 => "S", i32 => "I", i64 => "J", f32 => "F", f64 => "D", () => "V");

impl<T> JavaDescriptor for Array<T>
where
    T: JavaDescriptor,
{
    fn descriptor() -> String {
        let mut descriptor = String::from("[");
        descriptor.push_str(&T::descriptor());

        descriptor
    }
}

impl<T> JavaDescriptor for ClassInstanceRef<T>
where
    T: JavaDescriptor,
{
    fn descriptor() -> String {
        T::descriptor()
    }
}
