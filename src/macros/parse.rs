//! Helper parsing macros.

/// Parse a [`Bracketed`](crate::types::Bracketed).
///
/// # Usage
///
/// ```ignore
///  parse_bracketed!(
///     lexer,
///     errors,
///     "<error message>",
///     TokenType::Symbol(Symbol::OpeningParenthesis), // opening token
///     Symbol::ClosingParenthesis, // closing token
/// )
/// ```
#[macro_export]
#[doc(hidden)]
macro_rules! parse_bracketed {
    (
        $lexer: ident,
        $errors: ident,
        $error_message: literal,
        $opening: pat,
        $closing: expr,
    ) => {{
        maybe_next_token!($lexer, __opening_bracket, $opening);

        if let Some(__opening_bracket) = __opening_bracket {
            $crate::types::Bracketed::<_>::parse_with(
                __opening_bracket,
                $lexer,
                $errors,
                ($error_message, $closing),
            )
        } else {
            None
        }
    }};
}

/// Force parse a [`Bracketed`](crate::types::Bracketed) by putting fake tokens
/// if it failed to actually parse it.
///
/// # Usage
///
/// ```ignore
///  parse_bracketed!(
///     lexer,
///     errors,
///     "<error message>",
///     (
///         TokenType::Symbol(Symbol::OpeningParenthesis), // opening token
///         TokenType::Symbol(Symbol::OpeningParenthesis), // replacement in case of failure
///     )
///     Symbol::ClosingParenthesis, // closing token
/// )
#[macro_export]
#[doc(hidden)]
macro_rules! force_parse_bracketed {
    (
        $lexer: ident,
        $errors: ident,
        $error_message: literal,
        ($opening: pat, $opening_replacement: expr),
        $closing: expr,
    ) => {{
        next_token_recoverable!(
            $lexer,
            __opening_bracket,
            $opening,
            $opening_replacement,
            $errors,
            $error_message
        );

        $crate::types::Bracketed::<_>::parse_with(
            __opening_bracket,
            $lexer,
            $errors,
            ($error_message, $closing),
        )
        .unwrap_or_else(|| $crate::types::Bracketed {
            opening_bracket: Token::empty($opening_replacement),
            item: Default::default(),
            closing_bracket: Token::empty($closing.into()),
        })
    }};
}
