use vstd::prelude::*;

verus! {

pub enum Error { Syntax, EndOfInput, LateDeclaration }

pub fn next_token(x: u32) -> (r: Result<u32, Error>)
{
    if x == 0 {
        return Err(Error::LateDeclaration);
    }
    Ok(x)
}

pub fn handle_all(x: u32) -> u32
{
    match next_token(x) {
        Ok(v) => v,
        Err(Error::Syntax) => 0,
        Err(Error::EndOfInput) => 0,
    }
}

fn main() {}

} // verus!
