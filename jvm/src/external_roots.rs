use alloc::vec::Vec;

use crate::ClassInstance;

/// Discovers roots in a consumer's execution state during synchronous collection.
/// Implementations must not call JVM APIs, run guest code, or mutate the object inventory.
pub trait ExternalRoots: Send + Sync {
    fn roots(&self, objects: &[&dyn ClassInstance]) -> anyhow::Result<Vec<usize>>;
}
