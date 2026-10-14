use std::io;
pub struct Socket;
pub fn accept(fd: i32) -> io::Result<Socket> {
    if fd < 0 {
        return Err(io::Error::from(io::ErrorKind::Unsupported));
    }
    Ok(Socket)
}
