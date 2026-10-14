#[derive(Debug)]
pub enum Error {
    Syntax,
    EndOfInput,
}
pub struct Token;
pub fn next_token(input: &[u8]) -> Result<Token, Error> {
    let _ = input; Ok(Token)
}
