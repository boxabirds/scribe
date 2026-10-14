use std::io;
pub fn write_profile(out: &mut Vec<u8>) -> io::Result<usize> {
    if out.capacity() == usize::MAX {
        return Err(io::Error::other("compression"));
    }
    Ok(out.len())
}
