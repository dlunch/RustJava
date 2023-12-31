use alloc::{boxed::Box, rc::Rc};
use core::cell::RefCell;

use crate::{class_instance::ClassInstance, ClassInstanceRef};

pub type JavaChar = u16;

#[derive(Clone, Debug)]
pub enum JavaValue {
    Void,
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

impl From<bool> for JavaValue {
    fn from(x: bool) -> Self {
        JavaValue::Boolean(x)
    }
}

impl From<i8> for JavaValue {
    fn from(x: i8) -> Self {
        JavaValue::Byte(x)
    }
}

impl From<JavaChar> for JavaValue {
    fn from(x: JavaChar) -> Self {
        JavaValue::Char(x)
    }
}

impl From<i16> for JavaValue {
    fn from(x: i16) -> Self {
        JavaValue::Short(x)
    }
}

impl From<i32> for JavaValue {
    fn from(x: i32) -> Self {
        JavaValue::Int(x)
    }
}

impl From<i64> for JavaValue {
    fn from(x: i64) -> Self {
        JavaValue::Long(x)
    }
}

impl From<f32> for JavaValue {
    fn from(x: f32) -> Self {
        JavaValue::Float(x)
    }
}

impl From<f64> for JavaValue {
    fn from(x: f64) -> Self {
        JavaValue::Double(x)
    }
}

impl From<ClassInstanceRef> for JavaValue {
    fn from(x: ClassInstanceRef) -> Self {
        JavaValue::Object(Some(x))
    }
}

impl From<Option<ClassInstanceRef>> for JavaValue {
    fn from(x: Option<ClassInstanceRef>) -> Self {
        JavaValue::Object(x)
    }
}
