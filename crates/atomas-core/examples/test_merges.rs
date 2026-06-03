use atomas_core::{Atom, GameState, Action};

fn main() {
    // Test 1: cascade_stops_when_no_more_matches
    println!("=== Test 1: cascade_stops_when_no_more_matches ===");
    let state = GameState {
        ring: vec![Atom::new(2), Atom::new(2), Atom::new(5)],
        player_atom: Atom::new(2),
        score: 0,
        moves: 0,
    };
    println!("Initial ring: {:?}", state.ring);
    println!("Insert 2 at gap_index 1 (after position 1, between 2 and 5)");

    let action = Action::Insert { gap_index: 1 };
    let result = state.apply_action(&action).unwrap();

    println!("Final ring: {:?}", result.ring);
    println!("Ring size: {}", result.ring_size());
    println!("Score: {}", result.score);
    println!("Expected: Should have Atom(3) and Atom(5), size 2");

    // Test 2: ring_order_wraparound
    println!("\n=== Test 2: ring_order_wraparound ===");
    let state2 = GameState {
        ring: vec![Atom::new(1), Atom::new(2), Atom::new(3)],
        player_atom: Atom::new(1),
        score: 0,
        moves: 0,
    };
    println!("Initial ring: {:?}", state2.ring);
    println!("Insert 1 at gap_index 2 (after position 2, which is at the end)");

    let action2 = Action::Insert { gap_index: 2 };
    let result2 = state2.apply_action(&action2).unwrap();

    println!("Final ring: {:?}", result2.ring);
    println!("Ring size: {}", result2.ring_size());
    println!("Expected: size 4");
}
