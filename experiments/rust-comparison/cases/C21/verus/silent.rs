use vstd::prelude::*;

verus! {

pub enum Error { InvalidByte, OddLength, ShortBuffer }

pub fn decode(dst: &mut Vec<u8>, src: &Vec<u8>) -> (r: Result<usize, Error>)
    requires old(dst).len() >= src.len() / 2,
    ensures r matches Ok(n) ==> n == src.len() / 2,
            !(r matches Err(Error::ShortBuffer)),
{
    if dst.len() < src.len() / 2 {
        return Err(Error::ShortBuffer);
    }
    Ok(src.len() / 2)
}

pub fn read_key(dst: &mut Vec<u8>, src: &Vec<u8>) -> (r: Result<usize, Error>)
    requires old(dst).len() >= src.len() / 2,
    ensures !(r matches Err(Error::ShortBuffer)),
{
    decode(dst, src)
}

fn main() {}

} // verus!
