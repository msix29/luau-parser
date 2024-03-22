use super::{FunctionValue, SimpleValue, SingleToken, TableValue, TypeDefinition};

#[derive(Clone, Debug)]
pub enum ExpressionInner {
    Nil,
    Boolean(SimpleValue),
    Number(SimpleValue),
    String(SimpleValue),
    Function(FunctionValue),
    Prefixexp, //TODO:
    Table(TableValue),
    Unary {
        operator: SingleToken,
        expression: Box<Expression>,
    },
    Binary {
        left: Box<Expression>,
        operator: SingleToken,
        right: Box<Expression>,
    },
    Cast {
        expression: Box<Expression>,
        operator: SingleToken,
        cast_to: Box<TypeDefinition>,
    },
    IfExpression {
        if_token: SingleToken,
        condition: Box<Expression>,
        then_token: SingleToken,
        else_if_expressions: Box<Vec<ElseIfExpression>>,
        else_token: SingleToken,
        else_expression: Box<Expression>,
    },
}

impl Default for ExpressionInner {
    fn default() -> Self {
        Self::Nil
    }
}

#[derive(Clone, Debug)]
pub struct ElseIfExpression {
    pub else_if_token: SingleToken,
    pub condition: Box<Expression>,
    pub then_token: SingleToken,
    pub expression: Box<Expression>,
}

#[derive(Clone, Debug, Default)]
pub struct Expression {
    pub spaces_before: String,
    pub inner: Box<ExpressionInner>,
    pub spaces_after: String,
}
