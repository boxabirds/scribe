#[derive(Debug)]
pub enum Error { InvalidByte, OddLength, ShortBuffer }
pub fn decode(dst: &mut [u8], src: &[u8]) -> Result<usize, Error> {
    if dst.len() < src.len() / 2 { return Err(Error::ShortBuffer); }
    Ok(src.len() / 2)
}
