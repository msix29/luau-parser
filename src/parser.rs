use tree_sitter::Node;

use crate::ast::{
    type_definition::TypeDefinition, variable_declaration::VariableDeclaration, Ast, AstNode, Token,
};

fn print_all(node: Node, code: &str) {
    println!(
        "{:?}\n\t{}",
        node,
        &code[node.start_byte()..node.end_byte()]
    );
    if node.kind() == "variableDeclaration" {
        println!("Children count: {}", node.child_count());
        // println!("varialbe, string: {:?}", node.child_by_field_name("expression").unwrap());
    }

    for i in 0..node.child_count() {
        print_all(node.child(i).unwrap(), code);
    }
}

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
        let node = child.child(0).unwrap();

        if let Some(mut variable_declarations) =
            VariableDeclaration::try_from_node(node, &mut cursor, code_bytes)
        {
            ast.tokens.extend(
                variable_declarations
                    .iter_mut()
                    .map(|v| Token::VariableDeclaration(v.clone())),
            );
        } else if let Some(mut type_declarations) =
            TypeDefinition::try_from_node(node, &mut cursor, code_bytes)
        {
            ast.tokens.extend(
                type_declarations
                    .iter_mut()
                    .map(|v| Token::TypeDefinition(v.clone())),
            );
        }
    }

    // TODO: Remove
    // For debugging purposes.
    print_all(root, code);

    ast
}
