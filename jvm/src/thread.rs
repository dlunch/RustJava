use alloc::boxed::Box;
use downcast_rs::{impl_downcast, Downcast};

pub type ThreadId = usize;

pub trait ThreadContext: Downcast {}
impl_downcast!(ThreadContext);

pub trait ThreadContextProvider {
    fn thread_context(&self, thread_id: ThreadId) -> Box<dyn ThreadContext>;
}
