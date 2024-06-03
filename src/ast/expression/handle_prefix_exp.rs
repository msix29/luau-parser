//! Helps with parsing prefix expressions.

use std::{ops::Deref, sync::Arc};

use tree_sitter::Node;

use crate::{
    bad_range, prelude::{
        Expression, ExpressionWrap, FromNode, FromNodeWithArgs, FunctionArguments, FunctionCall, FunctionCallInvoked, HasRange, LuauStatement, PrefixExp, Range, StringLiteral, Table, TableAccess, TableAccessKey, TableAccessPrefix, TableKey, Token, Var, VariableName
    }, utils::get_range_from_boundaries
};

//TODO: Split
impl FromNode for TableAccess {
    fn from_node(node: Node, code_bytes: &[u8]) -> Option<Self> {
        let table_node = node.child_by_field_name("table").unwrap_or(node);
        let prefix = match table_node.kind() {
            "functionCall" => TableAccessPrefix::FunctionCall(Arc::new(FunctionCall::from_node(
                table_node, code_bytes,
            )?)),
            "exp_wrap" => match PrefixExp::from_node(table_node, code_bytes)? {
                PrefixExp::ExpressionWrap(value) => {
                    TableAccessPrefix::ExpressionWrap(Arc::new(value))
                }
                _ => unreachable!("This'll always evaluate to a wrap."),
            },
            _ => TableAccessPrefix::Name(Token::from_node(table_node, code_bytes)?),
        };

        let mut accessed_keys = Vec::new();
        for key in node.children_by_field_name("key", &mut node.walk()) {
            let table_access_key = match key.kind() {
                "field_named" => TableAccessKey::Name {
                    dot: Token::from_node(key.child(0)?, code_bytes)?,
                    name: Token::from_node(key.child(1)?, code_bytes)?,
                },
                "field_indexed" => TableAccessKey::Expression(TableKey::Expression {
                    open_square_brackets: Token::from_node(key.child(0)?, code_bytes)?,
                    expression: Arc::new(Expression::from_node(key.child(1)?, code_bytes)?),
                    close_square_brackets: Token::from_node(key.child(2)?, code_bytes)?,
                }),
                _ => unreachable!("Key can't be anything else. Got {}", key.to_sexp()),
            };
            accessed_keys.push(table_access_key);
        }

        Some(TableAccess {
            prefix,
            accessed_keys,
        })
    }
}

impl FromNode for FunctionCall {
    fn from_node(node: Node, code_bytes: &[u8]) -> Option<Self> {
        Some(Self {
            invoked: FunctionCallInvoked::from_node(node, code_bytes)?,
            arguments: FunctionArguments::from_node(
                node.child_by_field_name("arguments")?,
                code_bytes,
            )?,
        })
    }
}
impl FromNode for FunctionCallInvoked {
    fn from_node(node: Node, code_bytes: &[u8]) -> Option<Self> {
        if let Some(invoked) = node.child_by_field_name("invoked") {
            Some(Self::Function(Arc::new(PrefixExp::from_node(
                invoked, code_bytes,
            )?)))
        } else {
            Some(Self::TableMethod {
                table: Arc::new(PrefixExp::from_node(
                    node.child_by_field_name("table")?,
                    code_bytes,
                )?),
                colon: Token::from_node(node.child_by_field_name("colon")?, code_bytes)?,
                method: Token::from_node(node.child_by_field_name("method")?, code_bytes)?,
            })
        }
    }
}
impl FromNode for FunctionArguments {
    fn from_node(node: Node, code_bytes: &[u8]) -> Option<Self> {
        println!("\nInteresting..\n");
        let actual_argument = node.child(0)?;
        println!("\n{}\n", node.to_sexp());

        match actual_argument.kind() {
            "table" => Some(Self::Table(Table::from_node(actual_argument, code_bytes, ())?)),
            "string" => Some(Self::String(StringLiteral::from_node(
                actual_argument,
                code_bytes,
            )?)),
            _ => Some(Self::List {
                open_parenthesis: Token::from_node(
                    node.child_by_field_name("open_parenthesis")?,
                    code_bytes,
                )?,
                arguments: Expression::from_nodes(
                    node.children_by_field_name("arguments", &mut node.walk()),
                    code_bytes,
                ),
                close_parenthesis: Token::from_node(
                    node.child_by_field_name("close_parenthesis")?,
                    code_bytes,
                )?,
            }),
        }
    }
}

