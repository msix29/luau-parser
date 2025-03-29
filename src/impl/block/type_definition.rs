use luau_lexer::prelude::{
    Lexer, Literal, Operator, ParseError, PartialKeyword, Symbol, Token, TokenType,
};

use crate::{
    force_parse_bracketed, handle_error_token, parse_bracketed, safe_unwrap,
    types::{
        Bracketed, BracketedList, GenericDeclarationParameter, GenericParameterInfo,
        GenericParameterInfoDefault, Parse, ParseWithArgs, Pointer, Table, TryParse,
        TypeDefinition, TypeValue,
    },
    utils::get_token_type_display,
};

impl TypeValue {
    fn parse_from_name(
        base: Token,
        lexer: &mut Lexer,
        errors: &mut Vec<ParseError>,
    ) -> Option<Self> {
        let state = lexer.save_state();
        let maybe_dot = lexer.next_token();

        let actual_type = if maybe_dot == TokenType::Symbol(Symbol::Dot) {
            next_token_recoverable!(
                lexer,
                name,
                TokenType::Identifier(_) | TokenType::PartialKeyword(_),
                TokenType::Identifier("*error*".into()),
                errors,
                "Expected ".to_string() + get_token_type_display(&TokenType::Identifier("".into()))
            );

            Some((maybe_dot.clone(), name.clone()))
        } else {
            lexer.set_state(state);
            None
        };

        let generics = parse_bracketed!(
            lexer,
            errors,
            "Expected <generic declaration>",
            TokenType::Symbol(Symbol::OpeningAngleBrackets),
            Symbol::ClosingAngleBrackets,
        )
        .map(Pointer::new);

        if let Some((dot, name)) = actual_type {
            Some(Self::Module {
                module: base,
                dot,
                name,
                generics,
            })
        } else {
            maybe_next_token!(lexer, ellipsis, TokenType::Symbol(Symbol::Ellipses));

            if let Some(ellipsis) = ellipsis {
                Some(Self::GenericPack {
                    name: base,
                    ellipsis,
                })
            } else {
                Some(Self::Basic { base, generics })
            }
        }
    }

