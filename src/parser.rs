use std::collections::HashMap;

use tree_sitter::{Node, Tree};

use crate::prelude::{Ast, AstNode, Token, TypeDefinition, VariableDeclaration};

fn print_all(node: Node, code: &str) {
    if true {
        return;
    }
    println!(
        "{:?}\n\t{}",
        node,
        &code[node.start_byte()..node.end_byte()]
    );

    for i in 0..node.child_count() {
        print_all(node.child(i).unwrap(), code);
    }
}

/// A Luau parser.
#[derive(Clone, Debug)]
pub struct Parser<'a> {
    #[cfg(feature = "cache")]
    cache: HashMap<String, (Ast<'a>, Tree)>,
}

impl Parser<'_> {
    /// Parses Luau code into an AST.
    pub fn parse(&mut self, code: &str, #[cfg(feature = "cache")] uri: &str) -> &Ast<'_> {
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
        drop(cursor);

        #[cfg(feature = "cache")]
        {
            let uri = uri.to_string();
            self.cache.insert(uri.to_string(), (ast, tree));
            return &self.cache.get(&uri).unwrap().0;
        }

        &ast
    }
}
