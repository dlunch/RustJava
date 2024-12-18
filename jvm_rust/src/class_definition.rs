use alloc::{
    boxed::Box,
    collections::BTreeMap,
    string::{String, ToString},
    sync::Arc,
    vec::Vec,
};
use core::{
    fmt::{self, Debug, Formatter},
    ops::{Deref, DerefMut},
};

use parking_lot::RwLock;

use classfile::ClassInfo;
use java_class_proto::JavaClassProto;
use java_constants::{FieldAccessFlags, MethodAccessFlags};
use jvm::{ClassDefinition, ClassInstance, Field, JavaValue, Method, Result};

use crate::{class_instance::ClassInstanceImpl, field::FieldImpl, method::MethodImpl};

struct ClassDefinitionInner {
    name: String,
    super_class_name: Option<String>,
    methods: Vec<MethodImpl>,
    fields: Vec<FieldImpl>,
    storage: RwLock<BTreeMap<FieldImpl, JavaValue>>, // TODO we should use field offset or something
}

#[derive(Clone)]
pub struct ClassDefinitionImpl {
    inner: Arc<ClassDefinitionInner>,
}

impl ClassDefinitionImpl {
    pub fn new(name: &str, super_class_name: Option<String>, methods: Vec<MethodImpl>, fields: Vec<FieldImpl>) -> Self {
        Self {
            inner: Arc::new(ClassDefinitionInner {
                name: name.to_string(),
                super_class_name,
                methods,
                fields,
                storage: RwLock::new(BTreeMap::new()),
            }),
        }
    }

    pub fn from_class_proto<C, Context>(proto: JavaClassProto<C>, context: Context) -> Self
    where
        C: ?Sized + 'static + Send,
        Context: Sync + Send + DerefMut + Deref<Target = C> + Clone + 'static,
    {
        let methods = proto
            .methods
            .into_iter()
            .map(|x| MethodImpl::from_method_proto(x, context.clone()))
            .collect::<Vec<_>>();

        let fields = proto.fields.into_iter().map(FieldImpl::from_field_proto).collect::<Vec<_>>();

        Self::new(proto.name, proto.parent_class.map(|x| x.to_string()), methods, fields)
    }

    pub fn from_classfile(data: &[u8]) -> Result<Self> {
        let class = ClassInfo::parse(data).unwrap(); // TODO ClassFormatError
        assert_eq!(class.magic, 0xCAFEBABE);

        let fields = class.fields.into_iter().map(FieldImpl::from_field_info).collect::<Vec<_>>();

        let methods = class.methods.into_iter().map(MethodImpl::from_method_info).collect::<Vec<_>>();

        Ok(Self::new(&class.this_class, class.super_class.map(|x| x.to_string()), methods, fields))
    }

    pub fn fields(&self) -> &[FieldImpl] {
        &self.inner.fields
    }
}

#[async_trait::async_trait]
impl ClassDefinition for ClassDefinitionImpl {
    fn name(&self) -> String {
        self.inner.name.clone()
    }

    fn super_class_name(&self) -> Option<String> {
        self.inner.super_class_name.as_ref().map(|x| x.to_string())
    }

    fn instantiate(&self) -> Result<Box<dyn ClassInstance>> {
        Ok(Box::new(ClassInstanceImpl::new(self)))
    }

    fn method(&self, name: &str, descriptor: &str, is_static: bool) -> Option<Box<dyn Method>> {
        self.inner
            .methods
            .iter()
            .find(|&method| {
                method.name() == name && method.descriptor() == descriptor && method.access_flags().contains(MethodAccessFlags::STATIC) == is_static
            })
            .map(|x| Box::new(x.clone()) as Box<dyn Method>)
    }

    fn field(&self, name: &str, descriptor: &str, is_static: bool) -> Option<Box<dyn Field>> {
        self.inner
            .fields
            .iter()
            .find(|&field| {
                field.name() == name && field.descriptor() == descriptor && field.access_flags().contains(FieldAccessFlags::STATIC) == is_static
            })
            .map(|x| Box::new(x.clone()) as Box<dyn Field>)
    }

    fn fields(&self) -> Vec<Box<dyn Field>> {
        self.inner.fields.iter().map(|x| Box::new(x.clone()) as Box<dyn Field>).collect()
    }

    fn get_static_field(&self, field: &dyn Field) -> Result<JavaValue> {
        let field = field.as_any().downcast_ref::<FieldImpl>().unwrap();

        let storage = self.inner.storage.read();
        let value = storage.get(field);

        if let Some(x) = value {
            Ok(x.clone())
        } else {
            Ok(field.r#type().default())
        }
    }

    fn put_static_field(&mut self, field: &dyn Field, value: JavaValue) -> Result<()> {
        let field = field.as_any().downcast_ref::<FieldImpl>().unwrap();

        self.inner.storage.write().insert(field.clone(), value);

        Ok(())
    }
}

impl Debug for ClassDefinitionImpl {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Class({})", self.name())
    }
}
