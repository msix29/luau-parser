//! Holding all needed information for variable declarations.

use super::{Expression, List, NormalizedName, SingleToken};

/// A struct holding data for variable declarations.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct LocalAssignment {
    /// The keyword `local`.
    pub local_token: SingleToken,

    /// The name of the variable.
    pub name_list: List<NormalizedName>,

    /// The `=`.
    pub equal_token: Option<SingleToken>,

    /// The _[expression](Expression)_ of the variable. This may be an empty string if this
    /// value was returned from a function.
    pub expressions: List<Expression>,
}
