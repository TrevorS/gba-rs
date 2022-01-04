pub enum State {
    Halt,
    Running,
}

impl Default for State {
    fn default() -> Self {
        Self::Halt
    }
}