impl FromNode for PrefixExp {
    fn from_node(node: Node, code_bytes: &[u8]) -> Option<Self> {
        let kind = node.kind();
        match kind {
            "var" => {
                if node.child_count() == 1 {
                    Some(Self::Var(Var::Name(VariableName {
                        token: Token::from_node(node, code_bytes)?,
                    })))
                } else {
                    Some(Self::Var(Var::TableAccess(TableAccess::from_node(
                        node, code_bytes,
                    )?)))
                }
            }
            "functionCall" => Some(Self::FunctionCall(FunctionCall::from_node(
                node, code_bytes,
            )?)),
            "exp_wrap" => Some(Self::ExpressionWrap(ExpressionWrap {
                opening_parenthesis: Token::from_node(node.child(0)?, code_bytes)?,
                expression: Arc::new(Expression::from_node(node.child(1)?, code_bytes)?),
                closing_parenthesis: Token::from_node(node.child(2)?, code_bytes)?,
            })),
            _ => {
                eprintln!(
                    "Reached unhandled kind '{}' when parsing `PrefixExp`.",
                    kind
                );
                None
            }
        }
    }
}

impl Deref for VariableName {
    type Target = Token;

    fn deref(&self) -> &Self::Target {
        &self.token
    }
}

impl LuauStatement for FunctionCall {
    fn try_from_node<'a>(
        node: Node<'a>,
        _: &mut tree_sitter::TreeCursor<'a>,
        code_bytes: &[u8],
    ) -> Option<Self> {
        if node.kind() != "functionCall" {
            return None;
        }

        FunctionCall::from_node(node, code_bytes)
    }
}

impl HasRange for TableAccess {
    fn get_range(&self) -> Range {
        get_range_from_boundaries(
            self.prefix.get_range(),
            // There's at least one key.
            self.accessed_keys.last().unwrap().get_range(),
        )
    }
}
impl HasRange for TableAccessPrefix {
    fn get_range(&self) -> Range {
        match self {
            TableAccessPrefix::Name(value) => value.get_range(),
            TableAccessPrefix::FunctionCall(value) => value.get_range(),
            TableAccessPrefix::ExpressionWrap(value) => value.get_range(),
        }
    }
}
impl HasRange for TableAccessKey {
    fn get_range(&self) -> Range {
        match self {
            TableAccessKey::Expression(value) => value.get_range(),
            TableAccessKey::Name { dot, name } => {
                get_range_from_boundaries(dot.get_range(), name.get_range())
            }
        }
    }
}

impl HasRange for FunctionCall {
    fn get_range(&self) -> Range {
        get_range_from_boundaries(self.invoked.get_range(), self.arguments.get_range())
    }
}
impl HasRange for FunctionArguments {
    fn get_range(&self) -> Range {
        match self {
            FunctionArguments::String(value) => value.get_range(),
            FunctionArguments::Table(value) => value.get_range(),
            FunctionArguments::List {
                open_parenthesis,
                arguments: _,
                close_parenthesis,
            } => get_range_from_boundaries(
                open_parenthesis.get_range(),
                close_parenthesis.get_range(),
            ),
        }
    }
}
impl HasRange for FunctionCallInvoked {
    fn get_range(&self) -> Range {
        match self {
            FunctionCallInvoked::Function(value) => value.get_range(),
            FunctionCallInvoked::TableMethod {
                table,
                colon: _,
                method,
            } => get_range_from_boundaries(table.get_range(), method.get_range()),
        }
    }
}

impl HasRange for Var {
    fn get_range(&self) -> Range {
        match self {
            Self::ERROR => bad_range!("Var"),
            Self::Name(value) => value.get_range(),
            Self::TableAccess(value) => value.get_range(),
        }
    }
}
impl HasRange for ExpressionWrap {
    fn get_range(&self) -> Range {
        get_range_from_boundaries(
            self.opening_parenthesis.get_range(),
            self.closing_parenthesis.get_range(),
        )
    }
}

impl HasRange for PrefixExp {
    fn get_range(&self) -> Range {
        match self {
            PrefixExp::Var(value) => value.get_range(),
            PrefixExp::FunctionCall(value) => value.get_range(),
            PrefixExp::ExpressionWrap(value) => value.get_range(),
        }
    }
}
