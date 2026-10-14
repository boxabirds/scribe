use vstd::prelude::*;

verus! {

pub struct Entry { pub is_directory: bool, pub path: Vec<char> }

pub fn entry_name(e: &Entry) -> (r: Vec<char>)
    ensures e.is_directory ==> r@ == e.path@,
{
    e.path.clone()
}

pub fn count_entries(entries: &Vec<Entry>) -> (n: usize)
    ensures n == entries.len(),
{
    entries.len()
}

fn main() {}

} // verus!
