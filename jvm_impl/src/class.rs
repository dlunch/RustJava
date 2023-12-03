use alloc::{
    boxed::Box,
    string::{String, ToString},
    vec::Vec,
};

use classfile::ClassInfo;
use jvm::{Class, ClassInstance, Field, JavaValue, JvmResult, Method};

use crate::{class_instance::ClassInstanceImpl, field::FieldImpl, method::MethodImpl};

pub struct ClassImpl {
    name: String,
    methods: Vec<MethodImpl>,
    fields: Vec<FieldImpl>,
    storage: Vec<JavaValue>,
}

impl ClassImpl {
    pub fn new(name: &str, methods: Vec<MethodImpl>, fields: Vec<FieldImpl>) -> Self {
        let storage = fields.iter().filter(|x| x.is_static()).map(|x| x.r#type().default()).collect();

        Self {
            name: name.to_string(),
            methods,
            fields,
            storage,
        }
    }

    pub fn from_classfile(data: &[u8]) -> JvmResult<Self> {
        let class = ClassInfo::parse(data)?;

        let fields = class
            .fields
            .into_iter()
            .scan(0, |index, field| {
                let field = FieldImpl::from_fieldinfo(field, *index);
                *index += 1;

                Some(field)
            })
            .collect::<Vec<_>>();

        let methods = class.methods.into_iter().map(MethodImpl::from_methodinfo).collect::<Vec<_>>();

        Ok(Self::new(&class.this_class, methods, fields))
    }
}

impl Class for ClassImpl {
    fn get_static_field(&self, field: &dyn Field) -> JvmResult<JavaValue> {
        let field = field.as_any().downcast_ref::<FieldImpl>().unwrap();

        Ok(self.storage[field.index()].clone())
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn fields(&self) -> Vec<&dyn Field> {
        self.fields.iter().map(|x| x as &dyn jvm::Field).collect()
    }

    fn methods(&self) -> Vec<&dyn Method> {
        self.methods.iter().map(|x| x as &dyn jvm::Method).collect()
    }

    fn instantiate(&self) -> Box<dyn ClassInstance> {
        Box::new(ClassInstanceImpl::new(self))
    }
}
