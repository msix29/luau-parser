use std::sync::Arc;

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
    UnaryExpression {
        operator: SingleToken,
        expression: Arc<Expression>,
    },
    BinaryExpression {
        left: Arc<Expression>,
        operator: SingleToken,
        right: Arc<Expression>,
    },
    Cast {
        expression: Arc<Expression>,
        operator: SingleToken,
        cast_to: Arc<TypeDefinition>,
    },
    IfExpression {
        if_token: SingleToken,
        condition: Arc<Expression>,
        then_token: SingleToken,
        else_if_expressions: Arc<Vec<ElseIfExpression>>,
        else_token: SingleToken,
        else_expression: Arc<Expression>,
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
    pub condition: Arc<Expression>,
    pub then_token: SingleToken,
    pub expression: Arc<Expression>,
}

#[derive(Clone, Debug, Default)]
pub struct Expression {
    pub spaces_before: String,
    pub inner: Arc<ExpressionInner>,
    pub spaces_after: String,
}
