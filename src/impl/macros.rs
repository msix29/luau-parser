macro_rules! next_token {
    ($lexer: ident, $name: ident, $pattern: pat, $errors: ident, $error_message: expr) => {
        next_token_with_condition!(
            $lexer,
            $name,
            matches!(&$name.token_type, $pattern),
            $errors,
            $error_message
        )
    };
}

macro_rules! next_token_with_condition {
    ($lexer: ident, $name: ident, $condition: expr, $errors: ident, $error_message: expr) => {
        let state = $lexer.save_state();
        let $name = $lexer.next_token();
        if !$condition {
            $errors.push(
                luau_lexer::prelude::ParseError::new(
                    state.lexer_position(),
                    format!(
                        "{} found {}",
                        $error_message,
                        $crate::utils::get_token_type_display(&$name.token_type)
                    ),
                    Some(state.lexer_position()),
                )
                .into(),
            );
            $lexer.set_state(state);

            return None;
        }
    };
}

macro_rules! next_token_recoverable {
    (
        $lexer: ident,
        $name: ident,
        $pattern: pat,
        $replacement: expr,
        $errors: ident,
        $error_message: expr
    ) => {
        next_token_recoverable_with_condition!(
            $lexer,
            $name,
            matches!(&$name.token_type, $pattern),
            $replacement,
            $errors,
            $error_message
        )
    };
}

macro_rules! next_token_recoverable_with_condition {
    (
        $lexer: ident,
        $name: ident,
        $condition: expr,
        $replacement: expr,
        $errors: ident,
        $error_message: expr
    ) => {
        let state = $lexer.save_state();
        let mut $name = $lexer.next_token();
        if !$condition {
            $errors.push(
                luau_lexer::prelude::ParseError::new(
                    state.lexer_position(),
                    format!(
                        "{} found {}",
                        $error_message,
                        $crate::utils::get_token_type_display_extended(&$name.token_type)
                    ),
                    Some(state.lexer_position()),
                )
                .into(),
            );

            $name = luau_lexer::prelude::Token {
                start: state.lexer_position(),
                spaces_before: smol_str::SmolStr::new(""),
                token_type: $replacement,
                spaces_after: smol_str::SmolStr::new(""),
                end: state.lexer_position(),
            };
            $lexer.set_state(state);
        }
    };
}

macro_rules! maybe_next_token {
    ($lexer: ident, $name: ident, $pattern: pat) => {
        maybe_next_token_with_condition!($lexer, $name, matches!($name.token_type, $pattern))
    };
}

macro_rules! maybe_next_token_with_condition {
    ($lexer: ident, $name: ident, $condition: expr) => {
        let state = $lexer.save_state();
        let $name = $lexer.next_token();

        let $name = if $condition {
            Some($name)
        } else {
            $lexer.set_state(state);

            None
        };
    };
}

macro_rules! parse_function {
    (
        $function_keyword: expr,
        $lexer: ident,
        $errors: ident
        $(, let $fn_name: ident = $name: block )?
        $(, { $($extra_field:ident),* $(,)?})?
    ) => {{
        let state = $lexer.save_state();
        let function_keyword = $function_keyword;
        if function_keyword != TokenType::Keyword(Keyword::Function) {
            $lexer.set_state(state);

            return None;
        }

        $( let $fn_name = $name; )?

        let generics = parse_bracketed!(
            $lexer,
            $errors,
            "Expected <generic declaration>",
            TokenType::Symbol(Symbol::OpeningAngleBrackets),
            Symbol::ClosingAngleBrackets,
        )
        .map(Pointer::new);

        let parameters = force_parse_bracketed!(
            $lexer,
            $errors,
            "Expected <parameter>",
            (
                TokenType::Symbol(Symbol::OpeningParenthesis),
                TokenType::Symbol(Symbol::OpeningParenthesis)
            ),
            Symbol::ClosingParenthesis,
        );

        maybe_next_token!($lexer, maybe_colon, TokenType::Symbol(Symbol::Colon));
        let return_type = if maybe_colon.is_some() {
            Pointer::<TypeValue>::try_parse($lexer, $errors)
        } else {
            None
        };

        let body = Block::try_parse_with($lexer, $errors, TokenType::Keyword(Keyword::End))
            .unwrap_or_default();

        next_token_recoverable!(
            $lexer,
            end_keyword,
            TokenType::Keyword(Keyword::End),
            TokenType::Keyword(Keyword::End),
            $errors,
            "Expected ".to_string()
                + get_token_type_display_extended(&TokenType::Keyword(Keyword::End))
        );

        Some(Self {
            $($($extra_field,)*)?
            function_keyword,
            generics,
            parameters,
            colon: maybe_colon.map(Pointer::new),
            return_type,
            body,
            end_keyword,
        })
    }};
}
