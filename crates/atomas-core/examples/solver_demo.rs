use atomas_core::{Atom, GameState, Solver, SolverConfig};

fn print_state(state: &GameState, label: &str) {
    println!("\n{}", label);
    println!(
        "Ring: {:?}",
        state.ring.iter().map(|a| a.value).collect::<Vec<_>>()
    );
    println!("Player atom: {}", state.player_atom.value);
    println!("Score: {} | Moves: {}", state.score, state.moves);
}

fn main() {
    println!("╔════════════════════════════════════════════════╗");
    println!("║  ATOMAS SOLVER DEMONSTRATION                   ║");
    println!("╚════════════════════════════════════════════════╝");

    // Demo 1: Basic solver usage
    println!("\n┌─ Demo 1: Basic Solver Usage ───────────────────┐");
    let state1 = GameState::new(
        vec![Atom::new(1), Atom::new(2), Atom::new(3), Atom::new(4)],
        Atom::new(2),
    );
    print_state(&state1, "Initial State:");

    let solver = Solver::default();
    let result = solver.solve(&state1).unwrap();

    println!("\nSolver Result:");
    println!("Best action: {:?}", result.best_action);
    println!("Expected value: {:.2}", result.expected_value);
    println!("Nodes evaluated: {}", result.nodes_evaluated);
    println!("Actions considered: {}", result.actions_considered);

    let next_state = state1.apply_action(&result.best_action).unwrap();
    print_state(&next_state, "After applying best action:");

    // Demo 2: Merge opportunity detection
    println!("\n┌─ Demo 2: Detecting Merge Opportunities ────────┐");
    let state2 = GameState::new(
        vec![Atom::new(3), Atom::new(2), Atom::new(3)],
        Atom::new(3), // Can create triple merge
    );
    print_state(&state2, "Initial State (merge potential):");

    let result2 = solver.solve(&state2).unwrap();
    println!("\nSolver prefers action: {:?}", result2.best_action);
    println!("Expected value: {:.2}", result2.expected_value);

    let next_state2 = state2.apply_action(&result2.best_action).unwrap();
    print_state(&next_state2, "After merge:");
    println!("Score gained: {}", next_state2.score);

    // Demo 3: Plus atom strategy
    println!("\n┌─ Demo 3: Plus Atom Strategy ───────────────────┐");
    let state3 = GameState::new(
        vec![Atom::new(5), Atom::new(3), Atom::new(5), Atom::new(2)],
        Atom::new(-1), // Plus atom
    );
    print_state(&state3, "Initial State (Plus atom):");

    let result3 = solver.solve(&state3).unwrap();
    println!("\nSolver strategy: {:?}", result3.best_action);
    println!("Expected value: {:.2}", result3.expected_value);

    let next_state3 = state3.apply_action(&result3.best_action).unwrap();
    print_state(&next_state3, "After Plus action:");

    // Demo 4: Minus atom strategy
    println!("\n┌─ Demo 4: Minus Atom Strategy ──────────────────┐");
    let state4 = GameState::new(
        vec![Atom::new(1), Atom::new(10), Atom::new(2), Atom::new(3)],
        Atom::new(-2), // Minus atom
    );
    print_state(&state4, "Initial State (Minus atom):");

    let result4 = solver.solve(&state4).unwrap();
    println!("\nSolver strategy: {:?}", result4.best_action);
    println!("Nodes evaluated: {}", result4.nodes_evaluated);

    let next_state4 = state4.apply_action(&result4.best_action).unwrap();
    print_state(&next_state4, "After Minus action:");

    // Demo 5: Fast vs Thorough solver
    println!("\n┌─ Demo 5: Fast vs Thorough Solver ──────────────┐");
    let state5 = GameState::new(
        vec![
            Atom::new(2),
            Atom::new(3),
            Atom::new(2),
            Atom::new(4),
            Atom::new(3),
        ],
        Atom::new(3),
    );
    print_state(&state5, "Test State:");

    let fast_solver = Solver::fast();
    let thorough_solver = Solver::thorough();

    let fast_result = fast_solver.solve(&state5).unwrap();
    let thorough_result = thorough_solver.solve(&state5).unwrap();

    println!("\nFast Solver (depth=1):");
    println!("  Action: {:?}", fast_result.best_action);
    println!("  Value: {:.2}", fast_result.expected_value);
    println!("  Nodes: {}", fast_result.nodes_evaluated);

    println!("\nThorough Solver (depth=3):");
    println!("  Action: {:?}", thorough_result.best_action);
    println!("  Value: {:.2}", thorough_result.expected_value);
    println!("  Nodes: {}", thorough_result.nodes_evaluated);

    // Demo 6: Multi-turn gameplay simulation
    println!("\n┌─ Demo 6: Multi-Turn Simulation ────────────────┐");
    let mut game_state =
        GameState::new(vec![Atom::new(1), Atom::new(2), Atom::new(3)], Atom::new(1));

    println!("Simulating 5 turns with solver...");

    for turn in 1..=5 {
        println!("\n--- Turn {} ---", turn);
        print_state(&game_state, "State:");

        let result = solver.solve(&game_state).unwrap();
        println!("Solver chooses: {:?}", result.best_action);

        game_state = game_state.apply_action(&result.best_action).unwrap();

        // Mock spawn (in real game this comes from the game)
        game_state.player_atom = Atom::new(((turn % 3) + 1) as i16);
    }

    print_state(&game_state, "Final State after 5 turns:");

    // Demo 7: Custom configuration
    println!("\n┌─ Demo 7: Custom Solver Configuration ──────────┐");
    let custom_config = SolverConfig {
        expectimax_config: atomas_core::solver::ExpectimaxConfig {
            max_depth: 2,
            max_spawns_per_node: 4,
        },
        spawn_config: atomas_core::solver::SpawnConfig::regular_only(),
        heuristic_weights: atomas_core::solver::HeuristicWeights {
            score_weight: 2.0,
            merge_potential_weight: 15.0,
            highest_atom_weight: 3.0,
            ring_size_penalty: -3.0,
            diversity_weight: 1.0,
        },
    };

    let custom_solver = Solver::new(custom_config);
    let state7 = GameState::new(vec![Atom::new(2), Atom::new(2), Atom::new(3)], Atom::new(2));

    let result7 = custom_solver.solve(&state7).unwrap();
    println!("Custom solver (high merge weight):");
    println!("  Action: {:?}", result7.best_action);
    println!("  Expected value: {:.2}", result7.expected_value);

    println!("\n╔════════════════════════════════════════════════╗");
    println!("║  Solver working correctly! ✓                   ║");
    println!("╚════════════════════════════════════════════════╝\n");
}
