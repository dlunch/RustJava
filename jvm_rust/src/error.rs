use classfile::ClassFileError;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ClassDefinitionError {
    InvalidClassFile,
    UnsupportedClassVersion(u16),
    Verification,
    UnsupportedFeature(&'static str),
}

impl From<ClassFileError> for ClassDefinitionError {
    fn from(error: ClassFileError) -> Self {
        match error {
            ClassFileError::InvalidFormat => Self::InvalidClassFile,
            ClassFileError::UnsupportedVersion(version) => Self::UnsupportedClassVersion(version),
        }
    }
}
