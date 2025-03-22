use luau_lexer::prelude::{Lexer, ParseError, Token};
use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use crate::types::{List, ListItem, Parse};

impl<T> List<T> {
    #[inline]
    pub const fn new() -> Self {
        Self { items: Vec::new() }
    }
}

impl<T: Debug + Parse> Parse for List<T> {
    fn parse(token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        todo!()
    }
}

impl <T> Deref for List<T> {
    type Target = Vec<ListItem<T>>;

    fn deref(&self) -> &Self::Target {
        &self.items
    }
}
impl <T> DerefMut for List<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.items
    }
}

impl<T> Deref for ListItem<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            ListItem::Trailing { item, .. } => item,
            ListItem::NonTrailing(item) => item,
        }
    }
}
impl<T> DerefMut for ListItem<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            ListItem::Trailing { item, .. } => item,
            ListItem::NonTrailing(item) => item,
        }
    }
}
