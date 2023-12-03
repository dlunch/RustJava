use crate::ArrayClassInstance;

pub trait ClassInstance {
    fn class_name(&self) -> &str;
    fn as_array_instance_mut(&mut self) -> Option<&mut dyn ArrayClassInstance>; // is there a better way to do this?
}
