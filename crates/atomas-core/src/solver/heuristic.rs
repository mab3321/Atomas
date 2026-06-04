use crate::GameState;
use serde::{Deserialize, Serialize};

/// Weights for different heuristic components
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeuristicWeights {
    /// Weight for current score
    pub score_weight: f64,
    /// Weight for merge potential (number of mergeable pairs)
    pub merge_potential_weight: f64,
    /// Weight for highest atom value (progression)
    pub highest_atom_weight: f64,
    /// Weight for ring size penalty (too many atoms = bad)
    pub ring_size_penalty: f64,
    /// Weight for diversity (variety of atom values)
    pub diversity_weight: f64,
}

impl Default for HeuristicWeights {
    fn default() -> Self {
        Self {
            score_weight: 1.0,
            merge_potential_weight: 10.0,
            highest_atom_weight: 5.0,
            ring_size_penalty: -2.0,
            diversity_weight: 2.0,
        }
    }
}

/// Evaluate a game state and return a heuristic score
pub fn evaluate_state(state: &GameState, weights: &HeuristicWeights) -> f64 {
    let mut score = 0.0;

    // Component 1: Current score
    score += weights.score_weight * state.score as f64;

    // Component 2: Merge potential (count adjacent equal atoms)
    let merge_potential = count_merge_potential(state);
    score += weights.merge_potential_weight * merge_potential as f64;

    // Component 3: Highest atom value (reward progression)
    if let Some(max_atom) = state.ring.iter().map(|a| a.value).max() {
        score += weights.highest_atom_weight * max_atom as f64;
    }

    // Component 4: Ring size penalty (penalize overcrowding)
    let ring_size = state.ring_size() as f64;
    score += weights.ring_size_penalty * ring_size;

    // Component 5: Diversity (variety of unique values)
    let diversity = count_unique_values(state);
    score += weights.diversity_weight * diversity as f64;

    score
}

/// Count the number of mergeable pairs in the ring
fn count_merge_potential(state: &GameState) -> usize {
    if state.ring.is_empty() {
        return 0;
    }

    let mut count = 0;
    let n = state.ring.len();

    // Check adjacent pairs (including wraparound)
    for i in 0..n {
        let current = state.ring[i];
        let next = state.ring[(i + 1) % n];

        if current.is_regular() && next.is_regular() && current.value == next.value {
            count += 1;
        }
    }

    // Check for X-+-X patterns (two equal atoms with a plus between)
    for i in 0..n {
        let left_idx = (i + n - 1) % n;
        let right_idx = (i + 1) % n;

        let current = state.ring[i];
        let left = state.ring[left_idx];
        let right = state.ring[right_idx];

        if current.is_plus() && left.is_regular() && right.is_regular() && left.value == right.value
        {
            count += 2; // X-+-X is worth more
        }
    }

    count
}

/// Count unique atom values in the ring
fn count_unique_values(state: &GameState) -> usize {
    let mut values = std::collections::HashSet::new();
    for atom in &state.ring {
        if atom.is_regular() {
            values.insert(atom.value);
        }
    }
    values.len()
}

/// Evaluate the expected value after an action
#[allow(dead_code)]
pub fn evaluate_action_outcome(
    original_state: &GameState,
    resulting_state: &GameState,
    weights: &HeuristicWeights,
) -> f64 {
    // Immediate score gain
    let score_gain = (resulting_state.score - original_state.score) as f64;

    // Future potential (heuristic of resulting state)
    let future_value = evaluate_state(resulting_state, weights);

    // Combine immediate and future value
    score_gain * 2.0 + future_value * 0.5
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Atom;

    #[test]
    fn test_evaluate_empty_state() {
        let state = GameState::new(vec![], Atom::new(1));
        let weights = HeuristicWeights::default();
        let score = evaluate_state(&state, &weights);
        // Should not panic, should return some value
        assert!(score.is_finite());
    }

    #[test]
    fn test_evaluate_simple_state() {
        let state = GameState::new(vec![Atom::new(1), Atom::new(2), Atom::new(3)], Atom::new(1));
        let weights = HeuristicWeights::default();
        let score = evaluate_state(&state, &weights);
        assert!(score.is_finite());
        assert!(score != 0.0); // Should have some value
    }

    #[test]
    fn test_merge_potential_detection() {
        // State with mergeable pairs
        let state1 = GameState::new(vec![Atom::new(2), Atom::new(2), Atom::new(3)], Atom::new(1));
        let potential1 = count_merge_potential(&state1);
        assert!(potential1 > 0, "Should detect mergeable pair");

        // State without mergeable pairs
        let state2 = GameState::new(vec![Atom::new(1), Atom::new(2), Atom::new(3)], Atom::new(1));
        let potential2 = count_merge_potential(&state2);
        assert_eq!(potential2, 0, "Should detect no mergeable pairs");
    }

    #[test]
    fn test_higher_score_is_better() {
        let weights = HeuristicWeights::default();

        let state1 = GameState {
            ring: vec![Atom::new(1), Atom::new(2)],
            player_atom: Atom::new(1),
            score: 100,
            moves: 5,
        };

        let state2 = GameState {
            ring: vec![Atom::new(1), Atom::new(2)],
            player_atom: Atom::new(1),
            score: 200,
            moves: 5,
        };

        let score1 = evaluate_state(&state1, &weights);
        let score2 = evaluate_state(&state2, &weights);

        assert!(score2 > score1, "Higher game score should evaluate better");
    }

    #[test]
    fn test_merge_potential_is_valuable() {
        let weights = HeuristicWeights::default();

        // State with merge potential
        let state1 = GameState::new(vec![Atom::new(3), Atom::new(3), Atom::new(1)], Atom::new(1));

        // State without merge potential
        let state2 = GameState::new(vec![Atom::new(1), Atom::new(2), Atom::new(3)], Atom::new(1));

        let score1 = evaluate_state(&state1, &weights);
        let score2 = evaluate_state(&state2, &weights);

        assert!(
            score1 > score2,
            "State with merge potential should score higher"
        );
    }

    #[test]
    fn test_unique_values_count() {
        let state = GameState::new(
            vec![Atom::new(1), Atom::new(2), Atom::new(1), Atom::new(3)],
            Atom::new(1),
        );
        let unique = count_unique_values(&state);
        assert_eq!(unique, 3, "Should count 3 unique values (1, 2, 3)");
    }

    #[test]
    fn test_x_plus_x_pattern_detection() {
        // State with X-+-X pattern
        let state = GameState::new(
            vec![Atom::new(3), Atom::new(-1), Atom::new(3)],
            Atom::new(1),
        );
        let potential = count_merge_potential(&state);
        assert!(
            potential >= 2,
            "Should detect X-+-X pattern with high value"
        );
    }
}
