use alloc::boxed::Box;

use jvm::{BootstrapClassLoader, ClassDefinition, Jvm, Result};

use crate::{Runtime, RuntimeClassProto};

pub fn get_proto(name: &str) -> Option<RuntimeClassProto> {
    let protos = [
        crate::classes::java::io::BufferedReader::as_proto(),
        crate::classes::java::io::ByteArrayInputStream::as_proto(),
        crate::classes::java::io::DataInputStream::as_proto(),
        crate::classes::java::io::EOFException::as_proto(),
        crate::classes::java::io::File::as_proto(),
        crate::classes::java::io::FileDescriptor::as_proto(),
        crate::classes::java::io::FileInputStream::as_proto(),
        crate::classes::java::io::FileOutputStream::as_proto(),
        crate::classes::java::io::FilterOutputStream::as_proto(),
        crate::classes::java::io::InputStream::as_proto(),
        crate::classes::java::io::InputStreamReader::as_proto(),
        crate::classes::java::io::IOException::as_proto(),
        crate::classes::java::io::OutputStream::as_proto(),
        crate::classes::java::io::PrintStream::as_proto(),
        crate::classes::java::io::Reader::as_proto(),
        crate::classes::java::lang::Class::as_proto(),
        crate::classes::java::lang::ClassLoader::as_proto(),
        crate::classes::java::lang::Error::as_proto(),
        crate::classes::java::lang::Exception::as_proto(),
        crate::classes::java::lang::IllegalArgumentException::as_proto(),
        crate::classes::java::lang::IncompatibleClassChangeError::as_proto(),
        crate::classes::java::lang::IndexOutOfBoundsException::as_proto(),
        crate::classes::java::lang::Integer::as_proto(),
        crate::classes::java::lang::InterruptedException::as_proto(),
        crate::classes::java::lang::LinkageError::as_proto(),
        crate::classes::java::lang::Math::as_proto(),
        crate::classes::java::lang::NoClassDefFoundError::as_proto(),
        crate::classes::java::lang::NoSuchFieldError::as_proto(),
        crate::classes::java::lang::NoSuchMethodError::as_proto(),
        crate::classes::java::lang::NullPointerException::as_proto(),
        crate::classes::java::lang::Object::as_proto(),
        crate::classes::java::lang::Runnable::as_proto(),
        crate::classes::java::lang::Runtime::as_proto(),
        crate::classes::java::lang::RuntimeException::as_proto(),
        crate::classes::java::lang::SecurityException::as_proto(),
        crate::classes::java::lang::String::as_proto(),
        crate::classes::java::lang::StringBuffer::as_proto(),
        crate::classes::java::lang::System::as_proto(),
        crate::classes::java::lang::Thread::as_proto(),
        crate::classes::java::lang::Throwable::as_proto(),
        crate::classes::java::lang::UnsupportedOperationException::as_proto(),
        crate::classes::java::net::JarURLConnection::as_proto(),
        crate::classes::java::net::MalformedURLException::as_proto(),
        crate::classes::java::net::UnknownServiceException::as_proto(),
        crate::classes::java::net::URL::as_proto(),
        crate::classes::java::net::URLClassLoader::as_proto(),
        crate::classes::java::net::URLConnection::as_proto(),
        crate::classes::java::net::URLStreamHandler::as_proto(),
        crate::classes::java::util::AbstractCollection::as_proto(),
        crate::classes::java::util::AbstractList::as_proto(),
        crate::classes::java::util::Calendar::as_proto(),
        crate::classes::java::util::Date::as_proto(),
        crate::classes::java::util::Dictionary::as_proto(),
        crate::classes::java::util::Enumeration::as_proto(),
        crate::classes::java::util::GregorianCalendar::as_proto(),
        crate::classes::java::util::Hashtable::as_proto(),
        crate::classes::java::util::Properties::as_proto(),
        crate::classes::java::util::Random::as_proto(),
        crate::classes::java::util::Timer::as_proto(),
        crate::classes::java::util::TimerTask::as_proto(),
        crate::classes::java::util::Vector::as_proto(),
        crate::classes::java::util::jar::Attributes::as_proto(),
        crate::classes::java::util::jar::JarEntry::as_proto(),
        crate::classes::java::util::jar::JarFile::as_proto(),
        crate::classes::java::util::jar::JarFileEntries::as_proto(),
        crate::classes::java::util::jar::Manifest::as_proto(),
        crate::classes::java::util::zip::ZipEntry::as_proto(),
        crate::classes::java::util::zip::ZipFile::as_proto(),
        crate::classes::java::util::zip::ZipFileEntries::as_proto(),
        crate::classes::rustjava::RuntimeClassLoader::as_proto(),
        crate::classes::rustjava::net::FileURLConnection::as_proto(),
        crate::classes::rustjava::net::FileURLHandler::as_proto(),
        crate::classes::rustjava::net::JarURLConnection::as_proto(),
        crate::classes::rustjava::net::JarURLHandler::as_proto(),
    ];

    protos.into_iter().find(|proto| proto.name == name)
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
            Ok(Some(self.runtime.define_class_rust(jvm, proto).await?))
        } else {
            Ok(None)
        }
    }
}

pub fn get_bootstrap_class_loader(runtime: Box<dyn Runtime>) -> impl BootstrapClassLoader {
    JavaRuntimeClassLoader { runtime }
}
