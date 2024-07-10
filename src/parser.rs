//! The main item of this crate, the actual [`parser`](LuauParser).

#[cfg(feature = "references")]
use lazy_static::lazy_static;
#[cfg(feature = "references")]
use smol_str::SmolStr;
#[cfg(feature = "cache")]
use std::collections::HashMap;
use std::sync::Arc;
#[cfg(feature = "references")]
use std::sync::RwLock;
#[cfg(feature = "incremental-parsing")]
use tree_sitter::InputEdit;
#[cfg(feature = "cache")]
use tree_sitter::Tree;
use tree_sitter::{Node, Parser};

#[cfg(feature = "references")]
lazy_static! {
    pub(crate) static ref CURRENT_URI: RwLock<Arc<SmolStr>> =
        RwLock::new(Arc::new(SmolStr::new("")));
    pub(crate) static ref CURRENT_AST: RwLock<Arc<Ast>> = RwLock::new(Arc::new(Ast::default()));
}

use crate::{
    prelude::{Ast, AstStatus, FromNode, LastStatement, Statement, Statements, Token},
    utils::map_option,
};

/// Parses a code block and fills `tokens` with the parsed ones. The tokens can then
/// be used to make the syntax tre.
pub(crate) fn parse_block(body: &Node, code_bytes: &[u8], uri: Option<String>) -> Ast {
    let statements = Statements::new();
    #[cfg(feature = "references")]
    {
        *CURRENT_AST.write().unwrap() = Arc::new(Ast {
            statements,
            last_statement: map_option(
                body.child_by_field_name("lastStatement"),
                |last_statement| {
                    LastStatement::from_node(last_statement?, code_bytes).map(Arc::new)
                },
            ),
            status: if body.has_error() {
                AstStatus::HasErrors
            } else {
                AstStatus::Complete
            },
            #[cfg(feature = "references")]
            parent: if uri.is_some() {
                None
            } else {
                Some(CURRENT_AST.read().unwrap().clone())
            },
            uri: uri.map(|uri| uri.into()),
        });
    }

    for node in body.children_by_field_name("statement", &mut body.walk()) {
        // if node.has_error() {
        //     continue;
        // }

        let statement = (
            Statement::from_node(node.child(0).unwrap(), code_bytes).unwrap_or_default(),
            map_option(node.child(1), |node| Token::from_node(node?, code_bytes)),
        );

        #[cfg(not(feature = "references"))]
        statements.push(statement);
        #[cfg(feature = "references")]
        CURRENT_AST.write().unwrap().statements.push(statement);
    }

    #[cfg(feature = "references")]
    return {
        let current_ast = CURRENT_AST.read().unwrap();
        if let Some(parent) = &current_ast.parent {
            *CURRENT_AST.write().unwrap() = parent.clone();
        }

        (**current_ast).clone()
    };

    #[cfg(not(feature = "references"))]
    Ast {
        statements,
        last_statement: map_option(
            body.child_by_field_name("lastStatement"),
            |last_statement| LastStatement::from_node(last_statement?, code_bytes).map(Arc::new),
        ),
        status: if body.has_error() {
            AstStatus::HasErrors
        } else {
            AstStatus::Complete
        },
        #[cfg(feature = "references")]
        parent: if uri.is_some() {
            None
        } else {
            Some(CURRENT_AST.read().unwrap().clone())
        },
        uri: uri.map(|uri| uri.into()),
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
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_luau::language())
            .expect("Error loading Luau grammar");

        LuauParser {
            #[cfg(feature = "cache")]
            cache: HashMap::new(),
            parser,
        }
    }

    /// Edit a tree for incremental parsing.
    #[cfg(feature = "incremental-parsing")]
    pub fn edit_tree(&mut self, uri: &str, edits: Vec<InputEdit>) {
        if let Some(cached) = self.cache.get_mut(uri) {
            let tree = &mut cached.1;
            for edit in edits {
                tree.edit(&edit);
            }
        }
    }

    /// Parse Luau code into an [`ast`](Ast).
    pub fn parse(&mut self, code: &str, uri: &str) -> Ast {
        // NOTE: Can a text editor use `\r` by itself independant of the OS? If so, remove
        // this `cfg`.
        #[cfg(windows)]
        let code = &code.replace('\r', "");

        #[cfg(feature = "references")]
        #[allow(clippy::unnecessary_operation)] // False warning.
        {
            *CURRENT_URI.write().unwrap() = Arc::new(uri.into());
            // *CURRENT_AST.write().unwrap() = Arc::new(Ast::default());
        };

        #[cfg(not(feature = "incremental-parsing"))]
        let tree = self.parser.parse(code, None).unwrap();

        #[cfg(feature = "incremental-parsing")]
        let tree = {
            let old_tree = self.cache.get(uri).map(|cached| &cached.1);
            self.parser.parse(code, old_tree).unwrap()
        };

        let code_bytes = code.as_bytes();
        let root = tree.root_node();
        // println!("\n{}\n", &root.to_sexp());

        let ast = parse_block(&root, code_bytes, Some(uri.to_string()));

        #[cfg(feature = "cache")]
        {
            self.cache.insert(uri.to_string(), (ast, tree));

            return self.cache.get(uri).unwrap().0.to_owned();
        }

        #[cfg(not(feature = "cache"))]
        {
            // Only start a new scope because clippy warnings.
            ast
        }
    }

    /// Get a specific [`ast`] from the cache, this function assumes the ast does
    /// exist. If it may or may not exist, use [`maybe_get_ast`].
    ///
    /// [`maybe_get_ast`]: Self::maybe_get_ast
    /// [`ast`]: Ast
    #[cfg(feature = "cache")]
    pub fn get_ast(&self, uri: &str) -> &Ast {
        &self.cache.get(uri).unwrap().0
    }

    /// Get a specific [`ast`] from the cache, or parse `code` and return the
    /// [`ast`].
    ///
    /// [`ast`]: Ast
    pub fn get_or_create(&mut self, uri: &str, code: &str) -> Ast {
        #[cfg(feature = "cache")]
        if let Some(ast) = self.maybe_get_ast(uri) {
            return ast.to_owned();
        }

        self.parse(code, uri)
    }

    /// Get a specific [`ast`] from the cache, this function, unlike [`get_ast`], doesn't
    /// error when the [`ast`] isn't there.
    ///
    /// [`get_ast`]: LuauParser::get_ast.
    /// [`ast`]: Ast
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
