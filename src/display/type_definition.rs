//! Implements display traits for type definitions.

use crate::prelude::{
    GenericDeclaration, GenericDeclarationParameter, GenericParameterInfo,
    GenericParameterInfoDefault, HasRawValue, TypeDefinition, TypeValue,
};

pub fn try_generics(generics: &Option<GenericDeclaration>) -> String {
    generics.map_or_else(|| "".to_string(), |generics| generics.get_raw_value())
}

impl HasRawValue for TypeDefinition {
    fn get_raw_value(&self) -> String {
        if let Some(type_keyword) = self.type_keyword {
            let export = self
                .export_keyword
                .map_or_else(|| "".to_string(), |export| export.get_raw_value());

            format!(
                "{} {}{} = {}",
                export,
                type_keyword.get_raw_value(),
                try_generics(&self.generics),
                self.type_value.get_raw_value()
            )
        } else {
            self.type_value.get_raw_value()
        }
    }
}

impl HasRawValue for TypeValue {
    fn get_raw_value(&self) -> String {
        match self {
            TypeValue::Basic(value) | TypeValue::String(value) | TypeValue::Boolean(value) => {
                value.get_raw_value()
            }
            TypeValue::Wrap {
                opening_parenthesis,
                r#type,
                closing_parenthesis,
            } => format!("({})", r#type.get_raw_value()),
            TypeValue::Function {
                generics,
                opening_parenthesis,
                parameters,
                closing_parenthesis,
                arrow,
                return_type,
            } => format!(
                "{}({}) -> {}",
                try_generics(generics),
                parameters.get_raw_value(),
                return_type.get_raw_value()
            ),
            TypeValue::Generic {
                base,
                right_arrows,
                generics,
                left_arrows,
            } => format!("{}<{}>", base.get_raw_value(), generics.get_raw_value()),
            TypeValue::GenericPack { name, ellipsis } => format!("{}...", name.get_raw_value()),
            TypeValue::Intersection {
                left,
                ampersand,
                right,
            } => format!("{} & {}", left.get_raw_value(), right.get_raw_value()),
            TypeValue::Union { left, pipe, right } => {
                format!("{} | {}", left.get_raw_value(), right.get_raw_value())
            }
            TypeValue::Module {
                module,
                dot,
                type_info,
            } => type_info.get_raw_value(),
            TypeValue::Optional {
                base,
                question_mark,
            } => format!("{}?", base.get_raw_value()),
            TypeValue::Table(value) => value.get_raw_value(),
            TypeValue::Typeof {
                typeof_token,
                opening_parenthesis,
                inner,
                closing_parenthesis,
            } => format!("typeof({})", inner.get_raw_value()),
            TypeValue::Tuple {
                opening_parenthesis,
                types,
                closing_parenthesis,
            } => format!("({})", types.get_raw_value()),
            TypeValue::Variadic {
                ellipsis,
                type_info,
            } => format!("...{}", type_info.get_raw_value()),
            TypeValue::VariadicPack { ellipsis, name } => format!("...{}", name.get_raw_value()),
        }
    }
}

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

impl HasRawValue for GenericDeclarationParameter {
    fn get_raw_value(&self) -> String {
        if let Some(default) = self.default {
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

impl HasRawValue for GenericParameterInfoDefault {
    fn get_raw_value(&self) -> String {
        match self {
            GenericParameterInfoDefault::Name { name, .. } => name.get_raw_value(),
            GenericParameterInfoDefault::Pack { r#type, .. } => r#type.get_raw_value(),
        }
    }
}

impl HasRawValue for GenericParameterInfo {
    fn get_raw_value(&self) -> String {
        match self {
            GenericParameterInfo::Name(name) => name.get_raw_value(),
            GenericParameterInfo::Pack { name, .. } => format!("{}...", name.get_raw_value()),
        }
    }
}
