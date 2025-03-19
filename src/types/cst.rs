use luau_lexer::error::ParseError;
use smol_str::SmolStr;

use super::Block;

/// An enum representing different states of an CST.
#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum AstStatus {
    /// Indicates that the parsed CST is a perfect clone of the code passed to it and that no errors has occurred.
    #[default]
    Complete,

    /// Indicates that the parsed CST is incomplete because the code had syntax errors.
    HasErrors,
}

/// A struct representing a scope in a file. This CST is lossless, meaning it can be
/// printed back to the code it was created from without losing any details.
#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Cst {
    /// The path pointing to the file that this [`CST`](Cst) represents.
    pub uri: SmolStr,

    /// The [`block`](Block) of code for this scope.
    pub last_statement: Block,

    /// All [`syntactical errors`](ParseError) in this CST.
    pub errors: Vec<ParseError>,

    /// The status of the [`CST`](Cst). If it isn't [`complete`](AstStatus::Complete), it's
    /// better to not use it for operations which affect the source code, like formatting;
    /// the output will have missing parts of the code.
    pub status: AstStatus,
}
