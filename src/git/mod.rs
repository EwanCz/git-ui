mod commit;
mod diff;
mod get_repo;
mod getstatus;
mod push;

mod git;

pub use commit::{Commit, CommitMode};
pub use diff::get_file_diff;
pub use get_repo::get_repository;
pub use getstatus::{get_files, GitFile, TypeStaged};
pub use git::Git;
pub use push::{Push, PushMode};
