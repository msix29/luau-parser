use super::{FunctionValue, SimpleValue, SingleToken, TableValue};

#[derive(Clone, Debug)]
pub enum ExpressionInner {
    Nil(SimpleValue),
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
        operator: SingleToken,
        expression: Box<Expression>,
    },
    IfExpression {
        if_token: SingleToken,
        condition: Box<Expression>,
        then_token: SingleToken,
        if_expression: Box<Expression>,
        else_token: SingleToken,
        else_expression: Box<Expression>,
    },
}

pub struct ElseIfExpression {
    pub else_if_token: SingleToken,
    pub condition: Expression,
    pub then_token: SingleToken,
    pub expression: Expression,
}

impl Default for ExpressionInner {
    fn default() -> Self {
        Self::Nil(SimpleValue {
            value: "nil".to_string(),
        })
    }
}

#[derive(Clone, Debug, Default)]
pub struct Expression {
    pub inner: ExpressionInner,
}
