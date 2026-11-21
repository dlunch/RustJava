use alloc::{boxed::Box, sync::Arc, vec, vec::Vec};
use core::{
    fmt::{self, Debug, Formatter},
    hash::{Hash, Hasher},
};

use parking_lot::RwLock;

use jvm::{ArrayClassDefinition, ArrayClassInstance, ArrayRawBuffer, ArrayRawBufferMut, ClassDefinition, ClassInstance, Field, JavaValue, Result};
use jvm_types::JavaType;

use crate::array_class_definition::ArrayClassDefinitionImpl;

#[derive(Clone)]
enum ArrayElements {
    Primitive(Vec<u8>),
    NonPrimitive(Vec<JavaValue>),
}

struct ArrayClassInstanceInner {
    class: Box<dyn ClassDefinition>,
    length: usize,
    element_type: JavaType,
    elements: RwLock<ArrayElements>,
}

#[derive(Clone)]
pub struct ArrayClassInstanceImpl {
    inner: Arc<ArrayClassInstanceInner>,
}

impl ArrayClassInstanceImpl {
    pub fn new(class: &ArrayClassDefinitionImpl, length: usize) -> Self {
        let element_type = JavaType::parse(&class.element_type_name());

        let elements = if matches!(element_type, JavaType::Class(_) | JavaType::Array(_)) {
            let default_value = JavaValue::default_of(&element_type);
            ArrayElements::NonPrimitive(vec![default_value; length])
        } else {
            let element_size = Self::primitive_element_size(&element_type);
            ArrayElements::Primitive(vec![0; length * element_size])
        };

        Self {
            inner: Arc::new(ArrayClassInstanceInner {
                class: Box::new(class.clone()),
                length,
                element_type,
                elements: RwLock::new(elements),
            }),
        }
    }

    fn primitive_element_size(element_type: &JavaType) -> usize {
        match element_type {
            JavaType::Boolean => 1,
            JavaType::Byte => 1,
            JavaType::Char => 2,
            JavaType::Short => 2,
            JavaType::Int => 4,
            JavaType::Long => 8,
            JavaType::Float => 4,
            JavaType::Double => 8,
            _ => unreachable!(),
        }
    }

    fn write_primitive(&self, value: JavaValue, raw: &mut [u8]) {
        match self.inner.element_type {
            JavaType::Boolean => raw[0] = bool::from(value) as u8,
            JavaType::Byte => raw[0] = i8::from(value) as u8,
            JavaType::Char => raw.copy_from_slice(&u16::from(value).to_le_bytes()),
            JavaType::Short => raw.copy_from_slice(&i16::from(value).to_le_bytes()),
            JavaType::Int => raw.copy_from_slice(&i32::from(value).to_le_bytes()),
            JavaType::Long => raw.copy_from_slice(&i64::from(value).to_le_bytes()),
            JavaType::Float => raw.copy_from_slice(&f32::from(value).to_le_bytes()),
            JavaType::Double => raw.copy_from_slice(&f64::from(value).to_le_bytes()),
            _ => unreachable!(),
        }
    }

    fn read_primitive(&self, raw: &[u8]) -> JavaValue {
        match self.inner.element_type {
            JavaType::Boolean => JavaValue::Boolean(raw[0] != 0),
            JavaType::Byte => JavaValue::Byte(raw[0] as i8),
            JavaType::Char => JavaValue::Char(u16::from_le_bytes(raw.try_into().unwrap())),
            JavaType::Short => JavaValue::Short(i16::from_le_bytes(raw.try_into().unwrap())),
            JavaType::Int => JavaValue::Int(i32::from_le_bytes(raw.try_into().unwrap())),
            JavaType::Long => JavaValue::Long(i64::from_le_bytes(raw.try_into().unwrap())),
            JavaType::Float => JavaValue::Float(f32::from_le_bytes(raw.try_into().unwrap())),
            JavaType::Double => JavaValue::Double(f64::from_le_bytes(raw.try_into().unwrap())),
            _ => unreachable!(),
        }
    }
}

#[async_trait::async_trait]
impl ClassInstance for ArrayClassInstanceImpl {
    fn destroy(self: Box<Self>) {}

    fn identity(&self) -> usize {
        Arc::as_ptr(&self.inner) as usize
    }

