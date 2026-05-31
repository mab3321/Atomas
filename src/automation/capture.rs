//! Screenshot capture module
//!
//! Handles loading screenshots from disk (dry-run mode)
//! Future: ADB screencap integration

use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

/// Source of game screenshots
#[derive(Debug, Clone)]
pub enum ScreenshotSource {
    /// Load from a file on disk (dry-run mode)
    File(PathBuf),
    /// Future: Capture from ADB device
    Adb,
}

impl ScreenshotSource {
    /// Create a file-based source
    pub fn from_file<P: AsRef<Path>>(path: P) -> Self {
        Self::File(path.as_ref().to_path_buf())
    }

    /// Check if the screenshot source is available
    pub fn is_available(&self) -> Result<()> {
        match self {
            Self::File(path) => {
                if !path.exists() {
                    anyhow::bail!("Screenshot file not found: {:?}", path);
                }
                Ok(())
            }
            Self::Adb => {
                anyhow::bail!("ADB mode not implemented yet (Phase 2)")
            }
        }
    }

    /// Get the screenshot path (for dry-run mode)
    pub fn get_path(&self) -> Result<PathBuf> {
        match self {
            Self::File(path) => Ok(path.clone()),
            Self::Adb => {
                anyhow::bail!("ADB mode not available in Phase 1")
            }
        }
    }

    /// Capture a screenshot
    ///
    /// In dry-run mode, this just validates the file exists
    /// In ADB mode (Phase 2), this will execute `adb exec-out screencap -p`
    pub fn capture(&self, move_number: usize) -> Result<PathBuf> {
        match self {
            Self::File(path) => {
                log::info!("[Move {}/N] Capturing screenshot...", move_number);
                log::info!("  Source: {:?} (dry-run mode)", path);

                if !path.exists() {
                    anyhow::bail!("Screenshot file not found: {:?}", path);
                }

                Ok(path.clone())
            }
            Self::Adb => {
                anyhow::bail!("ADB capture not implemented (Phase 2)")
            }
        }
    }
}

impl Default for ScreenshotSource {
    fn default() -> Self {
        // Default to sample screenshot from Milestone 1
        Self::from_file("assets/jpg/board.jpg")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_source() {
        let source = ScreenshotSource::from_file("assets/jpg/board.jpg");
        assert!(matches!(source, ScreenshotSource::File(_)));
    }
}
