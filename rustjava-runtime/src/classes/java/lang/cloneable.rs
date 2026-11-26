use jvm_class_proto::{java_class, java_methods};

// interface java.lang.Cloneable
#[java_class(name = "java/lang/Cloneable", interface)]
pub struct Cloneable;

#[java_methods]
impl Cloneable {}