    fn parse_inner(token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        match token.token_type {
            TokenType::Error(error) => handle_error_token!(errors, error),
            TokenType::Literal(ref literal) => match literal {
                Literal::Number(_) => None,
                Literal::String(_) => Some(Self::String(token)),
                Literal::Boolean(_) => Some(Self::Boolean(token)),
            },
            TokenType::Identifier(_) => Self::parse_from_name(token, lexer, errors),
            TokenType::PartialKeyword(PartialKeyword::TypeOf) => Some(Self::Typeof {
                typeof_token: token,
                inner: force_parse_bracketed!(
                    lexer,
                    errors,
                    "Expected <expr>",
                    (
                        TokenType::Symbol(Symbol::OpeningParenthesis),
                        TokenType::Symbol(Symbol::OpeningParenthesis)
                    ),
                    Symbol::ClosingParenthesis,
                ),
            }),
            TokenType::PartialKeyword(_) => Self::parse_from_name(token, lexer, errors),
            TokenType::Symbol(Symbol::OpeningCurlyBrackets) => {
                Table::parse_with(token, lexer, errors, true).map(Self::Table)
            }
            TokenType::Symbol(Symbol::OpeningParenthesis) => {
                if let Some(bracketed) = BracketedList::<Pointer<TypeValue>>::parse_with(
                    token,
                    lexer,
                    errors,
                    ("Expected <type>", Symbol::ClosingParenthesis),
                ) {
                    if bracketed.items.len() == 1 {
                        Some(Self::Wrap(Bracketed {
                            item: (*bracketed.items[0]).clone(), // different order to fix deref issues.
                            opening_bracket: bracketed.opening_bracket,
                            closing_bracket: bracketed.closing_bracket,
                        }))
                    } else {
                        Some(Self::Tuple(bracketed))
                    }
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

impl Parse for TypeValue {
    fn parse(token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        let left = Self::parse_inner(token, lexer, errors)?;
        let state = lexer.save_state();
        let maybe_operator = lexer.next_token();

        match maybe_operator.token_type {
            TokenType::Operator(Operator::Optional) => Some(Self::Optional {
                base: Pointer::new(left),
                question_mark: maybe_operator,
            }),
            TokenType::Operator(Operator::Intersection) => Some(Self::Intersection {
                left: Pointer::new(left),
                ampersand: maybe_operator,
                right: safe_unwrap!(
                    lexer,
                    errors,
                    "Expected <type>",
                    Self::try_parse(lexer, errors).map(Pointer::new)
                ),
            }),
            TokenType::Operator(Operator::Union) => Some(Self::Union {
                left: Pointer::new(left),
                pipe: maybe_operator,
                right: safe_unwrap!(
                    lexer,
                    errors,
                    "Expected <type>",
                    Self::try_parse(lexer, errors).map(Pointer::new)
                ),
            }),
            _ => {
                lexer.set_state(state);
                Some(left)
            }
        }
    }
}
impl TryParse for TypeValue {}

impl Parse for TypeDefinition {
    fn parse(mut token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        let export_keyword = if token == TokenType::PartialKeyword(PartialKeyword::Export) {
            let temp = token;
            token = lexer.next_token();

            Some(temp)
        } else {
            None
        };
        if token != TokenType::PartialKeyword(PartialKeyword::Type) {
            return None;
        }

        let generics = parse_bracketed!(
            lexer,
            errors,
            "",
            TokenType::Symbol(Symbol::OpeningAngleBrackets),
            Symbol::ClosingAngleBrackets,
        )
        .map(Pointer::new);

        next_token_recoverable!(
            lexer,
            type_name,
            TokenType::Identifier(_) | TokenType::PartialKeyword(_),
            TokenType::Identifier("*error*".into()),
            errors,
            "Expected ".to_string() + get_token_type_display(&TokenType::Identifier("".into()))
        );
        next_token_recoverable!(
            lexer,
            equal_sign,
            TokenType::Symbol(Symbol::Equal),
            TokenType::Symbol(Symbol::Equal),
            errors,
            "Expected `=`"
        );

        let type_value = safe_unwrap!(
            lexer,
            errors,
            "Expected <type>",
            TypeValue::try_parse(lexer, errors).map(Pointer::new)
        );

        Some(Self {
            export_keyword,
            type_keyword: token,
            generics,
            type_name,
            equal_sign,
            type_value,
        })
    }
}
impl TryParse for TypeDefinition {}

impl Parse for GenericParameterInfo {
    fn parse(name: Token, lexer: &mut Lexer, _: &mut Vec<ParseError>) -> Option<Self> {
        if !matches!(
            name.token_type,
            TokenType::Identifier(_) | TokenType::PartialKeyword(_)
        ) {
            return None;
        }

        maybe_next_token!(lexer, ellipsis, TokenType::Symbol(Symbol::Ellipses));

        if let Some(ellipsis) = ellipsis {
            Some(Self::Pack { name, ellipsis })
        } else {
            Some(Self::Name(name))
        }
    }
}
impl TryParse for GenericParameterInfo {}

impl Parse for GenericDeclarationParameter {
    fn parse(token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        let parameter = GenericParameterInfo::parse(token, lexer, errors)?;

        let (equal, default);
        maybe_next_token!(lexer, maybe_equal, TokenType::Symbol(Symbol::Equal));

        if maybe_equal.is_some() {
            equal = maybe_equal;
            default =
                Some(GenericParameterInfoDefault::try_parse(lexer, errors).unwrap_or_default());
            // We use `unwrap_or_default` and `Some` to ensure it actually exists.
        } else {
            (equal, default) = (None, None);
        }

        Some(Self {
            parameter,
            equal,
            default,
        })
    }
}
impl TryParse for GenericDeclarationParameter {}

impl Parse for GenericParameterInfoDefault {
    fn parse(token: Token, lexer: &mut Lexer, errors: &mut Vec<ParseError>) -> Option<Self> {
        match token.token_type {
            TokenType::PartialKeyword(_) | TokenType::Identifier(_) => Some(Self::Name(token)),
            _ => match TypeValue::parse(token, lexer, errors) {
                type_value @ Some(
                    TypeValue::GenericPack { .. }
                    | TypeValue::VariadicPack { .. }
                    | TypeValue::Tuple { .. },
                ) => type_value.map(Self::Pack),
                _ => None,
            },
        }
    }
}
impl TryParse for GenericParameterInfoDefault {}
