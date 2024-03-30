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

#[derive(Clone, Debug)]
pub struct Comment(pub SingleToken);
