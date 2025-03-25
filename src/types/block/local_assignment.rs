//! Holding all needed information for local assignments.

use luau_lexer::prelude::Token;

use crate::types::{Expression, List, Name, Pointer};

/// A struct holding data for local assignments.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct LocalAssignment {
    /// The `local` keyword.
    pub local_token: Token,

    /// The List of [`names`](NormalizedName) before the `=` sign.
    pub name_list: List<Name>,

    /// The `=` sign.
    pub equal_token: Option<Token>,

    /// The list of [`expressions`](Expression) after the `=` sign.
    pub expressions: List<Pointer<Expression>>,
}
