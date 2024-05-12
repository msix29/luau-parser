//! Types representing all valid Luau expressions.

use std::sync::Arc;

use super::{
    Ast, GenericDeclaration, List, NormalizedName, Number, StringLiteral, Table, TableKey, Token,
    TypeValue,
};

/// An enum representing different ways in which a table value can be returned from.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum TableAccessPrefix {
    /// Just a simple access.
    ///
    /// ```lua
    /// local _ = t.name
    /// ```
    Name(Token),

    /// A function call
    ///
    /// ```lua
    /// local t = fn()
    /// ```
    FunctionCall(Arc<FunctionCall>),

    /// Accessing a table from `(...)`.
    ///
    /// ```lua
    /// local _ = ({ a = "Hello, World!" })
    /// local _ = (t)
    /// ```
    ExpressionWrap(Arc<ExpressionWrap>),
}

/// Represents an access to a table index.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct TableAccess {
    /// The actual table being indexed
    pub prefix: TableAccessPrefix,

    /// All keys accessed by the expression.
    ///
    /// ```lua
    /// local _ = t.a.b.c
    /// ```
    ///
    /// Will be `a`, `b`, `c` in this case.
    pub accessed_keys: Vec<TableAccessKey>,
}

/// Enum representing different ways in which a table's index can be accessed.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum TableAccessKey {
    /// An expression, this'll only have the enum [`TableKey::Expression`].
    Expression(TableKey),

    /// A simple name.
    Name {
        /// The `.` **before** the key.
        dot: Token,

        /// The actual key being accessed.
        name: Token,
    },
}

/// Name of a [`variable`](Var).
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct VariableName {
    /// The actual token holding the name.
    pub token: Token,
}

/// Possible ways in which a variable can be referenced.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum Var {
    /// A simple reference to the variable.
    ///
    /// ```lua
    /// local _ = foo
    /// ```
    Name(VariableName),

    /// A field accessed from a table.
    ///
    /// ```lua
    /// local _ = t.foo
    /// ```
    TableAccess(TableAccess),
}

/// Different ways a function can be called.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum FunctionCallInvoked {
    /// A standalone function call or one in a table.
    /// ```lua
    /// local _ = foo()
    /// local _ = t.bar()
    /// ```
    Function(Arc<PrefixExp>),

    /// A **method** in a function, a method is a function that's called with `:` instead
    /// of `.`.
    ///
    /// ```lua
    /// local _ = t:foo()
    /// ```
    TableMethod {
        /// The table this function is from.
        table: Arc<PrefixExp>,

        /// The colon between the table and the method name.
        colon: Token,

        /// The actual name of the method being called.
        method: Token,
    },
}

/// A struct representing a function call.
///
/// ```lua
/// local _ = foo(1, 2, 3)
/// ```
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct FunctionCall {
    /// The function being called.
    pub invoked: FunctionCallInvoked,

    /// The arguments passed to the function.
    pub arguments: FunctionArguments,
}

/// All possible arguments that can be passed to a function.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum FunctionArguments {
    /// A standalone string.
    ///
    /// ```lua
    /// local _ = foo"Hello, World!"
    /// ```
    String(StringLiteral),

    /// A standalone table.
    ///
    /// ```lua
    /// local _ = foo { bar = "Hello, World!" }
    /// ```
    Table(Table),

    /// A list of arguments.
    ///
    /// ```lua
    /// local _ = foo(1, 2, 3)
    /// ```
    List {
        /// The `(` character.
        open_parenthesis: Token,

        /// List of arguments passed to the function.
        arguments: List<Arc<Expression>>,

        /// The `)` character.
        close_parenthesis: Token,
    },
}

/// A struct representing an expression wrapped in parenthesis.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct ExpressionWrap {
    /// The `(` character.
    pub opening_parenthesis: Token,

    /// The actual [`expression`](Expression) being wrapped.
    pub expression: Arc<Expression>,

    /// The `)` character.
    pub closing_parenthesis: Token,
}

/// Part of expressions that are usually at the start of actual expressions.
///
/// ```lua
/// local _ = foo
/// local _ = foo()
/// local _ = (foo)
/// ```
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
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
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum Expression {
    /// Indicates that this expression had a syntax error.
    ERROR,

    /// The `nil` value.
    Nil(Token),

    /// A `true` or `false` value.
    Boolean(Token),

    /// Any number, be it a float, an unsigned integer, an integer or a hex digit.
    Number(Number),

    /// A string, be it double quotes, single quotes, interpolated string, or multi-line.
    //TODO: Support interpolated string as a type by itself for better diagnostics?
    String(StringLiteral),

    /// An **anonymous** function.
    ///
    /// ```lua
    /// local foo = function(arg1: number): boolean
    /// end
    /// ```
    Function {
        /// The `function` keyword at the start
        function_keyword: Token,

        /// The generics of this function.
        generics: Option<GenericDeclaration>,

        /// The `(` character.
        opening_parenthesis: Token,

        /// All [`parameters`](NormalizedName) of the function.
        parameters: List<NormalizedName>,

        /// The `)` character.
        closing_parenthesis: Token,

        /// The `:` character between closing parenthesis and returns.
        colon: Option<Token>,

        /// The return type of the function
        returns: Option<Arc<TypeValue>>,

        /// The body of the function.
        body: Ast,

        /// The `end` keyword.
        end_keyword: Token,
    },

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
        expression: Arc<Expression>,
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
        left: Arc<Expression>,

        /// The operator between the expressions.
        operator: Token,

        /// The right expression.
        right: Arc<Expression>,
    },

    /// A typecast.
    ///
    /// ```lua
    /// local foo = "hi" :: string
    /// local bar = foo :: number
    /// local qux = {} :: { number }
    /// ```
    Cast {
        /// The actual expression.
        expression: Arc<Expression>,

        /// The `::` operator.
        operator: Token,

        /// The type that's being casted to.
        cast_to: Arc<TypeValue>,
    },

    /// An if expression.
    IfExpression {
        /// The `if` keyword.
        if_token: Token,

        /// The condition after the `if` keyword.
        condition: Arc<Expression>,

        /// The `then` keyword after the condition.
        then_token: Token,

        /// The [`expression`](Expression) that this statement would resolve to if the
        /// [`condition`](Expression::IfExpression::condition) evaluated to `true`.
        if_expression: Arc<Expression>,

        /// All `elseif` expressions.
        else_if_expressions: Arc<Vec<ElseIfExpression>>,

        /// The final part of the expression, the `else` keyword.
        else_token: Token,

        /// The final value if all other conditions were not met.
        else_expression: Arc<Expression>,
    },
}

/// A struct representing an elseif **expression**, only exists in variable declarations.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct ElseIfExpression {
    /// The `elseif` keyword.
    pub else_if_token: Token,

    /// The condition after the `elseif`.
    pub condition: Arc<Expression>,

    /// The `then` keyword after the condition.
    pub then_token: Token,

    /// The [`expression`](Expression) that this statement would resolve to if the
    /// [`condition`](ElseIfExpression::condition) evaluated to `true`.
    pub expression: Arc<Expression>,
}
