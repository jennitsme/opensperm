use crate::docker::DockerSpec;
use crate::ipc::ToolCall;
use std::collections::HashMap;

#[derive(Clone)]
pub struct ToolSpec {
    pub command: String,
    pub args: Vec<String>,
    pub allow_egress: Vec<String>,
    pub docker: Option<DockerSpec>,
}

#[derive(Default, Clone)]
pub struct ToolRegistry {
    inner: HashMap<String, ToolSpec>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self { inner: HashMap::new() }
    }

    pub fn register(mut self, name: &str, spec: ToolSpec) -> Self {
        self.inner.insert(name.to_string(), spec);
        self
    }

    pub fn resolve(&self, call: &ToolCall) -> Option<ToolSpec> {
        self.inner.get(&call.tool).cloned()
    }
}
