use super::{FunctionValue, SimpleValue, SingleToken, TableValue, TypeDefinition};

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
        expression: Box<ExpressionInner>,
    },
    Binary {
        left: Box<ExpressionInner>,
        operator: SingleToken,
        right: Box<ExpressionInner>,
    },
    Cast {
        expression: Box<ExpressionInner>,
        operator: SingleToken,
        cast_to: Box<TypeDefinition>,
    },
    IfExpression {
        if_token: SingleToken,
        condition: Box<ExpressionInner>,
        then_token: SingleToken,
        else_if_expressions: Box<Vec<ElseIfExpression>>,
        else_token: SingleToken,
        else_expression: Box<ExpressionInner>,
    },
}

impl Default for ExpressionInner {
    fn default() -> Self {
        Self::Nil(SimpleValue {
            value: "nil".to_string(),
        })
    }
}

#[derive(Clone, Debug)]
pub struct ElseIfExpression {
    pub else_if_token: SingleToken,
    pub condition: ExpressionInner,
    pub then_token: SingleToken,
    pub expression: ExpressionInner,
}

#[derive(Clone, Debug, Default)]
pub struct Expression {
    pub spaces_before: String,
    pub inner: Box<ExpressionInner>,
    pub spaces_after: String,
}
