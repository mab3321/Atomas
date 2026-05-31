//! Milestone 1: Decision to Screen Action Mapping
//!
//! This module translates high-level decisions (INSERT/REMOVE) into concrete
//! screen coordinates for automation or human-in-the-loop verification.

use crate::detection::DetectionResult;
use crate::bbox::BBox;
use atomas_core::Element;
use serde::{Deserialize, Serialize};

/// A decision to perform an action in the game
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Decision {
    /// Insert the player atom at a gap between ring elements
    Insert { gap_index: usize },
    /// Remove an atom from the ring
    Remove { atom_index: usize },
}

/// Screen coordinates for executing a decision
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ActionCoordinates {
    pub x: i32,
    pub y: i32,
}

impl ActionCoordinates {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

/// Maps a Decision to concrete screen coordinates
///
/// # Arguments
/// * `decision` - The decision to execute (INSERT or REMOVE)
/// * `detection_result` - The detected game state with atom positions
///
/// # Returns
/// * `Ok(ActionCoordinates)` - The pixel coordinates to tap
/// * `Err` - If the decision is invalid for the current game state
pub fn map_decision_to_coordinates(
    decision: &Decision,
    detection_result: &DetectionResult,
) -> anyhow::Result<ActionCoordinates> {
    match decision {
        Decision::Insert { gap_index } => {
            calculate_gap_coordinates(&detection_result.ring_elements, *gap_index)
        }
        Decision::Remove { atom_index } => {
            get_atom_coordinates(&detection_result.ring_elements, *atom_index)
        }
    }
}

/// Calculate the gap coordinate as the midpoint between two adjacent ring elements
///
/// # Arguments
/// * `ring_elements` - The detected ring elements with their bounding boxes
/// * `gap_index` - The gap index (0 = between atoms 0 and 1, etc.)
///
/// # Details
/// For a ring with N atoms, there are N gaps:
/// - Gap 0: between atom[0] and atom[1]
/// - Gap 1: between atom[1] and atom[2]
/// - ...
/// - Gap N-1: between atom[N-1] and atom[0] (wrap-around)
fn calculate_gap_coordinates(
    ring_elements: &[(Element, BBox)],
    gap_index: usize,
) -> anyhow::Result<ActionCoordinates> {
    let n = ring_elements.len();

    if n == 0 {
        anyhow::bail!("Cannot calculate gap coordinates: ring is empty");
    }

    if n == 1 {
        anyhow::bail!("Cannot calculate gap coordinates: ring has only one atom (no gaps)");
    }

    // Wrap gap_index to valid range [0, n-1]
    let gap_idx = gap_index % n;

    // Gap between atom[gap_idx] and atom[gap_idx+1]
    let atom1_idx = gap_idx;
    let atom2_idx = (gap_idx + 1) % n;

    let center1 = ring_elements[atom1_idx].1.center();
    let center2 = ring_elements[atom2_idx].1.center();

    // Midpoint calculation
    let gap_x = (center1.x + center2.x) / 2;
    let gap_y = (center1.y + center2.y) / 2;

    println!(
        "INSERT gap calculation: gap_index={} → between atoms {} and {} → ({}, {})",
        gap_index, atom1_idx, atom2_idx, gap_x, gap_y
    );

    Ok(ActionCoordinates::new(gap_x, gap_y))
}

/// Get the center coordinates of a specific atom in the ring
///
/// # Arguments
/// * `ring_elements` - The detected ring elements with their bounding boxes
/// * `atom_index` - The index of the atom to remove
fn get_atom_coordinates(
    ring_elements: &[(Element, BBox)],
    atom_index: usize,
) -> anyhow::Result<ActionCoordinates> {
    if ring_elements.is_empty() {
        anyhow::bail!("Cannot get atom coordinates: ring is empty");
    }

    if atom_index >= ring_elements.len() {
        anyhow::bail!(
            "Invalid atom_index: {} (ring has {} atoms)",
            atom_index,
            ring_elements.len()
        );
    }

    let center = ring_elements[atom_index].1.center();

    println!(
        "REMOVE atom calculation: atom_index={} ({}) → ({}, {})",
        atom_index,
        ring_elements[atom_index].0.name,
        center.x,
        center.y
    );

    Ok(ActionCoordinates::new(center.x, center.y))
}

#[cfg(test)]
mod tests {
    use super::*;
    use atomas_core::elements::{Data, ElementType};
    use crate::bbox::BBox;

    fn mock_ring_elements() -> Vec<(Element<'static>, BBox)> {
        let data = Data::default();
        vec![
            (data.elements[0].clone(), BBox::new(100, 100, 50, 50, 1.0)),
            (data.elements[1].clone(), BBox::new(200, 100, 50, 50, 1.0)),
            (data.elements[2].clone(), BBox::new(200, 200, 50, 50, 1.0)),
            (data.elements[3].clone(), BBox::new(100, 200, 50, 50, 1.0)),
        ]
    }

    #[test]
    fn test_gap_coordinates() {
        let ring = mock_ring_elements();

        // Gap 0: between atoms 0 and 1
        let coords = calculate_gap_coordinates(&ring, 0).unwrap();
        assert_eq!(coords.x, 150); // (100+200)/2 + 25 (center offset)

        // Gap 3: wrap-around between atoms 3 and 0
        let coords = calculate_gap_coordinates(&ring, 3).unwrap();
        assert!(coords.x > 0);
    }

    #[test]
    fn test_atom_coordinates() {
        let ring = mock_ring_elements();

        let coords = get_atom_coordinates(&ring, 0).unwrap();
        assert_eq!(coords.x, 125); // 100 + 50/2
        assert_eq!(coords.y, 125);
    }

    #[test]
    fn test_empty_ring_error() {
        let ring = vec![];
        assert!(calculate_gap_coordinates(&ring, 0).is_err());
        assert!(get_atom_coordinates(&ring, 0).is_err());
    }
}
