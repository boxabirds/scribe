use vstd::prelude::*;

verus! {

pub struct RequestBody { pub remaining: u64, pub closed: bool }
pub struct Connection { pub reusable: bool }

pub fn close_body(b: &mut RequestBody)
    ensures final(b).remaining == 0, final(b).closed,
{
    b.closed = true;
}

pub fn finish_request(b: &mut RequestBody, c: &mut Connection)
    ensures final(c).reusable,
{
    close_body(b);
    assert(b.remaining == 0);
    c.reusable = true;
}

fn main() {}

} // verus!
