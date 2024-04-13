//! Implements display traits for type definitions.

use crate::{
    impl_print_enum, impl_print_struct, optional_print, prelude::{
        GenericDeclaration, GenericDeclarationParameter, GenericParameterInfo,
        GenericParameterInfoDefault, HasRawValue, TypeDefinition, TypeValue,
    }, print
};

/// Try turning generics to a string
pub fn try_generics_to_string(generics: &Option<GenericDeclaration>) -> String {
    generics
        .as_ref()
        .map_or_else(|| "".to_string(), |generics| generics.get_raw_value())
}

impl HasRawValue for TypeDefinition {
    fn get_raw_value(&self) -> String {
        if let Some(type_keyword) = &self.type_keyword {
            let export = self
                .export_keyword
                .as_ref()
                .map_or_else(|| "".to_string(), |export| export.get_raw_value());

            format!(
                "{} {}{} = {}",
                export,
                type_keyword.get_raw_value(),
                try_generics_to_string(&self.generics),
                self.type_value.get_raw_value()
            )
        } else {
            self.type_value.get_raw_value()
        }
    }
}
impl_print_struct!(
    TypeDefinition,
    { self.export_keyword, optional_print! },
    { self.type_keyword, optional_print! },
    { self.generics, optional_print! },
    { self.type_name, print! },
    { self.equal_sign, optional_print! },
    { self.type_value, print! }
);

impl HasRawValue for TypeValue {
    fn get_raw_value(&self) -> String {
        match self {
            TypeValue::Basic(value) | TypeValue::String(value) | TypeValue::Boolean(value) => {
                value.get_raw_value()
            }
            TypeValue::Wrap { r#type, .. } => format!("({})", r#type.get_raw_value()),
            TypeValue::Function {
                generics,
                parameters,
                return_type,
                ..
            } => format!(
                "{}({}) -> {}",
                try_generics_to_string(generics),
                parameters.get_raw_value(),
                return_type.get_raw_value()
            ),
            TypeValue::Generic { base, generics, .. } => {
                format!("{}<{}>", base.get_raw_value(), generics.get_raw_value())
            }
            TypeValue::GenericPack { name, .. } => format!("{}...", name.get_raw_value()),
            TypeValue::Intersection { left, right, .. } => {
                format!("{} & {}", left.get_raw_value(), right.get_raw_value())
            }
            TypeValue::Union { left, right, .. } => {
                format!("{} | {}", left.get_raw_value(), right.get_raw_value())
            }
            TypeValue::Module { type_info, .. } => type_info.get_raw_value(),
            TypeValue::Optional { base, .. } => format!("{}?", base.get_raw_value()),
            TypeValue::Table(value) => value.get_raw_value(),
            TypeValue::Typeof { inner, .. } => format!("typeof({})", inner.get_raw_value()),
            TypeValue::Tuple { types, .. } => format!("({})", types.get_raw_value()),
            TypeValue::Variadic { type_info, .. } => format!("...{}", type_info.get_raw_value()),
            TypeValue::VariadicPack { name, .. } => format!("...{}", name.get_raw_value()),
        }
    }
}
impl_print_enum!(
    TypeValue,
    {},
    {
        Basic,
        String,
        Boolean,
        Table,
    },
    {
        { Wrap, { opening_parenthesis, r#type, closing_parenthesis, } },
        { Function, {
            opening_parenthesis,
            parameters,
            closing_parenthesis,
            arrow,
            return_type,
            { generics },
        }},
        { Generic, { base, right_arrows, generics, left_arrows, } },
        { GenericPack, { name, ellipsis, } },
        { Intersection, { left, ampersand, right, } },
        { Union, { left, pipe, right, } },
        { Module, { module, dot, type_info, } },
        { Optional, { base, question_mark, } },
        { Typeof, { typeof_token, opening_parenthesis, inner, closing_parenthesis, } },
        { Tuple, { opening_parenthesis, types, closing_parenthesis, } },
        { Variadic, { ellipsis, type_info, } },
        { VariadicPack, { ellipsis, name, } },
    }
);

impl HasRawValue for GenericDeclaration {
    fn get_raw_value(&self) -> String {
        format!(
            "{}{}{}",
            self.right_arrow.get_raw_value(),
            self.generics
                .items
                .iter()
                .map(|generic| generic.get_raw_value())
                .collect::<Vec<String>>()
                .join(", "),
            self.left_arrow.get_raw_value()
        )
    }
}
impl_print_struct!(
    GenericDeclaration,
    { self.right_arrow, print! },
    { self.generics, print! },
    { self.left_arrow, print! }
);

impl HasRawValue for GenericDeclarationParameter {
    fn get_raw_value(&self) -> String {
        if let Some(default) = &self.default {
            format!(
                "{} = {}",
                self.parameter.get_raw_value(),
                default.get_raw_value()
            )
        } else {
            self.parameter.get_raw_value()
        }
    }
}
impl_print_struct!(
    GenericDeclarationParameter,
    { self.parameter, print! },
    { self.default, optional_print! }
);

impl HasRawValue for GenericParameterInfoDefault {
    fn get_raw_value(&self) -> String {
        match self {
            GenericParameterInfoDefault::Name { name, .. } => name.get_raw_value(),
            GenericParameterInfoDefault::Pack { r#type, .. } => r#type.get_raw_value(),
        }
    }
}
impl_print_enum!(
    GenericParameterInfoDefault,
    {},
    {},
    {
        { Name, { equal_sign, name, } },
        { Pack, { equal_sign, r#type, } },
    }
);

impl HasRawValue for GenericParameterInfo {
    fn get_raw_value(&self) -> String {
        match self {
            GenericParameterInfo::Name(name) => name.get_raw_value(),
            GenericParameterInfo::Pack { name, .. } => format!("{}...", name.get_raw_value()),
        }
    }
}
impl_print_enum!(
    GenericParameterInfo,
    {},
    { Name, },
    {
        { Pack, { name, ellipsis, } }
    }
);
