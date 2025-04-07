//! All `impl` blocks for [`Bracketed`].

use luau_lexer::prelude::{Lexer, ParseError, State, Symbol, Token, TokenType};
use std::ops::{Deref, DerefMut};

use crate::{
    types::{Bracketed, List, Parse, ParseWithArgs, Print, TryParse, TryParseWithArgs},
    utils::get_token_type_display_extended,
};

/// A private trait that indicates whether or not the current item is empty.
/// It's only implemented for a limited set of types.
trait IsEmpty {
    /// Whether or not this item is empty.
    fn is_empty(&self) -> bool {
        false
    }
}
impl<T> IsEmpty for List<T> {
    fn is_empty(&self) -> bool {
        (**self).is_empty()
    }
}

/// Implements [`IsEmpty`] for the passed type and for
/// [`Pointer`](crate::types::Pointer) of that type
macro_rules! __sealed_impl {
    ($($ty:ident $(<$generic:ident>)?),* $(,)?) => {
        $( impl $(<$generic>)? IsEmpty for $crate::types::$ty $(<$generic>)? {})*
        $( impl $(<$generic>)? IsEmpty for $crate::types::Pointer<$crate::types::$ty $(<$generic>)?> {})*
    };
}
__sealed_impl!(Bracketed<T>, TypeValue, Expression);

#[allow(private_bounds)]
impl<T: IsEmpty> Bracketed<T> {
    /// The actual parsing logic.
    fn parse(
        previous_state: State,
        maybe_parsed_item: Option<T>,
        opening_bracket: Token,
        lexer: &mut Lexer,
        errors: &mut Vec<ParseError>,
        (error_message, stop_at): (&str, Symbol),
    ) -> Option<Self> {
        let Some(item) = maybe_parsed_item else {
            let state = lexer.save_state();
            errors.push(ParseError::new(
                state.lexer_position(),
                error_message.to_string(),
                Some(state.lexer_position()),
            ));

            return None;
        };

        if item.is_empty() {
            lexer.set_state(previous_state);
        }

        next_token_recoverable_with_condition!(
            lexer,
            closing_bracket,
            closing_bracket.token_type == TokenType::Symbol(stop_at),
            TokenType::Symbol(stop_at),
            errors,
            format!(
                "Expected {}",
                get_token_type_display_extended(&TokenType::Symbol(stop_at))
            )
        );

        Some(Self {
            opening_bracket,
            item,
            closing_bracket,
        })
    }
}

impl<T: Parse + TryParse + IsEmpty> ParseWithArgs<(&str, Symbol)> for Bracketed<T> {
    #[inline]
    fn parse_with(
        opening_bracket: Token,
        lexer: &mut Lexer,
        errors: &mut Vec<ParseError>,
        (error_message, stop_at): (&str, Symbol),
    ) -> Option<Self> {
        Self::parse(
            lexer.save_state(),
            T::parse(lexer.next_token(), lexer, errors),
            opening_bracket,
            lexer,
            errors,
            (error_message, stop_at),
        )
    }
}
impl<A, T> ParseWithArgs<(&str, Symbol, A)> for Bracketed<T>
where
    T: ParseWithArgs<A> + TryParseWithArgs<A> + IsEmpty,
{
    #[inline]
    fn parse_with(
        opening_bracket: Token,
        lexer: &mut Lexer,
        errors: &mut Vec<ParseError>,
        (error_message, stop_at, args): (&str, Symbol, A),
    ) -> Option<Self> {
        Self::parse(
            lexer.save_state(),
            T::parse_with(lexer.next_token(), lexer, errors, args),
            opening_bracket,
            lexer,
            errors,
            (error_message, stop_at),
        )
    }
}

impl<T> Deref for Bracketed<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.item
    }
}

impl<T> DerefMut for Bracketed<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.item
    }
}

impl<T: Print> Print for Bracketed<T> {
    fn print(&self) -> String {
        self.opening_bracket.print().trim_end().to_string()
            + self.item.print().trim_end()
            + &self.closing_bracket.print()
    }
}
