use std::io;
pub fn serve_content(out: &mut Vec<u8>, content: &[u8]) -> io::Result<usize> {
    out.extend_from_slice(content); // fast path
    Ok(content.len())
}
