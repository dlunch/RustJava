use crate::as_any::AsAny;

pub type ThreadId = usize;

pub trait ThreadContext: AsAny {}
