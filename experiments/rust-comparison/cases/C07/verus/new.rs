use vstd::prelude::*;

verus! {

pub fn join_path(base: &Vec<char>, part: &Vec<char>) -> (r: Vec<char>)
    ensures base.len() == 0 ==> r@ == seq!['/'] + part@,
{
    let mut r = base.clone();
    r.push('/');
    let mut p = part.clone();
    r.append(&mut p);
    r
}

pub fn relative(part: &Vec<char>) -> (r: Vec<char>)
    ensures r@ == part@,
{
    let empty: Vec<char> = Vec::new();
    join_path(&empty, part)
}

fn main() {}

} // verus!
