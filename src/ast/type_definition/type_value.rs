use std::sync::Arc;
use tree_sitter::Node;

use crate::{
    prelude::{Expression, List, ListItem, SingleToken, TypeValue},
    utils::{get_location, get_spaces},
};

use super::functions::{
    build_function_type, build_table_type, /* from_simple_type, */ from_singleton_type,
};

impl From<(Node<'_>, &[u8])> for TypeValue {
    fn from((node, code_bytes): (Node<'_>, &[u8])) -> Self {
        match node.kind() {
            "namedtype" => {
                if let Some(module) = node.child_by_field_name("module") {
                    TypeValue::Module {
                        module: SingleToken::from((module, code_bytes)),
                        dot: SingleToken::from((
                            node.child_by_field_name("dot").unwrap(),
                            code_bytes,
                        )),
                        type_info: Arc::new(SingleToken::from((
                            node.child_by_field_name("name").unwrap(),
                            code_bytes,
                        ))),
                    }
                } else {
                    from_singleton_type(node, code_bytes)
                }
            }
            "wraptype" => TypeValue::Tuple {
                opening_parenthesis: SingleToken::from((node.child(0).unwrap(), code_bytes)),
                types: List {
                    items: vec![ListItem::NonTrailing(TypeValue::from((
                        node.child(1).unwrap(),
                        code_bytes,
                    )))],
                },
                closing_parenthesis: SingleToken::from((node.child(2).unwrap(), code_bytes)),
            },
            "typeof" => TypeValue::Typeof {
                typeof_token: SingleToken::from((node.child(0).unwrap(), code_bytes)),
                opening_parentheses: SingleToken::from((node.child(1).unwrap(), code_bytes)),
                inner: Arc::new(Expression::from((node.child(2).unwrap(), code_bytes))),
                closing_parentheses: SingleToken::from((node.child(3).unwrap(), code_bytes)),
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
