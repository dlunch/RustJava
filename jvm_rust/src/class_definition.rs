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

use classfile::{AttributeInfo, ClassFileError, ClassInfo, ConstantPoolReference, Opcode};
use java_class_proto::JavaClassProto;
use java_constants::{ClassAccessFlags, FieldAccessFlags, MethodAccessFlags};
use jvm::{ClassDefinition, ClassInstance, Field, JavaType, JavaValue, Jvm, Method, Result};

use crate::{class_instance::ClassInstanceImpl, field::FieldImpl, method::MethodImpl};

struct ClassDefinitionInner {
    name: String,
    super_class_name: Option<String>,
    interfaces: Vec<String>,
    access_flags: ClassAccessFlags,
    methods: Vec<MethodImpl>,
    fields: Vec<FieldImpl>,
    constant_values: Vec<(FieldImpl, ConstantPoolReference)>,
    storage: RwLock<BTreeMap<FieldImpl, JavaValue>>, // TODO we should use field offset or something
}

#[derive(Clone)]
pub struct ClassDefinitionImpl {
    inner: Arc<ClassDefinitionInner>,
}

impl ClassDefinitionImpl {
    pub fn new(
        name: &str,
        super_class_name: Option<String>,
        interfaces: Vec<String>,
        access_flags: ClassAccessFlags,
        methods: Vec<MethodImpl>,
        fields: Vec<FieldImpl>,
    ) -> Self {
        Self::with_constant_values(name, super_class_name, interfaces, access_flags, methods, fields, Vec::new())
    }

