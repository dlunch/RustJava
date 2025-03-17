use alloc::{boxed::Box, vec::Vec};

use bytemuck::cast_vec;

use crate::{Result, class_instance::ClassInstance, jvm::Jvm};

pub struct JavaIoInputStream;

impl JavaIoInputStream {
    #[allow(clippy::borrowed_box)]
    pub async fn read_until_end(jvm: &Jvm, this: &Box<dyn ClassInstance>) -> Result<Vec<u8>> {
        let mut buffer = Vec::new();

        let java_buffer = jvm.instantiate_array("B", 1024).await?;

        loop {
            let bytes_read: i32 = jvm.invoke_virtual(this, "read", "([B)I", (java_buffer.clone(),)).await?;

            if bytes_read == 0 || bytes_read == -1 {
                break;
            }

            buffer.resize(buffer.len() + bytes_read as usize, 0);
            let buffer_offset = buffer.len() - bytes_read as usize;
            jvm.array_raw_buffer(&java_buffer).await?.read(0, &mut buffer[buffer_offset..])?;
        }

        Ok(cast_vec(buffer))
    }
}
