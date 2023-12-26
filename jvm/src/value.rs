use alloc::{boxed::Box, rc::Rc};
use core::cell::RefCell;

use crate::class_instance::ClassInstance;

pub type JavaChar = u16;

#[derive(Clone)]
pub enum JavaValue {
    Void,
    Integer(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    Char(JavaChar),
    Object(Option<Rc<RefCell<Box<dyn ClassInstance>>>>),
}

impl JavaValue {
    pub fn as_integer(&self) -> i32 {
        match self {
            JavaValue::Integer(x) => *x,
            _ => panic!("invalid value"),
        }
    }

    pub fn as_long(&self) -> i64 {
        match self {
            JavaValue::Long(x) => *x,
            _ => panic!("invalid value"),
        }
    }

    pub fn as_float(&self) -> f32 {
        match self {
            JavaValue::Float(x) => *x,
            _ => panic!("invalid value"),
        }
    }

    pub fn as_double(&self) -> f64 {
        match self {
            JavaValue::Double(x) => *x,
            _ => panic!("invalid value"),
        }
    }

    pub fn as_char(&self) -> JavaChar {
        match self {
            JavaValue::Char(x) => *x,
            _ => panic!("invalid value"),
        }
    }

    pub fn as_object(&self) -> Option<&Rc<RefCell<Box<dyn ClassInstance>>>> {
        match self {
            JavaValue::Object(x) => x.as_ref(),
            _ => panic!("invalid value"),
        }
    }
}
