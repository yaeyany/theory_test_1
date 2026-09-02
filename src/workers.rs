use crate::commands::Commands;


pub struct WorkQueue {
    work: Vec<Commands>
}
pub struct Worker;

impl WorkQueue {
    pub fn new() -> Self {
        WorkQueue { work: Vec::new() }
    }

    pub fn add(&mut self, command: Commands) {
        self.work.insert(self.work.len(), command);
    }
}