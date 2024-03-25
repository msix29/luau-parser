use std::sync::Arc;

use super::{FunctionValue, List, Location, SimpleValue, SingleToken, TableValue, TypeDefinition};

#[derive(Clone, Debug)]
pub enum Var {
    Name(String),
}

#[derive(Clone, Debug)]
pub enum FunctionCallInvoked {
    Function(Arc<PrefixExp>),
    TableMethod {
        table: Arc<PrefixExp>,
        colon: SingleToken,
        method: String,
    }
}

#[derive(Clone, Debug)]
pub enum FunctionArguments {
    String(SingleToken),
    Table(TableValue),
    List(List<Arc<Expression>>),
}

#[derive(Clone, Debug)]
pub enum PrefixExp {
    Var(Var),
    FunctionCall {
        invoked: FunctionCallInvoked,
        arguments: FunctionArguments,
    },
    ExpressionWrap {
        /// The `(` character.
        opening_parenthesis: SingleToken,

        /// The actual _[expression](Expression)_ being wrapped.
        expression: Arc<Expression>,

        // The `)` character.
        closing_parenthesis: SingleToken,
    },
}

/// An enum representing all possible values for an expression.
#[derive(Clone, Debug, Default)]
pub enum ExpressionInner {
    /// Nil value.
    #[default]
    Nil,

    /// A `true` or `false` value.
    Boolean(SimpleValue),

    /// Any number, be it a float, an unsigned integer, or an integer.
    Number(SimpleValue),

    /// A string, be it double quotes, single quotes, interpolated string, or multi-line.
    //TODO: Support custom interpolated string for better diagnostics.
    String(SimpleValue),

    /// An **anonymous** function.
    Function(FunctionValue),

    /// A function call.
    ///
    /// ```lua
    /// local foo = bar()
    /// ```
    FunctionCall(PrefixExp),

    ExpressionWrap(PrefixExp),

    Var(Var),

    /// A Table `{...}`.
    Table(TableValue),

    /// A unary expression, these are expressions that have an operator before the actual
    /// expression, ex:
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
    /// local qux = 5 // 2
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

    /// The _[expression](Expression)_ that this statement would resolve to if `condition`
    /// is truthy.
    pub expression: Arc<Expression>,
}

/// A struct representing an expression with no actual type.
#[derive(Clone, Debug, Default)]
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
