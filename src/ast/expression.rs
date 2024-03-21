use std::fmt::Display;

use tree_sitter::Node;

use crate::prelude::{Expression, HasRawValue, SingleToken, TableValue, TypeDefinition};

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.get_raw_value())
    }
}

impl HasRawValue for Expression {
    fn get_raw_value(&self) -> String {
        match self {
            Expression::Nil(_) => todo!(),
            Expression::Boolean(_) => todo!(),
            Expression::Number(_) => todo!(),
            Expression::String(_) => todo!(),
            Expression::Function(_) => todo!(),
            Expression::Prefixexp => todo!(),
            Expression::Table(_) => todo!(),
            Expression::Unary {
                operator,
                expression,
            } => todo!(),
            Expression::Binary {
                left,
                operator,
                right,
            } => todo!(),
            Expression::Cast {
                expression,
                operator,
                cast_to,
            } => todo!(),
            Expression::IfExpression {
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

impl From<&str> for Expression {
    fn from(value: &str) -> Self {
        //TODO: Handle other cases.
        Self::String(crate::prelude::SimpleValue {
            value: value.to_string(),
        })
    }
}

impl Expression {
    pub fn from_nodes<'a>(
        nodes_iter: impl Iterator<Item = Node<'a>>,
        code_bytes: &[u8],
    ) -> Vec<Box<Expression>> {
        let mut values = Vec::new();

        for node in nodes_iter {
            match node.kind() {
                "nil" => values.push(Box::new(Expression::from("nil"))),
                "boolean" => values.push(Box::new(Expression::from(
                    node.utf8_text(code_bytes).unwrap(),
                ))),
                "number" => values.push(Box::new(Expression::from(
                    node.utf8_text(code_bytes).unwrap(),
                ))),
                "string" => values.push(Box::new(Expression::from(
                    node.utf8_text(code_bytes).unwrap(),
                ))),
                "string_interp" => values.push(Box::new(Expression::from(
                    node.utf8_text(code_bytes).unwrap(),
                ))),
                "anon_fn" => todo!(),
                "prefixexp" => todo!(),
                "table" => {
                    //TODO: Fill it
                    values.push(Box::new(Expression::Table(TableValue {
                        fields: Box::new(Vec::new()),
                    })));
                }
                "unexp" => println!("unexp"),
                "binexp" => println!("binexp"),
                "cast" => {
                    let temp_result = Expression::from_nodes(
                        node.children_by_field_name("arg", &mut node.walk()),
                        code_bytes,
                    );
                    let result = temp_result.iter().map(|expression| {
                        Box::new(Expression::Cast {
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
