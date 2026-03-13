use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryRecord {
    pub key: String,
    pub value: serde_json::Value,
    pub ttl_secs: Option<u64>,
}

pub trait MemoryProvider: Send + Sync {
    fn get(&self, key: &str) -> Option<serde_json::Value>;
    fn put(&self, record: MemoryRecord);
}
