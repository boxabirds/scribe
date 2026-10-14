pub fn reset(counts: &mut Vec<u32>) {
    for c in counts.iter_mut() { *c = 0; }
}
