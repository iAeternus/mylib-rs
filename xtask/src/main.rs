use anyhow::{Context, Result, bail};
use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;
use std::process::{Command, Stdio};

#[derive(Parser, Debug)]
#[command(author, version, about = "Workspace automation tasks")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Run CI checks in sequence (default): fmt, clippy, test, miri, doc
    Ci,
    /// Run one task and optionally forward extra args to cargo
    Run {
        /// Task name.
        #[arg(value_enum)]
        task: Task,
        /// Pass-through args for the underlying `cargo <task>` command (use `--`).
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, ValueEnum)]
enum Task {
    #[value(name = "fmt", help = "Format code (cargo fmt --all --check)")]
    Fmt,
    #[value(
        name = "clippy",
        help = "Lint workspace (cargo clippy --all-targets --all-features -D warnings)"
    )]
    Clippy,
    #[value(name = "test", help = "Run tests (cargo test --all-targets)")]
    Test,
    #[value(name = "bench", help = "Run benches (cargo bench --no-fail-fast)")]
    Bench,
    #[value(name = "doc", help = "Build docs (cargo doc --all-features --no-deps)")]
    Doc,
    #[value(name = "miri", help = "Run Miri (cargo +<toolchain> miri)")]
    Miri,
    #[value(name = "flamegraph", help = "Generate flamegraph (cargo flamegraph)")]
    Flamegraph,
}

fn workspace_root() -> Result<PathBuf> {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    dir.parent()
        .map(|p| p.to_path_buf())
        .context("failed to locate workspace root from xtask")
}

fn run_cargo(root: &PathBuf, name: &str, args: &[String]) -> Result<()> {
    println!("==> {name}: cargo {}", args.join(" "));
    let status = Command::new("cargo")
        .args(args)
        .current_dir(root)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .with_context(|| format!("failed to execute cargo for task `{name}`"))?;

    if !status.success() {
        bail!("task `{name}` failed with status {status}");
    }
    Ok(())
}

fn run_task(root: &PathBuf, task: Task) -> Result<()> {
    run_task_with_args(root, task, &[])
}

fn run_task_with_args(root: &PathBuf, task: Task, extra_args: &[String]) -> Result<()> {
    match task {
        Task::Fmt => {
            let mut args = vec![
                "fmt".to_string(),
                "--all".to_string(),
                "--check".to_string(),
            ];
            args.extend_from_slice(extra_args);
            run_cargo(root, "fmt", &args)
        }
        Task::Clippy => {
            let mut args = vec![
                "clippy".to_string(),
                "--workspace".to_string(),
                "--all-targets".to_string(),
                "--all-features".to_string(),
                "--".to_string(),
                "-D".to_string(),
                "warnings".to_string(),
            ];
            args.extend_from_slice(extra_args);
            run_cargo(root, "clippy", &args)
        }
        Task::Test => {
            let mut args = vec![
                "test".to_string(),
                "--workspace".to_string(),
                "--all-targets".to_string(),
            ];
            args.extend_from_slice(extra_args);
            run_cargo(root, "test", &args)
        }
        Task::Bench => {
            let mut args = vec![
                "bench".to_string(),
                "--workspace".to_string(),
                "--no-fail-fast".to_string(),
            ];
            args.extend_from_slice(extra_args);
            run_cargo(root, "bench", &args)
        }
        Task::Doc => {
            let mut args = vec![
                "doc".to_string(),
                "--workspace".to_string(),
                "--all-features".to_string(),
                "--no-deps".to_string(),
            ];
            args.extend_from_slice(extra_args);
            run_cargo(root, "doc", &args)
        }
        Task::Miri => {
            let toolchain = std::env::var("XTASK_MIRI_TOOLCHAIN")
                .ok()
                .filter(|v| !v.trim().is_empty())
                .unwrap_or_else(|| "nightly".to_string());
            let mut args = vec![format!("+{toolchain}"), "miri".to_string()];
            args.extend_from_slice(extra_args);
            run_cargo(root, "miri", &args)
        }
        Task::Flamegraph => {
            let mut args = vec!["flamegraph".to_string()];
            args.extend_from_slice(extra_args);
            run_cargo(root, "flamegraph", &args)
        }
    }
}

fn run_ci(root: &PathBuf) -> Result<()> {
    for task in [Task::Fmt, Task::Clippy, Task::Test, Task::Doc] {
        run_task(root, task)?;
    }
    run_task_with_args(
        root,
        Task::Miri,
        &[
            "test".to_string(),
            "--workspace".to_string(),
            "--lib".to_string(),
            "--tests".to_string(),
            "--exclude".to_string(),
            "macros".to_string(),
        ],
    )
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let root = workspace_root()?;

    match cli.command {
        None | Some(Commands::Ci) => run_ci(&root),
        Some(Commands::Run { task, args }) => run_task_with_args(&root, task, &args),
    }
}
