//! Holding all needed information for local assignments.

use std::sync::Arc;

use super::{Expression, List, NormalizedName, SingleToken};

/// A struct holding data for local assignments.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct LocalAssignment {
    /// The `local` keyword.
    pub local_token: SingleToken,

    /// The List of [`names`](NormalizedName) before the `=` sign.
    pub name_list: List<NormalizedName>,

    /// The `=` sign.
    pub equal_token: Option<SingleToken>,

    /// The list of [`expressions`](Expression) after the `=` sign.
    pub expressions: List<Arc<Expression>>,
}
