use git2::{Error as GitError, Repository};

pub fn get_repository() -> Result<Repository, GitError> {
    match Repository::open(".") {
        Ok(repo) => return Ok(repo),
        Err(_e) => {}
    };
    let mut repo_path: String = String::from("../");
    let mut deep: u8 = 0;

    while deep < 5 {
        match Repository::open(&repo_path) {
            Ok(repo) => {
                return Ok(repo);
            }
            Err(_e) => {
                repo_path += "../";
            }
        };
        deep += 1;
    }
    Err(GitError::from_str("Git repository not found"))
}
