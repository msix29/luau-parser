use luau_parser::{
    get_item_from_tuple_enum, prelude::LuauParser, types::HasRawValue, types::Statement,
};

#[test]
/// Tests local assignments with table expressions..
fn local_assignment_table() {
    let mut parser = LuauParser::new();

    let code = r#"
local t = {
    foo    =    "bar";
    fn = function() end,
            qux =   "Hello, World!"
}
"#;
    let ast = parser.parse(code, "");
    assert_eq!(ast.tokens.len(), 1);
    assert!(matches!(ast.tokens[0], Statement::LocalAssignment(_)));

    let assignment = get_item_from_tuple_enum!(&ast.tokens[0], Statement::LocalAssignment);
    assert_eq!(
        assignment.expressions.items[0].get_raw_value(),
        r#"{
    foo = "bar";
    fn = function(),
    qux = "Hello, World!"
}"#
    );
}
