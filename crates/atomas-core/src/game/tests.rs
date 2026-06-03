use super::*;

#[cfg(test)]
mod atom_tests {
    use super::*;

    #[test]
    fn test_atom_creation() {
        let atom = Atom::new(1);
        assert_eq!(atom.value, 1);
    }

    #[test]
    fn test_atom_is_plus() {
        let plus = Atom::new(-1);
        assert!(plus.is_plus());
        assert!(!plus.is_minus());
        assert!(plus.is_special());
        assert!(!plus.is_regular());
    }

    #[test]
    fn test_atom_is_minus() {
        let minus = Atom::new(-2);
        assert!(minus.is_minus());
        assert!(!minus.is_plus());
        assert!(minus.is_special());
        assert!(!minus.is_regular());
    }

    #[test]
    fn test_atom_is_regular() {
        let regular = Atom::new(5);
        assert!(regular.is_regular());
        assert!(!regular.is_special());
        assert!(!regular.is_plus());
        assert!(!regular.is_minus());
    }
}

#[cfg(test)]
mod gamestate_tests {
    use super::*;

    #[test]
    fn test_gamestate_creation() {
        let state = GameState::test_state();
        assert_eq!(state.ring_size(), 6);
        assert_eq!(state.score, 0);
        assert_eq!(state.moves, 0);
    }

    #[test]
    fn test_valid_gap_index() {
        let state = GameState::test_state();
        assert!(state.is_valid_gap(0));
        assert!(state.is_valid_gap(5));
        assert!(!state.is_valid_gap(6));
    }

    #[test]
    fn test_valid_atom_index() {
        let state = GameState::test_state();
        assert!(state.is_valid_index(0));
        assert!(state.is_valid_index(5));
        assert!(!state.is_valid_index(6));
    }
}

#[cfg(test)]
mod insert_tests {
    use super::*;

    #[test]
    fn test_simple_insert_no_merge() {
        // Ring: [1, 2, 3, 4, 5, 6], insert 7 at gap 2 (between 3 and 4)
        let state = GameState {
            ring: vec![
                Atom::new(1),
                Atom::new(2),
                Atom::new(3),
                Atom::new(4),
                Atom::new(5),
                Atom::new(6),
            ],
            player_atom: Atom::new(7),
            score: 0,
            moves: 0,
        };

        let action = Action::Insert { gap_index: 2 };
        let result = state.apply_action(&action).unwrap();

        assert_eq!(result.ring_size(), 7);
        assert_eq!(result.ring[3].value, 7); // Inserted after position 2
        assert_eq!(result.moves, 1);
        assert_eq!(result.score, 0); // No merge, no score
    }

    #[test]
    fn test_insert_with_simple_merge() {
        // Ring: [1, 2, 3, 3, 5], insert 3 at gap 2
        // Should create [1, 2, 3, 3, 3, 5] -> [1, 2, 4, 5] after merging
        let state = GameState {
            ring: vec![
                Atom::new(1),
                Atom::new(2),
                Atom::new(3),
                Atom::new(3),
                Atom::new(5),
            ],
            player_atom: Atom::new(3),
            score: 0,
            moves: 0,
        };

        let action = Action::Insert { gap_index: 2 };
        let result = state.apply_action(&action).unwrap();

        // After insertion and merge, should have one atom with value 4
        assert!(result.ring.iter().any(|a| a.value == 4));
        assert_eq!(result.moves, 1);
        assert!(result.score > 0); // Should have merge score
    }

    #[test]
    fn test_insert_at_start() {
        let state = GameState {
            ring: vec![Atom::new(2), Atom::new(3), Atom::new(4)],
            player_atom: Atom::new(1),
            score: 0,
            moves: 0,
        };

        let action = Action::Insert { gap_index: 0 };
        let result = state.apply_action(&action).unwrap();

        assert_eq!(result.ring_size(), 4);
        assert!(result.ring.contains(&Atom::new(1)));
    }

    #[test]
    fn test_insert_at_end() {
        let state = GameState {
            ring: vec![Atom::new(1), Atom::new(2), Atom::new(3)],
            player_atom: Atom::new(4),
            score: 0,
            moves: 0,
        };

        let action = Action::Insert { gap_index: 2 };
        let result = state.apply_action(&action).unwrap();

        assert_eq!(result.ring_size(), 4);
        assert!(result.ring.contains(&Atom::new(4)));
    }

