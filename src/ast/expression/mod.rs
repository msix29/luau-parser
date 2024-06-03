//! Implements various `From<>` traits for [`Expression`]s.

pub(crate) mod handle_prefix_exp;

use std::sync::Arc;
use tree_sitter::Node;

use super::type_definition::helper_functions::{
    build_function_parameters, build_function_returns, build_generics,
};
use crate::{
    prelude::{parse_block, FromNodeWithArgs},
    types::{
        Ast, ElseIfExpression, Expression, FromNode, HasRange, List, ListItem, Number, PrefixExp,
        Range, StringLiteral, Table, TableField, TableFieldValue, TableKey, Token, TypeValue,
    },
    unhandled_kind,
    utils::{get_range_from_boundaries, map_option},
};

impl FromNodeWithArgs<()> for Table {
    /// Creates a [`Table`] expression from the passed node. for [`Table`] types, pass the
    /// 3rd argument as `((), ())`.
    fn from_node(node: Node, code_bytes: &[u8], _: ()) -> Option<Self> {
        let mut index = 0;
        let fields = if let Some(field_list) = node.child_by_field_name("fieldList") {
            List::from_iter(
                field_list.children_by_field_name("field", &mut node.walk()),
                field_list,
                "sep",
                code_bytes,
                |_, node| {
                    let key = if let Some(key) = node.child_by_field_name("keyName") {
                        TableKey::String(StringLiteral::from_node(key, code_bytes)?)
                    } else if let Some(key) = node.child_by_field_name("keyExp") {
                        TableKey::Expression {
                            open_square_brackets: Token::from_node(
                                key.prev_sibling()?,
                                code_bytes,
                            )?,
                            expression: Arc::new(Expression::from_node(key, code_bytes)?),
                            close_square_brackets: Token::from_node(
                                key.next_sibling()?,
                                code_bytes,
                            )?,
                        }
                    } else {
                        index += 1;
                        TableKey::UndefinedNumber(index)
                    };

                    Some(TableField {
                        key: Arc::new(key),
                        equal_or_colon: node
                            .child_by_field_name("equal")
                            .map(|node| Token::from_node(node, code_bytes))?,
                        value: Arc::new(TableFieldValue::Expression(Expression::from_node(
                            node.child_by_field_name("value")?,
                            code_bytes,
                        )?)),
                    })
                },
            )
        } else {
            List::default()
        };

        Some(Table {
            opening_brackets: Token::from_node(
                node.child_by_field_name("opening_brackets")?,
                code_bytes,
            )?,
            fields,
            closing_brackets: Token::from_node(
                node.child_by_field_name("closing_brackets")?,
                code_bytes,
            )?,
        })
    }
}

impl Expression {
    /// Builds a list of [`expressions`](Expression) from an iterator over nodes.
    ///
    /// # Note
    ///
    /// This function assumes the passed iter iterates over valid expression nodes.
    pub(crate) fn from_nodes<'a>(
        nodes_iter: impl Iterator<Item = Node<'a>>,
        code_bytes: &[u8],
    ) -> List<Arc<Expression>> {
        let nodes = nodes_iter.collect::<Vec<Node>>();
        if nodes.is_empty() {
            return List::default();
        }

        let last_index = nodes.len() - 1;
        let mut items = Vec::new();

        for (i, node) in nodes.iter().enumerate().step_by(2) {
            let item = Arc::new(Expression::from_node(*node, code_bytes).unwrap_or_default());
            if i == last_index {
                items.push(ListItem::NonTrailing(item))
            } else if let Some(separator) = Token::from_node(nodes[i + 1], code_bytes) {
                items.push(ListItem::Trailing { item, separator })
            } else {
                items.push(ListItem::NonTrailing(item))
            }
        }

        List { items }
    }

    /// Try getting th body of this expression, will only be `Some` if this expression
    /// is a function.
    pub fn try_get_body(&self) -> Option<&Ast> {
        match self {
            Expression::Function { body, .. } => Some(body),
            _ => None,
        }
    }
}

impl From<PrefixExp> for Expression {
    fn from(value: PrefixExp) -> Self {
        match value {
            PrefixExp::Var(var) => Expression::Var(var),
            PrefixExp::FunctionCall(function_call) => Expression::FunctionCall(function_call),
            PrefixExp::ExpressionWrap(expression_wrap) => {
                Expression::ExpressionWrap(expression_wrap)
            }
        }
    }
}

