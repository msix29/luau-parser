//! Module holding all possible blocks in Luau (excluding functions).

use luau_lexer::prelude::Token;

use crate::types::Pointer;

reexport!(
    do_block,
    end_of_file,
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

/// A block of code that represents a single scope (and nested ones).
#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Block {
    /// The tokens in the of this [`block`](Block) **only**. Parent
    /// [`blocks`](Block)' tokens won't be included. The optional [`Token`]
    /// is the optional semicolon after the statement.
    pub statements: Vec<(Pointer<Statement>, Option<Token>)>,

    /// The [`last statement`](TerminationStatement) (aka termination statement)
    /// of this scope. The optional [`Token`] is the optional semicolon after the
    /// statement.
    pub last_statement: Option<(Pointer<TerminationStatement>, Option<Token>)>,
}
