//! Implements helper traits for statements.

use tree_sitter::Node;

use crate::{
    prelude::{
        CompoundSetExpression, DoBlock, Expression, FromNode, FunctionCall, GenericFor,
        GlobalFunction, IfStatement, LastStatement, LocalAssignment, LocalFunction, LuauStatement,
        NumericalFor, RepeatBlock, SetExpression, Statement, Token, TypeDefinition, WhileLoop,
    },
    utils::map_option,
};

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

impl From<(Node<'_>, &[u8])> for Statement {
    fn from((statement, code_bytes): (Node, &[u8])) -> Self {
        let mut cursor = statement.walk();
        if let Some(variable_declaration) =
            LocalAssignment::try_from_node(statement, &mut cursor, code_bytes)
        {
            Self::LocalAssignment(variable_declaration)
        } else if let Some(type_declaration) =
            TypeDefinition::try_from_node(statement, &mut cursor, code_bytes)
        {
            Self::TypeDefinition(type_declaration)
        } else if let Some(if_statement) =
            IfStatement::try_from_node(statement, &mut cursor, code_bytes)
        {
            Self::IfStatement(if_statement)
        } else if let Some(do_block) = DoBlock::try_from_node(statement, &mut cursor, code_bytes) {
            Self::DoBlock(do_block)
        } else if let Some(generic_for) =
            GenericFor::try_from_node(statement, &mut cursor, code_bytes)
        {
            Self::GenericFor(generic_for)
        } else if let Some(numerical_for) =
            NumericalFor::try_from_node(statement, &mut cursor, code_bytes)
        {
            Self::NumericalFor(numerical_for)
        } else if let Some(repeat_block) =
            RepeatBlock::try_from_node(statement, &mut cursor, code_bytes)
        {
            Self::RepeatBlock(repeat_block)
        } else if let Some(while_loop) =
            WhileLoop::try_from_node(statement, &mut cursor, code_bytes)
        {
            Self::WhileLoop(while_loop)
        } else if let Some(set_expression) =
            SetExpression::try_from_node(statement, &mut cursor, code_bytes)
        {
            Self::SetExpression(set_expression)
        } else if let Some(compound_set_expression) =
            CompoundSetExpression::try_from_node(statement, &mut cursor, code_bytes)
        {
            Self::CompoundSetExpression(compound_set_expression)
        } else if let Some(function_call) =
            FunctionCall::try_from_node(statement, &mut cursor, code_bytes)
        {
            Self::FunctionCall(function_call)
        } else if let Some(local_function) =
            LocalFunction::try_from_node(statement, &mut cursor, code_bytes)
        {
            Self::LocalFunction(local_function)
        } else if let Some(global_function) =
            GlobalFunction::try_from_node(statement, &mut cursor, code_bytes)
        {
            Self::GlobalFunction(global_function)
        } else {
            // Should be unreachable.
            unreachable!("Reached unhandled statement: {}", statement.to_sexp());
        }
    }
}
