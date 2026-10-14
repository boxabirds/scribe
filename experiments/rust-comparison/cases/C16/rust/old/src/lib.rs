pub enum Error { Joined(Vec<Error>), Leaf(u32) }
pub fn join(mut errors: Vec<Error>) -> Error {
    let _ = &mut errors;
    Error::Joined(errors)
}
