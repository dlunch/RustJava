use alloc::{rc::Rc, vec::Vec};
use core::cell::RefCell;

use crate::{class::Class, value::JavaValue};

pub struct ClassInstance {
    pub class: Rc<RefCell<Class>>,
    pub storage: Vec<JavaValue>,
}

impl ClassInstance {
    pub fn new(class: Rc<RefCell<Class>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self { class, storage: Vec::new() }))
    }
}
