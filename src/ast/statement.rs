//! Implements helper traits for statements.

use tree_sitter::Node;

use crate::prelude::{
    Comment, CompoundSetExpression, DoBlock, Expression, FunctionCall, GenericFor, GlobalFunction,
    IfStatement, LastStatement, LocalAssignment, LocalFunction, LuauStatement, NumericalFor,
    RepeatBlock, SetExpression, SingleToken, Statement, TypeDefinition, WhileLoop,
};

impl From<(Node<'_>, &[u8])> for LastStatement {
    fn from((node, code_bytes): (Node, &[u8])) -> Self {
        let semicolon = node
            .child_by_field_name("semicolon")
            .map(|semicolon| SingleToken::from((semicolon, code_bytes)));

        let node = node.child(0).unwrap();

        match node.kind() {
            "break" => Self::Break((SingleToken::from((node, code_bytes)), semicolon)),
            "continue" => Self::Continue((SingleToken::from((node, code_bytes)), semicolon)),
            "return_statement" => Self::Return {
                return_keyword: SingleToken::from((node.child(0).unwrap(), code_bytes)),
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
            Statement::LocalAssignment(variable_declaration)
        } else if let Some(type_declaration) =
            TypeDefinition::try_from_node(statement, &mut cursor, code_bytes)
        {
            Statement::TypeDefinition(type_declaration)
        } else if let Some(if_statement) =
            IfStatement::try_from_node(statement, &mut cursor, code_bytes)
        {
            Statement::IfStatement(if_statement)
        } else if let Some(do_block) = DoBlock::try_from_node(statement, &mut cursor, code_bytes) {
            Statement::DoBlock(do_block)
        } else if let Some(generic_for) =
            GenericFor::try_from_node(statement, &mut cursor, code_bytes)
        {
            Statement::GenericFor(generic_for)
        } else if let Some(numerical_for) =
            NumericalFor::try_from_node(statement, &mut cursor, code_bytes)
        {
            Statement::NumericalFor(numerical_for)
        } else if let Some(repeat_block) =
            RepeatBlock::try_from_node(statement, &mut cursor, code_bytes)
        {
            Statement::RepeatBlock(repeat_block)
        } else if let Some(while_loop) =
            WhileLoop::try_from_node(statement, &mut cursor, code_bytes)
        {
            Statement::WhileLoop(while_loop)
        } else if let Some(set_expression) =
            SetExpression::try_from_node(statement, &mut cursor, code_bytes)
        {
            Statement::SetExpression(set_expression)
        } else if let Some(compound_set_expression) =
            CompoundSetExpression::try_from_node(statement, &mut cursor, code_bytes)
        {
            Statement::CompoundSetExpression(compound_set_expression)
        } else if let Some(function_call) =
            FunctionCall::try_from_node(statement, &mut cursor, code_bytes)
        {
            Statement::FunctionCall(function_call)
        } else if let Some(local_function) =
            LocalFunction::try_from_node(statement, &mut cursor, code_bytes)
        {
            Statement::LocalFunction(local_function)
        } else if let Some(global_function) =
            GlobalFunction::try_from_node(statement, &mut cursor, code_bytes)
        {
            Statement::GlobalFunction(global_function)
        } else if let Some(comment) = Comment::try_from_node(statement, &mut cursor, code_bytes) {
            Statement::Comment(comment)
        } else {
            // Should be unreachable, but just to be sure, we won't continue the loop.
            unreachable!();
        }
    }
}
