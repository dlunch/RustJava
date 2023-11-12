#![no_std]
extern crate alloc;

use cafebabe::parse_class;

pub type JvmResult<T> = anyhow::Result<T>;

#[derive(Default)]
pub struct Jvm {}

impl Jvm {
    pub fn new() -> Jvm {
        Jvm {}
    }

    pub fn load_class(&mut self, class_data: &[u8]) -> JvmResult<()> {
        let class = parse_class(class_data)?;

        todo!("Loaded class {:?}", class);
    }
}
