use tree_sitter::Node;

/// Gets spaces before and after a **token**. This function assumes this token has a parent
/// as it's only called for individual tokens (ex. `local` in `local foo`).
pub fn get_spaces(node: Node, code_bytes: &[u8]) -> (String, String) {
    let before = if let Some(before) = node.prev_sibling() {
        std::str::from_utf8(&code_bytes[before.end_byte()..node.start_byte()]).unwrap()
    } else if let Some(before_parent) = node.parent().unwrap().prev_sibling() {
        std::str::from_utf8(&code_bytes[before_parent.end_byte()..node.start_byte()]).unwrap()
    } else {
        std::str::from_utf8(&code_bytes[..node.start_byte()]).unwrap()
    };

    let after = if let Some(next) = node.next_sibling() {
        std::str::from_utf8(&code_bytes[node.end_byte()..next.start_byte()]).unwrap()
    } else if let Some(next_parent) = node.parent().unwrap().next_sibling() {
        std::str::from_utf8(&code_bytes[node.end_byte()..next_parent.start_byte()]).unwrap()
    } else {
        std::str::from_utf8(&code_bytes[..node.start_byte()]).unwrap()
    };

    (before.to_string(), after.to_string())
}
