//! Implements helper traits for type values

use std::sync::Arc;
use tree_sitter::Node;

use crate::{
    prelude::{Expression, List, SingleToken, TypeValue},
    utils::{get_location, get_spaces},
};

use super::functions::{build_function_type, build_table_type, from_singleton_type};

impl From<(Node<'_>, &[u8])> for TypeValue {
    fn from((node, code_bytes): (Node<'_>, &[u8])) -> Self {
        match node.kind() {
            "namedtype" => {
                if let Some(module) = node.child_by_field_name("module") {
                    TypeValue::Module {
                        module: module.utf8_text(code_bytes).unwrap().to_string(),
                        dot: SingleToken::from((
                            node.child_by_field_name("dot").unwrap(),
                            code_bytes,
                        )),
                        type_info: node
                            .child_by_field_name("name")
                            .unwrap()
                            .utf8_text(code_bytes)
                            .unwrap()
                            .to_string(),
                    }
                } else {
                    from_singleton_type(node, code_bytes)
                }
            }
            "wraptype" => TypeValue::Wrap {
                opening_parenthesis: SingleToken::from((node.child(0).unwrap(), code_bytes)),
                r#type: Arc::new(TypeValue::from((node.child(1).unwrap(), code_bytes))),
                closing_parenthesis: SingleToken::from((node.child(2).unwrap(), code_bytes)),
            },
            "typeof" => TypeValue::Typeof {
                typeof_token: SingleToken::from((node.child(0).unwrap(), code_bytes)),
                opening_parenthesis: SingleToken::from((node.child(1).unwrap(), code_bytes)),
                inner: Arc::new(Expression::from((node.child(2).unwrap(), code_bytes))),
                closing_parenthesis: SingleToken::from((node.child(3).unwrap(), code_bytes)),
            },
            "functionType" => build_function_type(node, code_bytes),
            "tableType" => TypeValue::Table(build_table_type(node, code_bytes)),
            "singleton" => from_singleton_type(node, code_bytes),
            "bintype" => {
                let operator =
                    SingleToken::from((node.child_by_field_name("op").unwrap(), code_bytes));

                let left = TypeValue::from((node.child_by_field_name("arg0").unwrap(), code_bytes));
                let right =
                    TypeValue::from((node.child_by_field_name("arg1").unwrap(), code_bytes));

                if operator.word == "&" {
                    TypeValue::Intersection {
                        left: Arc::new(left),
                        ampersand: operator,
                        right: Arc::new(right),
                    }
                } else {
                    TypeValue::Union {
                        left: Arc::new(left),
                        pipe: operator,
                        right: Arc::new(right),
                    }
                }
            }
            "untype" => TypeValue::Optional {
                base: Arc::new(TypeValue::from((
                    node.child_by_field_name("arg").unwrap(),
                    code_bytes,
                ))),
                question_mark: SingleToken::from((
                    node.child_by_field_name("op").unwrap(),
                    code_bytes,
                )),
            },
            "typepack" => {
                let pack = node.child(0).unwrap();
                match pack.kind() {
                    "(" => {
                        let opening_parenthesis = SingleToken::from((
                            node.child_by_field_name("opening_parenthesis").unwrap(),
                            code_bytes,
                        ));
                        let closing_parenthesis = SingleToken::from((
                            node.child_by_field_name("closing_parenthesis").unwrap(),
                            code_bytes,
                        ));

                        let mut types = Vec::new();

                        for child in node.children_by_field_name("type", &mut node.walk()) {
                            types.push(TypeValue::from((child, code_bytes)));
                        }

                        TypeValue::Tuple {
                            opening_parenthesis,
                            types: List::default(),
                            closing_parenthesis,
                        }
                    }
                    "variadic" => TypeValue::Variadic {
                        ellipsis: SingleToken::from((node.child(0).unwrap(), code_bytes)),
                        type_info: Arc::new(TypeValue::from((pack.child(1).unwrap(), code_bytes))),
                    },
                    "genpack" => TypeValue::GenericPack {
                        name: SingleToken::from((pack.child(0).unwrap(), code_bytes)),
                        ellipsis: SingleToken::from((pack.child(1).unwrap(), code_bytes)),
                    },
                    _ => unreachable!(),
                }
            }
            _ => panic!("Reached unhandled type. {}", node.to_sexp()),
        }
    }
}
impl From<(&str, Node<'_>, &[u8])> for TypeValue {
    fn from((name, node, code_bytes): (&str, Node<'_>, &[u8])) -> Self {
        let (spaces_before, spaces_after) = get_spaces(node, code_bytes);

        TypeValue::Basic(SingleToken {
            spaces_before,
            word: name.to_string(),
            location: get_location(node),
            spaces_after,
        })
    }
}
