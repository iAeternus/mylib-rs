use anyhow::{Context, Result, bail};
use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;
use std::process::{Command, Stdio};

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "Workspace automation tasks",
    long_about = "A small task runner for common workspace checks."
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Run standard checks in sequence: fmt, clippy, test, doc
    All,
    /// Run one task and optionally forward extra args to cargo
    Run {
        /// Task name.
        #[arg(value_enum)]
        task: Task,
        /// Pass-through args for the underlying `cargo <task>` command.
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, ValueEnum)]
enum Task {
    Fmt,
    Clippy,
    Test,
    Bench,
    Doc,
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
    }
}

fn run_all(root: &PathBuf) -> Result<()> {
    for task in [Task::Fmt, Task::Clippy, Task::Test, Task::Doc] {
        run_task(root, task)?;
    }
    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let root = workspace_root()?;

    match cli.command {
        None | Some(Commands::All) => run_all(&root),
        Some(Commands::Run { task, args }) => run_task_with_args(&root, task, &args),
    }
}
