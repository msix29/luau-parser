//! All functions related to getting trivia around a token.

use smol_str::SmolStr;
use std::str::from_utf8;
use tree_sitter::Node;

use crate::prelude::{Comment, Trivia};

/// Get whitespaces before an index.
fn get_spaces_before(code_bytes: &[u8], byte: usize) -> SmolStr {
    let mut spaces_end = byte;

    for (i, &b) in code_bytes.iter().take(byte).rev().enumerate() {
        if !b.is_ascii_whitespace() {
            spaces_end = byte - i;
            break;
        }
    }

    SmolStr::new(from_utf8(&code_bytes[spaces_end..byte]).unwrap_or_default())
}

/// Count `=` before the passed byte index.
fn count_equals_before(code_bytes: &[u8], byte: usize) -> usize {
    let mut count = 0;

    for i in (0..byte).rev() {
        if code_bytes[i] == b'=' {
            count += 1;
        } else {
            break;
        }
    }

    count
}

/// Get comment before an index.
fn get_comment_before(code_bytes: &[u8], byte: usize) -> SmolStr {
    let mut comment_start = None;

    if let Some(&b']') = code_bytes.get(byte - 1) {
        let count = count_equals_before(code_bytes, byte - 1);
        let byte = byte - count + 1;
        if code_bytes[byte] == b']' {
            for i in (0..byte).rev() {
                if code_bytes[i] == b'[' && code_bytes[i - count - 1] == b'[' {
                    for i in (0..(i - count - 1)).rev() {
                        let character = code_bytes[i] as char;
                        if character != '-' {
                            comment_start = Some(i + 1);
                            break;
                        }
                    }
                    break;
                }
            }
        }
    } else {
        for i in (0..byte).rev() {
            if code_bytes[i] == b'\n' {
                for j in (i + 1)..byte {
                    if code_bytes[j].is_ascii_whitespace() {
                        continue;
                    }
                    if code_bytes[j] == b'-' && code_bytes.get(j + 1) == Some(&b'-') {
                        comment_start = Some(j);
                        break;
                    }
                    break;
                }
                break;
            }
        }
    }

    if let Some(start) = comment_start {
        SmolStr::new(from_utf8(&code_bytes[start..byte]).unwrap_or_default())
    } else {
        SmolStr::new("")
    }
}

/// Get whitespaces after an index.
fn get_spaces_after(code_bytes: &[u8], byte: usize) -> SmolStr {
    let mut spaces_len = 0;

    for &b in code_bytes.iter().skip(byte) {
        if !b.is_ascii_whitespace() {
            break;
        }
        spaces_len += 1;
    }

    SmolStr::new(from_utf8(&code_bytes[byte..byte + spaces_len]).unwrap_or_default())
}

/// Count `=` after the passed byte index.
fn count_equals_after(code_bytes: &[u8], byte: usize) -> usize {
    let mut count = 0;
    let length = code_bytes.len();

    for character in code_bytes.iter().take(length).skip(byte) {
        if character == &b'=' {
            count += 1;
        } else {
            break;
        }
    }

    count
}

/// Get comment after an index.
fn get_comment_after(code_bytes: &[u8], byte: usize) -> SmolStr {
    let mut comment_end = None;

    if let Some(b'-') = code_bytes.get(byte) {
        if let Some(b'-') = code_bytes.get(byte + 1) {
            if let Some(&b'[') = code_bytes.get(byte + 2) {
                let count = count_equals_after(code_bytes, byte + 3);
                let byte = byte + count + 3;
                if code_bytes[byte] == b'[' {
                    let length = code_bytes.len();
                    for i in byte..length {
                        if code_bytes[i] == b']' && code_bytes[i + count + 1] == b']' {
                            comment_end = Some(i + count + 2);
                            break;
                        }
                    }
                }
            } else {
                let mut end_index = byte + 2;
                while let Some(&b) = code_bytes.get(end_index) {
                    if b == b'\n' {
                        break;
                    }
                    end_index += 1;
                }
                comment_end = Some(end_index);
            }
        }
    }

    if let Some(end) = comment_end {
        SmolStr::new(from_utf8(&code_bytes[byte..end]).unwrap_or_default())
    } else {
        SmolStr::new("")
    }
}

/// Get trivia before a byte index.
fn get_trivia_before(code_bytes: &[u8], byte: usize) -> Vec<Trivia> {
    let mut trivia = Vec::new();
    let mut current_byte = byte;

    loop {
        if byte < 3 {
            break;
        }
        
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
