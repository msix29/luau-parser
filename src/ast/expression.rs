use std::{fmt::Display, sync::Arc};

use tree_sitter::Node;

use crate::{
    prelude::{
        ElseIfExpression, Expression, ExpressionInner, FunctionName, FunctionValue, HasRawValue,
        Print, SingleToken, TableField, TableFieldValue, TableKey, TableValue, TypeDefinition,
    },
    utils::get_spaces,
};

use super::type_definition::{build_function_parameters, build_function_returns};

impl Display for ExpressionInner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.get_raw_value())
    }
}
impl HasRawValue for ExpressionInner {
    fn get_raw_value(&self) -> String {
        match self {
            ExpressionInner::Nil => "nil".to_string(),
            ExpressionInner::Boolean(value) => value.get_raw_value(),
            ExpressionInner::Number(value) => value.get_raw_value(),
            ExpressionInner::String(value) => value.get_raw_value(),
            ExpressionInner::Function(value) => value.get_raw_value(),
            ExpressionInner::Prefixexp => todo!(),
            ExpressionInner::Table(value) => value.get_raw_value(),
            ExpressionInner::UnaryExpression {
                operator,
                expression,
            } => format!("{}{}", operator.get_raw_value(), expression.get_raw_value()),
            ExpressionInner::BinaryExpression {
                left,
                operator,
                right,
            } => format!(
                "{} {} {}",
                left.get_raw_value(),
                operator.get_raw_value(),
                right.get_raw_value()
            ),

            ExpressionInner::Cast {
                expression,
                operator,
                cast_to,
            } => {
                format!(
                    "{} {} {}",
                    expression,
                    operator.get_raw_value(),
                    cast_to.get_raw_value()
                )
            }
            ExpressionInner::IfExpression {
                if_token,
                condition,
                then_token,
                else_if_expressions,
                else_token,
                else_expression,
            } => {
                format!(
                    "{} {} {} {} {} {}",
                    if_token.get_raw_value(),
                    condition.get_raw_value(),
                    then_token.get_raw_value(),
                    else_if_expressions
                        .iter()
                        .map(|expression| expression.get_raw_value())
                        .collect::<Vec<String>>()
                        .join(" "),
                    else_token.get_raw_value(),
                    else_expression.get_raw_value(),
                )
            }
        }
    }
}
impl From<&str> for ExpressionInner {
    fn from(value: &str) -> Self {
        //TODO: Handle other cases.
        Self::String(crate::prelude::SimpleValue {
            value: value.to_string(),
        })
    }
}
impl ExpressionInner {
    pub fn from_nodes<'a>(
        nodes_iter: impl Iterator<Item = Node<'a>>,
        code_bytes: &[u8],
    ) -> Vec<Arc<ExpressionInner>> {
        let mut values = Vec::new();

        for node in nodes_iter {
            match node.kind() {
                "nil" => values.push(Arc::new(ExpressionInner::from("nil"))),
                "boolean" => values.push(Arc::new(ExpressionInner::from(
                    node.utf8_text(code_bytes).unwrap(),
                ))),
                "number" => values.push(Arc::new(ExpressionInner::from(
                    node.utf8_text(code_bytes).unwrap(),
                ))),
                "string" => values.push(Arc::new(ExpressionInner::from(
                    node.utf8_text(code_bytes).unwrap(),
                ))),
                "string_interp" => values.push(Arc::new(ExpressionInner::from(
                    node.utf8_text(code_bytes).unwrap(),
                ))),
                "anon_fn" => values.push(Arc::new(ExpressionInner::Function(FunctionValue {
                    local_keyword: None,
                    function_keyword: Some(SingleToken::from((
                        node.child_by_field_name("function").unwrap(),
                        code_bytes,
                    ))),
                    function_name: FunctionName::Anonymous,
                    parameters: Arc::new(build_function_parameters(node, code_bytes)),
                    returns: Arc::new(build_function_returns(node, code_bytes)),
                    body: todo!(),
                    end_keyword: todo!(),
                }))),
                "prefixexp" => todo!(),
                "table" => {
                    let mut index = 0;
                    let field_list = node.child_by_field_name("fieldList").unwrap();
                    let separators = field_list
                        .children_by_field_name("sep", &mut node.walk())
                        .collect::<Vec<Node>>();

                    values.push(Arc::new(ExpressionInner::Table(TableValue {
                        opening_brackets: SingleToken::from((
                            node.child_by_field_name("opening_brackets").unwrap(),
                            code_bytes,
                        )),
                        fields: Arc::new(
                            field_list
                                .children_by_field_name("field", &mut node.walk())
                                .enumerate()
                                .map(|(i, node)| {
                                    let key = if let Some(key) = node.child_by_field_name("keyName")
                                    {
                                        TableKey::String(
                                            key.utf8_text(code_bytes).unwrap().to_string(),
                                        )
                                    } else if let Some(key) = node.child_by_field_name("keyExp") {
                                        TableKey::Expression {
                                            open_square_brackets: SingleToken::from((
                                                key.prev_sibling().unwrap(),
                                                code_bytes,
                                            )),
                                            expression: Arc::new(Expression::from((
                                                key, code_bytes,
                                            ))),
                                            close_square_brackets: SingleToken::from((
                                                key.next_sibling().unwrap(),
                                                code_bytes,
                                            )),
                                        }
                                    } else {
                                        index += 1;
                                        TableKey::String(index.to_string())
                                    };
                                    let value = Expression::from((
                                        node.child_by_field_name("value").unwrap(),
                                        code_bytes,
                                    ));
                                    TableField {
                                        key: Arc::new(key),
                                        equal_or_colon: node
                                            .child_by_field_name("equal")
                                            .map(|node| SingleToken::from((node, code_bytes))),
                                        r#type: Arc::new(TypeDefinition::from(value.inner.clone())),
                                        value: Some(Arc::new(TableFieldValue::Expression(value))),
                                        separator: separators
                                            .get(i)
                                            .map(|node| SingleToken::from((*node, code_bytes))),
                                    }
                                })
                                .collect::<Vec<TableField>>(),
                        ),
                        closing_brackets: SingleToken::from((
                            node.child_by_field_name("opening_brackets").unwrap(),
                            code_bytes,
                        )),
                    })));
                }
                "unexp" => values.push(Arc::new(ExpressionInner::UnaryExpression {
                    operator: SingleToken::from((
                        node.child_by_field_name("op").unwrap(),
                        code_bytes,
                    )),
                    expression: Arc::new(Expression::from((
                        node.child_by_field_name("arg").unwrap(),
                        code_bytes,
                    ))),
                })),
                "binexp" => values.push(Arc::new(ExpressionInner::BinaryExpression {
                    left: Arc::new(Expression::from((
                        node.child_by_field_name("arg0").unwrap(),
                        code_bytes,
                    ))),
                    operator: SingleToken::from((
                        node.child_by_field_name("op").unwrap(),
                        code_bytes,
                    )),
                    right: Arc::new(Expression::from((
                        node.child_by_field_name("arg1").unwrap(),
                        code_bytes,
                    ))),
                })),
                "cast" => {
                    let mut cursor = node.walk();
                    let result = node.children_by_field_name("arg", &mut cursor).map(|node| {
                        Arc::new(ExpressionInner::Cast {
                            expression: Arc::new(Expression::from((node, code_bytes))),
                            cast_to: Arc::new(TypeDefinition::from((
                                node.child_by_field_name("cast").unwrap(),
                                code_bytes,
                                false,
                            ))),
                            operator: SingleToken::from((
                                node.child_by_field_name("op").unwrap(),
                                code_bytes,
                            )),
                        })
                    });
                    values.extend(result);
                }
                "ifexp" => values.push(Arc::new(ExpressionInner::IfExpression {
                    if_token: SingleToken::from((node.child(0).unwrap(), code_bytes)),
                    condition: Arc::new(Expression::from((node.child(1).unwrap(), code_bytes))),
                    then_token: SingleToken::from((node.child(2).unwrap(), code_bytes)),

                    else_if_expressions: Arc::new(
                        node.children_by_field_name("elseif", &mut node.walk())
                            .map(|node| ElseIfExpression {
                                else_if_token: SingleToken::from((
                                    node.child(0).unwrap(),
                                    code_bytes,
                                )),
                                condition: Arc::new(Expression::from((
                                    node.child(1).unwrap(),
                                    code_bytes,
                                ))),
                                then_token: SingleToken::from((node.child(2).unwrap(), code_bytes)),
                                expression: Arc::new(Expression::from((
                                    node.child(3).unwrap(),
                                    code_bytes,
                                ))),
                            })
                            .collect::<Vec<ElseIfExpression>>(),
                    ),

                    else_token: SingleToken::from((
                        node.child_by_field_name("else").unwrap(),
                        code_bytes,
                    )),
                    else_expression: Arc::new(Expression::from((
                        node.child_by_field_name("elseExpression").unwrap(),
                        code_bytes,
                    ))),
                })),
                _ => (),
            }
        }

        values
    }
}

