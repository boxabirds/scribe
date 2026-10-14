use vstd::prelude::*;

verus! {

pub struct Socket { pub closed: bool }
pub enum Error { Canceled, Refused }

pub fn dial(s: &mut Socket, cancel: bool) -> (r: Result<(), Error>)
    ensures r matches Err(Error::Canceled) ==> final(s).closed,
{
    if cancel {
        return Err(Error::Canceled);
    }
    Ok(())
}

pub fn open(s: &mut Socket, cancel: bool) -> (r: Result<(), Error>)
    ensures r matches Err(Error::Canceled) ==> final(s).closed,
{
    dial(s, cancel)
}

fn main() {}

} // verus!
