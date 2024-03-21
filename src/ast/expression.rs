use std::fmt::Display;

use tree_sitter::Node;

use crate::{
    prelude::{Expression, ExpressionInner, HasRawValue, Print, SingleToken, TableValue, TypeDefinition},
    utils::get_spaces,
};

impl Display for ExpressionInner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.get_raw_value())
    }
}
impl HasRawValue for ExpressionInner {
    fn get_raw_value(&self) -> String {
        match self {
            ExpressionInner::Nil(_) => todo!(),
            ExpressionInner::Boolean(_) => todo!(),
            ExpressionInner::Number(_) => todo!(),
            ExpressionInner::String(_) => todo!(),
            ExpressionInner::Function(_) => todo!(),
            ExpressionInner::Prefixexp => todo!(),
            ExpressionInner::Table(_) => todo!(),
            ExpressionInner::Unary {
                operator,
                expression,
            } => todo!(),
            ExpressionInner::Binary {
                left,
                operator,
                right,
            } => todo!(),
            ExpressionInner::Cast {
                expression,
                operator,
                cast_to,
            } => todo!(),
            ExpressionInner::IfExpression {
                if_token,
                condition,
                then_token,
                else_if_expressions,
                else_token,
                else_expression,
            } => todo!(),
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
    ) -> Vec<Box<ExpressionInner>> {
        let mut values = Vec::new();

        for node in nodes_iter {
            match node.kind() {
                "nil" => values.push(Box::new(ExpressionInner::from("nil"))),
                "boolean" => values.push(Box::new(ExpressionInner::from(
                    node.utf8_text(code_bytes).unwrap(),
                ))),
                "number" => values.push(Box::new(ExpressionInner::from(
                    node.utf8_text(code_bytes).unwrap(),
                ))),
                "string" => values.push(Box::new(ExpressionInner::from(
                    node.utf8_text(code_bytes).unwrap(),
                ))),
                "string_interp" => values.push(Box::new(ExpressionInner::from(
                    node.utf8_text(code_bytes).unwrap(),
                ))),
                "anon_fn" => todo!(),
                "prefixexp" => todo!(),
                "table" => {
                    //TODO: Fill it
                    values.push(Box::new(ExpressionInner::Table(TableValue {
                        fields: Box::new(Vec::new()),
                    })));
                }
                "unexp" => println!("unexp"),
                "binexp" => println!("binexp"),
                "cast" => {
                    let temp_result = ExpressionInner::from_nodes(
                        node.children_by_field_name("arg", &mut node.walk()),
                        code_bytes,
                    );
                    let result = temp_result.iter().map(|expression| {
                        Box::new(ExpressionInner::Cast {
                            expression: expression.clone(),
                            cast_to: Box::new(TypeDefinition::from((
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
                "ifexp" => println!("ifexp"),
                _ => (),
            }
        }

        values
    }
}

impl From<(Node<'_>, ExpressionInner, &[u8])> for Expression {
    fn from((node, expression_inner, code_bytes): (Node<'_>, ExpressionInner, &[u8])) -> Self {
        let (spaces_before, spaces_after) = get_spaces(node, code_bytes);

        Self {
            spaces_before,
            inner: Box::new(expression_inner),
            spaces_after,
        }
    }
}
impl From<ExpressionInner> for Expression {
    fn from(expression_inner: ExpressionInner) -> Self {
        Self {
            inner: Box::new(expression_inner),
            ..Default::default()
        }
    }
}
impl From<Box<ExpressionInner>> for Expression {
    fn from(expression_inner: Box<ExpressionInner>) -> Self {
        Self {
            inner: Box::new(*expression_inner),
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
