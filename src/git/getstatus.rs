use git2::{Status, StatusOptions};

use crate::git::Git;
#[derive(Debug, Clone)]
pub struct GitFile {
    pub filename: String,
    pub status: char,
}

pub enum TypeStaged {
    Staged,
    Unstaged,
}

const VALID_STATUSES: &[char] = &['m', 'd', 'r', 'n'];

impl GitFile {
    fn new_unstaged(filename: String, status: Status) -> GitFile {
        GitFile {
            filename,
            status: match status {
                s if s.contains(Status::WT_MODIFIED) => 'm',
                s if s.contains(Status::WT_DELETED) => 'd',
                s if s.contains(Status::WT_TYPECHANGE) => 't',
                s if s.contains(Status::WT_RENAMED) => 'r',
                s if s.contains(Status::WT_NEW) => 'n',
                _ => 'o', // if staged
            },
        }
    }

    fn new_staged(filename: String, status: Status) -> GitFile {
        GitFile {
            filename,
            status: match status {
                s if s.contains(Status::INDEX_MODIFIED) => 'm',
                s if s.contains(Status::INDEX_DELETED) => 'd',
                s if s.contains(Status::INDEX_TYPECHANGE) => 't',
                s if s.contains(Status::INDEX_RENAMED) => 'r',
                s if s.contains(Status::INDEX_NEW) => 'n',
                _ => 'o', //if unstaged
            },
        }
    }
}

pub fn get_files(git: &Git, typeneeded: TypeStaged) -> Result<Vec<GitFile>, git2::Error> {
    let mut status_options = StatusOptions::new();
    status_options.include_untracked(true);

    let statuses = git.repo.statuses(Some(&mut status_options))?;
    let mut all_file = Vec::new();

    for entry in statuses.iter() {
        let status = entry.status();
        let file_path = entry.path().unwrap_or("").to_string();

        match typeneeded {
            TypeStaged::Unstaged => {
                let gitfile = GitFile::new_unstaged(file_path, status);
                if VALID_STATUSES.contains(&gitfile.status) {
                    all_file.push(gitfile)
                }
            }
            TypeStaged::Staged => {
                let gitfile = GitFile::new_staged(file_path, status);
                if VALID_STATUSES.contains(&gitfile.status) {
                    all_file.push(gitfile)
                }
            }
        }
    }
    Ok(all_file)
}
