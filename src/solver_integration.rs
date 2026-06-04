//! Milestone 3 Stage 3: Solver Integration
//!
//! This module bridges the CV detection system with the expectimax solver.
//! It converts between different state representations and action types.

use anyhow::{Context, Result};
use atomas_core::{Atom, GameState as CoreGameState};
use atomas_cv::{Decision, DetectionResult};

/// Convert CV detection result to solver game state
///
/// This extracts the essential game state (atom values only) from the
/// detection result which contains full Element metadata.
pub fn detection_to_core_state(detection: &DetectionResult) -> Result<CoreGameState> {
    // Extract ring atom values
    let mut ring_atoms = Vec::new();
    for (element, _bbox) in &detection.ring_elements {
        let value = element.element_type.to_numeric();
        ring_atoms.push(Atom::new(value));
    }

    if ring_atoms.is_empty() {
        anyhow::bail!("Cannot create game state: no atoms detected in ring");
    }

    // Extract player atom value
    let player_atom = if let Some((element, _bbox)) = &detection.player_atom {
        let value = element.element_type.to_numeric();
        Atom::new(value)
    } else {
        // Default to Hydrogen if player atom not detected
        log::warn!("Player atom not detected, defaulting to H (value=1)");
        Atom::new(1)
    };

    // Create core game state
    let state = CoreGameState::new(ring_atoms, player_atom);

    log::debug!(
        "Converted detection to core state: ring_size={}, player_atom={}",
        state.ring_size(),
        state.player_atom.value
    );

    Ok(state)
}

/// Convert solver action to automation decision
///
/// Maps the solver's Action enum (which includes UsePlus and UseMinus)
/// to the simpler Decision enum used by the automation system.
pub fn solver_action_to_decision(action: &atomas_core::Action) -> Result<Decision> {
    match action {
        atomas_core::Action::Insert { gap_index } => Ok(Decision::Insert {
            gap_index: *gap_index,
        }),

        atomas_core::Action::UsePlus { plus_index } => {
            // Plus atom should be placed at a position
            // We map this to an Insert at that position
            log::debug!("Converting UsePlus(pos={}) to Insert", plus_index);
            Ok(Decision::Insert {
                gap_index: *plus_index,
            })
        }

        atomas_core::Action::UseMinus {
            minus_index,
            target_index,
        } => {
            // Minus atom removes both the minus position and target
            // We map this to Remove at the minus_index
            // Note: The actual game logic might need both indices,
            // but the current Decision enum only supports single Remove
            log::debug!(
                "Converting UseMinus(minus={}, target={}) to Insert at minus position",
                minus_index,
                target_index
            );
            // For now, place the minus atom at its position
            Ok(Decision::Insert {
                gap_index: *minus_index,
            })
        }
    }
}

/// Format a solver action for logging
pub fn format_solver_action(action: &atomas_core::Action) -> String {
    match action {
        atomas_core::Action::Insert { gap_index } => {
            format!("INSERT at gap {}", gap_index)
        }
        atomas_core::Action::UsePlus { plus_index } => {
            format!("USE PLUS at position {}", plus_index)
        }
        atomas_core::Action::UseMinus {
            minus_index,
            target_index,
        } => {
            format!(
                "USE MINUS at {} to remove {}",
                minus_index, target_index
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use atomas_core::{Element, ElementType};
    use atomas_cv::BBox;

    fn create_test_element(value: i16) -> Element<'static> {
        let element_type = ElementType::from_numeric(value).unwrap();
        Element {
            id: atomas_core::Id::Single('H'),
            element_type,
            name: "Test",
            rgb: (255, 255, 255),
        }
    }

    #[test]
    fn test_detection_to_core_state() {
        let ring_elements = vec![
            (create_test_element(1), BBox::new(0, 0, 10, 10)),
            (create_test_element(2), BBox::new(10, 10, 20, 20)),
            (create_test_element(3), BBox::new(20, 20, 30, 30)),
        ];

        let player_atom = Some((create_test_element(2), BBox::new(100, 100, 110, 110)));

        let detection = DetectionResult {
            ring_elements,
            player_atom,
        };

        let core_state = detection_to_core_state(&detection).unwrap();

        assert_eq!(core_state.ring_size(), 3);
        assert_eq!(core_state.player_atom.value, 2);
    }

    #[test]
    fn test_solver_action_insert_conversion() {
        let action = atomas_core::Action::Insert { gap_index: 2 };
        let decision = solver_action_to_decision(&action).unwrap();

        assert!(matches!(decision, Decision::Insert { gap_index: 2 }));
    }

    #[test]
    fn test_solver_action_plus_conversion() {
        let action = atomas_core::Action::UsePlus { plus_index: 1 };
        let decision = solver_action_to_decision(&action).unwrap();

        // Plus gets converted to Insert
        assert!(matches!(decision, Decision::Insert { .. }));
    }

    #[test]
    fn test_solver_action_minus_conversion() {
        let action = atomas_core::Action::UseMinus {
            minus_index: 1,
            target_index: 3,
        };
        let decision = solver_action_to_decision(&action).unwrap();

        // Minus gets converted to Insert (placing the minus)
        assert!(matches!(decision, Decision::Insert { .. }));
    }

    #[test]
    fn test_empty_ring_fails() {
        let detection = DetectionResult {
            ring_elements: vec![],
            player_atom: None,
        };

        let result = detection_to_core_state(&detection);
        assert!(result.is_err());
    }
}
