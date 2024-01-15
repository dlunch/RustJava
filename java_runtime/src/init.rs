use alloc::{boxed::Box, vec::Vec};

use jvm::{Class, Jvm, JvmResult};

use crate::RuntimeClassProto;

fn get_runtime_classes<T>(class_creator: &T) -> Vec<Box<dyn Class>>
where
    T: Fn(&str, RuntimeClassProto) -> Box<dyn Class>,
{
    [
        ("java/io/ByteArrayInputStream", crate::classes::java::io::ByteArrayInputStream::as_proto()),
        ("java/io/DataInputStream", crate::classes::java::io::DataInputStream::as_proto()),
        ("java/io/EOFException", crate::classes::java::io::EOFException::as_proto()),
        ("java/io/IOException", crate::classes::java::io::IOException::as_proto()),
        ("java/io/InputStream", crate::classes::java::io::InputStream::as_proto()),
        ("java/io/OutputStream", crate::classes::java::io::OutputStream::as_proto()),
        ("java/io/PrintStream", crate::classes::java::io::PrintStream::as_proto()),
        ("java/lang/Class", crate::classes::java::lang::Class::as_proto()),
        ("java/lang/ClassLoader", crate::classes::java::lang::ClassLoader::as_proto()),
        ("java/lang/Exception", crate::classes::java::lang::Exception::as_proto()),
        (
            "java/lang/IllegalArgumentException",
            crate::classes::java::lang::IllegalArgumentException::as_proto(),
        ),
        (
            "java/lang/IndexOutOfBoundsException",
            crate::classes::java::lang::IndexOutOfBoundsException::as_proto(),
        ),
        ("java/lang/Integer", crate::classes::java::lang::Integer::as_proto()),
        (
            "java/lang/InterruptedException",
            crate::classes::java::lang::InterruptedException::as_proto(),
        ),
        ("java/lang/Math", crate::classes::java::lang::Math::as_proto()),
        (
            "java/lang/NullPointerException",
            crate::classes::java::lang::NullPointerException::as_proto(),
        ),
        ("java/lang/Object", crate::classes::java::lang::Object::as_proto()),
        ("java/lang/Runnable", crate::classes::java::lang::Runnable::as_proto()),
        ("java/lang/Runtime", crate::classes::java::lang::Runtime::as_proto()),
        ("java/lang/RuntimeException", crate::classes::java::lang::RuntimeException::as_proto()),
        ("java/lang/SecurityException", crate::classes::java::lang::SecurityException::as_proto()),
        ("java/lang/String", crate::classes::java::lang::String::as_proto()),
        ("java/lang/StringBuffer", crate::classes::java::lang::StringBuffer::as_proto()),
        ("java/lang/System", crate::classes::java::lang::System::as_proto()),
        ("java/lang/Thread", crate::classes::java::lang::Thread::as_proto()),
        ("java/lang/Throwable", crate::classes::java::lang::Throwable::as_proto()),
        ("java/util/Calendar", crate::classes::java::util::Calendar::as_proto()),
        ("java/util/Date", crate::classes::java::util::Date::as_proto()),
        ("java/util/GregorianCalendar", crate::classes::java::util::GregorianCalendar::as_proto()),
        ("java/util/Hashtable", crate::classes::java::util::Hashtable::as_proto()),
        ("java/util/Random", crate::classes::java::util::Random::as_proto()),
        ("java/util/Timer", crate::classes::java::util::Timer::as_proto()),
        ("java/util/TimerTask", crate::classes::java::util::TimerTask::as_proto()),
        ("java/util/Vector", crate::classes::java::util::Vector::as_proto()),
        ("rustjava/ArrayClassLoader", crate::classes::rustjava::ArrayClassLoader::as_proto()),
        (
            "rustjava/ClassPathClassLoader",
            crate::classes::rustjava::ClassPathClassLoader::as_proto(),
        ),
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
    let java_lang_object = class_creator("java/lang/Object", crate::classes::java::lang::Object::as_proto());
    let java_lang_string = class_creator("java/lang/String", crate::classes::java::lang::String::as_proto());
    let java_lang_class = class_creator("java/lang/Class", crate::classes::java::lang::Class::as_proto());
    let java_lang_class_loader = class_creator("java/lang/ClassLoader", crate::classes::java::lang::ClassLoader::as_proto());
    let rustjava_runtime_class_loader = class_creator("rustjava/RuntimeClassLoader", crate::classes::rustjava::RuntimeClassLoader::as_proto());
    let rustjava_array_class_loader = class_creator("rustjava/ArrayClassLoader", crate::classes::rustjava::ArrayClassLoader::as_proto());
    let rustjava_class_path_class_loader = class_creator(
        "rustjava/ClassPathClassLoader",
        crate::classes::rustjava::ClassPathClassLoader::as_proto(),
    );

    jvm.register_class(java_lang_object).await?;
    jvm.register_class(java_lang_string).await?;
    jvm.register_class(java_lang_class).await?;
    jvm.register_class(java_lang_class_loader).await?;
    jvm.register_class(rustjava_runtime_class_loader).await?;
    jvm.register_class(rustjava_array_class_loader).await?;
    jvm.register_class(rustjava_class_path_class_loader).await?;

    jvm.init_system_class_loader().await?;

    let all_classes = get_runtime_classes(&class_creator);
    crate::classes::rustjava::RuntimeClassLoader::initialize(jvm, all_classes).await?;

    Ok(())
}
