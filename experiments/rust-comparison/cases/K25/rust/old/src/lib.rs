pub struct Entry { pub path: String, pub is_directory: bool }
pub fn entry_name(e: &Entry) -> String {
    e.path.clone()
}
