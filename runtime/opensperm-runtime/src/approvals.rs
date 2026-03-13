use std::{collections::HashSet, fs, io::{stdin, stdout, Write}};
use tokio::sync::{Mutex, OnceCell};
use std::sync::Arc;

#[derive(Clone, Default)]
pub struct ApprovalState {
    approved_scopes: Arc<Mutex<HashSet<String>>>,
    file_cache: Arc<OnceCell<HashSet<String>>>,
}

impl ApprovalState {
    pub async fn request(&self, scope: &str) -> bool {
        {
            let guard = self.approved_scopes.lock().await;
            if guard.contains(scope) {
                return true;
            }
        }

        if std::env::var("OPENSPERM_APPROVE_ALL").is_ok() {
            let mut guard = self.approved_scopes.lock().await;
            guard.insert(scope.to_string());
            return true;
        }

        if let Ok(path) = std::env::var("OPENSPERM_APPROVAL_FILE") {
            if let Some(set) = self.load_file(&path) {
                if set.contains(scope) {
                    let mut guard = self.approved_scopes.lock().await;
                    guard.insert(scope.to_string());
                    return true;
                }
            }
        }

        if std::env::var("OPENSPERM_APPROVAL_PROMPT").is_ok() {
            if Self::prompt(scope) {
                let mut guard = self.approved_scopes.lock().await;
                guard.insert(scope.to_string());
                return true;
            }
        }

        false
    }

    fn load_file(&self, path: &str) -> Option<HashSet<String>> {
        if let Some(cached) = self.file_cache.get() {
            return Some(cached.clone());
        }
        let data = fs::read_to_string(path).ok()?;
        let set: HashSet<String> = data
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        let _ = self.file_cache.set(set.clone());
        Some(set)
    }

    fn prompt(scope: &str) -> bool {
        print!("Approve scope '{scope}'? [y/N]: ");
        let _ = stdout().flush();
        let mut input = String::new();
        if stdin().read_line(&mut input).is_ok() {
            let resp = input.trim().to_lowercase();
            return resp == "y" || resp == "yes";
        }
        false
    }
}
