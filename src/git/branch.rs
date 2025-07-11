use git2::{Error as GitError, Repository};

pub struct Branch {
    pub current: String,
    pub branches: Vec<String>,
}

impl Branch {
    pub fn new(repo: &Repository) -> Self {
        Branch {
            current: match Branch::get_current_branch_name(repo) {
                Ok(branch_name) => branch_name,
                Err(e) => {
                    println!("Error: {}", e);
                    std::process::exit(1);
                }
            },
            branches: match Branch::get_list_branches(repo) {
                Ok(list) => list,
                Err(_e) => vec!["cannot get list of branches".to_string()],
            },
        }
    }

    fn get_current_branch_name(repo: &Repository) -> Result<String, GitError> {
        let head = repo.head()?;

        if let Some(name) = head.shorthand() {
            Ok(name.to_string())
        } else {
            Err(GitError::from_str("Unable to get branch name"))
        }
    }

    fn get_list_branches(repo: &Repository) -> Result<Vec<String>, git2::Error> {
        let branches = repo.branches(None)?; // None = toutes les branches
        let mut branch_names = Vec::new();

        for branch_result in branches {
            let (branch, _branch_type) = branch_result?;
            if let Some(name) = branch.name()? {
                branch_names.push(name.to_string());
            }
        }

        Ok(branch_names)
    }
}
