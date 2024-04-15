//! The main item of this crate, the actual [`parser`](LuauParser).

#[cfg(feature = "cache")]
use std::collections::HashMap;
use std::sync::Arc;

#[cfg(feature = "cache")]
use tree_sitter::Tree;
use tree_sitter::{Node, Parser};

use crate::prelude::{Ast, LastStatement, SingleToken, Statement};

/// Parses a code block and fills `tokens` with the parsed ones. The tokens can then
/// be used to make the syntax tre.
pub(crate) fn parse_block(body: &Node, code_bytes: &[u8], uri: Option<String>) -> Ast {
    let mut statements = Vec::new();

    for i in 0..body.child_count() {
        let node = body.child(i).unwrap();
        if node.has_error() {
            continue;
        }

        statements.push((
            Statement::from((node.child(0).unwrap(), code_bytes)),
            node.child(1)
                .map(|node| SingleToken::from((node, code_bytes))),
        ))
    }

    Ast {
        uri,
        statements: Arc::new(statements),
        last_statement: body
            .child_by_field_name("lastStatement")
            .map(|last_statement| Arc::new(LastStatement::from((last_statement, code_bytes)))),
    }
}

/// A Luau parser.
pub struct LuauParser {
    /// Cache, only works with the `cache` feature, this is useful when you need
    /// to use the [`ast`](Ast) more than once in 2 different places without
    /// reparsing or with the `uri` only.
    #[cfg(feature = "cache")]
    cache: HashMap<String, (Ast, Tree)>,

    /// The `tree-sitter` parser.
    parser: Parser,
}

impl LuauParser {
    /// Create a new [`parser`](LuauParser).
    pub fn new() -> Self {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree_sitter_luau::language())
            .expect("Error loading Luau grammar");

        LuauParser {
            #[cfg(feature = "cache")]
            cache: HashMap::new(),
            parser,
        }
    }

    /// Parse Luau code into an [`ast`](Ast).
    pub fn parse(&mut self, code: &str, uri: &str) -> Ast {
        let tree = self.parser.parse(code, None).unwrap();

        let code_bytes = code.as_bytes();
        let root = tree.root_node();
        // println!("\n{}\n", &root.to_sexp());

        let ast = parse_block(&root, code_bytes, Some(uri.to_string()));

        #[cfg(feature = "cache")]
        {
            self.cache.insert(uri.to_string(), (ast, tree));

            return self.cache.get(&uri.to_string()).unwrap().0.to_owned();
        }

        #[cfg(not(feature = "cache"))]
        {
            // Only start a new scope because clippy warnings.
            ast
        }
    }

    /// Get a specific [`ast`](Ast) from the cache, this function assumes the ast does
    /// exist. If it may or may not exist, use [`maybe_get_ast`](LuauParser::maybe_get_ast).
    #[cfg(feature = "cache")]
    pub fn get_ast(&self, uri: &str) -> &Ast {
        &self.cache.get(uri).unwrap().0
    }

    /// Get a specific [`ast`](Ast) from the cache, or parse `code` and return the
    /// [`ast`](Ast).
    pub fn get_or_create(&mut self, uri: &str, code: &str) -> Ast {
        #[cfg(feature = "cache")]
        if let Some(ast) = self.maybe_get_ast(uri) {
            return ast.to_owned();
        }

        self.parse(code, uri)
    }

    /// Get a specific ast from the cache, this function is the safer version of
    /// [`get_ast`](LuauParser::get_ast).
    #[cfg(feature = "cache")]
    pub fn maybe_get_ast(&self, uri: &str) -> Option<&Ast> {
        self.cache.get(uri).map(|item| &item.0)
    }

    /// Get all [`asts`](Ast) generated by the parser.
    #[cfg(feature = "cache")]
    pub fn get_all_asts(&self) -> HashMap<&String, &Ast> {
        self.cache
            .iter()
            .map(|(key, (ast, _))| (key, ast))
            .collect()
    }

    /// Get the cache generated by the parser. This could be useful if you need to use
    /// the actual trees parsed by tree sitter, if not, use
    /// [`get_all_asts`](LuauParser::get_all_asts).
    #[cfg(feature = "cache")]
    pub fn get_cache(&self) -> HashMap<&String, &(Ast, Tree)> {
        self.cache.iter().collect()
    }

    /// Clear the cache.
    #[cfg(feature = "cache")]
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }
}

impl Default for LuauParser {
    fn default() -> Self {
        Self::new()
    }
}
