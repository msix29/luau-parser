//! # Shared types
//!
//! Module holding types that'll be used everywhere around the parser and most likely
//! outside it too, like in a formatter or a lsp.

use smol_str::SmolStr;
use luau_lexer::prelude::Token;
use std::{
    marker::PhantomData,
    sync::{Arc, Mutex, MutexGuard, RwLock, RwLockReadGuard, RwLockWriteGuard},
};

use crate::prelude::{
    CompoundSetExpression, DoBlock, Expression, FunctionCall, GenericFor, GlobalFunction,
    IfStatement, List, LocalAssignment, LocalFunction, NumericalFor, RepeatBlock, SetExpression,
    TypeDefinition, WhileLoop,
};

pub type StatementInner = (Statement, Option<Token>);
pub type StatementsInner = Arc<RwLock<Vec<StatementInner>>>;
pub type ReadGuard<'a> = RwLockReadGuard<'a, Vec<StatementInner>>;
pub type WriteGuard<'a> = RwLockWriteGuard<'a, Vec<StatementInner>>;

/// All possible tokens in an [`ast`](Ast).
#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum Statement {
    /// This statement had an error and couldn't parse anything.
    #[default]
    ERROR,

    /// A variable declaration.
    ///
    /// ```lua
    /// local foo = bar
    /// local bar = function()
    /// end
    /// local qux = {}
    /// ```
    LocalAssignment(Box<LocalAssignment>),

    /// A type definition.
    ///
    /// ```lua
    /// type Foo = Bar<string, number>
    /// export type Bar<P, R> = (param: P) -> R
    /// type qux = {}
    /// ```
    TypeDefinition(Box<TypeDefinition>),

    /// An if statement.
    ///
    /// ```lua
    /// if a then
    ///     print("It's a")
    /// elseif b then
    ///     print("It's b")
    /// else
    ///     print("It's neither a or b :(")
    /// end
    /// ```
    IfStatement(Box<IfStatement>),

    /// A do block.
    ///
    /// ```lua
    /// do
    ///     print("Hello, World!")
    /// end
    /// ```
    ///
    /// # Note
    ///
    /// This struct isn't used for while or for loops, they have their own tokens, and have
    /// do blocks as part of their token.
    DoBlock(Box<DoBlock>),

    /// A generic for loop.
    ///
    /// ```lua
    /// for i, v in ipairs(t) do
    ///     print(`{i}: {v}`)
    /// end
    /// ```
    GenericFor(Box<GenericFor>),

    /// A numerical for loop.
    ///
    /// ```lua
    /// for i = 1, 100, 2 do
    ///     print(i)
    /// end
    /// ```
    NumericalFor(Box<NumericalFor>),

    /// A repeat block.
    ///
    /// ```lua
    /// local i = 0
    /// repeat
    ///     print(i)
    ///     i += 1
    /// until i == 10
    /// ```
    RepeatBlock(Box<RepeatBlock>),

    /// A while loop.
    ///
    /// ```lua
    /// local i = 0
    /// while i <= 10 do
    ///     print(i)
    ///     i += 1
    /// end
    /// ```
    WhileLoop(Box<WhileLoop>),

    /// A set expression.
    ///
    /// ```lua
    /// a = "test"
    /// b, c = true, false, 1
    /// d, e, f = foo()
    /// ```
    SetExpression(Box<SetExpression>),

    /// A compound set expression.
    ///
    /// ```lua
    /// foo += 1
    /// bar //= 2
    /// ```
    CompoundSetExpression(Box<CompoundSetExpression>),

    /// A function call.
    ///
    /// ```lua
    /// local _ = foo(1, 2, 3)
    /// ```
    FunctionCall(Box<FunctionCall>),

    /// A local function.
    ///
    /// ```lua
    /// local function foo(bar: string): Qux
    /// end
    /// ```
    LocalFunction(Box<LocalFunction>),

    /// A global function.
    ///
    /// ```lua
    /// function foo(bar: string): Qux
    /// end
    /// function foo:Qux(bar: string): Qux
    /// end
    /// ```
    GlobalFunction(Box<GlobalFunction>),
}

/// An enum representing different types of statements that can end a block of code.
/// These statements may or may not be present.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum LastStatement {
    /// The `break` keyword. The first is the `break` word and the second is the optional
    /// `;` after it.
    ///
    /// ```lua
    /// break
    /// ```
    Break((Token, Option<Token>)),

    /// The `continue` keyword. The first is the `continue` word and the second is the
    /// optional `;` after it.
    ///
    /// ```lua
    /// continue
    /// ```
    Continue((Token, Option<Token>)),

    /// A `return` statement. Can be in multiple forms:
    ///
    /// ```lua
    /// return
    /// -- or
    /// return value
    /// -- or
    /// return value1, value2, ...
    /// ```
    Return {
        /// The `return` keyword.
        return_keyword: Token,

        /// The list of expressions after it.
        expressions: List<Arc<Expression>>,

        /// The `;` character.
        semicolon: Option<Token>,
    },
}

/// An enum representing different states of an Ast.
#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum AstStatus {
    /// Indicates that the parsed Ast is a perfect clone of the code passed to it and that no errors has occurred.
    #[default]
    Complete,

    /// Indicates that the parsed Ast is incomplete because the code had syntax errors.
    HasErrors,
}

/// An enum representing printing errors that stopped [`Ast::try_print`] from working.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum PrintingError {
    /// The [`ast`](Ast) has syntax errors.
    IncompleteAst,
}

/// A struct representing a scope in a file. This ast is lossless, meaning it can be
/// printed back to the code it was created from without losing any details.
#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Ast {
    /// The path pointing to the file that this [`ast`](Ast) represents, if any.
    /// For scopes like functions, if statements, etc. it'll be `None` but for actual
    /// files it'll always be `Some`.
    pub uri: Option<SmolStr>,

    /// The tokens in the of this [`ast`](Ast) **only**. Parent [`asts`](Ast)' tokens won't
    /// be included. The optional [`SingleToken`] is the optional semicolon after the
    /// statement.
    pub statements: Vec<(Arc<Statement>, Option<Token>)>,

    /// The [`last statement`](LastStatement) in this scope.
    pub last_statement: Option<Arc<LastStatement>>,

    /// The status of the [`ast`](Ast). If it isn't [`complete`](AstStatus::Complete), it's
    /// better to not use it for operations which affect the source code, like formatting;
    /// the output will have missing parts of the code.
    pub status: AstStatus,
}
