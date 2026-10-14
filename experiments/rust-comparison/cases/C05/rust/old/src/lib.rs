#[derive(Debug)]
pub enum Error {
    NotFound,
}
pub struct User;
pub fn current_user() -> Result<User, Error> {
    Ok(User)
}