    #[test]
    fn test_insert_illegal_with_special_atom() {
        let state = GameState {
            ring: vec![Atom::new(1), Atom::new(2)],
            player_atom: Atom::new(-1), // Plus atom
            score: 0,
            moves: 0,
        };

        let action = Action::Insert { gap_index: 0 };
        assert!(!state.is_legal_action(&action));
        assert!(state.apply_action(&action).is_err());
    }
}

#[cfg(test)]
mod x_plus_x_merge_tests {
    use super::*;

    #[test]
    fn test_x_plus_x_basic_merge() {
        // Ring: [3, 3], insert Plus between them
        // Should become: [3, Plus, 3] -> [4]
        let state = GameState {
            ring: vec![Atom::new(3), Atom::new(3)],
            player_atom: Atom::new(-1), // Plus
            score: 0,
            moves: 0,
        };

        let action = Action::UsePlus { plus_index: 0 };
        let result = state.apply_action(&action).unwrap();

        // After X-+-X merge, should have single atom with value 4
        assert!(result.ring.len() >= 1);
        assert!(result.ring.iter().any(|a| a.value == 4));
        assert!(result.score >= 30); // Base score for merging 3s
    }

    #[test]
    fn test_x_plus_x_pattern_detection() {
        // Ring: [2, 3, 2], insert Plus at position 1
        // Should create [2, Plus, 2] around position 1 -> [3]
        let state = GameState {
            ring: vec![Atom::new(2), Atom::new(3), Atom::new(2)],
            player_atom: Atom::new(-1), // Plus
            score: 0,
            moves: 0,
        };

        let action = Action::UsePlus { plus_index: 1 };
        let result = state.apply_action(&action).unwrap();

        // Should merge the 2s around the plus
        assert!(result.ring.iter().any(|a| a.value == 3));
    }

    #[test]
    fn test_x_plus_x_no_merge_different_values() {
        // Ring: [2, 3, 3], insert Plus at position 1
        // Should NOT merge because neighbors are different (2 and 3)
        let state = GameState {
            ring: vec![Atom::new(2), Atom::new(3), Atom::new(3)],
            player_atom: Atom::new(-1), // Plus
            score: 0,
            moves: 0,
        };

        let action = Action::UsePlus { plus_index: 1 };
        let result = state.apply_action(&action).unwrap();

        // Should not merge, plus stays in ring
        assert!(result.ring.iter().any(|a| a.is_plus()));
        assert_eq!(result.score, 0); // No score from no merge
    }
}

#[cfg(test)]
mod cascade_merge_tests {
    use super::*;

    #[test]
    fn test_cascade_merge_two_levels() {
        // Ring: [3, 3, 3], insert 3 at gap 0
        // Should create [3, 3, 3, 3] -> [4, 3] -> done or further cascade
        let state = GameState {
            ring: vec![Atom::new(3), Atom::new(3), Atom::new(3)],
            player_atom: Atom::new(3),
            score: 0,
            moves: 0,
        };

        let action = Action::Insert { gap_index: 0 };
        let result = state.apply_action(&action).unwrap();

        // Should have cascaded merges
        assert!(result.score > 30); // Multiple merges
        assert!(result.ring.len() < 4); // Some atoms merged
    }

    #[test]
    fn test_cascade_stops_when_no_more_matches() {
        // Ring: [2, 2, 5], insert 2 at gap 1
        // Should create [2, 2, 2, 5]
        // Adjacent merges: first two 2s merge -> [3, 2, 5]
        // No more adjacent matches, so stops
        let state = GameState {
            ring: vec![Atom::new(2), Atom::new(2), Atom::new(5)],
            player_atom: Atom::new(2),
            score: 0,
            moves: 0,
        };

        let action = Action::Insert { gap_index: 1 };
        let result = state.apply_action(&action).unwrap();

        // Should merge adjacent 2s but then stop (3, 2, and 5 don't have adjacent matches)
        assert!(result.ring.contains(&Atom::new(3)));
        assert!(result.ring.contains(&Atom::new(5)));
        assert_eq!(result.ring_size(), 3);
    }

    #[test]
    fn test_long_cascade() {
        // Create a scenario where multiple cascades can happen
        // Ring: [2, 2, 2, 2], all same value
        let state = GameState {
            ring: vec![Atom::new(2), Atom::new(2), Atom::new(2), Atom::new(2)],
            player_atom: Atom::new(2),
            score: 0,
            moves: 0,
        };

        let action = Action::Insert { gap_index: 0 };
        let result = state.apply_action(&action).unwrap();

        // Should cascade multiple times
        // [2,2,2,2,2] -> various merges
        assert!(result.score > 0);
        assert!(result.ring.len() < 5);
    }
}

#[cfg(test)]
mod plus_action_tests {
    use super::*;

