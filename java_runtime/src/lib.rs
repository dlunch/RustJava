#![no_std]
extern crate alloc;

pub mod classes;
mod runtime;

use alloc::{boxed::Box, vec::Vec};

use java_class_proto::JavaClassProto;
use jvm::{Class, Jvm, JvmResult};

pub use runtime::Runtime;

pub(crate) type RuntimeContext = dyn Runtime;
pub(crate) type RuntimeClassProto = JavaClassProto<dyn Runtime>;

fn get_runtime_classes<T>(class_creator: &T) -> Vec<Box<dyn Class>>
where
    T: Fn(&str, RuntimeClassProto) -> Box<dyn Class>,
{
    [
        ("java/io/ByteArrayInputStream", self::classes::java::io::ByteArrayInputStream::as_proto()),
        ("java/io/DataInputStream", self::classes::java::io::DataInputStream::as_proto()),
        ("java/io/EOFException", self::classes::java::io::EOFException::as_proto()),
        ("java/io/IOException", self::classes::java::io::IOException::as_proto()),
        ("java/io/InputStream", self::classes::java::io::InputStream::as_proto()),
        ("java/io/OutputStream", self::classes::java::io::OutputStream::as_proto()),
        ("java/io/PrintStream", self::classes::java::io::PrintStream::as_proto()),
        ("java/lang/Class", self::classes::java::lang::Class::as_proto()),
        ("java/lang/ClassLoader", self::classes::java::lang::ClassLoader::as_proto()),
        ("java/lang/Exception", self::classes::java::lang::Exception::as_proto()),
        (
            "java/lang/IllegalArgumentException",
            self::classes::java::lang::IllegalArgumentException::as_proto(),
        ),
        (
            "java/lang/IndexOutOfBoundsException",
            self::classes::java::lang::IndexOutOfBoundsException::as_proto(),
        ),
        ("java/lang/Integer", self::classes::java::lang::Integer::as_proto()),
        (
            "java/lang/InterruptedException",
            self::classes::java::lang::InterruptedException::as_proto(),
        ),
        ("java/lang/Math", self::classes::java::lang::Math::as_proto()),
        (
            "java/lang/NullPointerException",
            self::classes::java::lang::NullPointerException::as_proto(),
        ),
        ("java/lang/Object", self::classes::java::lang::Object::as_proto()),
        ("java/lang/Runnable", self::classes::java::lang::Runnable::as_proto()),
        ("java/lang/Runtime", self::classes::java::lang::Runtime::as_proto()),
        ("java/lang/RuntimeException", self::classes::java::lang::RuntimeException::as_proto()),
        ("java/lang/SecurityException", self::classes::java::lang::SecurityException::as_proto()),
        ("java/lang/String", self::classes::java::lang::String::as_proto()),
        ("java/lang/StringBuffer", self::classes::java::lang::StringBuffer::as_proto()),
        ("java/lang/System", self::classes::java::lang::System::as_proto()),
        ("java/lang/Thread", self::classes::java::lang::Thread::as_proto()),
        ("java/lang/Throwable", self::classes::java::lang::Throwable::as_proto()),
        ("java/util/Calendar", self::classes::java::util::Calendar::as_proto()),
        ("java/util/Date", self::classes::java::util::Date::as_proto()),
        ("java/util/GregorianCalendar", self::classes::java::util::GregorianCalendar::as_proto()),
        ("java/util/Hashtable", self::classes::java::util::Hashtable::as_proto()),
        ("java/util/Random", self::classes::java::util::Random::as_proto()),
        ("java/util/Timer", self::classes::java::util::Timer::as_proto()),
        ("java/util/TimerTask", self::classes::java::util::TimerTask::as_proto()),
        ("java/util/Vector", self::classes::java::util::Vector::as_proto()),
        ("rustjava/ArrayClassLoader", self::classes::rustjava::ArrayClassLoader::as_proto()),
        ("rustjava/ClassPathClassLoader", self::classes::rustjava::ClassPathClassLoader::as_proto()),
    ]
    .into_iter()
    .map(|(name, proto)| class_creator(name, proto))
    .collect()
}

pub async fn initialize<T>(jvm: &mut Jvm, class_creator: T) -> JvmResult<()>
where
    T: Fn(&str, RuntimeClassProto) -> Box<dyn Class>,
{
    // minimum set of classes to instantiate and use classloader
    let java_lang_object = class_creator("java/lang/Object", self::classes::java::lang::Object::as_proto());
    let java_lang_string = class_creator("java/lang/String", self::classes::java::lang::String::as_proto());
    let java_lang_class = class_creator("java/lang/Class", self::classes::java::lang::Class::as_proto());
    let java_lang_class_loader = class_creator("java/lang/ClassLoader", self::classes::java::lang::ClassLoader::as_proto());
    let rustjava_runtime_class_loader = class_creator("rustjava/RuntimeClassLoader", self::classes::rustjava::RuntimeClassLoader::as_proto());
    let rustjava_array_class_loader = class_creator("rustjava/ArrayClassLoader", self::classes::rustjava::ArrayClassLoader::as_proto());
    let rustjava_class_path_class_loader = class_creator("rustjava/ClassPathClassLoader", self::classes::rustjava::ClassPathClassLoader::as_proto());

    jvm.register_class(java_lang_object).await?;
    jvm.register_class(java_lang_string).await?;
    jvm.register_class(java_lang_class).await?;
    jvm.register_class(java_lang_class_loader).await?;
    jvm.register_class(rustjava_runtime_class_loader).await?;
    jvm.register_class(rustjava_array_class_loader).await?;
    jvm.register_class(rustjava_class_path_class_loader).await?;

    jvm.init_system_class_loader().await?;

    let all_classes = get_runtime_classes(&class_creator);
    self::classes::rustjava::RuntimeClassLoader::initialize(jvm, all_classes).await?;

    Ok(())
}

#[cfg(test)]
pub mod test {
    use alloc::boxed::Box;

    use jvm::Jvm;
    use jvm_rust::{ClassImpl, JvmDetailImpl};

    use crate::{initialize, runtime::test::DummyRuntime};

    pub async fn test_jvm() -> anyhow::Result<Jvm> {
        let mut jvm = Jvm::new(JvmDetailImpl::new()).await?;

        initialize(&mut jvm, |name, proto| {
            Box::new(ClassImpl::from_class_proto(name, proto, Box::new(DummyRuntime) as Box<_>))
        })
        .await?;

        Ok(jvm)
    }
}
