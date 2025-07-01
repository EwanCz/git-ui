use crate::git::Git;

use git2::{Cred, Error as GitError, PushOptions, RemoteCallbacks};
use std::path::Path;

#[derive(PartialEq)]
pub enum PushMode {
    Push,
    Normal,
}

pub trait Push {
    const REMOTE_NAMES: [&str; 3] = ["origin", "upstream", "master"];

    fn execute_push(&mut self) -> Result<String, GitError>;

    fn check_push_prerequisites(&self) -> Result<(), GitError>;

    fn get_available_remote(&self) -> Result<git2::Remote, GitError>;

    fn setup_authentication_callbacks(&self, callbacks: &mut RemoteCallbacks);
    fn try_ssh_keys_static(username: Option<&str>) -> Result<Cred, GitError>;
}

impl Push for Git {
    fn execute_push(&mut self) -> Result<String, GitError> {
        // 1.  verifier les prerequis d'un push
        self.check_push_prerequisites()?;

        {
            // 2. get remote
            let mut remote = self.get_available_remote()?;

            // 3. handle credentials
            let mut callbacks = RemoteCallbacks::new();
            self.setup_authentication_callbacks(&mut callbacks);

            // 4. Configurer les options de push
            let mut push_options = PushOptions::new();
            push_options.remote_callbacks(callbacks);

            // 5. Définir la refspec pour le push
            let refspec = format!("refs/heads/{}:refs/heads/{}", self.branch, self.branch);
            let refspecs = &[refspec.as_str()];

            // 6. Exécuter le push
            remote.push(refspecs, Some(&mut push_options))?;
        }
        // 7. Successfully push
        Ok(format!("✅ Successfully pushed branch '{}'", self.branch))
    }

    fn check_push_prerequisites(&self) -> Result<(), GitError> {
        // Vérifier que le repo n'est pas bare
        if self.repo.is_bare() {
            return Err(GitError::from_str("Cannot push from bare repository"));
        }

        // Vérifier qu'il y a une HEAD
        let head = self
            .repo
            .head()
            .map_err(|_| GitError::from_str("No HEAD found - repository may be empty"))?;

        if head.target().is_none() {
            return Err(GitError::from_str("No commits found - nothing to push"));
        }

        // Vérifier que la branche existe
        let branch_ref = format!("refs/heads/{}", self.branch);
        if self.repo.find_reference(&branch_ref).is_err() {
            return Err(GitError::from_str(&format!(
                "Branch '{}' does not exist",
                self.branch
            )));
        }
        Ok(())
    }

    fn get_available_remote(&self) -> Result<git2::Remote, GitError> {
        let remotes = self.repo.remotes()?;

        if remotes.is_empty() {
            return Err(GitError::from_str(
                "No remotes configured. Add a remote with: git remote add origin <url>",
            ));
        }

        for remote_name in &Self::REMOTE_NAMES {
            if let Ok(remote) = self.repo.find_remote(remote_name) {
                if remote.url().is_some() {
                    return Ok(remote);
                }
            }
        }

        // Fallback: utiliser le premier remote disponible
        let first_remote_name = remotes
            .get(0)
            .ok_or_else(|| GitError::from_str("no remote available"))?;

        let remote = self.repo.find_remote(first_remote_name)?;

        if remote.url().is_none() {
            return Err(GitError::from_str("Remote has no URL configured"));
        }
        Ok(remote)
    }

    fn setup_authentication_callbacks(&self, callbacks: &mut RemoteCallbacks) {
        let attempt_count = std::cell::Cell::new(0u8);
        let ssh_agent_tried = std::cell::Cell::new(false);
        let ssh_keys_tried = std::cell::Cell::new(false);

        callbacks.credentials(move |_url, username_from_url, allowed_types| {
            let count = attempt_count.get();
            attempt_count.set(count + 1);

            if count >= 5 {
                return Err(GitError::from_str("Too many authentication attempts"));
            }

            // SSH Agent (essayer une seule fois)
            if allowed_types.contains(git2::CredentialType::SSH_KEY) && !ssh_agent_tried.get() {
                ssh_agent_tried.set(true);

                match Cred::ssh_key_from_agent(username_from_url.unwrap_or("git")) {
                    Ok(cred) => {
                        return Ok(cred);
                    }
                    Err(_e) => {}
                }
            }

            // SSH Keys (essayer une seule fois)
            if allowed_types.contains(git2::CredentialType::SSH_KEY) && !ssh_keys_tried.get() {
                ssh_keys_tried.set(true);

                if let Ok(cred) = Self::try_ssh_keys_static(username_from_url) {
                    return Ok(cred);
                }
            }
            Err(GitError::from_str("Authentication failed"))
        });
    }

    fn try_ssh_keys_static(username: Option<&str>) -> Result<Cred, GitError> {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());

        let keys = [
            ("id_ed25519", "id_ed25519.pub"),
            ("id_rsa", "id_rsa.pub"),
            ("id_ecdsa", "id_ecdsa.pub"),
        ];

        for (private, public) in &keys {
            let private_path = format!("{}/.ssh/{}", home, private);
            let public_path = format!("{}/.ssh/{}", home, public);

            if Path::new(&private_path).exists() {
                match Cred::ssh_key(
                    username.unwrap_or("git"),
                    Some(Path::new(&public_path)),
                    Path::new(&private_path),
                    None,
                ) {
                    Ok(cred) => {
                        return Ok(cred);
                    }
                    Err(_e) => {}
                }
            }
        }

        Err(GitError::from_str("No valid SSH keys found"))
    }
}
