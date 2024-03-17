pub mod location;
pub mod position;
pub mod variable_declaration;

use std::fmt::Display;
use tree_sitter::{Node, TreeCursor};

use self::{location::Location, variable_declaration::VariableDeclaration};

pub trait AstNode: Display + Sized {
    fn get_raw_value(&self) -> String;
    fn try_from_node<'a>(node: Node<'a>, cursor: &mut TreeCursor<'a>, code_bytes: &[u8]) -> Option<Vec<Self>>;
}

pub trait HasLocation {
    fn get_location(&self) -> Location;
}

#[derive(Clone, Debug)]
pub enum Token {
    VariableDeclaration(VariableDeclaration),
}

#[derive(Clone, Debug, Default)]
pub struct Ast<'a> {
    pub uri: &'a str,
    pub tokens: Vec<Token>,
}
