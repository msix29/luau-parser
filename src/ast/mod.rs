pub mod location;
pub mod position;
pub mod type_definition;
pub mod value;
pub mod variable_declaration;

use std::fmt::Display;
use tree_sitter::{Node, TreeCursor};

use self::{
    location::Location, type_definition::TypeDefinition, variable_declaration::VariableDeclaration,
};

pub trait HasRawValue: Display {
    fn get_raw_value(&self) -> String;
}

pub trait AstNode: HasRawValue + Sized {
    fn try_from_node<'a>(
        node: Node<'a>,
        cursor: &mut TreeCursor<'a>,
        code_bytes: &[u8],
    ) -> Option<Vec<Self>>;
}

pub trait HasLocation: AstNode {
    fn get_location(&self) -> Location;
}

#[derive(Clone, Debug)]
pub enum Token {
    VariableDeclaration(VariableDeclaration),
    TypeDefinition(TypeDefinition),
}

#[derive(Clone, Debug, Default)]
pub struct Ast<'a> {
    pub uri: &'a str,
    pub tokens: Vec<Token>,
}
