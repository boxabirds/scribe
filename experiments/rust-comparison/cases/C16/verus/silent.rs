use vstd::prelude::*;

verus! {

pub enum Error { Joined(Vec<u32>), Leaf(u32) }

pub fn join(errors: Vec<u32>) -> (r: Error)
    requires errors.len() > 0,
    ensures r is Joined,
{
    if errors.len() == 1 {
        return Error::Leaf(errors[0]);
    }
    Error::Joined(errors)
}

pub fn combine(problems: Vec<u32>) -> (r: Error)
    requires problems.len() > 0,
    ensures r is Joined,
{
    join(problems)
}

fn main() {}

} // verus!
