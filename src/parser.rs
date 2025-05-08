//! The main item of this crate, the actual [`parser`](Parser).

use luau_lexer::lexer::Lexer;
#[cfg(feature = "cache")]
use std::collections::HashMap;

use crate::types::{Cst, Pointer};

/// The cache used in [`Parser`] when `cache` feature is enabled.
#[cfg(feature = "cache")]
pub type ParserCache = HashMap<String, Pointer<Cst>>;

/// A Luau parser.
pub struct Parser<'a> {
    /// Cache, only works with the `cache` feature, this is useful when you need
    /// to use the [`CST`](Cst) more than once in 2 different places without
    /// re-parsing.
    #[cfg(feature = "cache")]
    cache: ParserCache,

    /// The `tree-sitter` parser.
    lexer: Lexer<'a>,
}

impl<'a> Parser<'a> {
    /// Create a new [`parser`](Parser).
    #[inline]
    pub fn new(input: &'a str) -> Self {
        Self {
            #[cfg(feature = "cache")]
            cache: HashMap::new(),
            lexer: Lexer::new(input),
        }
    }

    /// Set the parser's input. Meant to be chained.
    pub fn with_input(mut self, input: &'a str) -> Self {
        self.lexer = self.lexer.with_input(input);
        self
    }

    /// Set the parser's input.
    pub fn set_input(&mut self, input: &'a str) {
        self.lexer.set_input(input);
    }

    /// Parse Luau code into an [`CST`](Cst).
    pub fn parse(&mut self, uri: &str) -> Pointer<Cst> {
        let cst = Pointer::new(Cst::parse(self.lexer.next_token(), &mut self.lexer, uri));

        #[cfg(feature = "cache")]
        {
            self.cache.insert(uri.to_string(), cst);

            self.cache.get(uri).unwrap().to_owned()
        }

        #[cfg(not(feature = "cache"))]
        cst
    }

    /// Get a specific [`CST`](Cst) from the cache, this function assumes the
    /// cst does exist. If it may or may not exist, use
    /// [`maybe_get_ast`](Self::maybe_get_ast).
    #[cfg(feature = "cache")]
    #[inline]
    pub fn get_ast(&self, uri: &str) -> &Cst {
        self.cache.get(uri).unwrap()
    }

    /// Get a specific [`CST`](Cst) from the cache (if `cache` feature is enabled),
    /// or parse `code` and return the produced [`CST`](Cst)
    #[inline]
    pub fn get_or_create(&mut self, uri: &str, code: &'a str) -> Pointer<Cst> {
        #[cfg(feature = "cache")]
        if let Some(cst) = self.maybe_get_ast(uri) {
            return cst;
        }

        self.set_input(code);
        self.parse(uri)
    }

    /// Get a specific [`CST`](Cst) from the cache, this function, unlike
    /// [`get_ast`](Self::get_ast), doesn't error when the [`CST`](Cst) isn't
    /// there.
    #[cfg(feature = "cache")]
    #[inline]
    pub fn maybe_get_ast(&self, uri: &str) -> Option<Pointer<Cst>> {
        self.cache.get(uri).cloned()
    }

    /// Get all cached [`CST`](Cst)s.
    #[cfg(feature = "cache")]
    #[inline]
    pub fn get_all_asts(&self) -> &ParserCache {
        &self.cache
    }

    /// Clear the cache.
    #[cfg(feature = "cache")]
    #[inline]
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }
}
