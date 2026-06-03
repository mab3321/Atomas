use crate::elements::ElementType;
use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests;

/// Represents an atom in the ring (simplified version without full Element metadata)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Atom {
    pub value: i16,
}

impl Atom {
    pub fn new(value: i16) -> Self {
        Self { value }
    }

    pub fn from_element_type(element_type: ElementType) -> Self {
        Self {
            value: element_type.to_numeric(),
        }
    }

    pub fn is_plus(&self) -> bool {
        self.value == -1
    }

    pub fn is_minus(&self) -> bool {
        self.value == -2
    }

    pub fn is_special(&self) -> bool {
        self.value < 0
    }

    pub fn is_regular(&self) -> bool {
        self.value > 0
    }
}

/// Represents a game action
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Action {
    /// Insert the player atom at the specified gap index (between atoms)
    Insert { gap_index: usize },
    /// Use a plus atom to fuse adjacent atoms
    UsePlus { plus_index: usize },
    /// Use a minus atom to remove a target atom
    UseMinus { minus_index: usize, target_index: usize },
}

/// Represents the complete game state
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameState {
    /// The ring of atoms (circular arrangement)
    pub ring: Vec<Atom>,
    /// The current player atom (atom to be placed)
    pub player_atom: Atom,
    /// Game score
    pub score: u64,
    /// Number of moves made
    pub moves: u32,
}

impl GameState {
    /// Create a new game state
    pub fn new(ring: Vec<Atom>, player_atom: Atom) -> Self {
        Self {
            ring,
            player_atom,
            score: 0,
            moves: 0,
        }
    }

    /// Create a simple test state
    pub fn test_state() -> Self {
        Self {
            ring: vec![
                Atom::new(1),  // H
                Atom::new(2),  // He
                Atom::new(3),  // Li
                Atom::new(4),  // Be
                Atom::new(5),  // B
                Atom::new(6),  // C
            ],
            player_atom: Atom::new(1), // H
            score: 0,
            moves: 0,
        }
    }

    /// Get the number of atoms in the ring
    pub fn ring_size(&self) -> usize {
        self.ring.len()
    }

    /// Check if a gap index is valid
    pub fn is_valid_gap(&self, gap_index: usize) -> bool {
        gap_index < self.ring.len()
    }

    /// Check if an atom index is valid
    pub fn is_valid_index(&self, index: usize) -> bool {
        index < self.ring.len()
    }

    /// Check if an action is legal
    pub fn is_legal_action(&self, action: &Action) -> bool {
        match action {
            Action::Insert { gap_index } => {
                // Must be a regular atom and valid gap
                self.player_atom.is_regular() && self.is_valid_gap(*gap_index)
            }
            Action::UsePlus { plus_index } => {
                // Player atom must be a plus and target must exist
                self.player_atom.is_plus() && self.is_valid_index(*plus_index)
            }
            Action::UseMinus { minus_index, target_index } => {
                // Player atom must be minus, and both indices must be valid and different
                self.player_atom.is_minus()
                    && self.is_valid_index(*minus_index)
                    && self.is_valid_index(*target_index)
                    && minus_index != target_index
            }
        }
    }

    /// Apply an action and return the new state
    pub fn apply_action(&self, action: &Action) -> Result<GameState, String> {
        if !self.is_legal_action(action) {
            return Err(format!("Illegal action: {:?}", action));
        }

        match action {
            Action::Insert { gap_index } => self.apply_insert(*gap_index),
            Action::UsePlus { plus_index } => self.apply_plus(*plus_index),
            Action::UseMinus { minus_index, target_index } => {
                self.apply_minus(*minus_index, *target_index)
            }
        }
    }

    /// Apply an insert action
    fn apply_insert(&self, gap_index: usize) -> Result<GameState, String> {
        let mut new_ring = self.ring.clone();

        // Insert after the gap_index position
        // gap_index represents the gap AFTER position gap_index
        let insert_pos = gap_index + 1;
        if insert_pos >= new_ring.len() {
            new_ring.push(self.player_atom);
        } else {
            new_ring.insert(insert_pos, self.player_atom);
        }

        // Check for merges after insertion
        let (final_ring, merge_score) = self.process_merges(new_ring, insert_pos);

        Ok(GameState {
            ring: final_ring,
            player_atom: Atom::new(1), // Placeholder - will be replaced by spawn
            score: self.score + merge_score,
            moves: self.moves + 1,
        })
    }

    /// Apply a plus action (fuse adjacent atoms)
    fn apply_plus(&self, plus_index: usize) -> Result<GameState, String> {
        let mut new_ring = self.ring.clone();
        let n = new_ring.len();

        // Place the plus atom at the position
        new_ring[plus_index] = self.player_atom;

        // Get adjacent atoms
        let left_idx = (plus_index + n - 1) % n;
        let right_idx = (plus_index + 1) % n;

        let left_atom = new_ring[left_idx];
        let right_atom = new_ring[right_idx];

        // Only fuse if both neighbors are regular and equal
        if left_atom.is_regular() && right_atom.is_regular() && left_atom.value == right_atom.value {
            let fused_value = left_atom.value + 1;
            let base_score = (left_atom.value as u64) * 10;

            // Remove left, plus, and right atoms
            // Create new ring without these three atoms
            let mut temp_ring = Vec::new();
            for (i, atom) in new_ring.iter().enumerate() {
                if i != left_idx && i != plus_index && i != right_idx {
                    temp_ring.push(*atom);
                }
            }

            // Insert the fused atom at the position where left was
            let insert_pos = if left_idx < plus_index {
                left_idx.min(temp_ring.len())
            } else {
                0
            };

            if insert_pos >= temp_ring.len() {
                temp_ring.push(Atom::new(fused_value));
            } else {
                temp_ring.insert(insert_pos, Atom::new(fused_value));
            }

            // Process cascading merges
            let (final_ring, cascade_score) = self.process_merges(temp_ring, insert_pos);

            Ok(GameState {
                ring: final_ring,
                player_atom: Atom::new(1), // Placeholder
                score: self.score + base_score + cascade_score,
                moves: self.moves + 1,
            })
        } else {
            // Just place the plus without fusion
            Ok(GameState {
                ring: new_ring,
                player_atom: Atom::new(1), // Placeholder
                score: self.score,
                moves: self.moves + 1,
            })
        }
    }

