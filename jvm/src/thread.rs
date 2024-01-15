use dyn_clone::{clone_trait_object, DynClone};

use crate::as_any::AsAny;

pub type ThreadId = usize;

pub trait ThreadContext: AsAny + DynClone {}

clone_trait_object!(ThreadContext);
