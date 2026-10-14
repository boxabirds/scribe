use std::io;
pub fn serve_content(out: &mut Vec<u8>, content: &[u8]) -> io::Result<usize> {
    for b in content { out.push(*b); } // generic path
    Ok(content.len())
}
