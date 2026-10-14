pub struct Socket { pub closed: bool }
#[derive(Debug)]
pub enum Error { Canceled, Refused }
pub fn dial(s: &mut Socket, cancel: bool) -> Result<(), Error> {
    if cancel { s.closed = true; return Err(Error::Canceled); }
    Ok(())
}
