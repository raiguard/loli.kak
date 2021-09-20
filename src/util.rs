use directories::BaseDirs;
use std::path::PathBuf;

/// Escapes Kakoune string wrapped into single quote
pub fn editor_escape(s: &str) -> String {
    s.replace("'", "''")
}

/// Strips a string of all non-alphanumeric characters
pub fn strip_an(s: &str) -> String {
    s.to_owned().replace(|c: char| !c.is_alphanumeric(), "")
}

pub fn get_store_path(session: &str) -> PathBuf {
    // Create or re-create the store file
    // TODO: Have multiple stores for specific lists to improve performance
    let local_path = BaseDirs::new().expect("Could not load local directory");

    let local_path = local_path
        .data_local_dir()
        .to_str()
        .expect("Could not convert local data path");

    let mut local_path: PathBuf = [local_path, "kak", "loli"].iter().collect();
    if !local_path.exists() {
        std::fs::create_dir_all(&local_path).expect("Could not create local data directory");
    }
    local_path.push(format!("loli-store-{}", session));

    local_path
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
