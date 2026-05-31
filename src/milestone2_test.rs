//! Milestone 2 Test: Automation Loop
//!
//! Phase 1: Dry-run mode (no emulator required)
//! Phase 2: ADB integration

use anyhow::{Context, Result};
use clap::Parser;
use std::path::PathBuf;

mod automation;

use automation::executor::ExecutionMode;
use automation::{
    ActionExecutor, AutomationLoop, ScreenshotSource, SimpleSolver, SimpleStrategy, adb,
};

/// Milestone 2: Automation Loop for Atomas
#[derive(Parser, Debug)]
#[command(name = "milestone2")]
#[command(about = "Automation loop: Capture → Detect → Decide → Execute → Repeat")]
#[command(version = "0.2.0")]
struct Args {
    /// Enable ADB mode (requires emulator/device)
    #[arg(long)]
    adb: bool,

    /// Run in dry-run mode (no ADB required) [default if --adb not specified]
    #[arg(long)]
    dry_run: bool,

    /// Path to screenshot for dry-run mode
    #[arg(long, default_value = "assets/jpg/board.jpg")]
    screenshot: PathBuf,

    /// Device serial (optional, uses first device if not specified)
    #[arg(long)]
    device: Option<String>,

    /// Delay in milliseconds between moves (ADB mode only)
    #[arg(long, default_value_t = 1000)]
    delay_ms: u64,

    /// Screenshot capture path for ADB mode
    #[arg(long, default_value = "assets/png/runtime/current_screen.png")]
    screenshot_path: PathBuf,

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

    // Determine mode (default to dry-run if neither specified)
    let use_adb = args.adb || (!args.dry_run && !args.adb);
    let use_adb = args.adb; // Explicit ADB mode only

    if !args.adb && !args.dry_run {
        log::info!("No mode specified, defaulting to --dry-run");
    }

    // Parse strategy
    let strategy = SimpleStrategy::from_str(&args.strategy).ok_or_else(|| {
        anyhow::anyhow!(
            "Invalid strategy: '{}'. Valid options: random, prefer-insert, prefer-remove",
            args.strategy
        )
    })?;

    // Setup components based on mode
    let (screenshot_source, execution_mode) = if args.adb {
        // ADB Mode
        log::info!("=== ADB Mode ===");

        // Check ADB availability
        adb::check_adb_available().context(
            "ADB not available. Please install Android SDK Platform Tools and add to PATH.",
        )?;

        // Select device
        let device_serial =
            adb::select_device(args.device.as_deref()).context("Failed to select device")?;

        log::info!("Using device: {}", device_serial);
        log::info!("Screenshot path: {:?}", args.screenshot_path);
        log::info!("Delay between moves: {} ms", args.delay_ms);

        let source = ScreenshotSource::from_adb(device_serial.clone(), args.screenshot_path);
        let executor = ExecutionMode::Adb {
            device_serial,
            delay_ms: args.delay_ms,
        };

        (source, executor)
    } else {
        // Dry-run Mode
        log::info!("=== Dry-Run Mode ===");
        log::info!("Screenshot: {:?}", args.screenshot);

        let source = ScreenshotSource::from_file(&args.screenshot);
        let executor = ExecutionMode::DryRun;

        (source, executor)
    };

    // Validate source is available
    screenshot_source
        .is_available()
        .context("Screenshot source not available")?;

    let solver = SimpleSolver::new(strategy);
    let executor = ActionExecutor::new(execution_mode);

    // Create and run automation loop
    let mut automation_loop = AutomationLoop::new(screenshot_source, solver, executor, args.moves)?;

    automation_loop.run()?;

    // Check results
    let stats = automation_loop.stats();
    if stats.failed_moves > 0 {
        log::warn!("Some moves failed. Check logs above for details.");
        log::warn!(
            "Success rate: {:.1}% ({}/{} successful)",
            stats.success_rate(),
            stats.successful_moves,
            stats.total_moves
        );
    } else {
        log::info!("✓ All moves completed successfully!");
    }

    Ok(())
}
