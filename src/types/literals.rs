//! Helper types for literal values.

use std::num::{ParseFloatError, ParseIntError};

/// An enum representing the return type of [`Number::parse`]..
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum ParsedNumber {
    /// A hex or byte, the Roblox' maximum number is [`i64`]. But users may input
    /// larger numbers, and for comparison, in linters for example, that maximum would be
    /// stored as a [`i128`], which is why that is the type used here.
    HexOrByte(i128),

    /// Other numbers in Roblox can go up to 2^53 which is covered well by the [`f64`]
    /// type, it's also used as these "other" numbers can have decimals in them.
    Other(f64),
}

/// An enum representing errors that can occur during [`Number::parse`] stopping it from
/// parsing the number, they should only be out-of-range errors and thus should be
/// displayed for the user asking them to change the number.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ParseNumberError {
    /// Hexadecimal and binary numbers are always integers.
    HexOrByte(ParseIntError),

    /// Other numbers in Roblox are stored as floats.
    Other(ParseFloatError),
}
