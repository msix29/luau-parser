//! # Type Definition
//!
//! This module does the work of the whole type checker, from building
//! _[type definitions](TypeDefinition)_ from _[nodes](Node)_, to implementing helper
//! traits for both _[type definitions](TypeDefinition)_ and _[type values](TypeValue)_.
//!

pub(crate) mod functions;
mod type_value;

use std::{fmt::Display, sync::Arc};
use tree_sitter::Node;

use crate::{
    prelude::{AstNode, ExpressionInner, HasRawValue, SingleToken, TypeDefinition, TypeValue},
    utils::get_location,
};

impl Display for TypeDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.get_raw_value())
    }
}
impl HasRawValue for TypeDefinition {
    fn get_raw_value(&self) -> String {
        if self.type_name == "any" {
            return "any".to_string();
        }

        let prefix = self
            .export_keyword
            .as_ref()
            .map_or_else(|| "".to_string(), |export| export.get_raw_value());

        let start = if self.type_name.is_empty() {
            String::from("")
        } else {
            format!("type {} = ", self.type_name)
        };

        format!("{}{}{}", prefix, start, self.type_value.get_raw_value())
    }
}

impl AstNode for TypeDefinition {
    fn try_from_node<'a>(
        node: tree_sitter::Node<'a>,
        _: &mut tree_sitter::TreeCursor<'a>,
        code_bytes: &[u8],
    ) -> Option<Vec<Self>> {
        if node.kind() != "typeDefinition" {
            return None;
        }

        Some(vec![TypeDefinition::from((node, code_bytes, true))])
    }
}

impl From<(Node<'_>, &[u8], bool)> for TypeDefinition {
    fn from((node, code_bytes, is_definition): (Node, &[u8], bool)) -> Self {
        if is_definition {
            let name_node = node.child_by_field_name("typeName").unwrap();

            TypeDefinition {
                export_keyword: node
                    .child_by_field_name("export")
                    .map(|node| SingleToken::from((node, code_bytes))),
                type_keyword: node
                    .child_by_field_name("typeKeyword")
                    .map(|node| SingleToken::from((node, code_bytes))),
                type_name: name_node.utf8_text(code_bytes).unwrap().to_string(),
                equal_sign: node
                    .child_by_field_name("equal")
                    .map(|node| SingleToken::from((node, code_bytes))),
                type_value: Arc::new(TypeValue::from((
                    node.child_by_field_name("type").unwrap(),
                    code_bytes,
                ))),
                name_location: Some(get_location(name_node)),
            }
        } else {
            TypeDefinition {
                export_keyword: None,
                type_keyword: None,
                name_location: Some(get_location(node)),
                type_name: "".to_string(),
                equal_sign: None,
                type_value: Arc::new(TypeValue::from((node, code_bytes))),
            }
        }
    }
}

impl From<(&str, Node<'_>)> for TypeDefinition {
    fn from((type_name, node): (&str, Node<'_>)) -> Self {
        TypeDefinition {
            export_keyword: None,
            type_keyword: None,
            type_name: type_name.to_string(),
            equal_sign: None,
            type_value: Arc::new(TypeValue::from((type_name, node))),
            name_location: None,
        }
    }
}

impl From<(Arc<ExpressionInner>, Node<'_>)> for TypeDefinition {
    fn from((value, node): (Arc<ExpressionInner>, Node<'_>)) -> Self {
        TypeDefinition {
            export_keyword: None,
            type_keyword: None,
            type_name: "".to_string(),
            equal_sign: None,
            type_value: Arc::from(TypeValue::from((value, node))),
            name_location: None,
        }
    }
}
