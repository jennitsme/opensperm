use std::fs;
use std::path::Path;

pub fn scaffold(language: Option<String>) -> anyhow::Result<()> {
    let lang = language.unwrap_or_else(|| "typescript".into());
    match lang.as_str() {
        "typescript" => scaffold_ts()?,
        "rust" => scaffold_rust()?,
        _ => anyhow::bail!("unsupported language: {lang}"),
    }
    Ok(())
}

fn scaffold_ts() -> anyhow::Result<()> {
    fs::create_dir_all("skills/example-ts")?;
    fs::write(
        "skills/example-ts/manifest.json",
        r#"{
  "name": "example-ts",
  "version": "0.1.0",
  "language": "typescript",
  "entry": "dist/index.js",
  "inputs": {"text": {"type": "string"}},
  "outputs": {"text": {"type": "string"}},
  "capabilities": {},
  "policyScopes": []
}
"#,
    )?;
    fs::create_dir_all("skills/example-ts/src")?;
    fs::write(
        "skills/example-ts/src/index.ts",
        r#"import { defineSkill } from "@opensperm/sdk";

export default defineSkill({
  manifest: require("../manifest.json"),
  tools: [
    {
      name: "echo",
      inputSchema: z.object({ text: z.string() }),
      outputSchema: z.object({ text: z.string() }),
      handler: async ({ text }) => ({ text }),
    },
  ],
});
"#,
    )?;
    Ok(())
}

fn scaffold_rust() -> anyhow::Result<()> {
    fs::create_dir_all("skills/example-rust/src")?;
    fs::write(
        "skills/example-rust/Cargo.toml",
        r#"[package]
name = "example-rust"
version = "0.1.0"
edition = "2021"

[dependencies]
opensperm-sdk = { path = "../../sdk/rust" }
serde_json = "1"
"#,
    )?;
    fs::write(
        "skills/example-rust/src/lib.rs",
        r#"use opensperm_sdk::{define_skill, Manifest, SkillBundle, ToolDefinition};

pub fn skill() -> SkillBundle {
    define_skill(SkillBundle {
        manifest: Manifest {
            name: "example-rust".into(),
            description: None,
            version: "0.1.0".into(),
            language: "rust".into(),
            entry: "target/release/libexample_rust.so".into(),
        },
        tools: vec![ToolDefinition {
            name: "echo".into(),
            handler: |input| {
                let text = input.get("text").and_then(|v| v.as_str()).unwrap_or("").to_string();
                Ok(serde_json::json!({"text": text}))
            },
        }],
    })
    .expect("valid skill")
}
"#,
    )?;
    Ok(())
}
