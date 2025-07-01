use crate::git::Git;

use git2::{Cred, Error as GitError, PushOptions, RemoteCallbacks};
use std::path::Path;

#[derive(PartialEq)]
pub enum PushMode {
    Push,
    Normal,
}

pub trait Push {
    fn execute_push(&mut self) -> Result<(), GitError>;
}

impl Push for Git {
    fn execute_push(&mut self) -> Result<(), GitError> {
        let mut remote = self
            .repo
            .find_remote("origin")
            .or_else(|_| self.repo.find_remote("master"))
            .map_err(|_| GitError::from_str("No remote found"))?;

        let mut callbacks = RemoteCallbacks::new();

        // Essayer plusieurs méthodes d'authentification
        callbacks.credentials(|url, username_from_url, allowed_types| {
            println!("Authenticating to: {}", url);

            // Essayer SSH agent en premier
            if allowed_types.contains(git2::CredentialType::SSH_KEY) {
                if let Ok(cred) = Cred::ssh_key_from_agent(username_from_url.unwrap_or("git")) {
                    return Ok(cred);
                }
            }

            // Essayer les clés SSH par défaut
            if allowed_types.contains(git2::CredentialType::SSH_KEY) {
                let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
                let private_key = format!("{}/.ssh/id_rsa", home);
                let public_key = format!("{}/.ssh/id_rsa.pub", home);

                if Path::new(&private_key).exists() {
                    return Cred::ssh_key(
                        username_from_url.unwrap_or("git"),
                        Some(Path::new(&public_key)),
                        Path::new(&private_key),
                        None,
                    );
                }
            }

            Err(GitError::from_str("No authentication method worked"))
        });

        callbacks.push_update_reference(|refname, status| -> Result<(), GitError> {
            match status {
                Some(msg) => {
                    self.push_message =
                        String::from(format!("❌ Failed to push {}: {}", refname, msg));
                    Ok(())
                }
                None => {
                    self.push_message = String::from(format!("✅ Successfully pushed {}", refname));
                    Ok(())
                }
            }
        });

        let mut push_options = PushOptions::new();
        push_options.remote_callbacks(callbacks);

        let refspecs = &[format!(
            "refs/heads/{}:refs/heads/{}",
            self.branch, self.branch
        )];

        println!("Pushing branch '{}' to remote...", self.branch);
        remote.push(refspecs, Some(&mut push_options))?;
        Ok(())
    }
}
