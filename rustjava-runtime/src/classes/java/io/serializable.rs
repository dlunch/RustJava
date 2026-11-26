use jvm_class_proto::{java_class, java_methods};

// interface java.io.Serializable
#[java_class(name = "java/io/Serializable", interface)]
pub struct Serializable;

#[java_methods]
impl Serializable {}
