use alloc::boxed::Box;

use jvm::{BootstrapClassLoader, ClassDefinition, Jvm, Result};

use crate::{Runtime, RuntimeClassProto};

pub fn get_proto(name: &str) -> Option<RuntimeClassProto> {
    Some(match name {
        "java/io/BufferedReader" => crate::classes::java::io::BufferedReader::as_proto(),
        "java/io/ByteArrayInputStream" => crate::classes::java::io::ByteArrayInputStream::as_proto(),
        "java/io/DataInputStream" => crate::classes::java::io::DataInputStream::as_proto(),
        "java/io/EOFException" => crate::classes::java::io::EOFException::as_proto(),
        "java/io/File" => crate::classes::java::io::File::as_proto(),
        "java/io/FileDescriptor" => crate::classes::java::io::FileDescriptor::as_proto(),
        "java/io/FileInputStream" => crate::classes::java::io::FileInputStream::as_proto(),
        "java/io/FileOutputStream" => crate::classes::java::io::FileOutputStream::as_proto(),
        "java/io/FilterOutputStream" => crate::classes::java::io::FilterOutputStream::as_proto(),
        "java/io/InputStream" => crate::classes::java::io::InputStream::as_proto(),
        "java/io/InputStreamReader" => crate::classes::java::io::InputStreamReader::as_proto(),
        "java/io/IOException" => crate::classes::java::io::IOException::as_proto(),
        "java/io/OutputStream" => crate::classes::java::io::OutputStream::as_proto(),
        "java/io/PrintStream" => crate::classes::java::io::PrintStream::as_proto(),
        "java/io/Reader" => crate::classes::java::io::Reader::as_proto(),

        "java/lang/Class" => crate::classes::java::lang::Class::as_proto(),
        "java/lang/ClassLoader" => crate::classes::java::lang::ClassLoader::as_proto(),
        "java/lang/Error" => crate::classes::java::lang::Error::as_proto(),
        "java/lang/Exception" => crate::classes::java::lang::Exception::as_proto(),
        "java/lang/IllegalArgumentException" => crate::classes::java::lang::IllegalArgumentException::as_proto(),
        "java/lang/IncompatibleClassChangeError" => crate::classes::java::lang::IncompatibleClassChangeError::as_proto(),
        "java/lang/IndexOutOfBoundsException" => crate::classes::java::lang::IndexOutOfBoundsException::as_proto(),
        "java/lang/Integer" => crate::classes::java::lang::Integer::as_proto(),
        "java/lang/InterruptedException" => crate::classes::java::lang::InterruptedException::as_proto(),
        "java/lang/LinkageError" => crate::classes::java::lang::LinkageError::as_proto(),
        "java/lang/Math" => crate::classes::java::lang::Math::as_proto(),
        "java/lang/NoClassDefFoundError" => crate::classes::java::lang::NoClassDefFoundError::as_proto(),
        "java/lang/NoSuchFieldError" => crate::classes::java::lang::NoSuchFieldError::as_proto(),
        "java/lang/NoSuchMethodError" => crate::classes::java::lang::NoSuchMethodError::as_proto(),
        "java/lang/NullPointerException" => crate::classes::java::lang::NullPointerException::as_proto(),
        "java/lang/Object" => crate::classes::java::lang::Object::as_proto(),
        "java/lang/Runnable" => crate::classes::java::lang::Runnable::as_proto(),
        "java/lang/Runtime" => crate::classes::java::lang::Runtime::as_proto(),
        "java/lang/RuntimeException" => crate::classes::java::lang::RuntimeException::as_proto(),
        "java/lang/SecurityException" => crate::classes::java::lang::SecurityException::as_proto(),
        "java/lang/String" => crate::classes::java::lang::String::as_proto(),
        "java/lang/StringBuffer" => crate::classes::java::lang::StringBuffer::as_proto(),
        "java/lang/System" => crate::classes::java::lang::System::as_proto(),
        "java/lang/Thread" => crate::classes::java::lang::Thread::as_proto(),
        "java/lang/Throwable" => crate::classes::java::lang::Throwable::as_proto(),
        "java/lang/UnsupportedOperationException" => crate::classes::java::lang::UnsupportedOperationException::as_proto(),

        "java/net/JarURLConnection" => crate::classes::java::net::JarURLConnection::as_proto(),
        "java/net/MalformedURLException" => crate::classes::java::net::MalformedURLException::as_proto(),
        "java/net/UnknownServiceException" => crate::classes::java::net::UnknownServiceException::as_proto(),
        "java/net/URL" => crate::classes::java::net::URL::as_proto(),
        "java/net/URLClassLoader" => crate::classes::java::net::URLClassLoader::as_proto(),
        "java/net/URLConnection" => crate::classes::java::net::URLConnection::as_proto(),
        "java/net/URLStreamHandler" => crate::classes::java::net::URLStreamHandler::as_proto(),

        "java/util/AbstractCollection" => crate::classes::java::util::AbstractCollection::as_proto(),
        "java/util/AbstractList" => crate::classes::java::util::AbstractList::as_proto(),
        "java/util/Calendar" => crate::classes::java::util::Calendar::as_proto(),
        "java/util/Date" => crate::classes::java::util::Date::as_proto(),
        "java/util/Dictionary" => crate::classes::java::util::Dictionary::as_proto(),
        "java/util/Enumeration" => crate::classes::java::util::Enumeration::as_proto(),
        "java/util/GregorianCalendar" => crate::classes::java::util::GregorianCalendar::as_proto(),
        "java/util/Hashtable" => crate::classes::java::util::Hashtable::as_proto(),
        "java/util/Properties" => crate::classes::java::util::Properties::as_proto(),
        "java/util/Random" => crate::classes::java::util::Random::as_proto(),
        "java/util/Timer" => crate::classes::java::util::Timer::as_proto(),
        "java/util/TimerTask" => crate::classes::java::util::TimerTask::as_proto(),
        "java/util/Vector" => crate::classes::java::util::Vector::as_proto(),

        "java/util/jar/Attributes" => crate::classes::java::util::jar::Attributes::as_proto(),
        "java/util/jar/JarEntry" => crate::classes::java::util::jar::JarEntry::as_proto(),
        "java/util/jar/JarFile" => crate::classes::java::util::jar::JarFile::as_proto(),
        "java/util/jar/JarFile$Entries" => crate::classes::java::util::jar::JarFileEntries::as_proto(),
        "java/util/jar/Manifest" => crate::classes::java::util::jar::Manifest::as_proto(),

        "java/util/zip/ZipEntry" => crate::classes::java::util::zip::ZipEntry::as_proto(),
        "java/util/zip/ZipFile" => crate::classes::java::util::zip::ZipFile::as_proto(),
        "java/util/zip/ZipFile$Entries" => crate::classes::java::util::zip::ZipFileEntries::as_proto(),

        "rustjava/RuntimeClassLoader" => crate::classes::rustjava::RuntimeClassLoader::as_proto(),

        "rustjava/net/FileURLConnection" => crate::classes::rustjava::net::FileURLConnection::as_proto(),
        "rustjava/net/FileURLHandler" => crate::classes::rustjava::net::FileURLHandler::as_proto(),
        "rustjava/net/JarURLConnection" => crate::classes::rustjava::net::JarURLConnection::as_proto(),
        "rustjava/net/JarURLHandler" => crate::classes::rustjava::net::JarURLHandler::as_proto(),
        _ => return None,
    })
}

struct JavaRuntimeClassLoader {
    runtime: Box<dyn Runtime>,
}

#[async_trait::async_trait]
impl BootstrapClassLoader for JavaRuntimeClassLoader {
    async fn load_class(&self, jvm: &Jvm, name: &str) -> Result<Option<Box<dyn ClassDefinition>>> {
        if let Some(element_type_name) = name.strip_prefix('[') {
            return Ok(Some(self.runtime.define_array_class(jvm, element_type_name).await?));
        }

        let proto = get_proto(name);
        if let Some(proto) = proto {
            Ok(Some(self.runtime.define_class_rust(jvm, name, proto).await?))
        } else {
            Ok(None)
        }
    }
}

pub fn get_bootstrap_class_loader(runtime: Box<dyn Runtime>) -> impl BootstrapClassLoader {
    JavaRuntimeClassLoader { runtime }
}
