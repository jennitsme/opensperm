use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone, Default)]
pub struct ApprovalState {
    approved_scopes: Arc<Mutex<Vec<String>>>,
}

impl ApprovalState {
    pub async fn request(&self, scope: &str) -> bool {
        let mut guard = self.approved_scopes.lock().await;
        if guard.contains(&scope.to_string()) {
            return true;
        }
        // TODO: integrate real approval channel; auto-approve for now.
        guard.push(scope.to_string());
        true
    }
}
