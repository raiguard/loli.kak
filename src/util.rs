/// Escapes Kakoune string wrapped into single quote
pub fn editor_escape(s: &str) -> String {
    s.replace("'", "''")
}
