//! Main automation loop controller
//!
//! Orchestrates: Capture → Detect → Decide → Execute → Repeat

use super::{ActionExecutor, ScreenshotSource, SimpleSolver};
use anyhow::{Context, Result};
use atomas_core::elements::Data;
use atomas_cv::{DetectionConfig, GameStateDetector, map_decision_to_coordinates};

/// Statistics for the automation loop
#[derive(Debug, Default)]
pub struct LoopStats {
    pub total_moves: usize,
    pub successful_moves: usize,
    pub failed_moves: usize,
    pub detection_failures: usize,
}

impl LoopStats {
    pub fn success_rate(&self) -> f64 {
        if self.total_moves == 0 {
            0.0
        } else {
            (self.successful_moves as f64 / self.total_moves as f64) * 100.0
        }
    }
}

/// Main automation loop
pub struct AutomationLoop {
    screenshot_source: ScreenshotSource,
    solver: SimpleSolver,
    executor: ActionExecutor,
    detector: GameStateDetector,
    elements_data: Data,
    max_moves: usize,
    stats: LoopStats,
}

impl AutomationLoop {
    /// Create a new automation loop
    pub fn new(
        screenshot_source: ScreenshotSource,
        solver: SimpleSolver,
        executor: ActionExecutor,
        max_moves: usize,
    ) -> Result<Self> {
        // Load element data
        let elements_path = format!("{}/assets/txt/elements.txt", env!("CARGO_MANIFEST_DIR"));
        let elements_data = Data::load(&elements_path);

        // Create detector with accurate color matching config
        let mut config = DetectionConfig::accurate_color_matching();
        config.output_dir = format!("{}/assets/png/outputs", env!("CARGO_MANIFEST_DIR")).into();
        let detector =
            GameStateDetector::new(config).context("Failed to create game state detector")?;

        Ok(Self {
            screenshot_source,
            solver,
            executor,
            detector,
            elements_data,
            max_moves,
            stats: LoopStats::default(),
        })
    }

    /// Run the automation loop for the configured number of moves
    pub fn run(&mut self) -> Result<()> {
        log::info!("==========================================================");
        log::info!("MILESTONE 2: Automation Loop");
        log::info!("Mode: dry-run (Phase 1)");
        log::info!("Max moves: {}", self.max_moves);
        log::info!("==========================================================\n");

        // Validate screenshot source before starting
        self.screenshot_source
            .is_available()
            .context("Screenshot source not available")?;

        for move_number in 1..=self.max_moves {
            log::info!("----------------------------------------------------------");
            match self.execute_move(move_number) {
                Ok(_) => {
                    self.stats.successful_moves += 1;
                    log::info!("[Move {}/{}] ✓ Complete\n", move_number, self.max_moves);
                }
                Err(e) => {
                    self.stats.failed_moves += 1;
                    log::error!(
                        "[Move {}/{}] ✗ Failed: {}\n",
                        move_number,
                        self.max_moves,
                        e
                    );
                }
            }
            self.stats.total_moves += 1;
        }

        self.print_summary();

        Ok(())
    }

    /// Execute a single move
    fn execute_move(&mut self, move_number: usize) -> Result<()> {
        // Step 1: Capture screenshot
        let screenshot_path = self
            .screenshot_source
            .capture(move_number)
            .context("Failed to capture screenshot")?;

        // Step 2: Detect game state
        log::info!(
            "[Move {}/{}] Detecting game state...",
            move_number,
            self.max_moves
        );
        let detection_result = self
            .detector
            .detect_from_file(&screenshot_path, &self.elements_data)
            .context("Failed to detect game state")?;

        let num_ring_atoms = detection_result.ring_elements.len();
        let has_player = detection_result.player_atom.is_some();

        log::info!(
            "  Detected: {} ring atoms, {} player atom",
            num_ring_atoms,
            if has_player { "1" } else { "0" }
        );

        if num_ring_atoms == 0 {
            self.stats.detection_failures += 1;
            anyhow::bail!("No ring atoms detected - cannot choose move");
        }

        // Step 3: Choose move using simple strategy
        log::info!("[Move {}/{}] Choosing move...", move_number, self.max_moves);
        let decision = self
            .solver
            .choose_move(&detection_result)
            .context("Failed to choose move")?;

        // Step 4: Map decision to coordinates
        log::info!(
            "[Move {}/{}] Mapping to coordinates...",
            move_number,
            self.max_moves
        );
        let coordinates = map_decision_to_coordinates(&decision, &detection_result)
            .context("Failed to map decision to coordinates")?;

        log::info!("  Coordinate: ({}, {})", coordinates.x, coordinates.y);

        // Step 5: Execute tap (dry-run prints command)
        self.executor
            .execute_tap(coordinates, move_number)
            .context("Failed to execute tap")?;

        Ok(())
    }

    /// Print final summary
    fn print_summary(&self) {
        log::info!("==========================================================");
        log::info!("AUTOMATION LOOP SUMMARY");
        log::info!("==========================================================");
        log::info!("Total moves attempted: {}", self.stats.total_moves);
        log::info!("Successful: {}", self.stats.successful_moves);
        log::info!("Failed: {}", self.stats.failed_moves);
        log::info!("Detection failures: {}", self.stats.detection_failures);
        log::info!("Success rate: {:.1}%", self.stats.success_rate());
        log::info!("==========================================================\n");
    }

    /// Get current statistics
    pub fn stats(&self) -> &LoopStats {
        &self.stats
    }
}
