//! Milestone 2 Test: Automation Loop
//!
//! Phase 1: Dry-run mode (no emulator required)
//! Phase 2: ADB integration (future)

use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

mod automation;

use automation::executor::ExecutionMode;
use automation::{ActionExecutor, AutomationLoop, ScreenshotSource, SimpleSolver, SimpleStrategy};

/// Milestone 2: Automation Loop for Atomas
#[derive(Parser, Debug)]
#[command(name = "milestone2")]
#[command(about = "Automation loop: Capture → Detect → Decide → Execute → Repeat")]
#[command(version = "0.1.0")]
struct Args {
    /// Run in dry-run mode (no ADB required)
    #[arg(long, default_value_t = true)]
    dry_run: bool,

    /// Path to screenshot for dry-run mode
    #[arg(long, default_value = "assets/jpg/board.jpg")]
    screenshot: PathBuf,

    /// Number of moves to execute
    #[arg(long, default_value_t = 10)]
    moves: usize,

    /// Decision strategy: random, prefer-insert, prefer-remove
    #[arg(long, default_value = "prefer-insert")]
    strategy: String,

    /// Verbose logging
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Initialize logger
    let log_level = if args.verbose { "debug" } else { "info" };
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(log_level))
        .format_timestamp(None)
        .init();

    // Validate arguments
    if args.moves == 0 {
        anyhow::bail!("Number of moves must be greater than 0");
    }

    // Parse strategy
    let strategy = SimpleStrategy::from_str(&args.strategy).ok_or_else(|| {
        anyhow::anyhow!(
            "Invalid strategy: '{}'. Valid options: random, prefer-insert, prefer-remove",
            args.strategy
        )
    })?;

    // Setup components
    let screenshot_source = if args.dry_run {
        ScreenshotSource::from_file(&args.screenshot)
    } else {
        anyhow::bail!("ADB mode not implemented yet (Phase 2). Use --dry-run");
    };

    let execution_mode = if args.dry_run {
        ExecutionMode::DryRun
    } else {
        ExecutionMode::Adb
    };

    let solver = SimpleSolver::new(strategy);
    let executor = ActionExecutor::new(execution_mode);

    // Create and run automation loop
    let mut automation_loop = AutomationLoop::new(screenshot_source, solver, executor, args.moves)?;

    automation_loop.run()?;

    // Check results
    let stats = automation_loop.stats();
    if stats.failed_moves > 0 {
        log::warn!("Some moves failed. Check logs above for details.");
    }

    Ok(())
}