impl From<(Node<'_>, &[u8])> for ExpressionInner {
    fn from((node, code_bytes): (Node<'_>, &[u8])) -> Self {
        match node.kind() {
            "nil" => ExpressionInner::from("nil"),
            "boolean" => ExpressionInner::from(node.utf8_text(code_bytes).unwrap()),
            "number" => ExpressionInner::from(node.utf8_text(code_bytes).unwrap()),
            "string" => ExpressionInner::from(node.utf8_text(code_bytes).unwrap()),
            "string_interp" => ExpressionInner::from(node.utf8_text(code_bytes).unwrap()),
            "anon_fn" => {
                todo!()
            }
            "prefixexp" => todo!(),
            "table" => {
                let mut index = 0;
                let field_list = node.child_by_field_name("fieldList").unwrap();
                let separators = field_list
                    .children_by_field_name("sep", &mut node.walk())
                    .collect::<Vec<Node>>();

                return ExpressionInner::Table(TableValue {
                    opening_brackets: SingleToken::from((
                        node.child_by_field_name("opening_brackets").unwrap(),
                        code_bytes,
                    )),
                    fields: Arc::new(
                        field_list
                            .children_by_field_name("field", &mut node.walk())
                            .enumerate()
                            .map(|(i, node)| {
                                let key = if let Some(key) = node.child_by_field_name("keyName") {
                                    TableKey::String(key.utf8_text(code_bytes).unwrap().to_string())
                                } else if let Some(key) = node.child_by_field_name("keyExp") {
                                    TableKey::Expression {
                                        open_square_brackets: SingleToken::from((
                                            key.prev_sibling().unwrap(),
                                            code_bytes,
                                        )),
                                        expression: Arc::new(Expression::from((key, code_bytes))),
                                        close_square_brackets: SingleToken::from((
                                            key.next_sibling().unwrap(),
                                            code_bytes,
                                        )),
                                    }
                                } else {
                                    index += 1;
                                    TableKey::String(index.to_string())
                                };

                                let value = Expression::from((
                                    node.child_by_field_name("value").unwrap(),
                                    code_bytes,
                                ));

                                TableField {
                                    key: Arc::new(key),
                                    equal_or_colon: node
                                        .child_by_field_name("equal")
                                        .map(|node| SingleToken::from((node, code_bytes))),
                                    r#type: Arc::new(TypeDefinition::from(value.inner.clone())),
                                    value: Some(Arc::new(TableFieldValue::Expression(value))),
                                    separator: separators
                                        .get(i)
                                        .map(|node| SingleToken::from((*node, code_bytes))),
                                }
                            })
                            .collect::<Vec<TableField>>(),
                    ),
                    closing_brackets: SingleToken::from((
                        node.child_by_field_name("opening_brackets").unwrap(),
                        code_bytes,
                    )),
                });
            }
            "unexp" => {
                return ExpressionInner::UnaryExpression {
                    operator: SingleToken::from((
                        node.child_by_field_name("op").unwrap(),
                        code_bytes,
                    )),
                    expression: Arc::new(Expression::from((
                        node.child_by_field_name("arg").unwrap(),
                        code_bytes,
                    ))),
                }
            }
            "binexp" => {
                return ExpressionInner::BinaryExpression {
                    left: Arc::new(Expression::from((
                        node.child_by_field_name("arg0").unwrap(),
                        code_bytes,
                    ))),
                    operator: SingleToken::from((
                        node.child_by_field_name("op").unwrap(),
                        code_bytes,
                    )),
                    right: Arc::new(Expression::from((
                        node.child_by_field_name("arg1").unwrap(),
                        code_bytes,
                    ))),
                }
            }
            "cast" => {
                let arg = node.child_by_field_name("arg").unwrap();

                ExpressionInner::Cast {
                    expression: Arc::new(Expression::from((arg, code_bytes))),
                    cast_to: Arc::new(TypeDefinition::from((
                        node.child_by_field_name("cast").unwrap(),
                        code_bytes,
                        false,
                    ))),
                    operator: SingleToken::from((
                        node.child_by_field_name("op").unwrap(),
                        code_bytes,
                    )),
                }
            }
            "ifexp" => {
                return ExpressionInner::IfExpression {
                    if_token: SingleToken::from((node.child(0).unwrap(), code_bytes)),
                    condition: Arc::new(Expression::from((node.child(1).unwrap(), code_bytes))),
                    then_token: SingleToken::from((node.child(2).unwrap(), code_bytes)),

                    else_if_expressions: Arc::new(
                        node.children_by_field_name("elseif", &mut node.walk())
                            .map(|node| ElseIfExpression {
                                else_if_token: SingleToken::from((
                                    node.child(0).unwrap(),
                                    code_bytes,
                                )),
                                condition: Arc::new(Expression::from((
                                    node.child(1).unwrap(),
                                    code_bytes,
                                ))),
                                then_token: SingleToken::from((node.child(2).unwrap(), code_bytes)),
                                expression: Arc::new(Expression::from((
                                    node.child(3).unwrap(),
                                    code_bytes,
                                ))),
                            })
                            .collect::<Vec<ElseIfExpression>>(),
                    ),

                    else_token: SingleToken::from((
                        node.child_by_field_name("else").unwrap(),
                        code_bytes,
                    )),
                    else_expression: Arc::new(Expression::from((
                        node.child_by_field_name("elseExpression").unwrap(),
                        code_bytes,
                    ))),
                }
            }
            _ => todo!(), // This case will never be met, Rust just doesn't know.
        }
    }
}

impl From<(Node<'_>, &[u8])> for Expression {
    fn from((node, code_bytes): (Node<'_>, &[u8])) -> Self {
        let (spaces_before, spaces_after) = get_spaces(node, code_bytes);

        Self {
            spaces_before,
            inner: Arc::new(ExpressionInner::from((node, code_bytes))),
            spaces_after,
        }
    }
}
impl From<(Node<'_>, ExpressionInner, &[u8])> for Expression {
    fn from((node, expression_inner, code_bytes): (Node<'_>, ExpressionInner, &[u8])) -> Self {
        let (spaces_before, spaces_after) = get_spaces(node, code_bytes);

        Self {
            spaces_before,
            inner: Arc::new(expression_inner),
            spaces_after,
        }
    }
}
impl From<ExpressionInner> for Expression {
    fn from(expression_inner: ExpressionInner) -> Self {
        Self {
            inner: Arc::new(expression_inner),
            ..Default::default()
        }
    }
}
impl From<Arc<ExpressionInner>> for Expression {
    fn from(expression_inner: Arc<ExpressionInner>) -> Self {
        Self {
            inner: expression_inner,
            ..Default::default()
        }
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.get_raw_value())
    }
}
impl HasRawValue for Expression {
    fn get_raw_value(&self) -> String {
        self.inner.get_raw_value()
    }
}
impl Print for Expression {
    fn print(&self) -> String {
        format!("{}{}{}", self.spaces_before, self.inner, self.spaces_after)
    }
    fn print_leading(&self) -> String {
        format!("{}{}", self.spaces_before, self.inner)
    }
    fn print_trailing(&self) -> String {
        format!("{}{}", self.inner, self.spaces_after)
    }
}

impl Display for ElseIfExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.get_raw_value())
    }
}
impl HasRawValue for ElseIfExpression {
    fn get_raw_value(&self) -> String {
        format!(
            "{} {} {} {}",
            self.else_if_token.get_raw_value(),
            self.condition.get_raw_value(),
            self.then_token.get_raw_value(),
            self.expression.get_raw_value(),
        )
    }
}
