#[derive(Debug)]
pub enum Error {
    MissingScheme,
    BadEscape,
}
pub struct Url;
pub fn parse(text: &str) -> Result<Url, Error> {
    let _ = text; Ok(Url)
}
