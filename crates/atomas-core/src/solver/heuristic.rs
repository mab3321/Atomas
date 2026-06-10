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
            merge_potential_weight: 15.0,  // Increased - creating merge opportunities is critical
            highest_atom_weight: 8.0,      // Increased - progressing to higher atoms is key
            ring_size_penalty: -3.0,       // More aggressive - keep ring small
            diversity_weight: 1.0,         // Decreased - focus on merges, not variety
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

    // Component 4: Ring size penalty (TRULY EXPONENTIAL - must dominate ALL other factors)
    // Game typically ends at 18-20 atoms, so penalties must escalate dramatically
    let ring_size = state.ring_size();
    let ring_penalty = match ring_size {
        0..=10 => (ring_size as f64) * -5.0,          // Small penalty for normal play
        11 => -200.0,                                  // 2^7 = Start avoiding growth
        12 => -500.0,                                  // 2^9 = Getting serious
        13 => -1500.0,                                 // 2^11 = Very concerning
        14 => -5000.0,                                 // 2^13 = Critical!
        15 => -15000.0,                                // 2^14 = Extreme!
        16 => -50000.0,                                // 2^16 = Desperate!
        17 => -200000.0,                               // 2^18 = Game over imminent!
        _ => -1000000.0,                               // 2^20 = TERMINAL STATE!
    };
    score += ring_penalty;

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
            // Adjacent matching atoms - PRIME fusion opportunity
            // Weight by atom value - higher atoms are worth more
            let value_weight = (current.value as usize).max(1);
            count += 10 * value_weight; // Much higher base value, scaled by atom
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
            // Ready-to-use fusion - even more valuable
            let value_weight = (left.value as usize).max(1);
            count += 15 * value_weight; // X-+-X is immediate fusion potential
        }
    }

    // Reward having multiple different atoms that could potentially merge in future
    // (helps with ring management)
    let mut value_counts = std::collections::HashMap::new();
    for atom in &state.ring {
        if atom.is_regular() {
            *value_counts.entry(atom.value).or_insert(0) += 1;
        }
    }

    // For each value that appears 2+ times, that's potential for future merges
    for (_value, count_of_value) in value_counts {
        if count_of_value >= 2 {
            count += count_of_value; // Reward having multiple of same atom
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

    // CRITICAL BONUS: If this action caused a score increase (fusion happened), heavily reward it
    let fusion_bonus = if score_gain > 0.0 {
        score_gain * 10.0 // Massive bonus for successful fusions
    } else {
        0.0
    };

    // Combine immediate and future value
    score_gain * 2.0 + future_value * 0.5 + fusion_bonus
}

/// Check if a Plus atom placement at a position would create a fusion
/// Returns the score that would be gained (0 if no fusion)
pub fn evaluate_plus_placement(state: &GameState, plus_index: usize) -> f64 {
    if state.ring.len() < 2 {
        return 0.0;
    }

    let n = state.ring.len();
    let left_idx = (plus_index + n - 1) % n;
    let right_idx = (plus_index + 1) % n;

    let left = state.ring[left_idx];
    let right = state.ring[right_idx];

    // DEBUG: Log what we're evaluating
    eprintln!("[PLUS_EVAL] Position {}: left[{}]={}, right[{}]={}",
             plus_index, left_idx, left.value, right_idx, right.value);

    // Check if placing Plus here would cause a fusion
    if left.is_regular() && right.is_regular() && left.value == right.value {
        // This placement WILL cause a fusion!
        // Return a very high value based on the fusion value
        let fusion_value = left.value;
        let base_score = (fusion_value as f64) * 10.0;

        // Higher-value fusions are EXPONENTIALLY more valuable
        // He+He (2+2) → Li: ~200 points
        // Li+Li (3+3) → Be: ~450 points
        // Be+Be (4+4) → B: ~800 points
        // C+C (6+6) → N: ~1800 points
        let value_multiplier = (fusion_value as f64).powf(1.5);

        let mut reward = base_score * value_multiplier * 100.0;

        // CRITICAL: Plus atoms REDUCE ring size (3 atoms → 1 atom = -2 atoms)
        // When ring is full, Plus usage becomes EXPONENTIALLY more valuable!
        let ring_size_bonus = match n {
            0..=10 => 0.0,              // No bonus needed
            11..=12 => 5000.0,          // Good to reduce size
            13..=14 => 20000.0,         // Very valuable
            15..=16 => 100000.0,        // Extremely valuable!
            17 => 500000.0,             // CRITICAL - Plus might save the game!
            _ => 2000000.0,             // DESPERATE - Plus is the ONLY hope!
        };
        reward += ring_size_bonus;

        eprintln!("[PLUS_EVAL] ✅ FUSION! value={} ring_size={} -> reward={}", fusion_value, n, reward);
        reward // Massive reward for correct Plus placement
    } else {
        // Placing Plus here does NOTHING useful - MASSIVELY PENALIZE
        // Check if there ARE better opportunities available in the ring
        let has_better_option = check_for_fusion_opportunities(state);

        let penalty = if has_better_option {
            eprintln!("[PLUS_EVAL] ❌ BAD! Other fusion exists -> penalty=-500");
            -500.0 // Severe penalty for ignoring a good fusion opportunity
        } else {
            eprintln!("[PLUS_EVAL] ❌ BAD! No fusions available -> penalty=-100");
            -100.0 // Still penalize, but less severely
        };
        penalty
    }
}

/// Check if there are any fusion opportunities in the ring (adjacent matching atoms)
fn check_for_fusion_opportunities(state: &GameState) -> bool {
    let n = state.ring.len();
    if n < 2 {
        return false;
    }

    for i in 0..n {
        let current = state.ring[i];
        let next = state.ring[(i + 1) % n];

        // If we find ANY pair of matching regular atoms, that's a fusion opportunity
        if current.is_regular() && next.is_regular() && current.value == next.value {
            return true;
        }
    }

    false
}

/// Evaluate using a Minus atom to remove a target atom
/// Returns a score based on strategic value + ring management urgency
pub fn evaluate_minus_removal(state: &GameState, target_index: usize) -> f64 {
    if state.ring.is_empty() {
        return 0.0;
    }

    let n = state.ring.len();
    let target = state.ring[target_index];

    // Base value: prefer removing low-value atoms (they clutter the ring)
    // Removing H(1) is better than removing Be(4)
    let removal_value = if target.is_regular() {
        let atom_value = target.value as f64;
        // Lower values = higher removal reward (inverse relationship)
        let base = 100.0 / atom_value.max(1.0);  // H:100, He:50, Li:33, Be:25
        base * 50.0  // Scale up: H=5000, He=2500, Li=1666, Be=1250
    } else {
        0.0  // Can't remove special atoms
    };

    // CRITICAL: Minus atoms REDUCE ring size by 1
    // When ring is full, Minus usage becomes EXPONENTIALLY more valuable!
    let ring_urgency_bonus = match n {
        0..=10 => 0.0,              // No urgency
        11..=12 => 2000.0,          // Slightly valuable
        13..=14 => 10000.0,         // Very valuable
        15..=16 => 50000.0,         // Extremely valuable!
        17 => 250000.0,             // CRITICAL - Minus can save the game!
        _ => 1000000.0,             // DESPERATE - Minus is essential!
    };

    removal_value + ring_urgency_bonus
}

/// Evaluate an Insert action - prefer positions that create adjacent matching pairs
pub fn evaluate_insert_position(state: &GameState, gap_index: usize, atom_value: i16) -> f64 {
    if state.ring.is_empty() {
        return 0.0;
    }

    let n = state.ring.len();
    let left_idx = if gap_index == 0 { n - 1 } else { gap_index - 1 };
    let right_idx = gap_index % n;

    let left = state.ring[left_idx];
    let right = state.ring[right_idx];

    // Ring size penalty - TRULY EXPONENTIAL to DOMINATE all other factors
    // The penalty must be so large that NO strategic placement can override it
    let ring_size_penalty = match n {
        0..=10 => 0.0,              // Safe range
        11 => -100.0,               // Starting to fill
        12 => -300.0,               // Getting full
        13 => -800.0,               // Very full
        14 => -2000.0,              // Critical
        15 => -5000.0,              // Extreme
        16 => -15000.0,             // Desperate - avoid at all costs!
        17 => -50000.0,             // Game over imminent!
        _ => -200000.0,             // TERMINAL - game over in 1-2 moves!
    };

    // BEST: Inserting between two matching atoms creates merge chain opportunity
    if left.is_regular() && right.is_regular()
       && left.value == atom_value && right.value == atom_value {
        // Sandwiched between two matching atoms - HUGE reward
        let base = 500.0 * (atom_value as f64);
        return base + ring_size_penalty;
    }

    // GOOD: Inserting next to one matching atom creates merge opportunity
    if (left.is_regular() && left.value == atom_value)
       || (right.is_regular() && right.value == atom_value) {
        let base = 200.0 * (atom_value as f64);
        return base + ring_size_penalty;
    }

    // BAD: No merge potential - heavily penalize, especially when ring is full
    let no_merge_penalty = -100.0 + ring_size_penalty;
    no_merge_penalty
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
