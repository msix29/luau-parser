//! Implements helper traits for statements.

use tree_sitter::Node;

use crate::{
    types::{
        CompoundSetExpression, DoBlock, Expression, FromNode, FunctionCall, GenericFor,
        GlobalFunction, IfStatement, LastStatement, LocalAssignment, LocalFunction, LuauStatement,
        NumericalFor, RepeatBlock, SetExpression, Statement, Token, TypeDefinition, WhileLoop,
    },
    utils::map_option,
};

macro_rules! __handle_statement {
    ({ $statement: ident, $code_bytes: ident }, $first_name: ident $(, $name: ident)* $(,)?) => {{
        let mut cursor = $statement.walk();

        if let Some(statement) = $first_name::try_from_node($statement, &mut cursor, $code_bytes) {
            Some(Self::$first_name(statement))
        } $(else if let Some(statement) = $name::try_from_node($statement, &mut cursor, $code_bytes) {
            Some(Self::$name(statement))
        })* else {
            None
        }
    }};
}

impl FromNode for LastStatement {
    fn from_node(node: Node, code_bytes: &[u8]) -> Option<Self> {
        let semicolon = map_option(node.child_by_field_name("semicolon"), |semicolon| {
            Token::from_node(semicolon?, code_bytes)
        });
        let node = node.child(0)?;

        match node.kind() {
            "break" => Some(Self::Break((
                Token::from_node(node, code_bytes)?,
                semicolon,
            ))),
            "continue" => Some(Self::Continue((
                Token::from_node(node, code_bytes)?,
                semicolon,
            ))),
            "return_statement" => Some(Self::Return {
                return_keyword: Token::from_node(node.child(0)?, code_bytes)?,
                expressions: Expression::from_nodes(
                    node.children_by_field_name("expressions", &mut node.walk()),
                    code_bytes,
                ),
                semicolon,
            }),
            _ => unreachable!(),
        }
    }
}

impl FromNode for Statement {
    fn from_node(statement: Node, code_bytes: &[u8]) -> Option<Self> {
        __handle_statement!(
            { statement, code_bytes },
            CompoundSetExpression,
            DoBlock,
            FunctionCall,
            GenericFor,
            GlobalFunction,
            IfStatement,
            LocalAssignment,
            LocalFunction,
            NumericalFor,
            RepeatBlock,
            SetExpression,
            TypeDefinition,
            WhileLoop,
        )
    }
}
