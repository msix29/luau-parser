use luau_parser::{
    get_item_from_tuple_enum,
    prelude::LuauParser,
    types::{HasRawValue, Print, Statement},
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
    assert_eq!(ast.statements.len(), 1);
    assert!(matches!(ast.statements[0], Statement::LocalAssignment(_)));

    let assignment = get_item_from_tuple_enum!(&ast.statements[0], Statement::LocalAssignment);
    assert_eq!(
        assignment.expressions.items[0].get_raw_value(),
        r#"{
    foo = "bar";
    fn = function(),
    qux = "Hello, World!"
}"#
    );
    assert_eq!(assignment.expressions.items[0].print(), code[10..]);
}
