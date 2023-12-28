use alloc::{boxed::Box, rc::Rc};
use core::cell::RefCell;

use crate::class_instance::ClassInstance;

pub type JavaChar = u16;

#[derive(Clone, Debug)]
pub enum JavaValue {
    Boolean(bool),
    Byte(i8),
    Char(JavaChar),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    Object(Option<Rc<RefCell<Box<dyn ClassInstance>>>>),
}

impl JavaValue {
    pub fn as_boolean(&self) -> bool {
        match self {
            JavaValue::Boolean(x) => *x,
            _ => panic!("invalid value"),
        }
    }

    pub fn as_byte(&self) -> i8 {
        match self {
            JavaValue::Byte(x) => *x,
            _ => panic!("invalid value"),
        }
    }

    pub fn as_char(&self) -> JavaChar {
        match self {
            JavaValue::Char(x) => *x,
            _ => panic!("invalid value"),
        }
    }

    pub fn as_short(&self) -> i16 {
        match self {
            JavaValue::Short(x) => *x,
            _ => panic!("invalid value"),
        }
    }

    pub fn as_int(&self) -> i32 {
        match self {
            JavaValue::Int(x) => *x,
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

    pub fn as_object_ref(&self) -> Option<&Rc<RefCell<Box<dyn ClassInstance>>>> {
        match self {
            JavaValue::Object(x) => x.as_ref(),
            _ => panic!("invalid value"),
        }
    }

    pub fn as_object(self) -> Option<Rc<RefCell<Box<dyn ClassInstance>>>> {
        match self {
            JavaValue::Object(x) => x,
            _ => panic!("invalid value"),
        }
    }
}
