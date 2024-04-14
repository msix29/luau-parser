//! Implements display traits for type definitions.

use crate::{
    impl_print_enum, impl_print_struct, optional_print,
    prelude::{
        GenericDeclaration, GenericDeclarationParameter, GenericParameterInfo,
        GenericParameterInfoDefault, HasRawValue, TypeDefinition, TypeValue,
    },
    print,
};

/// Try turning generics to a string
pub fn try_generics_to_string(generics: &Option<GenericDeclaration>, add_space_before: bool) -> String {
    generics
        .as_ref()
        .map_or_else(|| "".to_string(), |generics| {
            if add_space_before {
                String::from(" ") + &generics.get_raw_value()
            } else {
                generics.get_raw_value()
            }
        })
}

impl HasRawValue for TypeDefinition {
    fn get_raw_value(&self) -> String {
        if let Some(type_keyword) = &self.type_keyword {
            let export = self
                .export_keyword
                .as_ref()
                .map_or_else(|| "".to_string(), |export| export.get_raw_value());

            format!(
                "{} {} {}{} = {}",
                export,
                type_keyword.get_raw_value(),
                self.type_name.get_raw_value(),
                try_generics_to_string(&self.generics, false),
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
                try_generics_to_string(generics, false),
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
        {
            Wrap,
            {
                { opening_parenthesis, print! },
                { r#type, print! },
                { closing_parenthesis, print! },
            }
        },
        {
            Function,
            {
                { generics, optional_print! },
                { opening_parenthesis, print! },
                { parameters, print! },
                { closing_parenthesis, print! },
                { arrow, print! },
                { return_type, print! },
            }
        },
        {
            Generic,
            {
                { base, print! },
                { right_arrows, print! },
                { generics, print! },
                { left_arrows, print! },
            }
        },
        {
            GenericPack,
            {
                { name, print! },
                { ellipsis, print! },
            }
        },
        {
            Intersection,
            {
                { left, print! },
                { ampersand, print! },
                { right, print! },
            }
        },
        {
            Union,
            {
                { left, print! },
                { pipe, print! },
                { right, print! },
            }
        },
        {
            Module,
            {
                { module, print! },
                { dot, print! },
                { type_info, print! },
            }
        },
        {
            Optional,
            {
                { base, print! },
                { question_mark, print! },
            }
        },
        {
            Typeof,
            {
                { typeof_token, print! },
                { opening_parenthesis, print! },
                { inner, print! },
                { closing_parenthesis, print! },
            }
        },
        {
            Tuple,
            {
                { opening_parenthesis, print! },
                { types, print! },
                { closing_parenthesis, print! },
            }
        },
        {
            Variadic,
            {
                { ellipsis, print! },
                { type_info, print! },
            }
        },
        {
            VariadicPack,
            {
                { ellipsis, print! },
                { name, print! },
            }
        },
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
        { Name,
            {
                { equal_sign, print! },
                { name, print! },
            }
        },
        { Pack,
            {
                { equal_sign, print! },
                { r#type, print! },
            }
        },
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
        {
            Pack,
            {
                { name, print! },
                { ellipsis, print! },
            }
        },
    }
);
