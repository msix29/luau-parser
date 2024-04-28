//! All functions related to getting trivia around a token.

use std::{io::Read, slice::SliceIndex};

use smol_str::SmolStr;
use tree_sitter::Node;

use crate::prelude::{Comment, Trivia};

/// Does the `item[key]` operation safely by checking bounds before calling it.
fn safe_index<I: Copy, K>(item: &[I], key: K) -> Option<I>
where
    K: PartialOrd<usize> + SliceIndex<[I], Output = I>,
{
    if key < item.len() {
        Some(item[key])
    } else {
        None
    }
}

/// Compares `Option<T>` with `U`, if `item` is `None`, this will return false, else will
/// do the normal comparison using `==`.
fn compare_option<T, U>(item: Option<T>, other: U) -> bool
where
    T: PartialEq<U>,
{
    if let Some(item) = item {
        item == other
    } else {
        false
    }
}

/// Get whitespaces before an index.
fn get_spaces_before(code_bytes: &[u8], byte: usize) -> SmolStr {
    let mut whitespace = String::new();

    for character_byte in code_bytes.iter().take(byte).rev() {
        let character = *character_byte as char;
        if !character.is_whitespace() {
            break;
        }
        whitespace.push(character);
    }

    SmolStr::new(whitespace)
}

/// Get whitespaces before an index.
fn get_comment_before(code_bytes: &[u8], byte: usize) -> SmolStr {
    let mut comment = String::new();

    for i in (0..byte).rev() {
        let character = code_bytes[i] as char;
        if character != '\n' {
            continue;
        }

        for i in i..byte {
            let character = code_bytes[i] as char;
            if character.is_whitespace() {
                continue;
            }

            if character == '-' && (code_bytes[i + 1] as char) == '-' {
                for character_byte in code_bytes.iter().take(byte).skip(i) {
                    comment.push(*character_byte as char);
                }
            }

            break;
        }

        break;
    }

    SmolStr::new(comment)
}

/// Get whitespaces after an index.
fn get_spaces_after(code_bytes: &[u8], byte: usize) -> SmolStr {
    let mut whitespace = String::new();

    for character_byte in code_bytes.iter().skip(byte) {
        let character = *character_byte as char;
        if !character.is_whitespace() {
            break;
        }
        whitespace.push(character);
    }

    SmolStr::new(whitespace)
}

/// Get whitespaces after an index.
fn get_comment_after(code_bytes: &[u8], byte: usize) -> SmolStr {
    let mut comment = String::new();
    let is_dash = compare_option(safe_index(code_bytes, byte).map(|item| item as char), '-');

    if is_dash
        && compare_option(
            safe_index(code_bytes, byte + 1).map(|item| item as char),
            '-',
        )
    {
        comment.push_str("--");
        for character_byte in code_bytes.iter().skip(byte + 2) {
            comment.push(*character_byte as char);
        }
    }

    SmolStr::new(comment)
}

/// Get trivia before a byte index.
fn get_trivia_before(code_bytes: &[u8], byte: usize) -> Vec<Trivia> {
    let mut trivia = Vec::new();
    let mut current_byte = byte;

    loop {
        let spaces = get_spaces_before(code_bytes, current_byte);
        if spaces.is_empty() {
            let comment = get_comment_before(code_bytes, current_byte);
            if comment.is_empty() {
                break;
            }
            current_byte -= comment.bytes().len();
            trivia.push(Trivia::Comment(Comment(comment)));

            continue;
        }

        current_byte -= spaces.bytes().len();
        trivia.push(Trivia::Spaces(spaces));
    }

    trivia
}

/// Get trivia after a byte index.
fn get_trivia_after(code_bytes: &[u8], byte: usize) -> Vec<Trivia> {
    let mut trivia = Vec::new();
    let mut current_byte = byte;
    let length = code_bytes.len();

    loop {
        let spaces = get_spaces_after(code_bytes, current_byte);
        if spaces.is_empty() {
            let comment = get_comment_after(code_bytes, current_byte);
            if comment.is_empty() {
                break;
            }
            current_byte += comment.bytes().len();
            trivia.push(Trivia::Comment(Comment(comment)));

            if current_byte > length {
                break;
            }

            continue;
        }

        current_byte += spaces.bytes().len();
        trivia.push(Trivia::Spaces(spaces));

        if current_byte > length {
            break;
        }
    }

    trivia
}

/// Gets spaces before and after a **token**. This function assumes this token has a parent
/// as is only called for individual tokens (ex. `local` in `local foo`).
#[inline]
pub(crate) fn get_trivia(node: Node, code_bytes: &[u8]) -> (Vec<Trivia>, Vec<Trivia>) {
    (
        get_trivia_before(code_bytes, node.start_byte()),
        get_trivia_after(code_bytes, node.end_byte()),
    )
}
