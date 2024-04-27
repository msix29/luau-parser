//! Comments

use super::Token;

/// A struct representing a comment. Single line comments:
///
/// ```lua
/// ------ weirdly long
/// -- comment
/// ```
///
/// Multi-line comments.
///
/// ```lua
/// --[[
///     nice
/// ]]
/// ```

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Comment(pub Token);
