#![no_std]
extern crate alloc;

pub mod classes;
mod runtime;

use alloc::{boxed::Box, vec::Vec};
use java_class_proto::JavaClassProto;

use jvm::Class;

pub use runtime::Runtime;

pub(crate) type RuntimeContext = dyn Runtime;
pub(crate) type RuntimeClassProto = JavaClassProto<dyn Runtime>;

pub fn get_class_proto(name: &str) -> Option<RuntimeClassProto> {
    Some(match name {
        "java/io/ByteArrayInputStream" => self::classes::java::io::ByteArrayInputStream::as_proto(),
        "java/io/DataInputStream" => self::classes::java::io::DataInputStream::as_proto(),
        "java/io/EOFException" => self::classes::java::io::EOFException::as_proto(),
        "java/io/IOException" => self::classes::java::io::IOException::as_proto(),
        "java/io/InputStream" => self::classes::java::io::InputStream::as_proto(),
        "java/io/OutputStream" => self::classes::java::io::OutputStream::as_proto(),
        "java/io/PrintStream" => self::classes::java::io::PrintStream::as_proto(),
        "java/lang/Class" => self::classes::java::lang::Class::as_proto(),
        "java/lang/ClassLoader" => self::classes::java::lang::ClassLoader::as_proto(),
        "java/lang/Exception" => self::classes::java::lang::Exception::as_proto(),
        "java/lang/IllegalArgumentException" => self::classes::java::lang::IllegalArgumentException::as_proto(),
        "java/lang/IndexOutOfBoundsException" => self::classes::java::lang::IndexOutOfBoundsException::as_proto(),
        "java/lang/Integer" => self::classes::java::lang::Integer::as_proto(),
        "java/lang/InterruptedException" => self::classes::java::lang::InterruptedException::as_proto(),
        "java/lang/Math" => self::classes::java::lang::Math::as_proto(),
        "java/lang/NullPointerException" => self::classes::java::lang::NullPointerException::as_proto(),
        "java/lang/Object" => self::classes::java::lang::Object::as_proto(),
        "java/lang/Runnable" => self::classes::java::lang::Runnable::as_proto(),
        "java/lang/Runtime" => self::classes::java::lang::Runtime::as_proto(),
        "java/lang/RuntimeException" => self::classes::java::lang::RuntimeException::as_proto(),
        "java/lang/SecurityException" => self::classes::java::lang::SecurityException::as_proto(),
        "java/lang/String" => self::classes::java::lang::String::as_proto(),
        "java/lang/StringBuffer" => self::classes::java::lang::StringBuffer::as_proto(),
        "java/lang/System" => self::classes::java::lang::System::as_proto(),
        "java/lang/Thread" => self::classes::java::lang::Thread::as_proto(),
        "java/lang/Throwable" => self::classes::java::lang::Throwable::as_proto(),
        "java/util/Calendar" => self::classes::java::util::Calendar::as_proto(),
        "java/util/Date" => self::classes::java::util::Date::as_proto(),
        "java/util/GregorianCalendar" => self::classes::java::util::GregorianCalendar::as_proto(),
        "java/util/Hashtable" => self::classes::java::util::Hashtable::as_proto(),
        "java/util/Random" => self::classes::java::util::Random::as_proto(),
        "java/util/Timer" => self::classes::java::util::Timer::as_proto(),
        "java/util/TimerTask" => self::classes::java::util::TimerTask::as_proto(),
        "java/util/Vector" => self::classes::java::util::Vector::as_proto(),
        "rustjava/RuntimeClassLoader" => self::classes::rustjava::RuntimeClassLoader::as_proto(),
        "rustjava/ClassPathClassLoader" => self::classes::rustjava::ClassPathClassLoader::as_proto(),
        _ => return None,
    })
}

pub fn get_bootstrap_classes(runtime: &dyn Runtime) -> Vec<Box<dyn Class>> {
    let bootstrap_classes = [
        "java/lang/Object",
        "java/lang/Class",
        "java/lang/ClassLoader",
        "rustjava/RuntimeClassLoader",
        "rustjava/ClassPathClassLoader",
    ];

    bootstrap_classes
        .iter()
        .map(|x| runtime.define_class_proto(x, get_class_proto(x).unwrap()))
        .collect()
}

#[cfg(test)]
pub mod test {
    use alloc::boxed::Box;

    use jvm::Jvm;
    use jvm_rust::{ClassImpl, JvmDetailImpl};

    use crate::{get_class_proto, runtime::test::DummyRuntime};

    pub fn test_jvm() -> Jvm {
        Jvm::new(JvmDetailImpl::new(move |class_name| {
            Ok(get_class_proto(class_name).map(|x| Box::new(ClassImpl::from_class_proto(class_name, x, Box::new(DummyRuntime) as Box<_>)) as Box<_>))
        }))
    }
}
