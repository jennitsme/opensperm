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
            tracing::info!(scope=%scope, "approval: approve_all env");
            crate::logging::append_json("approval", serde_json::json!({"scope": scope, "source": "approve_all", "status": "allow"}));
            let mut guard = self.approved_scopes.lock().await;
            guard.insert(scope.to_string());
            return true;
        }

        if let Ok(path) = std::env::var("OPENSPERM_APPROVAL_FILE") {
            if let Some(set) = self.load_file(&path) {
                if set.contains(scope) {
                    tracing::info!(scope=%scope, file=%path, "approval: allowed by file");
                    crate::logging::append_json("approval", serde_json::json!({"scope": scope, "source": "file", "file": path, "status": "allow"}));
                    let mut guard = self.approved_scopes.lock().await;
                    guard.insert(scope.to_string());
                    return true;
                }
            }
            tracing::warn!(scope=%scope, file=%path, "approval: file present but scope not allowed");
            crate::logging::append_json("approval", serde_json::json!({"scope": scope, "source": "file", "file": path, "status": "deny"}));
        }

        // Enforce webhook for secure:* scopes
        let secure_scope = scope.starts_with("secure:");

        if let Ok(url) = std::env::var("OPENSPERM_APPROVAL_WEBHOOK") {
            let secret = std::env::var("OPENSPERM_APPROVAL_WEBHOOK_SECRET").unwrap_or_default();
            if secure_scope && secret.is_empty() {
                tracing::warn!(scope=%scope, "approval: secure scope requires webhook secret");
            } else if self.call_webhook(&url, scope, &secret).await {
                tracing::info!(scope=%scope, url=%url, "approval: webhook success");
                crate::logging::append_json("approval", serde_json::json!({"scope": scope, "source": "webhook", "status": "allow"}));
                let mut guard = self.approved_scopes.lock().await;
                guard.insert(scope.to_string());
                return true;
            } else {
                tracing::warn!(scope=%scope, url=%url, "approval: webhook denied/failed");
                crate::logging::append_json("approval", serde_json::json!({"scope": scope, "source": "webhook", "status": "deny"}));
            }
        }

        if std::env::var("OPENSPERM_APPROVAL_PROMPT").is_ok() {
            if Self::prompt(scope) {
                tracing::info!(scope=%scope, "approval: prompt approved");
                crate::logging::append_json("approval", serde_json::json!({"scope": scope, "source": "prompt", "status": "allow"}));
                let mut guard = self.approved_scopes.lock().await;
                guard.insert(scope.to_string());
                return true;
            } else {
                tracing::warn!(scope=%scope, "approval: prompt denied");
                crate::logging::append_json("approval", serde_json::json!({"scope": scope, "source": "prompt", "status": "deny"}));
            }
        }

        tracing::warn!(scope=%scope, "approval: denied");
        crate::logging::append_json("approval", serde_json::json!({"scope": scope, "status": "deny"}));
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

    async fn call_webhook(&self, url: &str, scope: &str, secret: &str) -> bool {
        let client = reqwest::Client::new();
        let payload = serde_json::json!({"scope": scope});
        let mut req = client.post(url).json(&payload);
        if !secret.is_empty() {
            req = req.header("X-Approval-Secret", secret);
        }
        match req.send().await {
            Ok(resp) => resp.status().is_success(),
            Err(_) => false,
        }
    }
}