    #[test]
    fn test_plus_action_legal() {
        let state = GameState {
            ring: vec![Atom::new(3), Atom::new(4), Atom::new(3)],
            player_atom: Atom::new(-1), // Plus
            score: 0,
            moves: 0,
        };

        let action = Action::UsePlus { plus_index: 1 };
        assert!(state.is_legal_action(&action));
    }

    #[test]
    fn test_plus_action_illegal_not_plus() {
        let state = GameState {
            ring: vec![Atom::new(3), Atom::new(4), Atom::new(3)],
            player_atom: Atom::new(5), // Regular atom, not plus
            score: 0,
            moves: 0,
        };

        let action = Action::UsePlus { plus_index: 1 };
        assert!(!state.is_legal_action(&action));
    }

    #[test]
    fn test_plus_action_invalid_index() {
        let state = GameState {
            ring: vec![Atom::new(3), Atom::new(4)],
            player_atom: Atom::new(-1), // Plus
            score: 0,
            moves: 0,
        };

        let action = Action::UsePlus { plus_index: 5 };
        assert!(!state.is_legal_action(&action));
    }

    #[test]
    fn test_plus_fuses_equal_neighbors() {
        // Ring: [4, 5, 4], use plus at position 1
        let state = GameState {
            ring: vec![Atom::new(4), Atom::new(5), Atom::new(4)],
            player_atom: Atom::new(-1), // Plus
            score: 0,
            moves: 0,
        };

        let action = Action::UsePlus { plus_index: 1 };
        let result = state.apply_action(&action).unwrap();

        // Should fuse the 4s into a 5
        assert!(result.score > 0);
        // After fusion, ring should be smaller
        assert!(result.ring_size() < state.ring_size());
    }
}

#[cfg(test)]
mod minus_action_tests {
    use super::*;

    #[test]
    fn test_minus_action_legal() {
        let state = GameState {
            ring: vec![Atom::new(3), Atom::new(4), Atom::new(5)],
            player_atom: Atom::new(-2), // Minus
            score: 0,
            moves: 0,
        };

        let action = Action::UseMinus {
            minus_index: 1,
            target_index: 2,
        };
        assert!(state.is_legal_action(&action));
    }

    #[test]
    fn test_minus_action_illegal_not_minus() {
        let state = GameState {
            ring: vec![Atom::new(3), Atom::new(4), Atom::new(5)],
            player_atom: Atom::new(5), // Regular atom
            score: 0,
            moves: 0,
        };

        let action = Action::UseMinus {
            minus_index: 1,
            target_index: 2,
        };
        assert!(!state.is_legal_action(&action));
    }

    #[test]
    fn test_minus_action_illegal_same_index() {
        let state = GameState {
            ring: vec![Atom::new(3), Atom::new(4), Atom::new(5)],
            player_atom: Atom::new(-2), // Minus
            score: 0,
            moves: 0,
        };

        let action = Action::UseMinus {
            minus_index: 1,
            target_index: 1,
        };
        assert!(!state.is_legal_action(&action));
    }

    #[test]
    fn test_minus_removes_two_atoms() {
        let state = GameState {
            ring: vec![
                Atom::new(1),
                Atom::new(2),
                Atom::new(3),
                Atom::new(4),
                Atom::new(5),
            ],
            player_atom: Atom::new(-2), // Minus
            score: 0,
            moves: 0,
        };

        let action = Action::UseMinus {
            minus_index: 1,
            target_index: 3,
        };
        let result = state.apply_action(&action).unwrap();

        // Should remove 2 atoms (minus position and target)
        assert_eq!(result.ring_size(), 3);
        assert!(result.score > 0); // Should get score for removal
    }

    #[test]
    fn test_minus_action_invalid_index() {
        let state = GameState {
            ring: vec![Atom::new(3), Atom::new(4)],
            player_atom: Atom::new(-2), // Minus
            score: 0,
            moves: 0,
        };

        let action = Action::UseMinus {
            minus_index: 0,
            target_index: 5,
        };
        assert!(!state.is_legal_action(&action));
    }
}

#[cfg(test)]
mod edge_case_tests {
    use super::*;

