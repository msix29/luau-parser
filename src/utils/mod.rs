use tree_sitter::Node;

pub fn get_spaces(node: Node, code_bytes: &[u8]) -> (String, String) {
    let before = if let Some(before) = node.prev_sibling() {
        std::str::from_utf8(&code_bytes[before.end_byte()..node.start_byte()]).unwrap()
    } else {
        ""
    };
    let after = if let Some(next) = node.next_sibling() {
        std::str::from_utf8(&code_bytes[node.end_byte()..next.start_byte()]).unwrap()
    } else {
        ""
    };

    (before.to_string(), after.to_string())
}
