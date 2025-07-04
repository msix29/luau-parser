//! # Shared types
//!
//! Module holding types that'll be used everywhere around the parser and most likely
//! outside it too, like in a formatter or a lsp.

use luau_lexer::prelude::Token;
use luau_parser_derive::{Print, Range};

use crate::types::{
    CompoundSetExpression, DoBlock, EndOfFile, Expression, FunctionCall, GenericFor,
    GlobalFunction, IfStatement, List, LocalAssignment, LocalFunction, NumericalFor, Pointer,
    RepeatBlock, SetExpression, TypeDefinition, TypeFunction, WhileLoop,
};

/// Helper macro to generate the [`Statement`] enum.
macro_rules! generate_statement {
    ($(
        $( #[$meta:meta] )*
        $name:ident($ty:ty)
    ),* $(,)?) => {
        /// All possible statements in a [`CST`](crate::types::Cst), excluding
        /// [ending ones](TerminationStatement).
        #[derive(Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord, Range, Print)]
        #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
        pub enum Statement {
            /// This statement had an error and couldn't parse anything.
            #[default]
            ERROR,

            $( $( #[$meta] )* $name(Pointer<$ty>) ,)*
        }

        impl Statement {
            pub(crate) fn __parse(
                token: luau_lexer::prelude::Token,
                lexer: &mut luau_lexer::prelude::Lexer,
                errors: &mut Vec<luau_lexer::prelude::ParseError>
            ) -> Option<Self> {
                use $crate::types::Parse as _;

                $( if let Some(value) = <$ty>::parse(token.clone(), lexer, errors) {
                    Some(Self::$name(Pointer::new(value)))
                } else )* {
                    None
                }
            }
        }
    };
}

generate_statement! {
    /// A local function.
    ///
    /// ```lua
    /// local function foo(bar: string): Qux
    /// end
    /// ```
    LocalFunction(LocalFunction),

    /// A variable declaration.
    ///
    /// ```lua
    /// local foo = bar
    /// local bar = function()
    /// end
    /// local qux = {}
    /// ```
    LocalAssignment(LocalAssignment),

    /// A type definition.
    ///
    /// ```lua
    /// type Foo = Bar<string, number>
    /// export type Bar<P, R> = (param: P) -> R
    /// type qux = {}
    /// ```
    TypeDefinition(TypeDefinition),

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
    IfStatement(IfStatement),

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
    DoBlock(DoBlock),

    /// A generic for loop.
    ///
    /// ```lua
    /// for i, v in ipairs(t) do
    ///     print(`{i}: {v}`)
    /// end
    /// ```
    GenericFor(GenericFor),

    /// A numerical for loop.
    ///
    /// ```lua
    /// for i = 1, 100, 2 do
    ///     print(i)
    /// end
    /// ```
    NumericalFor(NumericalFor),

    /// A repeat block.
    ///
    /// ```lua
    /// local i = 0
    /// repeat
    ///     print(i)
    ///     i += 1
    /// until i == 10
    /// ```
    RepeatBlock(RepeatBlock),

    /// A while loop.
    ///
    /// ```lua
    /// local i = 0
    /// while i <= 10 do
    ///     print(i)
    ///     i += 1
    /// end
    /// ```
    WhileLoop(WhileLoop),

    /// A set expression.
    ///
    /// ```lua
    /// a = "test"
    /// b, c = true, false, 1
    /// d, e, f = foo()
    /// ```
    SetExpression(SetExpression),

    /// A compound set expression.
    ///
    /// ```lua
    /// foo += 1
    /// bar //= 2
    /// ```
    CompoundSetExpression(CompoundSetExpression),

    /// A function call.
    ///
    /// ```lua
    /// local _ = foo(1, 2, 3)
    /// ```
    FunctionCall(FunctionCall),

    /// A global function.
    ///
    /// ```lua
    /// function foo(bar: string): Qux
    /// end
    /// function foo:Qux(bar: string): Qux
    /// end
    /// ```
    GlobalFunction(GlobalFunction),

    /// A type function
    ///
    /// ```lua
    /// type function Foo(ty)
    ///     return types.unionof(types.string, ty)
    /// end
    ///
    /// export type function Bar(ty)
    ///     return types.unionof(types.number, ty)
    /// end
    ///
    /// type qux = Bar<Foo<boolean>> -- number | string | boolean
    /// ```
    TypeFunction(TypeFunction),

    /// The [`EndOfFile`] token. Read it's documentation for it's uses.
    EndOfFile(EndOfFile),
}

/// An enum representing different types of statements that can end a block of code.
/// These statements may or may not be present.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Range)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum TerminationStatement {
    /// The `break` keyword.
    ///
    /// ```lua
    /// break
    /// ```
    Break(Token),

    /// The `continue` keyword.
    ///
    /// ```lua
    /// continue
    /// ```
    Continue(Token),

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
        #[range_or = "return_keyword"]
        expressions: Option<List<Pointer<Expression>>>,
    },
}
