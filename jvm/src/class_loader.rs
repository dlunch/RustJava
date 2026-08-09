use alloc::{boxed::Box, sync::Arc};

use event_listener::{Event, EventListener};
use parking_lot::{Mutex, RwLock};

use crate::{
    ClassDefinition, ClassInstance, Jvm, Result,
    runtime::{JavaLangClass, JavaLangClassLoader},
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum InitState {
    NotInitialized,
    InProgress,
    Initialized,
    Erroneous,
}

pub(crate) enum InitializationAction {
    Initialize,
    Recursive,
    Wait(EventListener),
    Initialized,
    Erroneous,
}

struct ClassInitializationState {
    status: InitState,
    owner: Option<u64>,
}

struct ClassInitialization {
    state: Mutex<ClassInitializationState>,
    completed: Event,
}

#[derive(Clone)]
pub struct Class {
    pub definition: Box<dyn ClassDefinition>,
    java_class: Arc<RwLock<Option<Box<dyn ClassInstance>>>>,
    initialization: Arc<ClassInitialization>,
}

impl Class {
    pub fn new(definition: Box<dyn ClassDefinition>, java_class: Option<Box<dyn ClassInstance>>) -> Self {
        Self {
            definition,
            java_class: Arc::new(RwLock::new(java_class)),
            initialization: Arc::new(ClassInitialization {
                state: Mutex::new(ClassInitializationState {
                    status: InitState::NotInitialized,
                    owner: None,
                }),
                completed: Event::new(),
            }),
        }
    }

    pub(crate) fn initialization_action(&self, thread_id: u64) -> InitializationAction {
        let listener = self.initialization.completed.listen();
        let mut state = self.initialization.state.lock();

        match state.status {
            InitState::NotInitialized => {
                state.status = InitState::InProgress;
                state.owner = Some(thread_id);
                InitializationAction::Initialize
            }
            InitState::InProgress if state.owner == Some(thread_id) => InitializationAction::Recursive,
            InitState::InProgress => InitializationAction::Wait(listener),
            InitState::Initialized => InitializationAction::Initialized,
            InitState::Erroneous => InitializationAction::Erroneous,
        }
    }

    pub(crate) fn finish_initialization(&self, status: InitState) {
        {
            let mut state = self.initialization.state.lock();
            state.status = status;
            state.owner = None;
        }
        self.initialization.completed.notify(usize::MAX);
    }

    pub fn set_java_class(&self, java_class: Box<dyn ClassInstance>) {
        *self.java_class.write() = Some(java_class);
    }

    pub fn java_class(&self) -> Box<dyn ClassInstance> {
        self.java_class.read().clone().unwrap()
    }
}

#[async_trait::async_trait]
pub trait BootstrapClassLoader: Sync + Send {
    async fn load_class(&self, jvm: &Jvm, name: &str) -> Result<Option<Box<dyn ClassDefinition>>>;
}

#[async_trait::async_trait]
pub trait ClassLoaderWrapper: Sync + Send {
    async fn load_class(&self, jvm: &Jvm, name: &str) -> Result<Option<Class>>;
}

pub struct BootstrapClassLoaderWrapper<'a> {
    bootstrap_class_loader: &'a dyn BootstrapClassLoader,
}

impl<'a> BootstrapClassLoaderWrapper<'a> {
    pub fn new(bootstrap_class_loader: &'a dyn BootstrapClassLoader) -> Self {
        Self { bootstrap_class_loader }
    }
}

#[async_trait::async_trait]
impl ClassLoaderWrapper for BootstrapClassLoaderWrapper<'_> {
    async fn load_class(&self, jvm: &Jvm, name: &str) -> Result<Option<Class>> {
        let definition = self.bootstrap_class_loader.load_class(jvm, name).await?;
        if let Some(definition) = definition {
            let java_class = JavaLangClass::from_rust_class(jvm, definition.clone(), None).await?;
            let class = Class::new(definition, Some(java_class));
            jvm.register_class_internal(class.clone(), Some(self)).await?;

            Ok(Some(class))
        } else {
            Ok(None)
        }
    }
}

pub struct JavaClassLoaderWrapper {
    class_loader: Box<dyn ClassInstance>,
}

impl JavaClassLoaderWrapper {
    pub fn new(class_loader: Box<dyn ClassInstance>) -> Self {
        Self { class_loader }
    }
}

#[async_trait::async_trait]
impl ClassLoaderWrapper for JavaClassLoaderWrapper {
    async fn load_class(&self, jvm: &Jvm, name: &str) -> Result<Option<Class>> {
        let class = JavaLangClassLoader::load_class(jvm, &self.class_loader, name).await?;

        if let Some(class) = class {
            let definition = JavaLangClass::to_rust_class(jvm, &class).await?;
            Ok(Some(Class::new(definition, Some(class))))
        } else {
            Ok(None)
        }
    }
}
