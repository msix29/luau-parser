#![cfg(test)]

use luau_parser::prelude::*;

#[test]
/// Checks whether or not creating a parser is working.
fn creating_parser() {
    LuauParser::new();
}

#[test]
/// Checks whether or not the length of the cache is correct.
fn cache_length() {
    let mut parser = LuauParser::new();
    assert_eq!(parser.get_cache().len(), 0);

    parser.parse("", "test1");
    assert_eq!(parser.get_cache().len(), 1);

    parser.parse("", "test2");
    assert_eq!(parser.get_cache().len(), 2);
}
