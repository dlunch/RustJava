#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ClassFileError {
    InvalidFormat,
    UnsupportedVersion(u16),
}
