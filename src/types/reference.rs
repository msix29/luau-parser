//! Position

use smol_str::SmolStr;

use super::Range;

/// A struct representing a specific point in a document. Lines and characters are
/// zero-based.
#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Reference {
    /// The file that this reference is in.
    uri: SmolStr,

    /// The position of the reference.
    range: Range,
}
