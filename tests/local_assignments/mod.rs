use luau_parser::{
    get_item_from_tuple_enum,
    prelude::LuauParser,
    types::{HasRawValue, Statement},
};

#[test]
/// Tests simple local assignments.
fn creating_parser() {
    let mut parser = LuauParser::new();

    let ast_1 = parser.parse(r#"local foo = "Hello, World!""#, "1");
    assert_eq!(ast_1.tokens.len(), 1);
    assert!(matches!(ast_1.tokens[0], Statement::LocalAssignment(_)));
    let assignment_1 = get_item_from_tuple_enum!(&ast_1.tokens[0], Statement::LocalAssignment);
    assert_eq!(assignment_1.name_list.items.len(), 1);
    assert_eq!(assignment_1.name_list.items[0].name.word, "foo");
    assert_eq!(assignment_1.expressions.items.len(), 1);
    assert_eq!(assignment_1.expressions.items[0].get_raw_value(), r#""Hello, World!""#);
}
