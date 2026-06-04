//! Expectimax solver integration for Milestone 3
//!
//! This module wraps the atomas-core solver for use in the automation pipeline.

use anyhow::{Context, Result};
use atomas_core::{Solver, SolverConfig};
use atomas_cv::{Decision, DetectionResult};

use crate::solver_integration::{detection_to_core_state, solver_action_to_decision, format_solver_action};

/// Solver strategy selection
#[derive(Debug, Clone, Copy)]
pub enum SolverStrategy {
    /// Use expectimax tree search
    Expectimax,
    /// Use fast expectimax (depth=1)
    ExpectimaxFast,
    /// Use thorough expectimax (depth=3)
    ExpectimaxThorough,
}

impl SolverStrategy {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "expectimax" => Some(Self::Expectimax),
            "expectimax-fast" | "fast" => Some(Self::ExpectimaxFast),
            "expectimax-thorough" | "thorough" => Some(Self::ExpectimaxThorough),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Expectimax => "Expectimax",
            Self::ExpectimaxFast => "ExpectimaxFast",
            Self::ExpectimaxThorough => "ExpectimaxThorough",
        }
    }
}

/// Expectimax solver for game automation
pub struct ExpectimaxSolver {
    solver: Solver,
    strategy: SolverStrategy,
}

impl ExpectimaxSolver {
    /// Create a new expectimax solver with the given strategy
    pub fn new(strategy: SolverStrategy) -> Self {
        let solver = match strategy {
            SolverStrategy::Expectimax => Solver::default(),
            SolverStrategy::ExpectimaxFast => Solver::fast(),
            SolverStrategy::ExpectimaxThorough => Solver::thorough(),
        };

        log::info!("=================================================");
        log::info!("Using EXPECTIMAX solver: {}", strategy.as_str());
        log::info!(
            "  Depth: {}",
            solver.config().expectimax_config.max_depth
        );
        log::info!(
            "  Spawn samples: {}",
            solver.config().expectimax_config.max_spawns_per_node
        );
        log::info!("=================================================");

        Self { solver, strategy }
    }

    /// Create with custom depth
    pub fn with_depth(depth: usize) -> Self {
        let mut config = SolverConfig::default();
        config.expectimax_config.max_depth = depth;

        let solver = Solver::new(config);

        log::info!("=================================================");
        log::info!("Using EXPECTIMAX solver with custom depth: {}", depth);
        log::info!("=================================================");

        Self {
            solver,
            strategy: SolverStrategy::Expectimax,
        }
    }

    /// Choose a move using expectimax search
    pub fn choose_move(&self, detection_result: &DetectionResult) -> Result<Decision> {
        // Convert detection to core game state
        let core_state = detection_to_core_state(detection_result)
            .context("Failed to convert detection to game state")?;

        log::debug!(
            "  Core state: ring_size={}, player_atom={}",
            core_state.ring_size(),
            core_state.player_atom.value
        );

        // Run solver
        let solver_result = self
            .solver
            .solve(&core_state)
            .context("Solver failed to find a move")?;

        log::info!(
            "  Solver action: {} (value={:.2}, nodes={})",
            format_solver_action(&solver_result.best_action),
            solver_result.expected_value,
            solver_result.nodes_evaluated
        );

        // Convert solver action to decision
        let decision = solver_action_to_decision(&solver_result.best_action)
            .context("Failed to convert solver action to decision")?;

        log::info!("  Decision: {:?}", decision);

        Ok(decision)
    }

    /// Get the strategy being used
    pub fn strategy(&self) -> SolverStrategy {
        self.strategy
    }
}

impl Default for ExpectimaxSolver {
    fn default() -> Self {
        Self::new(SolverStrategy::Expectimax)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strategy_parsing() {
        assert!(matches!(
            SolverStrategy::from_str("expectimax"),
            Some(SolverStrategy::Expectimax)
        ));
        assert!(matches!(
            SolverStrategy::from_str("fast"),
            Some(SolverStrategy::ExpectimaxFast)
        ));
        assert!(matches!(
            SolverStrategy::from_str("thorough"),
            Some(SolverStrategy::ExpectimaxThorough)
        ));
    }

    #[test]
    fn test_solver_creation() {
        let solver = ExpectimaxSolver::new(SolverStrategy::Expectimax);
        assert!(matches!(solver.strategy(), SolverStrategy::Expectimax));
    }

    #[test]
    fn test_solver_with_depth() {
        let solver = ExpectimaxSolver::with_depth(3);
        assert_eq!(solver.solver.config().expectimax_config.max_depth, 3);
    }
}
