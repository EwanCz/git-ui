mod getstatus;
mod git;

pub use getstatus::{get_files, GitFile, TypeStaged};
pub use git::Git;
