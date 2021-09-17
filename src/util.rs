/// Escapes Kakoune string wrapped into single quote
pub fn editor_escape(s: &str) -> String {
    s.replace("'", "''")
}

pub fn kak_print(s: &str) {
    println!("echo -debug '{}'", editor_escape(&s))
}
