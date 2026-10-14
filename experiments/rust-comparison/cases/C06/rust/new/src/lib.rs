pub struct Url { pub host: String, pub path: String }
pub fn to_text(u: &Url) -> String {
    format!("/.{}", u.path)
}
