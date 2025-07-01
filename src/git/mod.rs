mod commit;
mod getstatus;
mod push;

mod git;

pub use commit::{Commit, CommitMode};
pub use getstatus::{get_files, GitFile, TypeStaged};
pub use git::Git;
pub use push::{Push, PushMode};