    /// Apply a minus action (remove a target atom)
    fn apply_minus(&self, minus_index: usize, target_index: usize) -> Result<GameState, String> {
        let mut new_ring = self.ring.clone();

        // Remove the target atom first (adjust indices accordingly)
        let (first_remove, second_remove) = if target_index < minus_index {
            (target_index, minus_index - 1)
        } else {
            (minus_index, target_index)
        };

        let removed_value = new_ring[first_remove].value;
        new_ring.remove(second_remove);
        new_ring.remove(first_remove);

        let removal_score = if removed_value > 0 {
            (removed_value as u64) * 5
        } else {
            0
        };

        Ok(GameState {
            ring: new_ring,
            player_atom: Atom::new(1), // Placeholder
            score: self.score + removal_score,
            moves: self.moves + 1,
        })
    }

    /// Process merges starting from a position (handles X-+-X and cascades)
    fn process_merges(&self, mut ring: Vec<Atom>, start_pos: usize) -> (Vec<Atom>, u64) {
        let mut total_score = 0u64;
        let mut changed = true;
        let max_iterations = 20; // Prevent infinite loops
        let mut iteration = 0;

        while changed && ring.len() >= 2 && iteration < max_iterations {
            changed = false;
            iteration += 1;

            // Check all positions for possible merges, starting from start_pos
            for offset in 0..ring.len() {
                let check_pos = (start_pos + offset) % ring.len();

                // First check for X-+-X pattern at this position
                if ring.len() >= 3 {
                    if let Some((new_ring, score)) = self.check_and_merge_x_plus_x(&ring, check_pos) {
                        ring = new_ring;
                        total_score += score;
                        changed = true;
                        break; // Restart scan after any merge
                    }
                }

                // Then check for simple adjacent merge at this position
                if ring.len() >= 2 {
                    if let Some((new_ring, score)) = self.check_and_merge_adjacent(&ring, check_pos) {
                        ring = new_ring;
                        total_score += score;
                        changed = true;
                        break; // Restart scan after any merge
                    }
                }
            }
        }

        (ring, total_score)
    }

    /// Check and merge X-+-X pattern at position
    fn check_and_merge_x_plus_x(&self, ring: &[Atom], pos: usize) -> Option<(Vec<Atom>, u64)> {
        if ring.len() < 3 {
            return None;
        }

        let n = ring.len();
        let left_idx = (pos + n - 1) % n;
        let right_idx = (pos + 1) % n;

        let current = ring[pos];
        let left = ring[left_idx];
        let right = ring[right_idx];

        // Check for X-+-X: left and right are equal regular atoms, center is plus
        if current.is_plus() && left.is_regular() && right.is_regular() && left.value == right.value {
            let fused_value = left.value + 1;
            let score = (left.value as u64) * 10;

            // Remove the three atoms and insert fused atom
            let mut new_ring = Vec::new();
            for (i, atom) in ring.iter().enumerate() {
                if i != left_idx && i != pos && i != right_idx {
                    new_ring.push(*atom);
                }
            }

            // Insert fused atom at the position of the leftmost removed atom
            let insert_pos = if left_idx < pos {
                left_idx.min(new_ring.len())
            } else if left_idx > pos && left_idx > right_idx {
                0
            } else {
                new_ring.len()
            };

            if insert_pos >= new_ring.len() {
                new_ring.push(Atom::new(fused_value));
            } else {
                new_ring.insert(insert_pos, Atom::new(fused_value));
            }

            return Some((new_ring, score));
        }

        None
    }

    /// Check and merge two adjacent equal atoms at position
    fn check_and_merge_adjacent(&self, ring: &[Atom], pos: usize) -> Option<(Vec<Atom>, u64)> {
        if ring.len() < 2 {
            return None;
        }

        let n = ring.len();
        let next_idx = (pos + 1) % n;

        let current = ring[pos];
        let next = ring[next_idx];

        // Check if both are regular and equal
        if current.is_regular() && next.is_regular() && current.value == next.value {
            let fused_value = current.value + 1;
            let score = (current.value as u64) * 10;

            // Remove both atoms and insert fused atom
            let mut new_ring = Vec::new();
            for (i, atom) in ring.iter().enumerate() {
                if i != pos && i != next_idx {
                    new_ring.push(*atom);
                }
            }

            // Insert at the position of the first atom
            let insert_pos = pos.min(new_ring.len());
            if insert_pos >= new_ring.len() {
                new_ring.push(Atom::new(fused_value));
            } else {
                new_ring.insert(insert_pos, Atom::new(fused_value));
            }

            return Some((new_ring, score));
        }

        None
    }
}
