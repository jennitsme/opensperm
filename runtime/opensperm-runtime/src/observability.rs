use serde::{Deserialize, Serialize};
use tracing::Span;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceCtx {
    pub trace_id: String,
    pub span_id: String,
}

impl TraceCtx {
    pub fn enter_span(&self, name: &str) -> Span {
        tracing::info!(trace_id=%self.trace_id, span_id=%self.span_id, "entering {name}");
        tracing::span!(tracing::Level::INFO, name, trace_id = %self.trace_id, span_id = %self.span_id)
    }
}
