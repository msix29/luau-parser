//! Comments

use super::SingleToken;

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
pub struct Comment(pub SingleToken);
