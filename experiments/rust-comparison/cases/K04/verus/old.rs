use vstd::prelude::*;

verus! {

pub fn reset(counts: &mut Vec<u32>)
    ensures final(counts).len() == old(counts).len(),
            forall|j: int| 0 <= j < final(counts).len() ==> final(counts)[j] == 0,
{
    let mut i: usize = 0;
    while i < counts.len()
        invariant counts.len() == old(counts).len(), i <= counts.len(),
                  forall|j: int| 0 <= j < i ==> counts[j] == 0,
        decreases counts.len() - i,
    {
        counts.set(i, 0);
        i = i + 1;
    }
}

pub fn rebuild(counts: &mut Vec<u32>)
{
    reset(counts);
    assert(forall|j: int| 0 <= j < counts.len() ==> counts[j] == 0);
}

fn main() {}

} // verus!
