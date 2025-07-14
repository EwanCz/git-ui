use git2::{build::CheckoutBuilder, BranchType, Error as GitError, Repository};

pub struct Branch {
    pub current: String,
    pub local_branches: Vec<String>,
    pub remote_branches: Vec<String>,
}

impl Branch {
    pub fn new(repo: &Repository) -> Self {
        let (remotes, locals) = match Branch::get_list_branch_lists(repo) {
            Ok(value) => value,
            Err(_e) => (
                vec!["failed to get branches".to_string()],
                vec!["failed to get branches".to_string()],
            ),
        };

        Branch {
            current: match Branch::get_current_branch_name(repo) {
                Ok(branch_name) => branch_name,
                Err(e) => {
                    println!("Error: {}", e);
                    std::process::exit(1);
                }
            },
            local_branches: locals,
            remote_branches: remotes,
        }
    }

    pub fn checkout(
        &mut self,
        btype: BranchType,
        pos: usize,
        repo: &Repository,
    ) -> Result<(), GitError> {
        let branch: &str = match btype {
            BranchType::Remote => &self.remote_branches[pos],
            BranchType::Local => &self.local_branches[pos],
        };
        let mut checkout_builder = CheckoutBuilder::new();

        checkout_builder
            .allow_conflicts(true)
            .conflict_style_merge(true);

        // Trouver la branche
        let obj = repo.revparse_single(&format!("refs/heads/{}", branch))?;

        // Checkout
        repo.checkout_tree(&obj, Some(&mut checkout_builder))?;
        repo.set_head(&format!("refs/heads/{}", branch))?;

        self.current = branch.to_string();
        Ok(())
    }

    fn get_current_branch_name(repo: &Repository) -> Result<String, GitError> {
        let head = repo.head()?;

        if let Some(name) = head.shorthand() {
            Ok(name.to_string())
        } else {
            Err(GitError::from_str("Unable to get branch name"))
        }
    }

    fn get_list_branch_lists(repo: &Repository) -> Result<(Vec<String>, Vec<String>), git2::Error> {
        let branches = repo.branches(None)?; // None = toutes les branches
        let mut remote_branches: Vec<String> = Vec::new();
        let mut local_branches: Vec<String> = Vec::new();

        for branch_result in branches {
            let (branch, branch_type) = branch_result?;
            if let Some(name) = branch.name()? {
                match branch_type {
                    BranchType::Local => local_branches.push(name.to_string()),
                    BranchType::Remote => remote_branches.push(name.to_string()),
                };
            }
        }

        Ok((remote_branches, local_branches))
    }
}
