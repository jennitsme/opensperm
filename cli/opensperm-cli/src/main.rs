use clap::{Parser, Subcommand};
use opensperm_runtime::{AgentConfig, AgentRuntime};
use tracing_subscriber::EnvFilter;

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
            println!("init skill scaffold (language={:?})", language);
            // TODO: scaffold files based on language
        }
        Commands::Run { plan } => {
            let plan_path = plan.unwrap_or_else(|| "plan.json".to_string());
            println!("run agent with plan {}", plan_path);
            // TODO: load plan, construct runtime, execute
            let runtime = AgentRuntime::new(
                opensperm_runtime::sandbox::SandboxManager::new(),
                opensperm_runtime::policy::PolicyEngine::new(),
            );
            let _config = AgentConfig {
                id: "agent-1".into(),
                policy_scopes: vec![],
                budget_ms: 10_000,
            };
            let _ = runtime;
        }
        Commands::Test {} => {
            println!("run contract tests + golden transcripts");
        }
        Commands::Package {} => {
            println!("package and sign skill bundle");
        }
        Commands::PolicyCheck {} => {
            println!("validate policy files");
        }
    }
}
