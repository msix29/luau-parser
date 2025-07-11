//! All `impl` blocks for CST-related types.

#[macro_use]
mod macros;

mod block;
mod bracketed;
mod cst;
mod expression;
mod list;
mod name;
mod value;

use lsp_types::Range;
use luau_lexer::{
    prelude::{Comment, Lexer, ParseError, Token, Trivia},
    token::TokenType,
};

use crate::types::{
    GetRange, GetRangeError, Parse, ParseWithArgs, Pointer, Print, TryParse, TryParseWithArgs,
};

impl<T: Parse> Parse for Pointer<T> {
    #[inline]
    fn parse(token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        T::parse(token, lexer, errors).map(Self::new)
    }
}
impl<T: TryParse + Parse> TryParse for Pointer<T> {
    #[inline]
    fn try_parse(lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        T::try_parse(lexer, errors).map(Self::new)
    }
}

impl<T: Parse> Parse for Vec<T> {
    #[inline]
    fn parse(mut token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        let mut values = Vec::new();
        let mut state = lexer.save_state();

        while let Some(value) = T::parse(token, lexer, errors) {
            values.push(value);
            state = lexer.save_state();
            token = lexer.next_token();
        }

        lexer.set_state(state);

        (!values.is_empty()).then_some(values)
    }
}
impl<T: TryParse + Parse> TryParse for Vec<T> {}

impl<T: ParseWithArgs<A>, A: Clone> ParseWithArgs<A> for Vec<T> {
    #[inline]
    fn parse_with(
        mut token: Token,
        lexer: &mut Lexer,
        errors: &mut Vec<ParseError>,
        args: A,
    ) -> Option<Self> {
        let mut values = Vec::new();
        let mut state = lexer.save_state();

        while let Some(value) = T::parse_with(token, lexer, errors, args.clone()) {
            values.push(value);
            state = lexer.save_state();
            token = lexer.next_token();
        }

        lexer.set_state(state);

        (!values.is_empty()).then_some(values)
    }
}
impl<T: ParseWithArgs<A>, A: Clone> TryParseWithArgs<A> for T {}

impl GetRange for Token {
    #[inline]
    fn get_range(&self) -> Result<Range, GetRangeError> {
        Ok(Range::new(self.start, self.end))
    }
}

impl Print for Comment {
    #[inline]
    fn print(&self) -> String {
        match self {
            Comment::SingleLine(smol_str) | Comment::MultiLine(smol_str) => smol_str.to_string(),
        }
    }

    fn print_final_trivia(&self) -> String {
        unreachable!()
    }
    fn print_without_final_trivia(&self) -> String {
        unreachable!()
    }
}
impl Print for Trivia {
    #[inline]
    fn print(&self) -> String {
        match self {
            Trivia::Spaces(smol_str) => smol_str.to_string(),
            Trivia::Comment(comment) => comment.print(),
        }
    }

    fn print_final_trivia(&self) -> String {
        unreachable!()
    }
    fn print_without_final_trivia(&self) -> String {
        unreachable!()
    }
}

/// [`Print`] implementation for [`Vec<Trivia>`] as the default one won't work.
/// It takes `&[Trivia]` so we don't need to `.clone()`.
fn print_trivia(trivia: &[Trivia]) -> String {
    trivia
        .iter()
        .fold("".to_string(), |str, item| str + &item.print())
}

impl Print for Token {
    #[inline]
    fn print_final_trivia(&self) -> String {
        if self.token_type == TokenType::EndOfFile {
            print_trivia(&self.leading_trivia)
        } else {
            print_trivia(&self.trailing_trivia)
        }
    }

    #[inline]
    fn print_without_final_trivia(&self) -> String {
        self.token_type
            .try_as_string()
            .map(|token_type| print_trivia(&self.leading_trivia) + &token_type)
            .unwrap_or_default()
    }

    #[inline]
    fn print(&self) -> String {
        self.token_type
            .try_as_string()
            .map(|token_type| {
                print_trivia(&self.leading_trivia)
                    + &token_type
                    + &print_trivia(&self.trailing_trivia)
            })
            .unwrap_or_default()
    }
}

impl<T: GetRange> GetRange for Pointer<T> {
    #[inline]
    fn get_range(&self) -> Result<Range, GetRangeError> {
        (**self).get_range()
    }
}
impl<T: Print> Print for Pointer<T> {
    #[inline]
    fn print(&self) -> String {
        (**self).print()
    }

    #[inline]
    fn print_final_trivia(&self) -> String {
        (**self).print_final_trivia()
    }

    #[inline]
    fn print_without_final_trivia(&self) -> String {
        (**self).print_without_final_trivia()
    }
}

impl<T: Print> Print for Option<T> {
    #[inline]
    fn print(&self) -> String {
        match self {
            Some(item) => item.print(),
            None => "".to_string(),
        }
    }

    fn print_final_trivia(&self) -> String {
        match self {
            Some(item) => item.print_final_trivia(),
            None => "".to_string(),
        }
    }

    fn print_without_final_trivia(&self) -> String {
        match self {
            Some(item) => item.print_without_final_trivia(),
            None => "".to_string(),
        }
    }
}

impl<T: GetRange> GetRange for Vec<T> {
    #[inline]
    fn get_range(&self) -> Result<Range, GetRangeError> {
        if self.is_empty() {
            Err(GetRangeError::EmptyList)
        } else if self.len() == 1 {
            self[0].get_range()
        } else {
            Ok(Range::new(
                self[0].get_range()?.start,
                self.last().unwrap().get_range()?.end,
            ))
        }
    }
}
impl<T: Print> Print for Vec<T> {
    #[inline]
    fn print_final_trivia(&self) -> String {
        self.last()
            .map(|item| item.print_final_trivia())
            .unwrap_or_default()
    }

    #[inline]
    fn print_without_final_trivia(&self) -> String {
        self.iter().fold("".to_string(), |str, item| {
            str + &item.print_without_final_trivia()
        })
    }
}

impl<T: Print, U: Print> Print for (T, U) {
    #[inline]
    fn print(&self) -> String {
        let end = self.1.print();

        if end.is_empty() {
            self.0.print()
        } else {
            self.0.print_without_final_trivia() + &end
        }
    }

    #[inline]
    fn print_final_trivia(&self) -> String {
        let maybe_final_trivia = self.1.print_final_trivia();
        if maybe_final_trivia.is_empty() {
            self.0.print_final_trivia()
        } else {
            maybe_final_trivia
        }
    }

    #[inline]
    fn print_without_final_trivia(&self) -> String {
        self.0.print_without_final_trivia() + &self.1.print_without_final_trivia()
    }
}
