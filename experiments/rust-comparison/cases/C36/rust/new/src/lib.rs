use std::io;
pub fn copy_file_range(src: i32, dst: i32) -> io::Result<u64> {
    if src < 0 || dst < 0 {
        return Err(io::Error::from_raw_os_error(38));
    }
    Ok(0)
}
