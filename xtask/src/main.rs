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
    /// Run every standard check in sequence: fmt, clippy, test, bench, doc
    All,
    /// Run a single task
    Run {
        #[arg(value_enum)]
        task: Task,
    },
    /// List available tasks
    List,
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

fn run_cargo(root: &PathBuf, name: &str, args: &[&str]) -> Result<()> {
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
    match task {
        Task::Fmt => run_cargo(root, "fmt", &["fmt", "--all", "--check"]),
        Task::Clippy => run_cargo(
            root,
            "clippy",
            &[
                "clippy",
                "--workspace",
                "--all-targets",
                "--all-features",
                "--",
                "-D",
                "warnings",
            ],
        ),
        Task::Test => run_cargo(root, "test", &["test", "--workspace", "--all-targets"]),
        Task::Bench => run_cargo(root, "bench", &["bench", "--workspace", "--no-fail-fast"]),
        Task::Doc => run_cargo(
            root,
            "doc",
            &["doc", "--workspace", "--all-features", "--no-deps"],
        ),
    }
}

fn run_all(root: &PathBuf) -> Result<()> {
    for task in [Task::Fmt, Task::Clippy, Task::Test, Task::Bench, Task::Doc] {
        run_task(root, task)?;
    }
    Ok(())
}

fn print_tasks() {
    println!("Available tasks:");
    println!("  all");
    println!("  run fmt");
    println!("  run clippy");
    println!("  run test");
    println!("  run bench");
    println!("  run doc");
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let root = workspace_root()?;

    match cli.command {
        None | Some(Commands::All) => run_all(&root),
        Some(Commands::Run { task }) => run_task(&root, task),
        Some(Commands::List) => {
            print_tasks();
            Ok(())
        }
    }
}
