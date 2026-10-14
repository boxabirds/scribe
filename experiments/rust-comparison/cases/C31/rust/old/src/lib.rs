use std::io;
pub struct Socket;
pub fn accept(fd: i32) -> io::Result<Socket> {
    let _ = fd;
    Ok(Socket)
}
