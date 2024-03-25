use std::sync::Arc;

use tree_sitter::Node;

use crate::{
    prelude::{
        Ast, ExpressionInner, FunctionName, FunctionParameter, FunctionReturn, FunctionValue,
        NormalizedName, SingleToken, TableField, TableKey, TableValue, TypeDefinition,
    },
    utils::get_location,
};

pub fn from_singleton_type(node: Node, code_bytes: &[u8]) -> ExpressionInner {
    match node.kind() {
        "string" => ExpressionInner::from((node.utf8_text(code_bytes).unwrap(), node)),
        "name" => ExpressionInner::from(("<other value here>", node)), // TODO: Look for it.
        "false" => ExpressionInner::from(("false", node)),
        "true" => ExpressionInner::from(("true", node)),
        _ => ExpressionInner::from(("any", node)), // Should never be matched when done.
    }
}

pub fn build_table_type(node: Node, code_bytes: &[u8]) -> TableValue {
    let opening_brackets = SingleToken::from((
        node.child_by_field_name("opening_brackets").unwrap(),
        code_bytes,
    ));
    let closing_brackets = SingleToken::from((
        node.child_by_field_name("closing_brackets").unwrap(),
        code_bytes,
    ));

    let Some(fields_list) = node
        .child_by_field_name("fields")
        .map(|node| node.child(0).unwrap())
    else {
        return TableValue {
            opening_brackets,
            fields: Arc::<Vec<TableField>>::default(),
            closing_brackets,
            location: get_location(node),
        };
    };
    let separators = fields_list
        .children_by_field_name("sep", &mut node.walk())
        .collect::<Vec<Node>>();
    let mut table_fields: Vec<TableField> = Vec::new();
    match fields_list.kind() {
        "type" => {
            table_fields.push(TableField {
                key: Arc::new(TableKey::String("[number]".to_string())),
                equal_or_colon: None,
                r#type: Arc::new(TypeDefinition::from((fields_list, code_bytes, false))),
                value: None,
                separator: None,
                location: get_location(node),
                key_location: None,
                value_location: get_location(node),
            });
        }
        "propList" => {
            for (i, field) in fields_list
                .children_by_field_name("field", &mut fields_list.walk())
                .enumerate()
            {
                let separator = separators
                    .get(i)
                    .map(|separator| SingleToken::from((*separator, code_bytes)));

                match field.kind() {
                    "tableProperty" => {
                        table_fields.push(TableField {
                            key: Arc::new(TableKey::String(
                                field
                                    .child(0)
                                    .unwrap()
                                    .utf8_text(code_bytes)
                                    .unwrap()
                                    .to_string(),
                            )),
                            equal_or_colon: Some(SingleToken::from((
                                field.child(1).unwrap(),
                                code_bytes,
                            ))),
                            r#type: Arc::new(TypeDefinition::from((
                                field.child(2).unwrap(),
                                code_bytes,
                                false,
                            ))),
                            value: None,
                            separator,
                            //TODO
                            location: get_location(node),
                            key_location: Some(get_location(field.child(0).unwrap())),
                            value_location: get_location(field.child(0).unwrap()),
                        });
                    }
                    "tableIndexer" => {
                        table_fields.push(TableField {
                            key: Arc::new(TableKey::Type {
                                open_square_brackets: SingleToken::from((
                                    field.child(0).unwrap(),
                                    code_bytes,
                                )),
                                r#type: Arc::new(TypeDefinition::from((
                                    field.child(1).unwrap(),
                                    code_bytes,
                                    false,
                                ))),
                                close_square_brackets: SingleToken::from((
                                    field.child(2).unwrap(),
                                    code_bytes,
                                )),
                            }),
                            equal_or_colon: Some(SingleToken::from((
                                field.child(3).unwrap(),
                                code_bytes,
                            ))),
                            r#type: Arc::new(TypeDefinition::from((
                                field.child(4).unwrap(),
                                code_bytes,
                                false,
                            ))),
                            value: None,
                            separator,
                            //TODO
                            location: get_location(node),
                            key_location: Some(get_location(field.child(0).unwrap())),
                            value_location: get_location(field.child(0).unwrap()),
                        });
                    }
                    _ => (),
                }
            }
        }
        _ => (),
    }

    TableValue {
        opening_brackets,
        fields: Arc::new(table_fields),
        closing_brackets,
        location: get_location(node),
    }
}

