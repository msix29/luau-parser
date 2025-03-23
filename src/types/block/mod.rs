//! Module holding all possible blocks in Luau (excluding functions).

use luau_lexer::prelude::Token;
use std::sync::Arc;

reexport!(
    do_block,
    generic_for,
    if_statement,
    local_assignment,
    numerical_for,
    function,
    repeat_block,
    set_expressions,
    statement,
    type_definition,
    while_loop,
);

#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Block {
    /// The tokens in the of this [`block`](Block) **only**. Parent
    /// [`blocks`](Block)' tokens won't be included. The optional [`Token`]
    /// is the optional semicolon after the statement.
    pub statements: Vec<(Arc<Statement>, Option<Token>)>,

    /// The [`last statement`](TerminationStatement) (aka termination statement)
    /// of this scope. The optional [`Token`] is the optional semicolon after the
    /// statement.
    pub last_statement: Option<(Arc<TerminationStatement>, Option<Token>)>,
}
