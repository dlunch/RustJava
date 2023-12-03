use alloc::rc::Rc;
use core::cell::RefCell;

use crate::class_instance::ClassInstance;

#[derive(Clone)]
pub enum JavaValue {
    Void,
    Integer(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    Object(Option<Rc<RefCell<ClassInstance>>>),
}
