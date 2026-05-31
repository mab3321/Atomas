//! Action execution module
//!
//! Handles executing tap actions on the game
//! Phase 1: Dry-run mode (prints commands)
//! Phase 2: Real ADB tap execution

use anyhow::Result;
use atomas_cv::ActionCoordinates;

/// Execution mode
#[derive(Debug, Clone, Copy)]
pub enum ExecutionMode {
    /// Dry-run: print commands without executing
    DryRun,
    /// Real ADB execution (Phase 2)
    Adb,
}

/// Action executor
pub struct ActionExecutor {
    mode: ExecutionMode,
}

impl ActionExecutor {
    /// Create a new executor with the given mode
    pub fn new(mode: ExecutionMode) -> Self {
        Self { mode }
    }

    /// Execute a tap at the given coordinates
    ///
    /// In dry-run mode, this prints the command that would be executed
    /// In ADB mode, this executes `adb shell input tap x y`
    pub fn execute_tap(&self, coordinates: ActionCoordinates, move_number: usize) -> Result<()> {
        match self.mode {
            ExecutionMode::DryRun => self.dry_run_tap(coordinates, move_number),
            ExecutionMode::Adb => {
                anyhow::bail!("ADB execution not implemented yet (Phase 2)")
            }
        }
    }

    /// Dry-run mode: print the tap command without executing
    fn dry_run_tap(&self, coordinates: ActionCoordinates, move_number: usize) -> Result<()> {
        log::info!("[Move {}/N] Executing tap (DRY-RUN)...", move_number);
        log::info!(
            "  Would execute: adb shell input tap {} {}",
            coordinates.x,
            coordinates.y
        );
        log::info!("  Status: Skipped (dry-run mode)");

        Ok(())
    }
}

impl Default for ActionExecutor {
    fn default() -> Self {
        Self::new(ExecutionMode::DryRun)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dry_run_executor() {
        let executor = ActionExecutor::new(ExecutionMode::DryRun);
        let coords = ActionCoordinates::new(100, 200);
        assert!(executor.execute_tap(coords, 1).is_ok());
    }
}
