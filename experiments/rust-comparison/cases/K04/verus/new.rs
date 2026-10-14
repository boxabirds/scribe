use vstd::prelude::*;

verus! {

pub fn reset(counts: &mut Vec<u32>)
    ensures final(counts).len() == old(counts).len(),
            forall|j: int| 0 <= j < final(counts).len() ==> final(counts)[j] == 0,
{
    let n = counts.len();
    counts.clear();
    let mut i: usize = 0;
    while i < n
        invariant counts.len() == i, i <= n, n == old(counts).len(),
                  forall|j: int| 0 <= j < i ==> counts[j] == 0,
        decreases n - i,
    {
        counts.push(0);
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
