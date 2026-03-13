use clap::{Parser, Subcommand};
use opensperm_runtime::{planner::Plan, AgentConfig, AgentRuntime};
use std::fs;
use tracing_subscriber::EnvFilter;

mod init;
mod policy;
mod package;
mod test_cmd;
mod run_config;

#[derive(Parser)]
#[command(name = "opensperm", version, about = "Agentic runtime CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a skill scaffold (TS or Rust)
    Init { #[arg(long)] language: Option<String> },
    /// Run an agent locally with a policy config
    Run { #[arg(long)] plan: Option<String>, #[arg(long)] policy: Option<String>, #[arg(long)] run: Option<String> },
    /// Run contract tests / golden transcripts
    Test { #[arg(long)] transcript: Option<String> },
    /// Package and sign a skill bundle
    Package { #[arg(long)] manifest: Option<String>, #[arg(long)] secret: Option<String> },
    /// Validate policies
    PolicyCheck { #[arg(long)] file: Option<String> },
}

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let cli = Cli::parse();
    match cli.command {
        Commands::Init { language } => {
            if let Err(e) = init::scaffold(language) {
                eprintln!("init failed: {e}");
            }
        }
        Commands::Run { plan, policy: pol, run } => {
            let plan_path = plan.unwrap_or_else(|| "plan.json".to_string());
            let data = fs::read_to_string(&plan_path).expect("read plan");
            let plan: Plan = serde_json::from_str(&data).expect("parse plan");
            let policy_cfg = pol.and_then(|p| policy::load(&p).ok());
            let policy_engine = match policy_cfg {
                Some(cfg) => opensperm_runtime::policy::PolicyEngine::with_scopes(cfg.allowed_scopes.unwrap_or_default()),
                None => opensperm_runtime::policy::PolicyEngine::new(),
            };

            let (sandbox_cfg, registry_opt) = match run {
                Some(r) => run_config::load(&r).unwrap_or_else(|e| panic!("run config load failed: {e}")),
                None => (opensperm_runtime::sandbox::SandboxManager::new().config, None),
            };

            let mut sandbox = opensperm_runtime::sandbox::SandboxManager::with_config(sandbox_cfg);
            if let Some(reg) = registry_opt {
                sandbox = sandbox.with_registry(reg);
            }

            let runtime = AgentRuntime::new(sandbox, policy_engine);
            let _config = AgentConfig {
                id: "agent-1".into(),
                policy_scopes: vec![],
                budget_ms: 10_000,
            };
            let rt = tokio::runtime::Runtime::new().expect("rt");
            let trace = opensperm_runtime::observability::TraceCtx { trace_id: "trace".into(), span_id: "span".into() };
            rt.block_on(async move {
                if let Err(e) = runtime.execute_plan(plan, trace).await {
                    eprintln!("run failed: {e}");
                }
            });
        }
        Commands::Test { transcript } => {
            let t = transcript.unwrap_or_else(|| "tests/golden/sample.json".into());
            if let Err(e) = test_cmd::run_transcript(&t) {
                eprintln!("test failed: {e}");
            } else {
                println!("test passed: {t}");
            }
        }
        Commands::Package { manifest, secret } => {
            let manifest = manifest.unwrap_or_else(|| "manifest.json".into());
            let secret = secret.unwrap_or_else(|| std::env::var("OPENSPERM_SECRET").unwrap_or_default());
            if secret.is_empty() {
                eprintln!("missing secret key (base64)");
            } else if let Err(e) = package::sign(&manifest, &secret) {
                eprintln!("package failed: {e}");
            } else {
                println!("signed manifest: {manifest}");
            }
        }
        Commands::PolicyCheck { file } => {
            let file = file.unwrap_or_else(|| "policy.json".into());
            match policy::load(&file) {
                Ok(cfg) => println!("policy valid: allowed_scopes={:?}", cfg.allowed_scopes.unwrap_or_default()),
                Err(e) => eprintln!("policy invalid: {e}"),
            }
        }
    }
}
