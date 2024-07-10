//! `Impl`s for [`Reference`] and [`References`].

use std::{
    ops::Deref,
    sync::{LockResult, MutexGuard, PoisonError},
};

use crate::prelude::{Reference, References, ReferencesInner};

/// The type describing the [`MutexGuard`] for [`References`].
pub type Guard<'a> = MutexGuard<'a, Vec<Reference>>;

impl References {
    #[inline]
    /// Create a new empty [`References`] object.
    pub fn new() -> Self {
        Self::default()
    }

    /// Obtains the lock from [`Mutex::lock`].
    pub fn lock(&self) -> LockResult<Guard> {
        self.0.lock()
    }

    /// Appends a [`reference`](Reference) to the back of the references..
    pub fn push(&self, reference: Reference) -> Result<(), PoisonError<Guard>> {
        self.lock()?.push(reference);
        Ok(())
    }
}

impl Deref for References {
    type Target = ReferencesInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
