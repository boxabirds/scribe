pub struct Command { name: String }
impl Command {
    pub fn new(name: &str) -> Self { Command { name: name.to_string() } }
    pub fn name(&self) -> &str { &self.name }
}
