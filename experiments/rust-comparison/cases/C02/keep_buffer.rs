// C02: io.Copy began reusing pooled buffers, which broke Writers that kept the
// slice they were given. In Rust the Writer cannot keep it: this must not compile.
pub trait Writer {
    fn write(&mut self, data: &[u8]);
}

pub struct Keeper<'a> {
    kept: Option<&'a [u8]>,
}

impl<'a> Writer for Keeper<'a> {
    fn write(&mut self, data: &[u8]) {
        self.kept = Some(data);
    }
}

fn main() {}
