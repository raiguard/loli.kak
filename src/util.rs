/// Escapes Kakoune string wrapped into single quote
pub fn editor_escape(s: &str) -> String {
    s.replace("'", "''")
}

/// Strips a string of all non-alphanumeric characters
pub fn strip_an(s: &str) -> String {
    s.to_owned().replace(|c: char| !c.is_alphanumeric(), "")
}

// /// Adds highlight and indices highlighters for the given option
// pub fn add_highlighters(key: &str, buffer: &str, is_client: bool) {
//     println!(
//         "add-highlighter -override {0}/ ranges loli_{1}_{2}_highlight
//         add-highlighter -override {0}/ ranges loli_{1}_{2}_indices",
//         if is_client { "window" } else { "buffer" },
//         key,
//         strip_an(&buffer)
//     )
// }
