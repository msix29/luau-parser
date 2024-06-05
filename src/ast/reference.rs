use std::{
    ops::Deref,
    sync::{LockResult, MutexGuard},
};

use crate::prelude::{Reference, References, ReferencesInner};

impl References {
    #[inline]
    /// Create a new empty [`References`] object.
    pub fn new() -> Self {
        Self::default()
    }

    /// Obtains the lock from [`Mutex::lock`].
    pub fn lock(&self) -> LockResult<MutexGuard<'_, Vec<Reference>>> {
        self.0.lock()
    }
}

impl Deref for References {
    type Target = ReferencesInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
