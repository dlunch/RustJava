use alloc::{borrow::Cow, boxed::Box, sync::Arc};

use hashbrown::HashMap;
use parking_lot::RwLock;

use crate::{class_loader::Class, field::Field, method::Method};

// a resolved method shared by the cache and every frame executing it, so a call clones no class
pub struct ResolvedMethod {
    pub class: Class,
    pub method: Box<dyn Method>,
}

pub type ResolvedField = Arc<(Class, Box<dyn Field>)>;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub enum Lookup {
    Static,
    Instance,
    // invokespecial of <init>: constructors are not inherited (JVMS 6.5 invokespecial), so only the named class is searched
    Constructor,
}

// Key of a cached resolution. `receiver` is empty except for the method selected for a virtual call on an instance of
// that receiver class (JVMS 5.4.6). A lookup borrows its strings; a stored key owns them.
#[derive(Hash, PartialEq, Eq)]
pub struct MemberKey<'a> {
    pub lookup: Lookup,
    pub receiver: Cow<'a, str>,
    pub class: Cow<'a, str>,
    pub name: Cow<'a, str>,
    pub descriptor: Cow<'a, str>,
}

impl MemberKey<'_> {
    fn into_owned(self) -> MemberKey<'static> {
        MemberKey {
            lookup: self.lookup,
            receiver: self.receiver.into_owned().into(),
            class: self.class.into_owned().into(),
            name: self.name.into_owned().into(),
            descriptor: self.descriptor.into_owned().into(),
        }
    }
}

// Successful field and method resolutions, reused like resolved constant pool entries (JVMS 5.4.3). A successful resolution
// never changes: registered classes are never replaced, a class is registered after its superclass and superinterfaces,
// and definitions are immutable.
#[derive(Default)]
pub struct ResolutionCache {
    fields: RwLock<HashMap<MemberKey<'static>, ResolvedField>>,
    methods: RwLock<HashMap<MemberKey<'static>, Arc<ResolvedMethod>>>,
}

impl ResolutionCache {
    pub fn field(&self, key: &MemberKey) -> Option<ResolvedField> {
        self.fields.read().get(key).cloned()
    }

    pub fn insert_field(&self, key: MemberKey, resolved: ResolvedField) {
        self.fields.write().insert(key.into_owned(), resolved);
    }

    pub fn method(&self, key: &MemberKey) -> Option<Arc<ResolvedMethod>> {
        self.methods.read().get(key).cloned()
    }

    pub fn insert_method(&self, key: MemberKey, resolved: Arc<ResolvedMethod>) {
        self.methods.write().insert(key.into_owned(), resolved);
    }
}
