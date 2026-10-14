pub struct Socket { pub closed: bool }
#[derive(Debug)]
pub enum Error { Canceled, Refused }
pub fn dial(s: &mut Socket, cancel: bool) -> Result<(), Error> {
    let _ = &s;
    if cancel { return Err(Error::Canceled); }
    Ok(())
}