    fn shallow_clone(&self) -> Result<Box<dyn ClassInstance>> {
        Ok(Box::new(Self {
            inner: Arc::new(ArrayClassInstanceInner {
                class: self.inner.class.clone(),
                length: self.inner.length,
                element_type: self.inner.element_type.clone(),
                elements: RwLock::new(self.inner.elements.read().clone()),
            }),
        }))
    }

    fn class_definition(&self) -> Box<dyn ClassDefinition> {
        self.inner.class.clone()
    }

    fn equals(&self, other: &dyn ClassInstance) -> Result<bool> {
        let other = other.as_any().downcast_ref::<ArrayClassInstanceImpl>();
        if other.is_none() {
            return Ok(false);
        }
        let other = other.unwrap();

        Ok(Arc::ptr_eq(&self.inner, &other.inner))
    }

    fn as_array_instance(&self) -> Option<&dyn ArrayClassInstance> {
        Some(self)
    }

    fn as_array_instance_mut(&mut self) -> Option<&mut dyn ArrayClassInstance> {
        Some(self)
    }

    fn get_field(&self, _field: &dyn Field) -> Result<JavaValue> {
        panic!("Array classes do not have fields")
    }

    fn put_field(&mut self, _field: &dyn Field, _value: JavaValue) -> Result<()> {
        panic!("Array classes do not have fields")
    }
}

impl ArrayClassInstance for ArrayClassInstanceImpl {
    fn store(&mut self, offset: usize, values: Box<[JavaValue]>) -> Result<()> {
        match &mut *self.inner.elements.write() {
            ArrayElements::Primitive(x) => {
                let element_size = Self::primitive_element_size(&self.inner.element_type);
                let raw = &mut x[offset * element_size..(offset + values.len()) * element_size];

                for (value, raw) in values.into_vec().into_iter().zip(raw.chunks_exact_mut(element_size)) {
                    self.write_primitive(value, raw);
                }
            }
            ArrayElements::NonPrimitive(x) => {
                x.splice(offset..offset + values.len(), values.into_vec());
            }
        }

        Ok(())
    }

    fn load(&self, offset: usize, length: usize) -> Result<Vec<JavaValue>> {
        Ok(match &*self.inner.elements.read() {
            ArrayElements::Primitive(x) => {
                let element_size = Self::primitive_element_size(&self.inner.element_type);
                let values_raw = &x[offset * element_size..offset * element_size + length * element_size];

                values_raw.chunks_exact(element_size).map(|raw| self.read_primitive(raw)).collect()
            }
            ArrayElements::NonPrimitive(x) => x[offset..offset + length].to_vec(),
        })
    }

    fn raw_buffer(&self) -> Result<Box<dyn ArrayRawBuffer>> {
        Ok(Box::new(ArrayRawBufferImpl { inner: self.inner.clone() }))
    }

    fn raw_buffer_mut(&mut self) -> Result<Box<dyn ArrayRawBufferMut>> {
        Ok(Box::new(ArrayRawBufferImpl { inner: self.inner.clone() }))
    }

    fn length(&self) -> usize {
        self.inner.length
    }
}

impl Hash for ArrayClassInstanceImpl {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Arc::as_ptr(&self.inner).hash(state);
    }
}

impl Debug for ArrayClassInstanceImpl {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "ArrayClassInstance({})", self.inner.class.name())
    }
}

struct ArrayRawBufferImpl {
    inner: Arc<ArrayClassInstanceInner>,
}

impl ArrayRawBuffer for ArrayRawBufferImpl {
    fn read(&self, offset: usize, buffer: &mut [u8]) -> Result<()> {
        match &*self.inner.elements.read() {
            ArrayElements::Primitive(x) => {
                let element_size = ArrayClassInstanceImpl::primitive_element_size(&self.inner.element_type);
                let values_raw = &x[offset * element_size..offset * element_size + buffer.len() * element_size];

                buffer.copy_from_slice(values_raw);
            }
            ArrayElements::NonPrimitive(_) => {
                panic!("Expected primitive array");
            }
        }

        Ok(())
    }
}

impl ArrayRawBufferMut for ArrayRawBufferImpl {
    fn write(&mut self, offset: usize, buffer: &[u8]) -> Result<()> {
        if let ArrayElements::Primitive(x) = &mut *self.inner.elements.write() {
            let element_size = ArrayClassInstanceImpl::primitive_element_size(&self.inner.element_type);

            x[offset * element_size..offset * element_size + buffer.len() * element_size].copy_from_slice(buffer);
        } else {
            panic!("Expected primitive array");
        }

        Ok(())
    }
}