pub fn build_function_parameters(
    parameters_node: Node,
    code_bytes: &[u8],
    is_type: bool,
) -> Vec<FunctionParameter> {
    let mut parameters = Vec::new();

    for parameter in
        parameters_node.children_by_field_name("parameter", &mut parameters_node.walk())
    {
        let normalized_name = NormalizedName::from((parameter, code_bytes));

        if let Some(r#type) = normalized_name.r#type {
            parameters.push(FunctionParameter {
                name: normalized_name.name,
                is_variadic: false,
                r#type,
                location: get_location(parameter),
            });
        } else if !is_type {
            parameters.push(FunctionParameter {
                name: normalized_name.name,
                is_variadic: false,
                r#type: Arc::new(TypeDefinition::from(("any", parameter))),
                location: get_location(parameter),
            });
        } else {
            // Pretty sure this isn't in the spec, but if the name is missing in a type
            // definition of a function, then it's the type and the name is empty, but for
            // the sake of making it "better", we use `_`, which is globally known as a
            // placeholder.
            parameters.push(FunctionParameter {
                name: "_".to_string(),
                is_variadic: false,
                r#type: Arc::new(TypeDefinition::from((
                    normalized_name.name.as_str(),
                    parameter,
                ))),
                location: get_location(parameter),
            });
        }
    }

    parameters
}

pub fn build_function_returns(node: Node, code_bytes: &[u8]) -> Vec<FunctionReturn> {
    let mut returns = Vec::new();

    if let Some(r#type) = node.child_by_field_name("return") {
        returns.push(FunctionReturn {
            r#type: Arc::new(TypeDefinition::from((r#type, code_bytes, false))),
            is_variadic: false,
            location: get_location(r#type),
        })
    } else if let Some(returns_node) = node.child_by_field_name("returns") {
        for r#return in returns_node.children(&mut returns_node.walk()) {
            returns.push(FunctionReturn {
                r#type: Arc::new(TypeDefinition::from((r#return, code_bytes, false))),
                is_variadic: false,
                location: get_location(r#return),
            });
        }
    }

    returns
}

//TODO: Make it work
pub fn build_function_type(node: Node, code_bytes: &[u8]) -> FunctionValue {
    let parameters = build_function_parameters(node, code_bytes, true);

    FunctionValue {
        local_keyword: None,
        function_keyword: None,
        function_name: FunctionName::Anonymous,
        parameters: Arc::new(parameters),
        returns: Arc::new(build_function_returns(node, code_bytes)),
        body: Arc::new(Ast::default()),
        end_keyword: None,
    }
}

pub fn from_simple_type(node: Node, code_bytes: &[u8]) -> ExpressionInner {
    match node.kind() {
        "singleton" => from_singleton_type(node, code_bytes),
        "namedtype" => ExpressionInner::from((node.utf8_text(code_bytes).unwrap(), node)), //TODO: indexing from a table.
        "typeof" => ExpressionInner::from(("typeof<T>(...)", node)), //TODO: typeof(<expression>)
        "tableType" => ExpressionInner::Table(build_table_type(node, code_bytes)),
        "functionType" => ExpressionInner::Function(build_function_type(node, code_bytes)),
        "wraptype" => from_simple_type(node.child(1).unwrap(), code_bytes),
        // "untype"
        _ => todo!("Why did this match? {}", node.to_sexp()), // Should never be matched when done.
    }
}
