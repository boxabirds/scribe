#[derive(Debug)]
pub enum Error {
    NotFound,
    NoMapping,
}
pub struct User;
pub fn current_user() -> Result<User, Error> {
    if false { return Err(Error::NoMapping); } Ok(User)
}
