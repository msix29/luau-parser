#[macro_use]
mod macros;

mod block;
mod bracketed;
mod cst;
mod expression;
mod list;
mod literals;
mod name;
mod range;
mod value;

use luau_lexer::prelude::{Lexer, ParseError, Token};

use crate::{
    prelude::PrintError,
    types::{
        GetRange, GetRangeError, Parse, ParseWithArgs, Pointer, Print, Range, TryParse,
        TryParseWithArgs,
    },
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
impl Print for Token {
    #[inline]
    fn print_with_leading(&self) -> Result<String, PrintError> {
        self.token_type
            .try_as_string()
            .ok_or(PrintError::ErrorVariant)
            .map(|token_type| self.spaces_before.to_string() + &token_type)
    }

    #[inline]
    fn print(&self) -> Result<String, PrintError> {
        self.token_type
            .try_as_string()
            .ok_or(PrintError::ErrorVariant)
            .map(|token_type| self.spaces_before.to_string() + &token_type + &self.spaces_after)
    }

    #[inline]
    fn print_with_trailing(&self) -> Result<String, PrintError> {
        self.token_type
            .try_as_string()
            .ok_or(PrintError::ErrorVariant)
            .map(|token_type| token_type.to_string() + &self.spaces_after)
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
    fn print_with_leading(&self) -> Result<String, PrintError> {
        (**self).print_with_leading()
    }

    #[inline]
    fn print(&self) -> Result<String, PrintError> {
        (**self).print()
    }

    #[inline]
    fn print_with_trailing(&self) -> Result<String, PrintError> {
        (**self).print_with_trailing()
    }
}

impl<T: Print> Print for Option<T> {
    #[inline]
    fn print_with_leading(&self) -> Result<String, PrintError> {
        match self {
            Some(item) => item.print_with_leading(),
            None => Err(PrintError::NoneValue),
        }
    }

    #[inline]
    fn print(&self) -> Result<String, PrintError> {
        match self {
            Some(item) => item.print(),
            None => Err(PrintError::NoneValue),
        }
    }

    #[inline]
    fn print_with_trailing(&self) -> Result<String, PrintError> {
        match self {
            Some(item) => item.print_with_trailing(),
            None => Err(PrintError::NoneValue),
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
    fn print_with_leading(&self) -> Result<String, PrintError> {
        let mut str = String::new();

        for item in self.iter() {
            str += &item.print_with_leading()?;
        }

        Ok(str)
    }

    fn print(&self) -> Result<String, PrintError> {
        let mut str = String::new();
        let last_index = self.len() - 1;

        for (i, item) in self.iter().enumerate() {
            if i == last_index {
                str += &item.print()?;
            } else {
                str += &item.print_with_leading()?;
            }
        }

        Ok(str)
    }

    fn print_with_trailing(&self) -> Result<String, PrintError> {
        let mut str = String::new();

        for item in self.iter() {
            str += &item.print_with_trailing()?;
        }

        Ok(str)
    }
}

impl<T: Print, U: Print> Print for (T, U) {
    fn print_with_leading(&self) -> Result<String, PrintError> {
        if let Ok(item) = self.1.print_with_leading() {
            Ok(self.0.print_with_leading()? + &item)
        } else {
            self.0.print_with_leading()
        }
    }

    fn print(&self) -> Result<String, PrintError> {
        if let Ok(item) = self.1.print() {
            Ok(self.0.print_with_leading()? + &item)
        } else {
            self.0.print()
        }
    }

    fn print_with_trailing(&self) -> Result<String, PrintError> {
        if let Ok(item) = self.1.print_with_trailing() {
            Ok(self.0.print_with_trailing()? + &item)
        } else {
            self.0.print_with_trailing()
        }
    }
}
