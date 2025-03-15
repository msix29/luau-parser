//! The main item of this crate, the actual [`parser`](LuauParser).

use luau_lexer::lexer::Lexer;
#[cfg(feature = "cache")]
use std::collections::HashMap;
use std::sync::Arc;

use crate::prelude::{Cst, AstStatus, TerminationStatement, Statement};

/// A Luau parser.
pub struct LuauParser<'a> {
    /// Cache, only works with the `cache` feature, this is useful when you need
    /// to use the [`CST`](Cst) more than once in 2 different places without
    /// reparsing or with the `uri` only.
    #[cfg(feature = "cache")]
    cache: HashMap<String, Cst>,

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

    /// Parse Luau code into an [`CST`](Cst).
    pub fn parse(&mut self, code: &str, uri: &str) -> Cst {
        // NOTE: Can a text editor use `\r` by itself independant of the OS? If so, remove
        // this `cfg`.
        #[cfg(windows)]
        let code = &code.replace('\r', "");

        let cst = Cst::default();

        #[cfg(feature = "cache")]
        {
            self.cache.insert(uri.to_string(), cst);

            self.cache.get(uri).unwrap().to_owned()
        }

        #[cfg(not(feature = "cache"))]
        {
            // Only start a new scope because clippy warnings.
            cst
        }
    }

    /// Get a specific [`CST`]Cfrom the cache, this function assumes the cst does
    /// exist. If it may or may not exist, use [`maybe_get_ast`].
    ///
    /// [`maybe_get_ast`]: Self::maybe_get_ast
    /// [`CST`]:CAst
    #[cfg(feature = "cache")]
    #[inline]
    pub fn get_ast(&self, uri: &str) -> &Cst {
        self.cache.get(uri).unwrap()
    }

    /// Get a specific [`CST`]Cfrom the cache, or parse `code` and return the
    /// [`CST`].C    ///
    /// [`CST`]:CAst
    #[inline]
    pub fn get_or_create(&mut self, uri: &str, code: &str) -> Cst {
        #[cfg(feature = "cache")]
        if let Some(cst) = self.maybe_get_ast(uri) {
            return cst.to_owned();
        }

        self.parse(code, uri)
    }

    /// Get a specific [`CST`]Cfrom the cache, this function, unlike [`get_ast`], doesn't
    /// error when the [`CST`]Cisn't there.
    ///
    /// [`get_ast`]: LuauParser::get_ast.
    /// [`CST`]:CAst
    #[cfg(feature = "cache")]
    #[inline]
    pub fn maybe_get_ast(&self, uri: &str) -> Option<&Cst> {
        self.cache.get(uri)
    }

    /// Get all cached [`CST`](Cst)s.
    #[cfg(feature = "cache")]
    #[inline]
    pub fn get_all_asts(&self) -> &HashMap<String, Cst> {
        &self.cache
    }

    /// Clear the cache.
    #[cfg(feature = "cache")]
    #[inline]
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }
}
