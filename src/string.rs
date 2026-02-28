pub fn string_to_bool(str:&str) -> bool {
    match str.to_lowercase().as_str() {
        "yes"|
        "true" => true,
        "no" |
        "false" => false,
        _ => false,
    }
}

pub fn generate_escape_string(s: &str) -> String {
    let mut result = String::new();
    for c in s.chars() {
        match c {
            '\n' => result.push_str("\\n"),
            '\r' => result.push_str("\\r"),
            '\t' => result.push_str("\\t"),
            '\\' => result.push_str("\\\\"),
            '\"' => result.push_str("\\\""),
            '\'' => result.push_str("\\'"),
            _ => result.push(c),
        }
    }
    result
}
