//! Types representing all valid Luau expressions.

use std::sync::Arc;

use super::{FunctionValue, List, Location, SingleToken, TableKey, TableValue, TypeDefinition};

/// An enum representing different ways a table can be used.
#[derive(Clone, Debug)]
pub enum TableAccessPrefix {
    /// Just a simple access.
    ///
    /// ```lua
    /// local _ = t.name
    /// ```
    Name(String),

    /// A nested table access.
    ///
    /// ```lua
    /// local _ = t.name.name2
    /// ```
    TableAccess(Arc<TableAccess>),

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
#[derive(Clone, Debug)]
pub struct TableAccess {
    /// The actual table being indexed
    pub prefix: TableAccessPrefix,

    /// The final key accessed by the expression.
    ///
    /// ```lua
    /// local _ = t.a.b.c
    /// ```
    ///
    /// It'll be `c` in this case.
    pub last_accessed_key: Arc<TableKey>,
}

/// Possible ways in which a variable can be used.
#[derive(Clone, Debug)]
pub enum Var {
    /// A simple reference to the variable.
    ///
    /// ```lua
    /// local _ = foo
    /// ```
    Name(String),

    /// A variable accessed from a table.
    ///
    /// ```lua
    /// local _ = t.foo
    /// ```
    TableAccess(TableAccess),
}

/// Different ways a function can be called.
#[derive(Clone, Debug)]
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
        colon: SingleToken,
        /// The actual name of the method being called.
        method: String,
    },
}

/// A struct representing a function call.
///
/// ```lua
/// local _ = foo(1, 2, 3)
/// ```
#[derive(Clone, Debug)]
pub struct FunctionCall {
    /// The function being called.
    pub invoked: FunctionCallInvoked,

    /// The arguments passed to the function.
    pub arguments: FunctionArguments,
}

/// All possible arguments that can be passed to a function.
#[derive(Clone, Debug)]
pub enum FunctionArguments {
    /// A string.
    ///
    /// ```lua
    /// local _ = foo"Hello, World!"
    /// ```
    String(SingleToken),

    /// A table.
    ///
    /// ```lua
    /// local _ = foo { bar = "Hello, World!" }
    /// ```
    Table(TableValue),

    /// A list of items.
    ///
    /// ```lua
    /// local _ = foo(1, 2, 3)
    /// ```
    List {
        /// The `(` character.
        open_parenthesis: SingleToken,

        /// List of arguments passed to the function.
        arguments: List<Expression>,

        /// The `)` character.
        close_parenthesis: SingleToken,
    },
}

/// A struct representing an expression wrapped in parenthesis.
#[derive(Clone, Debug)]
pub struct ExpressionWrap {
    /// The `(` character.
    pub opening_parenthesis: SingleToken,

    /// The actual _[expression](Expression)_ being wrapped.
    pub expression: Arc<Expression>,

    /// The `)` character.
    pub closing_parenthesis: SingleToken,
}

/// Part of expressions that are usually at the start of actual expressions.
///
/// ```lua
/// local _ = foo
/// local _ = foo()
/// local _ = (foo)
/// ```
#[derive(Clone, Debug)]
pub enum PrefixExp {
    /// A normal variable.
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
#[derive(Clone, Debug)]
pub enum ExpressionInner {
    /// Nil value.
    Nil(SingleToken),

    /// A `true` or `false` value.
    Boolean(SingleToken),

    /// Any number, be it a float, an unsigned integer, or an integer.
    Number(SingleToken),

    /// A string, be it double quotes, single quotes, interpolated string, or multi-line.
    //TODO: Support interpolated string as a type by itself for better diagnostics.
    String(SingleToken),

    /// An **anonymous** function.
    ///
    /// ```lua
    /// local foo = function(arg1: number): boolean
    /// end
    /// ```
    Function(FunctionValue),

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
    Table(TableValue),

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
        operator: SingleToken,

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
        operator: SingleToken,

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
        operator: SingleToken,

        /// The type that's being casted to.
        cast_to: Arc<TypeDefinition>,
    },

    /// An if expression.
    IfExpression {
        /// The `if` keyword.
        if_token: SingleToken,

        /// The condition after the `if` keyword.
        condition: Arc<Expression>,

        /// The `then` keyword after the condition.
        then_token: SingleToken,

        /// All `elseif` expressions.
        else_if_expressions: Arc<Vec<ElseIfExpression>>,

        /// The final part of the expression, the `else` keyword.
        else_token: SingleToken,

        /// The final value if all other conditions were not met.
        else_expression: Arc<Expression>,
    },
}

/// A struct representing an elseif **expression**, only exists in variable declarations.
#[derive(Clone, Debug)]
pub struct ElseIfExpression {
    /// The `elseif` keyword.
    pub else_if_token: SingleToken,

    /// The condition after the `elseif`.
    pub condition: Arc<Expression>,

    /// The `then` keyword after the condition.
    pub then_token: SingleToken,

    /// The _[expression](Expression)_ that this statement would resolve to if the
    /// _[condition](ElseIfExpression::condition)_ evaluated to `true`.
    pub expression: Arc<Expression>,
}

/// A struct representing an expression with no actual type.
#[derive(Clone, Debug)]
pub struct Expression {
    /// All Spaces before the expression.
    pub spaces_before: String,

    /// The actual vlaue of the expression.
    pub inner: Arc<ExpressionInner>,

    /// All spaces after the expression.
    pub spaces_after: String,

    /// Exact location of the node
    pub location: Location,
}
