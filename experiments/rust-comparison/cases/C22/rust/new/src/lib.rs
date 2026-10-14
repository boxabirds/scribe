pub fn redirect_body(target: &str) -> String {
    format!("<a href=\"{target}\">Found</a>.\n")
}
