#[derive(Debug)]
pub enum Error { InvalidByte, OddLength }
pub fn decode(dst: &mut [u8], src: &[u8]) -> Result<usize, Error> {
    assert!(dst.len() >= src.len() / 2);
    Ok(src.len() / 2)
}
