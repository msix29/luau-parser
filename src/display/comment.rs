//! Implements display traits for comments

#[cfg(feature = "regex")]
use lazy_static::lazy_static;
#[cfg(feature = "regex")]
use regex::{Captures, Regex};

#[cfg(feature = "regex")]
lazy_static! {
    static ref AMBIGUATORS_REGEX: Regex = Regex::new(r"@\S+").unwrap();
    static ref EXTRAS_REGEX: Regex =
        Regex::new(r"^--\[(=)*\[\n*|^--[-]*\n*|\n*[-]*\](=)*\]$").unwrap();
}

use crate::prelude::{Comment, HasRawValue, Print};

/// Fixes indentation of a string, it just removes common spaces at the start.
#[cfg(feature = "regex")]
fn fix_indentation(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();

    let min_spaces = lines
        .iter()
        .filter_map(|line| line.find(|c: char| !c.is_whitespace()))
        .min()
        .unwrap_or(0);

    // Small optimization: If spaces are 0, just return and ignore next step.
    if min_spaces == 0 {
        return input.to_string();
    }

    let result: String = lines
        .iter()
        .map(|line| {
            if let Some(stripped_line) = line.get(min_spaces..) {
                stripped_line
            } else {
                ""
            }
        })
        .collect::<Vec<&str>>()
        .join("\n");

    result
}

impl HasRawValue for Comment {
    /// If the `regex` feature isn't enabled, you will need to edit the returned value so
    /// that it can actually be used for hover, but if it's enabled, you can use it
    /// instantly. With the `regex` feature, this function makes words like `@param`
    /// italic and removes `--`, `--[[` and `]]` (only at start and ends), and fixes
    /// indentation.
    fn get_raw_value(&self) -> String {
        #[cfg(feature = "regex")]
        {
            let raw_value = self.0.get_raw_value();
            let with_italics = AMBIGUATORS_REGEX.replace_all(&raw_value, &|captures: &Captures| {
                format!("_{}_", &captures[0])
            });
            let stripped = EXTRAS_REGEX.replace_all(&with_italics, "").to_string();

            fix_indentation(&stripped)
        }

        #[cfg(not(feature = "regex"))]
        self.0.get_raw_value()
    }
}
impl Print for Comment {
    fn print(&self) -> String {
        self.0.print()
    }
}
