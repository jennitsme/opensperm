use clap::{Parser, Subcommand};
use opensperm_runtime::{planner::Plan, AgentConfig, AgentRuntime};
use std::fs;
use tracing_subscriber::EnvFilter;

mod init;

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
    Run { #[arg(long)] plan: Option<String> },
    /// Run contract tests / golden transcripts
    Test {},
    /// Package and sign a skill bundle
    Package {},
    /// Validate policies
    PolicyCheck {},
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
        Commands::Run { plan } => {
            let plan_path = plan.unwrap_or_else(|| "plan.json".to_string());
            let data = fs::read_to_string(&plan_path).expect("read plan");
            let plan: Plan = serde_json::from_str(&data).expect("parse plan");
            let runtime = AgentRuntime::new(
                opensperm_runtime::sandbox::SandboxManager::new(),
                opensperm_runtime::policy::PolicyEngine::new(),
            );
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
        Commands::Test {} => {
            println!("run contract tests + golden transcripts (TODO)");
        }
        Commands::Package {} => {
            println!("package and sign skill bundle (TODO)");
        }
        Commands::PolicyCheck {} => {
            println!("validate policy files (TODO)");
        }
    }
}
