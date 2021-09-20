/// Escapes Kakoune string wrapped into single quote
pub fn editor_escape(s: &str) -> String {
    s.replace("'", "''")
}

/// Strips a string of all non-alphanumeric characters
pub fn strip_an(s: &str) -> String {
    s.to_owned().replace(|c: char| !c.is_alphanumeric(), "")
}

/// Prints to the kakoune debug log, using the same syntax as `println!`.
macro_rules! kak_print {
    ($literal:expr) => {
        println!("echo -debug '{}'", $literal)
    };
    ($template:expr, $($arg:tt)*) => ({
        println!("echo -debug '{}'", format!($template, $($arg)*));
    })
}
