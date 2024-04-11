use luau_parser::{
    get_item_from_tuple_enum,
    prelude::LuauParser,
    types::{HasRawValue, Statement},
};

#[test]
/// Tests simple local assignments.
fn simple_local_assignment() {
    let mut parser = LuauParser::new();

    let ast = parser.parse(r#"local foo = "Hello, World!""#, "1");
    assert_eq!(ast.tokens.len(), 1);
    assert!(matches!(ast.tokens[0], Statement::LocalAssignment(_)));
    let assignment = get_item_from_tuple_enum!(&ast.tokens[0], Statement::LocalAssignment);
    assert_eq!(assignment.name_list.items.len(), 1);
    assert_eq!(assignment.name_list.items[0].name.word, "foo");
    assert_eq!(assignment.expressions.items.len(), 1);
    assert_eq!(assignment.expressions.items[0].get_raw_value(), r#""Hello, World!""#);
}
