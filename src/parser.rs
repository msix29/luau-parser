use crate::ast::{variable_declaration::VariableDeclaration, Ast, AstNode, Token};

pub fn parse(code: &str) -> Ast<'_> {
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_luau::language())
        .expect("Error loading Luau grammar");
    let tree = parser.parse(code, None).unwrap();

    let mut ast = Ast::default();
    let code_bytes = code.as_bytes();

    let root = tree.root_node();
    let mut cursor = tree.walk();
    for i in 0..root.child_count() {
        let child = root.child(i).unwrap();
        if let Some(mut variable_declarations) =
            VariableDeclaration::try_from_node(child, &mut cursor, code_bytes)
        {
            ast.tokens.extend(
                variable_declarations
                    .iter_mut()
                    .map(|v| Token::VariableDeclaration(v.clone())),
            );
        }
    }

    ast
}
