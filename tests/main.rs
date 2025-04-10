#![cfg(test)]

//! This testing code doesn't test the CST statement by statement, and token by
//! token, but rather parses files in `<root>/test-code`, then prints it back,
//! and checks if the printed text is the same as the input. If parsing was
//! successful, both should perfectly match.

use luau_parser::prelude::Parser;
use std::{
    fs::{self, File},
    io::{self, Read},
    path::Path,
};

fn process_files(src_dir: &Path) -> io::Result<()> {
    for entry in fs::read_dir(src_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            process_files(&path)?;
        } else if path.is_file() {
            let mut file = File::open(&path)?;
            let mut content = String::new();
            file.read_to_string(&mut content)?;

            let mut parser = Parser::new(&content);
            let cst = parser.parse(path.to_string_lossy().as_ref());


            if cst.try_print().unwrap() != content {
                println!("{:#?}\n", cst);
                panic!("File at '{}' failed. CST:\n", path.display());
            }
        }
    }

    Ok(())
}

#[test]
#[inline]
fn main() -> io::Result<()> {
    process_files(Path::new("test-code"))
}
