//! Implements helper traits for statements.

use tree_sitter::Node;

use crate::prelude::{
    Comment, CompoundSetExpression, DoBlock, Expression, FunctionCall, GenericFor, GlobalFunction,
    IfStatement, LastStatement, LocalAssignment, LocalFunction, LuauStatement, NumericalFor,
    RepeatBlock, SetExpression, Token, Statement, TypeDefinition, WhileLoop,
};

impl From<(Node<'_>, &[u8])> for LastStatement {
    fn from((node, code_bytes): (Node, &[u8])) -> Self {
        let semicolon = node
            .child_by_field_name("semicolon")
            .map(|semicolon| Token::from((semicolon, code_bytes)));

        let node = node.child(0).unwrap();

        match node.kind() {
            "break" => Self::Break((Token::from((node, code_bytes)), semicolon)),
            "continue" => Self::Continue((Token::from((node, code_bytes)), semicolon)),
            "return_statement" => Self::Return {
                return_keyword: Token::from((node.child(0).unwrap(), code_bytes)),
                expressions: Expression::from_nodes(
                    node.children_by_field_name("expressions", &mut node.walk()),
                    code_bytes,
                ),
                semicolon,
            },
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
        } else if let Some(comment) = Comment::try_from_node(statement, &mut cursor, code_bytes) {
            Self::Comment(comment)
        } else {
            // Should be unreachable.
            unreachable!("Reached unhandled statement: {}", statement.to_sexp());
        }
    }
}
