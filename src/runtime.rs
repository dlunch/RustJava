use alloc::{boxed::Box, rc::Rc, string::String, vec::Vec};
use core::{cell::RefCell, time::Duration};
use std::io::Write;

use java_runtime::Runtime;
use jvm::JvmCallback;

pub struct RuntimeImpl<T>
where
    T: Write,
{
    stdout: Rc<RefCell<T>>,
}

impl<T> RuntimeImpl<T>
where
    T: Write,
{
    pub fn new(stdout: T) -> Self {
        Self {
            stdout: Rc::new(RefCell::new(stdout)),
        }
    }
}

#[async_trait::async_trait(?Send)]
impl<T> Runtime for RuntimeImpl<T>
where
    T: Write,
{
    async fn sleep(&self, _duration: Duration) {
        todo!()
    }

    async fn r#yield(&self) {
        todo!()
    }

    fn spawn(&self, _callback: Box<dyn JvmCallback>) {
        todo!()
    }

    fn now(&self) -> u64 {
        todo!()
    }

    fn encode_str(&self, _s: &str) -> Vec<u8> {
        todo!()
    }

    fn decode_str(&self, _bytes: &[u8]) -> String {
        todo!()
    }

    fn load_resource(&self, _name: &str) -> Option<Vec<u8>> {
        todo!()
    }

    fn println(&mut self, s: &str) {
        writeln!(self.stdout.borrow_mut(), "{}", s).unwrap();
    }
}

impl<T> Clone for RuntimeImpl<T>
where
    T: Write,
{
    fn clone(&self) -> Self {
        Self { stdout: self.stdout.clone() }
    }
}