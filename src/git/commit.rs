use git2::{Error as GitError, Oid, Signature};

use crate::git::Git;

#[derive(PartialEq)]
pub enum CommitMode {
    Normal,
    Commit,
}

pub trait Commit {
    fn enter_char_commit(&mut self, new_char: char);

    fn byte_index(&self) -> usize;

    fn move_cursor_left(&mut self);

    fn move_cursor_right(&mut self);

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize;

    fn get_git_signature_info(&self) -> Result<(String, String), GitError>;

    fn git_commit(&self) -> Result<Oid, GitError>;

    fn delete_char(&mut self);
}

impl Commit for Git {
    fn enter_char_commit(&mut self, new_char: char) {
        let index = self.byte_index();
        self.input.insert(index, new_char);
        self.move_cursor_right();
    }

    fn byte_index(&self) -> usize {
        self.input
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.character_index)
            .unwrap_or(self.input.len())
    }

    fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.character_index != 0;
        if is_not_cursor_leftmost {
            // Method "remove" is not used on the saved text for deleting the selected char.
            // Reason: Using remove on String works on bytes instead of the chars.
            // Using remove would require special care because of char boundaries.

            let current_index = self.character_index;
            let from_left_to_current_index = current_index - 1;

            // Getting all characters before the selected character.
            let before_char_to_delete = self.input.chars().take(from_left_to_current_index);
            // Getting all characters after selected character.
            let after_char_to_delete = self.input.chars().skip(current_index);

            // Put all characters together except the selected one.
            // By leaving the selected one out, it is forgotten and therefore deleted.
            self.input = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.character_index.saturating_sub(1);
        self.character_index = self.clamp_cursor(cursor_moved_left);
    }

    fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.character_index.saturating_add(1);
        self.character_index = self.clamp_cursor(cursor_moved_right);
    }
    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input.chars().count())
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
                    Some("HEAD"), // Update HEAD
                    &signature,   // Author
                    &signature,   // Committer
                    &self.input,  // Commit message
                    &tree,        // Tree
                    &[&parent],   // Parents
                )?
            }
            None => {
                // Initial commit (no parents)
                self.repo.commit(
                    Some("HEAD"),
                    &signature,
                    &signature,
                    &self.input,
                    &tree,
                    &[], // No parents
                )?
            }
        };
        Ok(commit_id)
    }
}
