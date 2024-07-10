//! Reference struct.

use smol_str::SmolStr;
use std::sync::{Arc, Mutex};

use super::Range;

/// A struct representing a reference to an item.
#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Reference {
    /// The file that this reference is in.
    pub uri: Arc<SmolStr>,

    /// The position of the reference.
    pub range: Range,
}

/// The inner type of [`References`].
pub type ReferencesInner = Arc<Mutex<Vec<Reference>>>;

/// A [`Vec`] of [`Reference`]s that allows mutability.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct References(pub(crate) ReferencesInner);
