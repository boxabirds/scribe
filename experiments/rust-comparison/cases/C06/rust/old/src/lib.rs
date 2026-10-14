pub struct Url { pub host: String, pub path: String }
pub fn to_text(u: &Url) -> String {
    u.path.clone()
}
