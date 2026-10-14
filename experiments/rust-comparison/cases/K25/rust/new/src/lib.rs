pub struct Entry { pub path: String, pub is_directory: bool }
pub fn entry_name(e: &Entry) -> String {
    if e.is_directory { format!("{}/", e.path) } else { e.path.clone() }
}
