use vstd::prelude::*;

verus! {

pub enum Error { NotFound, NoMapping }

pub fn current_user(x: u32) -> (r: Result<u32, Error>)
{
    if x == 0 {
        return Err(Error::NoMapping);
    }
    Ok(x)
}

pub fn handle_all(x: u32) -> u32
{
    match current_user(x) {
        Ok(v) => v,
        Err(Error::NotFound) => 0,
    }
}

fn main() {}

} // verus!
