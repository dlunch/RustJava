use classfile::ClassFileError;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ClassDefinitionError {
    ClassFile(ClassFileError),
    Verification,
    UnsupportedFeature(&'static str),
}

impl From<ClassFileError> for ClassDefinitionError {
    fn from(error: ClassFileError) -> Self {
        Self::ClassFile(error)
    }
}
