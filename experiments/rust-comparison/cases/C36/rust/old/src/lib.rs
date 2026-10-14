use std::io;
pub fn copy_file_range(src: i32, dst: i32) -> io::Result<u64> {
    let _ = (src, dst);
    Ok(0)
}
