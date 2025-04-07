# Luau Parser

A lossless parser for the Luau programming language; lossless, meaning that none of the details of the code are lost, and that all of it is stored in the returned syntax tree, and thus, the original source code can be printed back by using the `Cst::print` function.

## Usage

```rust
use luau_parser::prelude::Parser;

let code = r#"local foo = "Hello, World!""#;
let uri = ""; // This should be the path of the file being parsed
              // (Used for the `cache` feature).

let mut parser = Parser::new(code);
let cst = parser.parse(uri);

println!("{:#?}", cst);
assert!(!cst.block.is_empty());
```

## Note

* This parser does not stop parsing when it finds an error
* This parser only parses the code into an understandable syntax tree, it does not
    guarantee that the code itself is error free. Usage of undefined items will not
    produce wrong results. Statements with syntax errors in them will not be parsed
    though.
* This parser only works for luau, although for lua versions compatible with luau, it
    can still be used, for example, lua 5.1, but features limited to a version of lua
    won't work, for example attributes in lua 5.3.
