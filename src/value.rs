use alloc::rc::Rc;
use core::cell::RefCell;

use crate::class::ClassInstance;

pub enum JavaValue {
    Void,
    Integer(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ObjectRef(Rc<RefCell<ClassInstance>>),
}
