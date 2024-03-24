#[cfg(feature = "cache")]
use std::collections::HashMap;
use std::sync::Arc;

use tree_sitter::Node;
#[cfg(feature = "cache")]
use tree_sitter::Tree;

use crate::prelude::{Ast, AstNode, Token, TypeDefinition, VariableDeclaration};

fn print_all(node: Node, code: &str) {
    // if true {
    //     return;
    // }
    println!(
        "{:?}\n\t{}",
        node,
        &code[node.start_byte()..node.end_byte()]
    );

    for i in 0..node.child_count() {
        print_all(node.child(i).unwrap(), code);
    }
}

fn parse_block(body: Node, tokens: &mut Vec<Token>, full_code_bytes: &[u8]) {
    let mut cursor = body.walk();
    for i in 0..body.child_count() {
        let node = body.child(i).unwrap();

        if let Some(variable_declarations) =
            VariableDeclaration::try_from_node(node, &mut cursor, full_code_bytes)
        {
            tokens.extend(
                variable_declarations
                    .iter()
                    .map(|v| Token::VariableDeclaration(v.clone())),
            );
            drop(variable_declarations);
        } else if let Some(mut type_declarations) =
            TypeDefinition::try_from_node(node, &mut cursor, full_code_bytes)
        {
            tokens.extend(
                type_declarations
                    .iter_mut()
                    .map(|v| Token::TypeDefinition(v.clone())),
            );
            drop(type_declarations);
        }
    }
}

/// A Luau parser.
#[derive(Clone, Debug)]
pub struct Parser {
    #[cfg(feature = "cache")]
    cache: HashMap<String, (Ast, Tree)>,
}

impl Parser {
    pub fn new() -> Self {
        Parser {
            #[cfg(feature = "cache")]
            cache: HashMap::new(),
        }
    }

    /// Parses Luau code into an AST.
    pub fn parse(&mut self, code: &str, uri: &str) -> Ast {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_luau::language())
            .expect("Error loading Luau grammar");
        let tree = parser.parse(code, None).unwrap();

        let mut tokens = Vec::default();
        let code_bytes = code.as_bytes();

        let root = tree.root_node();
        parse_block(root, &mut tokens, code_bytes);

        // TODO: Remove
        // For debugging purposes.
        if false {
            // Disabled for now, printing full tree may be enough.
            print_all(root, code);
        }
        println!("{}", &root.to_sexp());

        let ast = Ast {
            tokens: Arc::new(tokens),
            uri: Some(uri.to_string()),
        };

        #[cfg(feature = "cache")]
        {
            let uri = uri.to_string();
            self.cache.insert(uri.to_string(), (ast, tree));

            return self.cache.get(&uri).unwrap().0.to_owned();
        }

        #[cfg(not(feature = "cache"))]
        {
            // Only start a new scope because clippy warnings.
            ast
        }
    }

    /// Gets all ASTs generated by the parser
    #[cfg(feature = "cache")]
    pub fn get_all_asts(&self) -> HashMap<&String, &Ast> {
        self.cache
            .iter()
            .map(|(key, (ast, _))| (key, ast))
            .collect()
    }

    /// Clears the cache.
    #[cfg(feature = "cache")]
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }
}

impl Default for Parser {
    fn default() -> Self {
        Self::new()
    }
}
