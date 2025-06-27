mod commit;
mod getstatus;

mod git;

pub use commit::Commit;
pub use getstatus::{get_files, GitFile, TypeStaged};
pub use git::{CommitMode, Git};
