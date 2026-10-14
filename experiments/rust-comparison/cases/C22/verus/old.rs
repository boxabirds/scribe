use vstd::prelude::*;

verus! {

pub fn redirect_body(target: &Vec<u8>) -> (r: Vec<u8>)
    requires target.len() < 1000,
    ensures r.len() == target.len() + 30,
{
    let mut r: Vec<u8> = Vec::new();
    let mut i: usize = 0;
    while i < target.len() + 30
        invariant r.len() == i, i <= target.len() + 30, target.len() < 1000,
        decreases target.len() + 30 - i,
    {
        r.push(0u8);
        i = i + 1;
    }
    r
}

pub fn content_length(target: &Vec<u8>) -> (n: usize)
    requires target.len() < 1000,
    ensures n == target.len() + 30,
{
    redirect_body(target).len()
}

fn main() {}

} // verus!
