use alloc::boxed::Box;

use crate::as_any::AsAny;

pub type ThreadId = usize;

pub trait ThreadContext: AsAny {}

pub trait ThreadContextProvider {
    fn thread_context(&self, thread_id: ThreadId) -> Box<dyn ThreadContext>;
}
