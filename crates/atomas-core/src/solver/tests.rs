use super::*;
use crate::{Atom, GameState};

#[cfg(test)]
mod solver_integration_tests {
    use super::*;

    #[test]
    fn test_solver_returns_legal_action() {
        let state = GameState::new(vec![Atom::new(1), Atom::new(2), Atom::new(3)], Atom::new(1));

        let solver = Solver::default();
        let result = solver.solve(&state).unwrap();

        assert!(state.is_legal_action(&result.best_action));
        assert!(result.nodes_evaluated > 0);
        assert!(result.actions_considered > 0);
    }

    #[test]
    fn test_solver_with_plus_atom() {
        let state = GameState::new(
            vec![Atom::new(3), Atom::new(2), Atom::new(3)],
            Atom::new(-1), // Plus
        );

        let solver = Solver::default();
        let result = solver.solve(&state).unwrap();

        assert!(state.is_legal_action(&result.best_action));
        assert!(matches!(result.best_action, Action::UsePlus { .. }));
    }

    #[test]
    fn test_solver_with_minus_atom() {
        let state = GameState::new(
            vec![Atom::new(1), Atom::new(2), Atom::new(3), Atom::new(4)],
            Atom::new(-2), // Minus
        );

        let solver = Solver::default();
        let result = solver.solve(&state).unwrap();

        assert!(state.is_legal_action(&result.best_action));
        assert!(matches!(result.best_action, Action::UseMinus { .. }));
    }

    #[test]
    fn test_fast_solver() {
        let state = GameState::new(vec![Atom::new(1), Atom::new(2), Atom::new(3)], Atom::new(1));

        let solver = Solver::fast();
        let result = solver.solve(&state).unwrap();

        assert!(state.is_legal_action(&result.best_action));
    }

    #[test]
    fn test_thorough_solver() {
        let state = GameState::new(vec![Atom::new(1), Atom::new(2), Atom::new(3)], Atom::new(1));

        let solver = Solver::thorough();
        let result = solver.solve(&state).unwrap();

        assert!(state.is_legal_action(&result.best_action));
    }

    #[test]
    fn test_solver_deterministic() {
        let state = GameState::new(vec![Atom::new(1), Atom::new(2), Atom::new(3)], Atom::new(1));

        let solver = Solver::default();
        let result1 = solver.solve(&state).unwrap();
        let result2 = solver.solve(&state).unwrap();

        assert_eq!(result1.best_action, result2.best_action);
        assert_eq!(result1.expected_value, result2.expected_value);
    }

    #[test]
    fn test_solver_prefers_merges() {
        // State where inserting at one position creates a merge
        let state = GameState::new(
            vec![Atom::new(2), Atom::new(3), Atom::new(2)],
            Atom::new(2), // Inserting between first and second 2 creates merge
        );

        let solver = Solver::default();
        let result = solver.solve(&state).unwrap();

        // Should prefer an action that leads to merges
        assert!(state.is_legal_action(&result.best_action));
        // The solver should recognize merge potential
        assert!(result.expected_value > 0.0);
    }

    #[test]
    fn test_solver_handles_small_ring() {
        let state = GameState::new(vec![Atom::new(1), Atom::new(2)], Atom::new(1));

        let solver = Solver::default();
        let result = solver.solve(&state).unwrap();

        assert!(state.is_legal_action(&result.best_action));
        assert_eq!(result.actions_considered, 2); // Two gaps
    }

    #[test]
    fn test_solver_handles_large_ring() {
        let state = GameState::new(
            vec![Atom::new(1); 20], // Large ring
            Atom::new(2),
        );

        let solver = Solver::fast(); // Use fast for large ring
        let result = solver.solve(&state).unwrap();

        assert!(state.is_legal_action(&result.best_action));
        assert_eq!(result.actions_considered, 20); // Twenty gaps
    }

    #[test]
    fn test_solver_config_update() {
        let mut solver = Solver::default();
        let new_config = SolverConfig::fast();

        solver.set_config(new_config.clone());

        assert_eq!(solver.config().expectimax_config.max_depth, 1);
    }

    #[test]
    fn test_solve_convenience_function() {
        let state = GameState::new(vec![Atom::new(1), Atom::new(2), Atom::new(3)], Atom::new(1));

        let result = solve(&state).unwrap();

        assert!(state.is_legal_action(&result.best_action));
    }

    #[test]
    fn test_solve_fast_convenience_function() {
        let state = GameState::new(vec![Atom::new(1), Atom::new(2), Atom::new(3)], Atom::new(1));

        let result = solve_fast(&state).unwrap();

        assert!(state.is_legal_action(&result.best_action));
    }
}

#[cfg(test)]
mod depth_comparison_tests {
    use super::*;

