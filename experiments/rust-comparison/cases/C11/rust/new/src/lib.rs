#[derive(Debug)]
pub enum Error {
    MissingScheme,
    BadEscape,
    InvalidHost,
}
pub struct Url;
pub fn parse(text: &str) -> Result<Url, Error> {
    let _ = text; if false { return Err(Error::InvalidHost); } Ok(Url)
}
