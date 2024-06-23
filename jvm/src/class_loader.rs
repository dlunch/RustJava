use alloc::{boxed::Box, sync::Arc};

use async_lock::RwLock;

use crate::{
    runtime::{JavaLangClass, JavaLangClassLoader},
    ClassDefinition, ClassInstance, Jvm, Result,
};

#[derive(Clone)]
pub struct Class {
    pub definition: Box<dyn ClassDefinition>,
    java_class: Arc<RwLock<Option<Box<dyn ClassInstance>>>>,
}

impl Class {
    pub fn new(definition: Box<dyn ClassDefinition>, java_class: Option<Box<dyn ClassInstance>>) -> Self {
        Self {
            definition,
            java_class: Arc::new(RwLock::new(java_class)),
        }
    }

    pub async fn java_class(&mut self, jvm: &Jvm) -> Result<Box<dyn ClassInstance>> {
        let java_class = self.java_class.read().await;
        if let Some(x) = &*java_class {
            Ok(x.clone())
        } else {
            drop(java_class);

            // class registered while bootstrapping might not have java/lang/Class, so instantiate it lazily
            let java_class = JavaLangClass::from_rust_class(jvm, self.definition.clone(), None).await?;

            self.java_class.write().await.replace(java_class.clone());

            Ok(java_class)
        }
    }
}

#[async_trait::async_trait]
pub trait ClassLoaderWrapper: Sync + Send {
    async fn load_class(&self, jvm: &Jvm, name: &str) -> Result<Option<Class>>;
}

#[async_trait::async_trait]
pub trait BootstrapClassLoader: Sync + Send {
    async fn load_class(&self, jvm: &Jvm, name: &str) -> Result<Option<Box<dyn ClassDefinition>>>;
}

pub struct BootstrapClassLoaderWrapper {
    load_class: Box<dyn BootstrapClassLoader>,
}

impl BootstrapClassLoaderWrapper {
    pub fn new<C>(bootstrap_class_loader: C) -> Self
    where
        C: BootstrapClassLoader + 'static,
    {
        Self {
            load_class: Box::new(bootstrap_class_loader),
        }
    }
}

#[async_trait::async_trait]
impl ClassLoaderWrapper for BootstrapClassLoaderWrapper {
    async fn load_class(&self, jvm: &Jvm, name: &str) -> Result<Option<Class>> {
        let definition = self.load_class.load_class(jvm, name).await?;
        if let Some(definition) = definition {
            Ok(Some(Class {
                definition,
                java_class: Arc::new(RwLock::new(None)),
            }))
        } else {
            Ok(None)
        }
    }
}

pub struct JavaClassLoaderWrapper {}

impl JavaClassLoaderWrapper {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait::async_trait]
impl ClassLoaderWrapper for JavaClassLoaderWrapper {
    async fn load_class(&self, jvm: &Jvm, name: &str) -> Result<Option<Class>> {
        let system_class_loader = JavaLangClassLoader::get_system_class_loader(jvm).await?;
        let class = JavaLangClassLoader::load_class(jvm, system_class_loader, name).await?;

        if let Some(class) = class {
            let definition = JavaLangClass::to_rust_class(jvm, &class).await?;
            Ok(Some(Class {
                definition,
                java_class: Arc::new(RwLock::new(Some(class))),
            }))
        } else {
            Ok(None)
        }
    }
}