    #[allow(clippy::too_many_arguments)]
    fn with_constant_values(
        name: &str,
        super_class_name: Option<String>,
        interfaces: Vec<String>,
        access_flags: ClassAccessFlags,
        methods: Vec<MethodImpl>,
        fields: Vec<FieldImpl>,
        constant_values: Vec<(FieldImpl, ConstantPoolReference)>,
    ) -> Self {
        Self {
            inner: Arc::new(ClassDefinitionInner {
                name: name.to_string(),
                super_class_name,
                interfaces,
                access_flags,
                methods,
                fields,
                constant_values,
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

        let interfaces = proto.interfaces.into_iter().map(|x| x.to_string()).collect();

        Self::new(
            proto.name,
            proto.parent_class.map(|x| x.to_string()),
            interfaces,
            proto.access_flags,
            methods,
            fields,
        )
    }

    pub fn from_classfile(data: &[u8]) -> core::result::Result<Self, ClassFileError> {
        let class = ClassInfo::parse(data)?;

        if class.this_class.is_empty()
            || class.this_class.starts_with('[')
            || class.super_class.as_ref().is_some_and(|name| name.is_empty() || name.starts_with('['))
            || class.interfaces.iter().any(|name| name.is_empty() || name.starts_with('['))
        {
            return Err(ClassFileError::InvalidFormat);
        }
        for field in &class.fields {
            let Some(r#type) = JavaType::try_parse(&field.descriptor) else {
                return Err(ClassFileError::InvalidFormat);
            };
            if matches!(r#type, JavaType::Void | JavaType::Method(_, _)) {
                return Err(ClassFileError::InvalidFormat);
            }

            let constant_values = field
                .attributes
                .iter()
                .filter_map(|attribute| match attribute {
                    AttributeInfo::ConstantValue(value) => Some(value),
                    _ => None,
                })
                .collect::<Vec<_>>();
            if constant_values.len() > 1
                || constant_values.first().is_some_and(|value| {
                    !matches!(
                        (field.descriptor.as_str(), *value),
                        ("Z" | "B" | "C" | "S" | "I", ConstantPoolReference::Integer(_))
                            | ("J", ConstantPoolReference::Long(_))
                            | ("F", ConstantPoolReference::Float(_))
                            | ("D", ConstantPoolReference::Double(_))
                            | ("Ljava/lang/String;", ConstantPoolReference::String(_))
                    )
                })
            {
                return Err(ClassFileError::InvalidFormat);
            }
        }
        for method in &class.methods {
            if !matches!(JavaType::try_parse(&method.descriptor), Some(JavaType::Method(_, _))) {
                return Err(ClassFileError::InvalidFormat);
            }

            for attribute in &method.attributes {
                let AttributeInfo::Code(code) = attribute else {
                    continue;
                };
                for opcode in code.code.values() {
                    match opcode {
                        Opcode::Getfield(ConstantPoolReference::Field(reference))
                        | Opcode::Getstatic(ConstantPoolReference::Field(reference))
                        | Opcode::Putfield(ConstantPoolReference::Field(reference))
                        | Opcode::Putstatic(ConstantPoolReference::Field(reference)) => {
                            let Some(r#type) = JavaType::try_parse(&reference.descriptor) else {
                                return Err(ClassFileError::InvalidFormat);
                            };
                            if reference.class.is_empty()
                                || reference.class.starts_with('[')
                                || matches!(r#type, JavaType::Void | JavaType::Method(_, _))
                            {
                                return Err(ClassFileError::InvalidFormat);
                            }
                        }
                        Opcode::Invokeinterface(ConstantPoolReference::InterfaceMethodref(reference), _, _)
                        | Opcode::Invokespecial(ConstantPoolReference::Method(reference))
                        | Opcode::Invokestatic(ConstantPoolReference::Method(reference)) => {
                            if reference.class.is_empty()
                                || reference.class.starts_with('[')
                                || !matches!(JavaType::try_parse(&reference.descriptor), Some(JavaType::Method(_, _)))
                            {
                                return Err(ClassFileError::InvalidFormat);
                            }
                        }
                        Opcode::Invokevirtual(ConstantPoolReference::Method(reference)) => {
                            if reference.class.is_empty()
                                || (reference.class.starts_with('[') && !matches!(JavaType::try_parse(&reference.class), Some(JavaType::Array(_))))
                                || !matches!(JavaType::try_parse(&reference.descriptor), Some(JavaType::Method(_, _)))
                            {
                                return Err(ClassFileError::InvalidFormat);
                            }
                        }
                        Opcode::Anewarray(ConstantPoolReference::Class(name))
                        | Opcode::Checkcast(ConstantPoolReference::Class(name))
                        | Opcode::Instanceof(ConstantPoolReference::Class(name))
                        | Opcode::Ldc(ConstantPoolReference::Class(name))
                        | Opcode::LdcW(ConstantPoolReference::Class(name))
                        | Opcode::New(ConstantPoolReference::Class(name)) => {
                            if name.is_empty()
                                || (name.starts_with('[') && !matches!(JavaType::try_parse(name), Some(JavaType::Array(_))))
                                || (!name.starts_with('[') && name.contains(['.', ';', '[']))
                            {
                                return Err(ClassFileError::InvalidFormat);
                            }
                        }
                        Opcode::Multianewarray(ConstantPoolReference::Class(name), dimensions) => {
                            let Some(mut r#type) = JavaType::try_parse(name) else {
                                return Err(ClassFileError::InvalidFormat);
                            };
                            let mut available_dimensions = 0;
                            while let JavaType::Array(element) = r#type {
                                available_dimensions += 1;
                                r#type = *element;
                            }
                            if available_dimensions < *dimensions as usize {
                                return Err(ClassFileError::InvalidFormat);
                            }
                        }
                        Opcode::Invokedynamic(_) => return Err(ClassFileError::InvalidFormat),
                        _ => {}
                    }
                }
            }

            let code_attributes = method
                .attributes
                .iter()
                .filter(|attribute| matches!(attribute, AttributeInfo::Code(_)))
                .count();
            if method.access_flags.intersects(MethodAccessFlags::ABSTRACT | MethodAccessFlags::NATIVE) {
                if code_attributes != 0 {
                    return Err(ClassFileError::InvalidFormat);
                }
            } else if code_attributes != 1 {
                return Err(ClassFileError::InvalidFormat);
            }
        }

        let mut constant_values = Vec::new();
        let fields = class
            .fields
            .into_iter()
            .map(|field_info| {
                let constant = field_info.attributes.iter().find_map(|x| match x {
                    AttributeInfo::ConstantValue(value) => Some(value.clone()),
                    _ => None,
                });

                let field = FieldImpl::from_field_info(field_info);
                if let Some(x) = constant
                    && field.access_flags().contains(FieldAccessFlags::STATIC)
                {
                    constant_values.push((field.clone(), x));
                }

                field
            })
            .collect::<Vec<_>>();

        let methods = class.methods.into_iter().map(MethodImpl::from_method_info).collect::<Vec<_>>();

        let interfaces = class.interfaces.into_iter().map(|x| x.to_string()).collect();

        Ok(Self::with_constant_values(
            &class.this_class,
            class.super_class.map(|x| x.to_string()),
            interfaces,
            class.access_flags,
            methods,
            fields,
            constant_values,
        ))
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

    fn interface_names(&self) -> Vec<String> {
        self.inner.interfaces.clone()
    }

    fn access_flags(&self) -> ClassAccessFlags {
        self.inner.access_flags
    }

    async fn instantiate(&self, _: &Jvm) -> Result<Box<dyn ClassInstance>> {
        Ok(Box::new(ClassInstanceImpl::new(self)))
    }

    async fn prepare(&self, jvm: &Jvm) -> Result<()> {
        for (field, constant) in &self.inner.constant_values {
            let value = match constant {
                ConstantPoolReference::Integer(x) => match field.descriptor().as_str() {
                    "Z" => JavaValue::Boolean(*x != 0),
                    "B" => JavaValue::Byte(*x as i8),
                    "C" => JavaValue::Char(*x as u16),
                    "S" => JavaValue::Short(*x as i16),
                    _ => JavaValue::Int(*x),
                },
                ConstantPoolReference::Long(x) => JavaValue::Long(*x),
                ConstantPoolReference::Float(x) => JavaValue::Float(*x),
                ConstantPoolReference::Double(x) => JavaValue::Double(*x),
                ConstantPoolReference::String(x) => JavaValue::Object(Some(jvm.intern_string(x).await?)),
                _ => continue,
            };

            self.inner.storage.write().insert(field.clone(), value);
        }

        Ok(())
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
            Ok(JavaType::parse(&field.descriptor()).default())
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
