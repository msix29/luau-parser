use luau_lexer::prelude::{Lexer, ParseError, Symbol, Token, TokenType};
use std::ops::{Deref, DerefMut};

use crate::types::{
    GetRange, GetRangeError, List, ListItem, Parse, ParseWithArgs, Print, Range, TryParse,
};

impl<T> List<T> {
    #[inline]
    pub const fn new() -> Self {
        Self { items: Vec::new() }
    }

    fn parse<C: FnMut(Token, &mut Lexer) -> Option<T>>(
        mut token: Token,
        lexer: &mut Lexer,
        mut parse: C,
    ) -> Option<Self> {
        let mut items = Vec::new();
        let mut state = lexer.save_state();

        while let Some(item) = parse(token, lexer) {
            maybe_next_token!(lexer, maybe_comma, TokenType::Symbol(Symbol::Comma));
            state = lexer.save_state();

            if let Some(comma) = maybe_comma {
                items.push(ListItem::Trailing {
                    item,
                    separator: comma,
                });
            } else {
                items.push(ListItem::NonTrailing(item));

                break;
            }

            token = lexer.next_token();
        }

        lexer.set_state(state);

        Some(Self { items })
    }
}

impl<T> Default for List<T> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Parse> Parse for List<T> {
    #[inline]
    fn parse(token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        Self::parse(token, lexer, |token, lexer| T::parse(token, lexer, errors))
    }
}
impl<T: Parse> TryParse for List<T> {}

impl<A: Clone, T: ParseWithArgs<A>> ParseWithArgs<A> for List<T> {
    #[inline]
    fn parse_with(
        token: Token,
        lexer: &mut Lexer,
        errors: &mut Vec<ParseError>,
        args: A,
    ) -> Option<Self> {
        Self::parse(token, lexer, |token, lexer| {
            T::parse_with(token, lexer, errors, args.clone())
        })
    }
}

impl<T> Deref for List<T> {
    type Target = Vec<ListItem<T>>;

    fn deref(&self) -> &Self::Target {
        &self.items
    }
}
impl<T> DerefMut for List<T> {
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

impl<T: GetRange> GetRange for List<T> {
    fn get_range(&self) -> Result<Range, GetRangeError> {
        (**self).get_range()
    }
}

impl<T: GetRange> GetRange for ListItem<T> {
    fn get_range(&self) -> Result<Range, GetRangeError> {
        match self {
            ListItem::Trailing { item, separator } => Ok(Range::new(
                item.get_range()?.start,
                separator.get_range()?.end,
            )),
            ListItem::NonTrailing(item) => item.get_range(),
        }
    }
}

impl<T: Print> Print for List<T> {
    #[inline]
    fn print(&self) -> String {
        self.items.print()
    }
}

impl<T: Print> Print for ListItem<T> {
    #[inline]
    fn print(&self) -> String {
        match self {
            Self::Trailing { item, separator } => {
                item.print().trim_end().to_string() + &separator.print()
            }
            Self::NonTrailing(item) => item.print(),
        }
    }
}
