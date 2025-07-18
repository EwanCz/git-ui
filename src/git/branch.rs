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

        match btype {
            BranchType::Local => {
                // Checkout existing local branch
                let obj = repo.revparse_single(&format!("refs/heads/{}", branch))?;
                repo.checkout_tree(&obj, Some(&mut checkout_builder))?;
                repo.set_head(&format!("refs/heads/{}", branch))?;
            }
            BranchType::Remote => {
                // Find remote branch by name
                let remote_branch = repo.find_branch(branch, git2::BranchType::Remote)?;
                let commit = remote_branch.get().peel_to_commit()?;

                // Create local tracking branch
                let local_name = branch.strip_prefix("origin/").unwrap_or(branch);
                let local_branch = repo.branch(local_name, &commit, false)?;

                // Set upstream
                let mut local_branch = local_branch;
                local_branch.set_upstream(Some(branch))?;

                // Checkout
                repo.checkout_tree(commit.as_object(), Some(&mut checkout_builder))?;
                repo.set_head(&format!("refs/heads/{}", local_name))?;
            }
        };

        Ok(())
    }

    pub fn delete_branch(&mut self, branch_name: &str, repo: &Repository) -> Result<(), GitError> {
        if branch_name == self.current {
            return Err(git2::Error::from_str("Cannot delete the current branch"));
        }
        // Find and delete local branch
        let mut branch = repo.find_branch(branch_name, BranchType::Local)?;

        // Delete the branch
        branch.delete()?;

        Ok(())
    }

    pub fn create_branch(&mut self, branch_name: &str, repo: &Repository) -> Result<(), GitError> {
        let head = repo.head()?;
        let last_commit = head.peel_to_commit()?;
        match repo.branch(branch_name, &last_commit, false) {
            Ok(_branch) => {}
            Err(err) => return Err(err),
        };
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
