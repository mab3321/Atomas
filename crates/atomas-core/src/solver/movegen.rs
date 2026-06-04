use crate::{Action, GameState};

/// Generate all legal actions for a given game state
pub fn generate_legal_actions(state: &GameState) -> Vec<Action> {
    let mut actions = Vec::new();

    if state.player_atom.is_regular() {
        // Generate all Insert actions (one per gap)
        for gap_index in 0..state.ring_size() {
            let action = Action::Insert { gap_index };
            if state.is_legal_action(&action) {
                actions.push(action);
            }
        }
    } else if state.player_atom.is_plus() {
        // Generate all UsePlus actions (one per position)
        for plus_index in 0..state.ring_size() {
            let action = Action::UsePlus { plus_index };
            if state.is_legal_action(&action) {
                actions.push(action);
            }
        }
    } else if state.player_atom.is_minus() {
        // Generate all UseMinus actions (minus_index × target_index combinations)
        for minus_index in 0..state.ring_size() {
            for target_index in 0..state.ring_size() {
                if minus_index != target_index {
                    let action = Action::UseMinus {
                        minus_index,
                        target_index,
                    };
                    if state.is_legal_action(&action) {
                        actions.push(action);
                    }
                }
            }
        }
    }

    actions
}

/// Count the number of legal actions for a state
pub fn count_legal_actions(state: &GameState) -> usize {
    if state.player_atom.is_regular() {
        state.ring_size()
    } else if state.player_atom.is_plus() {
        state.ring_size()
    } else if state.player_atom.is_minus() {
        let n = state.ring_size();
        n * (n - 1) // All pairs except (i, i)
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Atom;

    #[test]
    fn test_generate_insert_actions() {
        let state = GameState::new(
            vec![Atom::new(1), Atom::new(2), Atom::new(3)],
            Atom::new(4), // Regular atom
        );

        let actions = generate_legal_actions(&state);
        assert_eq!(actions.len(), 3); // One insert per gap
        assert!(actions.iter().all(|a| matches!(a, Action::Insert { .. })));
    }

    #[test]
    fn test_generate_plus_actions() {
        let state = GameState::new(
            vec![Atom::new(1), Atom::new(2), Atom::new(3)],
            Atom::new(-1), // Plus atom
        );

        let actions = generate_legal_actions(&state);
        assert_eq!(actions.len(), 3); // One plus per position
        assert!(actions.iter().all(|a| matches!(a, Action::UsePlus { .. })));
    }

    #[test]
    fn test_generate_minus_actions() {
        let state = GameState::new(
            vec![Atom::new(1), Atom::new(2), Atom::new(3)],
            Atom::new(-2), // Minus atom
        );

        let actions = generate_legal_actions(&state);
        assert_eq!(actions.len(), 6); // 3 * 2 = 6 (n * (n-1))
        assert!(actions.iter().all(|a| matches!(a, Action::UseMinus { .. })));
    }

    #[test]
    fn test_all_generated_actions_are_legal() {
        let states = vec![
            GameState::new(vec![Atom::new(1), Atom::new(2)], Atom::new(3)),
            GameState::new(vec![Atom::new(1), Atom::new(2)], Atom::new(-1)),
            GameState::new(
                vec![Atom::new(1), Atom::new(2), Atom::new(3)],
                Atom::new(-2),
            ),
        ];

        for state in states {
            let actions = generate_legal_actions(&state);
            for action in actions {
                assert!(
                    state.is_legal_action(&action),
                    "Generated illegal action: {:?}",
                    action
                );
            }
        }
    }

    #[test]
    fn test_count_legal_actions() {
        let state1 = GameState::new(vec![Atom::new(1); 5], Atom::new(2));
        assert_eq!(count_legal_actions(&state1), 5);

        let state2 = GameState::new(vec![Atom::new(1); 5], Atom::new(-1));
        assert_eq!(count_legal_actions(&state2), 5);

        let state3 = GameState::new(vec![Atom::new(1); 4], Atom::new(-2));
        assert_eq!(count_legal_actions(&state3), 12); // 4 * 3
    }
}
