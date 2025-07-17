use git2::{Error as GitError, Oid, Signature};

use crate::git::Git;

pub trait Commit {
    fn git_commit(&self) -> Result<Oid, GitError>;

    fn get_git_signature_info(&self) -> Result<(String, String), GitError>;
}

impl Commit for Git {
    fn git_commit(&self) -> Result<Oid, GitError> {
        // Get the current index
        let mut index = self.repo.index()?;

        // Write the index to a tree
        let tree_id = index.write_tree()?;
        let tree = self.repo.find_tree(tree_id)?;

        let sig_info: (String, String) = self.get_git_signature_info()?;
        // Create signature for author and committer
        let signature = Signature::now(&sig_info.0, &sig_info.1)?;

        // Get the HEAD commit (parent)
        let parent_commit = match self.repo.head() {
            Ok(head) => Some(head.peel_to_commit()?),
            Err(_) => None, // This is the initial commit
        };

        // Create the commit
        let commit_id = match parent_commit {
            Some(parent) => {
                // Regular commit with parent
                self.repo.commit(
                    Some("HEAD"),             // Update HEAD
                    &signature,               // Author
                    &signature,               // Committer
                    &self.commit_popup.input, // Commit message
                    &tree,                    // Tree
                    &[&parent],               // Parents
                )?
            }
            None => {
                // Initial commit (no parents)
                self.repo.commit(
                    Some("HEAD"),
                    &signature,
                    &signature,
                    &self.commit_popup.input,
                    &tree,
                    &[], // No parents
                )?
            }
        };
        Ok(commit_id)
    }
    fn get_git_signature_info(&self) -> Result<(String, String), GitError> {
        let config = self.repo.config()?;

        let name = config
            .get_string("user.name")
            .map_err(|_| GitError::from_str("Git user.name not configured"))?;

        let email = config
            .get_string("user.email")
            .map_err(|_| GitError::from_str("Git user.email not configured"))?;

        Ok((name, email))
    }
}
