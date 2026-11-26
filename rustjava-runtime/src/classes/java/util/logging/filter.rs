use jvm::{ClassInstanceRef, Jvm, Result};
use jvm_class_proto::{java_class, java_methods};

use crate::{RuntimeContext, classes::java::util::logging::LogRecord};

// public interface java.util.logging.Filter
#[java_class(name = "java/util/logging/Filter", public, interface, abstract)]
pub struct Filter;

#[java_methods]
impl Filter {
    #[method(public)]
    async fn is_loggable(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, record: ClassInstanceRef<LogRecord>) -> Result<bool>;
}
