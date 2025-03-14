use std::sync::Arc;
use luau_lexer::token::Token;
use smol_str::SmolStr;

use super::{TerminationStatement, Statement};

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
    /// The path pointing to the file that this [`CST`](Cst) represents, if any.
    /// For scopes like functions, if statements, etc. it'll be `None` but for actual
    /// files it'll always be `Some`.
    pub uri: Option<SmolStr>,

    /// The tokens in the of this [`CST`](Cst) **only**. Parent [`asts`](Cst)' tokens won't
    /// be included. The optional [`SingleToken`] is the optional semicolon after the
    /// statement.
    pub statements: Vec<(Arc<Statement>, Option<Token>)>,

    /// The [`last statement`](LastStatement) in this scope.
    pub last_statement: Option<Arc<TerminationStatement>>,

    /// The status of the [`CST`](Cst). If it isn't [`complete`](AstStatus::Complete), it's
    /// better to not use it for operations which affect the source code, like formatting;
    /// the output will have missing parts of the code.
    pub status: AstStatus,
}
