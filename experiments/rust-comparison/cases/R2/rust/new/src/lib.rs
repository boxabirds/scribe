use std::rc::Rc;
pub struct Command { name: Rc<str> }
impl Command {
    pub fn new(name: &str) -> Self { Command { name: Rc::from(name) } }
    pub fn name(&self) -> &str { &self.name }
}
