pub struct RequestBody { pub remaining: u64, pub closed: bool }
pub fn close_body(b: &mut RequestBody) {
    b.closed = true;
}
