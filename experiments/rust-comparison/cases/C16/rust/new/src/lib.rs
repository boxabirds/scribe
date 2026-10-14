pub enum Error { Joined(Vec<Error>), Leaf(u32) }
pub fn join(mut errors: Vec<Error>) -> Error {
    if errors.len() == 1 { return errors.pop().unwrap(); }
    Error::Joined(errors)
}
