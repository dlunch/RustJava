use crate::as_any::AsAny;

pub trait ClassInstance: AsAny {
    fn class_name(&self) -> &str;
}