    #[test]
    fn test_ring_order_wraparound() {
        // Test that ring wraparound works correctly
        // Ring: [1, 2, 3], insert 1 at gap 2 (after position 2)
        // Creates: [1, 2, 3, 1]
        // The 1s at the ends will merge because it's a ring
        // [1, 2, 3, 1] -> [2, 2, 3] -> [3, 3] -> [4]
        // This is actually a full cascade merge scenario!
        let state = GameState {
            ring: vec![Atom::new(1), Atom::new(2), Atom::new(3)],
            player_atom: Atom::new(1),
            score: 0,
            moves: 0,
        };

        // Insert at last gap (after last element)
        let action = Action::Insert { gap_index: 2 };
        let result = state.apply_action(&action).unwrap();

        // After cascading merges, we should have fewer atoms
        // The exact result depends on merge order, but size should be < 4
        assert!(result.ring_size() < 4);
        assert!(result.score > 0); // Should have scored from merges
    }

    #[test]
    fn test_minimum_ring_size() {
        // Test with smallest possible ring
        let state = GameState {
            ring: vec![Atom::new(1), Atom::new(2)],
            player_atom: Atom::new(3),
            score: 0,
            moves: 0,
        };

        let action = Action::Insert { gap_index: 0 };
        let result = state.apply_action(&action).unwrap();

        assert_eq!(result.ring_size(), 3);
    }

    #[test]
    fn test_single_atom_ring() {
        let state = GameState {
            ring: vec![Atom::new(5)],
            player_atom: Atom::new(3),
            score: 0,
            moves: 0,
        };

        let action = Action::Insert { gap_index: 0 };
        let result = state.apply_action(&action).unwrap();

        assert_eq!(result.ring_size(), 2);
    }

    #[test]
    fn test_merge_creates_empty_ring() {
        // Edge case: what if all atoms merge into one?
        let state = GameState {
            ring: vec![Atom::new(2), Atom::new(2)],
            player_atom: Atom::new(2),
            score: 0,
            moves: 0,
        };

        let action = Action::Insert { gap_index: 0 };
        let result = state.apply_action(&action).unwrap();

        // All three 2s should merge eventually
        assert!(result.ring_size() >= 1);
        assert!(result.score > 0);
    }

    #[test]
    fn test_score_accumulation() {
        let mut state = GameState {
            ring: vec![Atom::new(2), Atom::new(3), Atom::new(4)],
            player_atom: Atom::new(5),
            score: 100,
            moves: 5,
        };

        let action = Action::Insert { gap_index: 0 };
        state = state.apply_action(&action).unwrap();

        assert_eq!(state.moves, 6);
        assert!(state.score >= 100); // Score should not decrease
    }

    #[test]
    fn test_moves_increment() {
        let state = GameState::test_state();
        let initial_moves = state.moves;

        let action = Action::Insert { gap_index: 0 };
        let result = state.apply_action(&action).unwrap();

        assert_eq!(result.moves, initial_moves + 1);
    }
}

#[cfg(test)]
mod complex_scenario_tests {
    use super::*;

    #[test]
    fn test_complex_cascade_scenario() {
        // Create a complex scenario with potential for long cascade
        // Ring: [3, 3, 4, 4], insert 3 to trigger cascades
        let state = GameState {
            ring: vec![Atom::new(3), Atom::new(3), Atom::new(4), Atom::new(4)],
            player_atom: Atom::new(3),
            score: 0,
            moves: 0,
        };

        let action = Action::Insert { gap_index: 1 };
        let result = state.apply_action(&action).unwrap();

        // Should create interesting merge patterns
        assert!(result.score > 0);
        assert!(result.ring_size() < 5);
    }

    #[test]
    fn test_multiple_actions_sequence() {
        let mut state = GameState {
            ring: vec![
                Atom::new(1),
                Atom::new(2),
                Atom::new(3),
                Atom::new(4),
            ],
            player_atom: Atom::new(5),
            score: 0,
            moves: 0,
        };

        // Action 1: Insert
        state = state.apply_action(&Action::Insert { gap_index: 0 }).unwrap();
        assert_eq!(state.moves, 1);

        // Action 2: Another insert
        state.player_atom = Atom::new(6);
        state = state.apply_action(&Action::Insert { gap_index: 2 }).unwrap();
        assert_eq!(state.moves, 2);

        assert!(state.ring_size() >= 4);
    }

    #[test]
    fn test_plus_in_mixed_ring() {
        // Ring with different values, test plus behavior
        let state = GameState {
            ring: vec![
                Atom::new(5),
                Atom::new(3),
                Atom::new(5),
                Atom::new(2),
            ],
            player_atom: Atom::new(-1), // Plus
            score: 0,
            moves: 0,
        };

        let action = Action::UsePlus { plus_index: 1 };
        let result = state.apply_action(&action).unwrap();

        // Plus at position 1 should check if neighbors (5 and 5) match
        // They do! So should fuse to 6
        assert!(result.ring.iter().any(|a| a.value == 6));
    }
}
