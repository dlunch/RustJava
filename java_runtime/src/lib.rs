#![no_std]
extern crate alloc;

use java_runtime_base::Platform;

pub mod java;

pub(crate) type JavaContext = dyn Platform;
pub(crate) type JavaClassProto = java_runtime_base::JavaClassProto<dyn Platform>;

pub fn get_class_proto(name: &str) -> Option<JavaClassProto> {
    Some(match name {
        "java/io/ByteArrayInputStream" => crate::java::io::ByteArrayInputStream::as_proto(),
        "java/io/DataInputStream" => crate::java::io::DataInputStream::as_proto(),
        "java/io/EOFException" => crate::java::io::EOFException::as_proto(),
        "java/io/IOException" => crate::java::io::IOException::as_proto(),
        "java/io/InputStream" => crate::java::io::InputStream::as_proto(),
        "java/io/OutputStream" => crate::java::io::OutputStream::as_proto(),
        "java/io/PrintStream" => crate::java::io::PrintStream::as_proto(),
        "java/lang/Class" => crate::java::lang::Class::as_proto(),
        "java/lang/Exception" => crate::java::lang::Exception::as_proto(),
        "java/lang/IllegalArgumentException" => crate::java::lang::IllegalArgumentException::as_proto(),
        "java/lang/IndexOutOfBoundsException" => crate::java::lang::IndexOutOfBoundsException::as_proto(),
        "java/lang/Integer" => crate::java::lang::Integer::as_proto(),
        "java/lang/InterruptedException" => crate::java::lang::InterruptedException::as_proto(),
        "java/lang/Math" => crate::java::lang::Math::as_proto(),
        "java/lang/NullPointerException" => crate::java::lang::NullPointerException::as_proto(),
        "java/lang/Object" => crate::java::lang::Object::as_proto(),
        "java/lang/Runnable" => crate::java::lang::Runnable::as_proto(),
        "java/lang/Runtime" => crate::java::lang::Runtime::as_proto(),
        "java/lang/RuntimeException" => crate::java::lang::RuntimeException::as_proto(),
        "java/lang/SecurityException" => crate::java::lang::SecurityException::as_proto(),
        "java/lang/String" => crate::java::lang::String::as_proto(),
        "java/lang/StringBuffer" => crate::java::lang::StringBuffer::as_proto(),
        "java/lang/System" => crate::java::lang::System::as_proto(),
        "java/lang/Thread" => crate::java::lang::Thread::as_proto(),
        "java/lang/Throwable" => crate::java::lang::Throwable::as_proto(),
        "java/util/Calendar" => crate::java::util::Calendar::as_proto(),
        "java/util/Date" => crate::java::util::Date::as_proto(),
        "java/util/GregorianCalendar" => crate::java::util::GregorianCalendar::as_proto(),
        "java/util/Hashtable" => crate::java::util::Hashtable::as_proto(),
        "java/util/Random" => crate::java::util::Random::as_proto(),
        "java/util/Timer" => crate::java::util::Timer::as_proto(),
        "java/util/TimerTask" => crate::java::util::TimerTask::as_proto(),
        "java/util/Vector" => crate::java::util::Vector::as_proto(),
        _ => return None,
    })
}
