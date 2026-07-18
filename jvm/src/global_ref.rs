use alloc::{boxed::Box, collections::BTreeMap, sync::Arc};
use core::{ops::Deref, sync::atomic::AtomicU64};

use parking_lot::RwLock;

use crate::{ClassInstance, ClassInstanceRef};

pub(crate) struct GlobalReferences {
    pub(crate) next_id: AtomicU64,
    pub(crate) objects: RwLock<BTreeMap<u64, Box<dyn ClassInstance>>>,
}

pub struct GlobalRef<T> {
    pub(crate) references: Arc<GlobalReferences>,
    pub(crate) id: u64,
    pub(crate) reference: ClassInstanceRef<T>,
}

impl<T> Deref for GlobalRef<T> {
    type Target = ClassInstanceRef<T>;

    fn deref(&self) -> &Self::Target {
        &self.reference
    }
}

impl<T> Drop for GlobalRef<T> {
    fn drop(&mut self) {
        self.references.objects.write().remove(&self.id);
    }
}
