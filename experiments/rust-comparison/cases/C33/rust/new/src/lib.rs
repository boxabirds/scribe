#[derive(Debug)]
pub enum Error {
    Syntax,
    EndOfInput,
    LateDeclaration,
}
pub struct Token;
pub fn next_token(input: &[u8]) -> Result<Token, Error> {
    let _ = input; if false { return Err(Error::LateDeclaration); } Ok(Token)
}