    #[test]
    fn test_depth_1_vs_depth_2() {
        let state = GameState::new(
            vec![Atom::new(2), Atom::new(2), Atom::new(3), Atom::new(3)],
            Atom::new(2),
        );

        let config_depth1 = SolverConfig {
            expectimax_config: ExpectimaxConfig {
                max_depth: 1,
                max_spawns_per_node: 3,
            },
            spawn_config: SpawnConfig::uniform(3),
            heuristic_weights: HeuristicWeights::default(),
        };

        let config_depth2 = SolverConfig {
            expectimax_config: ExpectimaxConfig {
                max_depth: 2,
                max_spawns_per_node: 3,
            },
            spawn_config: SpawnConfig::uniform(3),
            heuristic_weights: HeuristicWeights::default(),
        };

        let solver1 = Solver::new(config_depth1);
        let solver2 = Solver::new(config_depth2);

        let result1 = solver1.solve(&state).unwrap();
        let result2 = solver2.solve(&state).unwrap();

        // Both should return legal actions
        assert!(state.is_legal_action(&result1.best_action));
        assert!(state.is_legal_action(&result2.best_action));

        // Depth 2 should evaluate more nodes
        assert!(result2.nodes_evaluated >= result1.nodes_evaluated);
    }
}

#[cfg(test)]
mod edge_case_tests {
    use super::*;

    #[test]
    fn test_solver_with_single_gap() {
        let state = GameState::new(vec![Atom::new(1)], Atom::new(2));

        let solver = Solver::default();
        let result = solver.solve(&state).unwrap();

        assert!(state.is_legal_action(&result.best_action));
        assert_eq!(result.actions_considered, 1);
    }

    #[test]
    fn test_solver_with_all_same_values() {
        let state = GameState::new(vec![Atom::new(3); 5], Atom::new(3));

        let solver = Solver::default();
        let result = solver.solve(&state).unwrap();

        // Should handle mergeable ring
        assert!(state.is_legal_action(&result.best_action));
    }

    #[test]
    fn test_solver_with_mixed_special_atoms() {
        let state = GameState::new(
            vec![Atom::new(1), Atom::new(-1), Atom::new(2)],
            Atom::new(3),
        );

        let solver = Solver::default();
        let result = solver.solve(&state).unwrap();

        assert!(state.is_legal_action(&result.best_action));
    }

    #[test]
    fn test_solver_high_value_atoms() {
        let state = GameState::new(
            vec![Atom::new(10), Atom::new(11), Atom::new(12)],
            Atom::new(10),
        );

        let solver = Solver::default();
        let result = solver.solve(&state).unwrap();

        assert!(state.is_legal_action(&result.best_action));
    }
}

#[cfg(test)]
mod scenario_tests {
    use super::*;

    #[test]
    fn test_obvious_merge_scenario() {
        // Clear scenario: inserting 3 between two 3s should be preferred
        let state = GameState::new(vec![Atom::new(3), Atom::new(1), Atom::new(3)], Atom::new(3));

        let solver = Solver::default();
        let result = solver.solve(&state).unwrap();

        assert!(state.is_legal_action(&result.best_action));
        // Should have positive expected value due to merge potential
        assert!(result.expected_value > 0.0);
    }

    #[test]
    fn test_plus_fusion_scenario() {
        // Plus atom between two equal atoms
        let state = GameState::new(
            vec![Atom::new(5), Atom::new(3), Atom::new(5)],
            Atom::new(-1), // Plus
        );

        let solver = Solver::default();
        let result = solver.solve(&state).unwrap();

        // Should prefer placing plus at position 1 (between the 5s)
        assert!(state.is_legal_action(&result.best_action));
        if let Action::UsePlus { plus_index } = result.best_action {
            // Might choose position 1 to fuse the 5s
            assert!(plus_index < state.ring_size());
        } else {
            panic!("Expected UsePlus action");
        }
    }

    #[test]
    fn test_minus_removal_scenario() {
        // Minus atom with various targets
        let state = GameState::new(
            vec![Atom::new(1), Atom::new(10), Atom::new(2)],
            Atom::new(-2), // Minus
        );

        let solver = Solver::default();
        let result = solver.solve(&state).unwrap();

        assert!(state.is_legal_action(&result.best_action));
        assert!(matches!(result.best_action, Action::UseMinus { .. }));
    }

    #[test]
    fn test_multiple_solver_calls() {
        let mut state =
            GameState::new(vec![Atom::new(1), Atom::new(2), Atom::new(3)], Atom::new(1));

        let solver = Solver::default();

        // Simulate multiple turns
        for _ in 0..3 {
            let result = solver.solve(&state).unwrap();
            assert!(state.is_legal_action(&result.best_action));

            // Apply the action and continue
            state = state.apply_action(&result.best_action).unwrap();
            state.player_atom = Atom::new(2); // Mock new spawn
        }
    }
}
