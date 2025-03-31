//! Types representing all valid Luau expressions.

use luau_lexer::prelude::Token;
use luau_parser_derive::Range;

use crate::types::{Bracketed, Pointer, Table, TypeValue};

reexport!(table, var, function);

/// A struct representing an expression wrapped in parenthesis.
pub type ExpressionWrap = Bracketed<Pointer<Expression>>;

/// Part of expressions that are usually at the start of actual expressions.
///
/// ```lua
/// local _ = foo
/// local _ = foo()
/// local _ = (foo)
/// ```
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Range)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum PrefixExp {
    /// A normal variable reference.
    ///
    /// ```lua
    /// local _ = foo
    /// ```
    Var(Var),

    /// A function call.
    /// ```lua
    /// local _ = foo()
    /// ```
    FunctionCall(FunctionCall),

    /// An expression wrapped in parenthesis
    ///
    /// ```lua
    /// local _ = (foo)
    /// ```
    ExpressionWrap(ExpressionWrap),
}

/// An enum representing all possible values for an expression.
#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord, Range)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum Expression {
    /// This [`Expression`] had a syntax error.
    #[default]
    ERROR,

    /// The `nil` value.
    Nil(Token),

    /// A `true` or `false` value.
    Boolean(Token),

    /// Any number, be it a float, an unsigned integer, an integer or a hex digit.
    Number(Token),

    /// A string, be it double quotes, single quotes, interpolated string, or multi-line.
    //TODO: Support interpolated string as a type by itself for better diagnostics?
    String(Token),

    /// An **anonymous** function.
    ///
    /// ```lua
    /// local foo = function(arg1: number): boolean
    /// end
    /// ```
    Closure(Closure),

    /// A function call.
    ///
    /// ```lua
    /// local foo = bar()
    /// ```
    FunctionCall(FunctionCall),

    /// An expression wrapped in parenthesis.
    ///
    /// ```lua
    /// local _ = (foo)
    /// ```
    ExpressionWrap(ExpressionWrap),

    /// A reference to another variable.
    ///
    /// ```lua
    /// local _ = foo
    /// ```
    Var(Var),

    /// A Table.
    ///
    /// ```lua
    /// local _ = { foo = "bar" }
    /// ```
    Table(Table),

    /// A unary expression, these are expressions that have an operator before the actual
    /// expression:
    ///
    /// ```lua
    /// local foo = not 1
    /// local bar = -1
    /// local qux = #t
    /// ```
    UnaryExpression {
        /// The operator.
        operator: Token,

        /// The actual expression this operator is affecting.
        expression: Pointer<Expression>,
    },

    /// A binary expression, this represents any 2 expressions with an operator between
    /// them.
    ///
    /// ```lua
    /// local foo = 1 + 1
    /// local bar = 1 == 1
    /// local qux = bar // 2
    /// ```
    BinaryExpression {
        /// The left expression.
        left: Pointer<Expression>,

        /// The operator between the expressions.
        operator: Token,

        /// The right expression.
        right: Pointer<Expression>,
    },

    /// A typecast.
    ///
    /// ```lua
    /// local foo = "hi" :: string
    /// local bar = foo :: number
    /// local qux = {} :: { number }
    /// ```
    TypeCast {
        /// The actual expression.
        expression: Pointer<Expression>,

        /// The `::` operator.
        operator: Token,

        /// The type that's being casted to.
        cast_to: Pointer<TypeValue>,
    },

    /// An if expression.
    IfExpression(IfExpression),
}

/// A struct representing an elseif **expression**, only exists in variable declarations.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Range)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct IfExpression {
    /// The `if` keyword.
    pub if_keyword: Token,

    /// The condition after the `if` keyword.
    pub condition: Pointer<Expression>,

    /// The `then` keyword after the condition.
    pub then_keyword: Token,

    /// The [`expression`](Expression) that this statement would resolve to if the
    /// [`condition`](Expression::IfExpression::condition) evaluated to `true`.
    pub if_expression: Pointer<Expression>,

    /// All `elseif` expressions.
    pub else_if_expressions: Pointer<Vec<ElseIfExpression>>,

    /// The final part of the expression, the `else` keyword.
    pub else_keyword: Token,

    /// The final value if all other conditions were not met.
    pub else_expression: Pointer<Expression>,
}

/// A struct representing an elseif **expression**, only exists in expressions.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Range)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct ElseIfExpression {
    /// The `elseif` keyword.
    pub else_if_keyword: Token,

    /// The condition after the `elseif`.
    pub condition: Pointer<Expression>,

    /// The `then` keyword after the condition.
    pub then_keyword: Token,

    /// The [`expression`](Expression) that this statement would resolve to if the
    /// [`condition`](ElseIfExpression::condition) evaluated to `true`.
    pub expression: Pointer<Expression>,
}
