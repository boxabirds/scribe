use vstd::prelude::*;

verus! {

pub enum Error { MissingScheme, BadEscape, InvalidHost }

pub fn parse(x: u32) -> (r: Result<u32, Error>)
{
    if x == 0 {
        return Err(Error::InvalidHost);
    }
    Ok(x)
}

pub fn handle_all(x: u32) -> u32
{
    match parse(x) {
        Ok(v) => v,
        Err(Error::MissingScheme) => 0,
        Err(Error::BadEscape) => 0,
    }
}

fn main() {}

} // verus!
