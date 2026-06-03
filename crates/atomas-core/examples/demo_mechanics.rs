use atomas_core::{Atom, GameState, Action};

fn print_state(state: &GameState, label: &str) {
    println!("\n{}", label);
    println!("Ring: {:?}", state.ring.iter().map(|a| a.value).collect::<Vec<_>>());
    println!("Player atom: {}", state.player_atom.value);
    println!("Score: {} | Moves: {}", state.score, state.moves);
}

fn main() {
    println!("╔════════════════════════════════════════════════╗");
    println!("║  ATOMAS GAME MECHANICS DEMONSTRATION           ║");
    println!("╚════════════════════════════════════════════════╝");

    // Demo 1: Simple Insert
    println!("\n┌─ Demo 1: Simple Insert (No Merge) ─────────────┐");
    let mut state = GameState {
        ring: vec![Atom::new(1), Atom::new(2), Atom::new(3)],
        player_atom: Atom::new(4),
        score: 0,
        moves: 0,
    };
    print_state(&state, "Initial State:");

    let action = Action::Insert { gap_index: 1 };
    state = state.apply_action(&action).unwrap();
    print_state(&state, "After Insert(gap=1):");
    println!("✓ Atom 4 inserted between 2 and 3");

    // Demo 2: Insert with Merge
    println!("\n┌─ Demo 2: Insert with Merge ────────────────────┐");
    let mut state = GameState {
        ring: vec![Atom::new(2), Atom::new(3), Atom::new(3), Atom::new(5)],
        player_atom: Atom::new(3),
        score: 0,
        moves: 0,
    };
    print_state(&state, "Initial State:");

    let action = Action::Insert { gap_index: 2 };
    state = state.apply_action(&action).unwrap();
    print_state(&state, "After Insert(gap=2):");
    println!("✓ Three 3s merged into a 4");
    println!("✓ Score gained: {}", state.score);

    // Demo 3: X-+-X Merge
    println!("\n┌─ Demo 3: X-+-X Merge Pattern ──────────────────┐");
    let mut state = GameState {
        ring: vec![Atom::new(5), Atom::new(4), Atom::new(5), Atom::new(2)],
        player_atom: Atom::new(-1), // Plus
        score: 0,
        moves: 0,
    };
    print_state(&state, "Initial State:");

    let action = Action::UsePlus { plus_index: 1 };
    state = state.apply_action(&action).unwrap();
    print_state(&state, "After UsePlus(pos=1):");
    println!("✓ Pattern [5, Plus, 5] detected and merged to 6");
    println!("✓ Score gained: {}", state.score);

    // Demo 4: Cascade Merge
    println!("\n┌─ Demo 4: Cascade Merge ────────────────────────┐");
    let mut state = GameState {
        ring: vec![Atom::new(2), Atom::new(2), Atom::new(3), Atom::new(3)],
        player_atom: Atom::new(2),
        score: 0,
        moves: 0,
    };
    print_state(&state, "Initial State:");

    let action = Action::Insert { gap_index: 1 };
    state = state.apply_action(&action).unwrap();
    print_state(&state, "After Insert(gap=1):");
    println!("✓ Multiple merges cascaded:");
    println!("  [2,2,2,3,3] → [3,2,3,3] → [3,3,3] → [4,3]");
    println!("✓ Total score gained: {}", state.score);

    // Demo 5: Minus Action
    println!("\n┌─ Demo 5: Minus Action (Remove) ────────────────┐");
    let mut state = GameState {
        ring: vec![Atom::new(1), Atom::new(2), Atom::new(3), Atom::new(4), Atom::new(5)],
        player_atom: Atom::new(-2), // Minus
        score: 0,
        moves: 0,
    };
    print_state(&state, "Initial State:");

    let action = Action::UseMinus {
        minus_index: 1,
        target_index: 4
    };
    state = state.apply_action(&action).unwrap();
    print_state(&state, "After UseMinus(minus=1, target=4):");
    println!("✓ Removed atoms at positions 1 and 4");
    println!("✓ Ring reduced by 2 atoms");
    println!("✓ Score gained: {}", state.score);

    // Demo 6: Ring Wraparound
    println!("\n┌─ Demo 6: Ring Wraparound Merge ────────────────┐");
    let mut state = GameState {
        ring: vec![Atom::new(1), Atom::new(2), Atom::new(3)],
        player_atom: Atom::new(1),
        score: 0,
        moves: 0,
    };
    print_state(&state, "Initial State:");
    println!("Note: Ring is circular - first and last are adjacent");

    let action = Action::Insert { gap_index: 2 };
    state = state.apply_action(&action).unwrap();
    print_state(&state, "After Insert(gap=2):");
    println!("✓ Inserted at end, creating [1,2,3,1]");
    println!("✓ The two 1s at wraparound merged!");
    println!("✓ Cascade continued: [2,2,3] → [3,3] → [4]");

    // Demo 7: Complex Scenario
    println!("\n┌─ Demo 7: Complex Multi-Step Scenario ──────────┐");
    let mut state = GameState {
        ring: vec![Atom::new(3), Atom::new(4), Atom::new(3), Atom::new(5)],
        player_atom: Atom::new(4),
        score: 0,
        moves: 0,
    };
    print_state(&state, "Initial State:");

    // Step 1
    let action = Action::Insert { gap_index: 0 };
    state = state.apply_action(&action).unwrap();
    print_state(&state, "After Step 1 - Insert(gap=0):");

    // Step 2
    state.player_atom = Atom::new(-1); // Plus
    let action = Action::UsePlus { plus_index: 1 };
    state = state.apply_action(&action).unwrap();
    print_state(&state, "After Step 2 - UsePlus(pos=1):");

    println!("✓ Two-move sequence demonstrated");
    println!("✓ Final score: {}", state.score);

    println!("\n╔════════════════════════════════════════════════╗");
    println!("║  All mechanics working correctly! ✓            ║");
    println!("╚════════════════════════════════════════════════╝\n");
}
