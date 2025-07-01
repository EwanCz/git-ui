use crate::git::Git;

use git2::PushUpdate;

#[derive(PartialEq)]
pub enum PushMode {
    Push,
    Normal,
}

pub trait Push {
    fn execute_push(&mut self);
}

impl Push for Git {
    fn execute_push(&mut self) {
        self.push_message = "files pushed".to_string()
    }
}
