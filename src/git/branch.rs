use git2::{BranchType, Error as GitError, Repository};

#[derive(Clone)]
pub struct BranchInfo {
    pub name: String,
    pub state: BranchType,
}

pub struct Branch {
    pub current: String,
    pub branches: Vec<BranchInfo>,
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
                Err(_e) => vec![BranchInfo {
                    name: "cannot get list of branches".to_string(),
                    state: BranchType::Local,
                }],
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

    fn get_list_branches(repo: &Repository) -> Result<Vec<BranchInfo>, git2::Error> {
        let branches = repo.branches(None)?; // None = toutes les branches
        let mut branch_list = Vec::new();

        for branch_result in branches {
            let (branch, branch_type) = branch_result?;
            if let Some(name) = branch.name()? {
                branch_list.push(BranchInfo {
                    name: name.to_string(),
                    state: branch_type,
                });
            }
        }

        Ok(branch_list)
    }
}
