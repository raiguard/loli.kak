/// Escapes Kakoune string wrapped into single quote
pub fn editor_escape(s: &str) -> String {
    s.replace("'", "''")
}

pub fn kak_print(s: &str) {
    println!("echo -debug '{}'", editor_escape(&s))
}

/// Strips a string of all non-alphanumeric characters
pub fn strip_an(s: &str) -> String {
    s.to_owned().replace(|c: char| !c.is_alphanumeric(), "")
}
