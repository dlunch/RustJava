type ClassFileResult<T> = anyhow::Result<T>;

pub struct ClassFile {}

pub fn parse_class(_file: &[u8]) -> ClassFileResult<ClassFile> {
    Ok(ClassFile {})
}
