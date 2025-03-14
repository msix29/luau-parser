//! The main item of this crate, the actual [`parser`](LuauParser).

use luau_lexer::lexer::Lexer;
#[cfg(feature = "cache")]
use std::collections::HashMap;
use std::sync::Arc;

use crate::prelude::{Ast, AstStatus, LastStatement, Statement, Token};

/// A Luau parser.
pub struct LuauParser<'a> {
    /// Cache, only works with the `cache` feature, this is useful when you need
    /// to use the [`ast`](Ast) more than once in 2 different places without
    /// reparsing or with the `uri` only.
    #[cfg(feature = "cache")]
    cache: HashMap<String, Ast>,

    /// The `tree-sitter` parser.
    lexer: Lexer<'a>,
}

impl<'a> LuauParser<'a> {
    /// Create a new [`parser`](LuauParser).
    #[inline]
    pub fn new(input: &'a str) -> Self {
        LuauParser {
            #[cfg(feature = "cache")]
            cache: HashMap::new(),
            lexer: Lexer::new(input),
        }
    }

    /// Parse Luau code into an [`ast`](Ast).
    pub fn parse(&mut self, code: &str, uri: &str) -> Ast {
        // NOTE: Can a text editor use `\r` by itself independant of the OS? If so, remove
        // this `cfg`.
        #[cfg(windows)]
        let code = &code.replace('\r', "");

        let ast = Ast::default();

        #[cfg(feature = "cache")]
        {
            self.cache.insert(uri.to_string(), ast);

            self.cache.get(uri).unwrap().to_owned()
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
    #[inline]
    pub fn get_ast(&self, uri: &str) -> &Ast {
        self.cache.get(uri).unwrap()
    }

    /// Get a specific [`ast`] from the cache, or parse `code` and return the
    /// [`ast`].
    ///
    /// [`ast`]: Ast
    #[inline]
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
    #[inline]
    pub fn maybe_get_ast(&self, uri: &str) -> Option<&Ast> {
        self.cache.get(uri)
    }

    /// Get all cached [`asts`](Ast).
    #[cfg(feature = "cache")]
    #[inline]
    pub fn get_all_asts(&self) -> &HashMap<String, Ast> {
        &self.cache
    }

    /// Clear the cache.
    #[cfg(feature = "cache")]
    #[inline]
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }
}
