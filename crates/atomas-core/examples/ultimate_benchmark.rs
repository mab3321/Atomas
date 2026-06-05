use atomas_core::{Atom, GameState, Solver, SolverConfig};
use std::time::Instant;

/// Game session statistics
#[derive(Debug, Clone)]
struct GameSession {
    final_score: u64,
    moves: u32,
    highest_atom: i16,
    final_ring_size: usize,
    duration_ms: u128,
}

/// Spawn a new random atom based on game progression
fn spawn_atom(moves: u32) -> Atom {
    use rand::Rng;
    let mut rng = rand::thread_rng();

    // Difficulty progression
    let max_value = if moves < 10 {
        3
    } else if moves < 30 {
        5
    } else if moves < 60 {
        7
    } else {
        10
    };

    // 10% chance of special atom after move 15
    if moves > 15 && rng.r#gen::<f64>() < 0.1 {
        if rng.r#gen::<bool>() {
            return Atom::new(-1); // Plus
        } else {
            return Atom::new(-2); // Minus
        }
    }

    // Regular atom weighted towards lower values
    let roll = rng.r#gen::<f64>();
    let value = if roll < 0.4 {
        1
    } else if roll < 0.7 {
        2
    } else if roll < 0.85 {
        3
    } else if max_value >= 4 {
        rng.r#gen_range(4..=max_value)
    } else {
        max_value
    };

    Atom::new(value)
}

/// Run a single game session until game over
fn run_game_session(solver: &Solver, max_ring_size: usize, max_moves: u32) -> GameSession {
    let start_time = Instant::now();

    let mut game_state = GameState::new(
        vec![Atom::new(1), Atom::new(2), Atom::new(1)],
        Atom::new(2),
    );

    let mut moves_without_growth = 0;

    loop {
        if game_state.ring_size() >= max_ring_size {
            break;
        }

        if game_state.moves >= max_moves {
            break;
        }

        if moves_without_growth > 25 {
            break;
        }

        let result = match solver.solve(&game_state) {
            Ok(r) => r,
            Err(_) => break,
        };

        let prev_size = game_state.ring_size();
        game_state = match game_state.apply_action(&result.best_action) {
            Ok(state) => state,
            Err(_) => break,
        };

        game_state.player_atom = spawn_atom(game_state.moves);

        if game_state.ring_size() < prev_size {
            moves_without_growth = 0;
        } else {
            moves_without_growth += 1;
        }
    }

    let duration = start_time.elapsed();
    let highest_atom = game_state.ring.iter().map(|a| a.value).max().unwrap_or(0);

    GameSession {
        final_score: game_state.score,
        moves: game_state.moves,
        highest_atom,
        final_ring_size: game_state.ring_size(),
        duration_ms: duration.as_millis(),
    }
}

fn main() {
    println!("╔════════════════════════════════════════════════════╗");
    println!("║  🎯 ULTIMATE BENCHMARK - MAXIMUM SCORE HUNT 🎯     ║");
    println!("╚════════════════════════════════════════════════════╝");
    println!("\n🚀 Running 100 sessions to find the HIGHEST possible score!\n");

    // Create an optimized configuration focused on score maximization
    let optimized_config = SolverConfig {
        expectimax_config: atomas_core::solver::ExpectimaxConfig {
            max_depth: 3,
            max_spawns_per_node: 5,
        },
        spawn_config: atomas_core::solver::SpawnConfig::regular_only(),
        heuristic_weights: atomas_core::solver::HeuristicWeights {
            score_weight: 3.0,           // Prioritize immediate score
            merge_potential_weight: 20.0, // Highly value merge setups
            highest_atom_weight: 5.0,     // Build big atoms
            ring_size_penalty: -5.0,      // Avoid filling the ring
            diversity_weight: 0.5,        // Less important
        },
    };

    let solver = Solver::new(optimized_config);
    let num_sessions = 100;
    let max_ring_size = 18;
    let max_moves = 400; // Allow even more moves

    let mut sessions = Vec::new();
    let mut current_high = 0u64;

    for session_num in 1..=num_sessions {
        let session = run_game_session(&solver, max_ring_size, max_moves);

        if session.final_score > current_high {
            current_high = session.final_score;
            println!("  🔥 NEW HIGH SCORE: {} (Session {}/{})", current_high, session_num, num_sessions);
        } else if session_num % 10 == 0 {
            print!("  Progress: {}/{}... Current high: {}\r", session_num, num_sessions, current_high);
            use std::io::Write;
            std::io::stdout().flush().unwrap();
        }

        sessions.push(session);
    }
    println!(); // New line

    // Calculate statistics
    sessions.sort_by_key(|s| std::cmp::Reverse(s.final_score));
    let highest_score = sessions[0].final_score;
    let average: f64 = sessions.iter().map(|s| s.final_score).sum::<u64>() as f64 / sessions.len() as f64;
    let median = sessions[sessions.len() / 2].final_score;

    println!("\n╔════════════════════════════════════════════════════╗");
    println!("║  📊 FINAL STATISTICS                               ║");
    println!("╚════════════════════════════════════════════════════╝");
    println!("Sessions completed: {}", sessions.len());
    println!("Highest score:      {}", highest_score);
    println!("Average score:      {:.2}", average);
    println!("Median score:       {}", median);

    println!("\n╔════════════════════════════════════════════════════╗");
    println!("║  🏆 TOP 10 HIGHEST SCORES ACHIEVED 🏆              ║");
    println!("╚════════════════════════════════════════════════════╝");

    for (i, session) in sessions.iter().take(10).enumerate() {
        println!("{}. Score: {:<6} | Moves: {:<4} | Highest Atom: {:<3} | Ring: {} | Time: {}ms",
                 i + 1, session.final_score, session.moves, session.highest_atom,
                 session.final_ring_size, session.duration_ms);
    }

    println!("\n╔════════════════════════════════════════════════════╗");
    println!("║  🎉 RECORD HOLDER - DETAILED VIEW 🎉               ║");
    println!("╚════════════════════════════════════════════════════╝");
    let best = &sessions[0];
    println!("🥇 HIGHEST SCORE:    {}", best.final_score);
    println!("   Total Moves:      {}", best.moves);
    println!("   Highest Atom:     {}", best.highest_atom);
    println!("   Final Ring Size:  {}", best.final_ring_size);
    println!("   Session Duration: {}ms", best.duration_ms);
    println!("   Avg Score/Move:   {:.2}", best.final_score as f64 / best.moves as f64);

    println!("\n╔════════════════════════════════════════════════════╗");
    println!("║  ✅ ULTIMATE BENCHMARK COMPLETE ✅                 ║");
    println!("╚════════════════════════════════════════════════════╝\n");
}
