#[macro_export]
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

#[macro_export]
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

#[macro_export]
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

#[macro_export]
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

#[macro_export]
macro_rules! maybe_next_token {
    ($lexer: ident, $name: ident, $pattern: pat) => {
        maybe_next_token_with_condition!($lexer, $name, matches!($name.token_type, $pattern))
    };
}

#[macro_export]
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
