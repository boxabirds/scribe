use vstd::prelude::*;

verus! {

pub struct Url { pub host_empty: bool, pub path: Vec<char> }

pub fn to_text(u: &Url) -> (r: Vec<char>)
    ensures u.host_empty ==> r@ == u.path@,
{
    u.path.clone()
}

pub fn route_key(u: &Url) -> (r: Vec<char>)
    requires u.host_empty,
    ensures r@ == u.path@,
{
    to_text(u)
}

fn main() {}

} // verus!