impl FromNode for Expression {
    fn from_node(node: Node, code_bytes: &[u8]) -> Option<Self> {
        let kind = node.kind();
        match kind {
            "nil" => Some(Expression::Nil(Token::from_node(node, code_bytes)?)),
            "boolean" => Some(Expression::Boolean(Token::from_node(node, code_bytes)?)),
            "number" => Some(Expression::Number(Number::from_node(node, code_bytes)?)),
            "string" => Some(Expression::String(StringLiteral::from_node(
                node, code_bytes,
            )?)),
            "string_interp" => Some(Expression::String(StringLiteral::from_node(
                node, code_bytes,
            )?)),
            "anon_fn" => Some(Expression::Function {
                function_keyword: Token::from_node(
                    node.child_by_field_name("function")?,
                    code_bytes,
                )?,
                generics: build_generics(node, code_bytes),
                opening_parenthesis: Token::from_node(
                    node.child_by_field_name("opening_parenthesis")?,
                    code_bytes,
                )?,

                parameters: build_function_parameters(node, code_bytes, false),
                closing_parenthesis: Token::from_node(
                    node.child_by_field_name("closing_parenthesis")?,
                    code_bytes,
                )?,
                returns: build_function_returns(node, code_bytes),
                colon: map_option(node.child_by_field_name("colon"), |colon| {
                    Token::from_node(colon?, code_bytes)
                }),
                body: node
                    .child_by_field_name("body")
                    .map(|body| parse_block(&body, code_bytes, None))
                    .unwrap_or_default(),
                end_keyword: Token::from_node(node.child_by_field_name("end")?, code_bytes)?,
            }),
            "var" | "functionCall" | "exp_wrap" => {
                Some(Expression::from(PrefixExp::from_node(node, code_bytes)?))
            }
            "table" => Some(Expression::Table(Table::from_node(node, code_bytes, ())?)),
            "unexp" => Some(Expression::UnaryExpression {
                operator: Token::from_node(node.child_by_field_name("op")?, code_bytes)?,
                expression: Arc::new(Expression::from_node(
                    node.child_by_field_name("arg")?,
                    code_bytes,
                )?),
            }),
            "binexp" => Some(Expression::BinaryExpression {
                left: Arc::new(Expression::from_node(
                    node.child_by_field_name("arg0")?,
                    code_bytes,
                )?),
                operator: Token::from_node(node.child_by_field_name("op")?, code_bytes)?,
                right: Arc::new(Expression::from_node(
                    node.child_by_field_name("arg1")?,
                    code_bytes,
                )?),
            }),
            "cast" => Some(Expression::Cast {
                expression: Arc::new(Expression::from_node(
                    node.child_by_field_name("arg")?,
                    code_bytes,
                )?),
                cast_to: TypeValue::from_node(node.child_by_field_name("cast")?, code_bytes)
                    .map(Arc::new)?,
                operator: Token::from_node(node.child_by_field_name("op")?, code_bytes)?,
            }),
            "ifexp" => {
                let mut else_if_expressions = Vec::new();
                for node in node.children_by_field_name("elseif", &mut node.walk()) {
                    else_if_expressions.push(ElseIfExpression {
                        else_if_token: Token::from_node(node.child(0)?, code_bytes)?,
                        condition: Arc::new(Expression::from_node(node.child(1)?, code_bytes)?),
                        then_token: Token::from_node(node.child(2)?, code_bytes)?,
                        expression: Arc::new(Expression::from_node(node.child(3)?, code_bytes)?),
                    });
                }

                Some(Expression::IfExpression {
                    if_token: Token::from_node(node.child(0)?, code_bytes)?,
                    condition: Arc::new(Expression::from_node(node.child(1)?, code_bytes)?),
                    then_token: Token::from_node(node.child(2)?, code_bytes)?,
                    if_expression: Arc::new(Expression::from_node(node.child(3)?, code_bytes)?),
                    else_if_expressions: Arc::new(else_if_expressions),
                    else_token: Token::from_node(node.child_by_field_name("else")?, code_bytes)?,
                    else_expression: Arc::new(Expression::from_node(
                        node.child_by_field_name("elseExpression")?,
                        code_bytes,
                    )?),
                })
            }
            _ => unhandled_kind!(kind, "Expression"),
        }
    }
}

impl HasRange for Expression {
    fn get_range(&self) -> Range {
        match self {
            Expression::ERROR => Range::default(),
            Expression::Nil(value) => value.get_range(),
            Expression::Boolean(value) => value.get_range(),
            Expression::Number(value) => value.get_range(),
            Expression::String(value) => value.get_range(),
            Expression::Function {
                function_keyword,
                end_keyword,
                ..
            } => get_range_from_boundaries(function_keyword.get_range(), end_keyword.get_range()),
            Expression::FunctionCall(value) => value.get_range(),
            Expression::ExpressionWrap(value) => value.get_range(),
            Expression::Var(value) => value.get_range(),
            Expression::Table(value) => value.get_range(),
            Expression::UnaryExpression {
                operator,
                expression,
            } => get_range_from_boundaries(operator.get_range(), expression.get_range()),
            Expression::BinaryExpression {
                left,
                operator: _,
                right,
            } => get_range_from_boundaries(left.get_range(), right.get_range()),
            Expression::Cast {
                expression,
                operator: _,
                cast_to,
            } => get_range_from_boundaries(expression.get_range(), cast_to.get_range()),
            Expression::IfExpression {
                if_token,
                else_expression,
                ..
            } => get_range_from_boundaries(if_token.get_range(), else_expression.get_range()),
        }
    }
}
impl HasRange for ElseIfExpression {
    fn get_range(&self) -> Range {
        get_range_from_boundaries(self.else_if_token.get_range(), self.expression.get_range())
    }
}
