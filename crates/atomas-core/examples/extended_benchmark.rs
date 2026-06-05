use atomas_core::{Atom, GameState, Solver};
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

/// Aggregate benchmark results
#[derive(Debug)]
struct BenchmarkResults {
    sessions: Vec<GameSession>,
    config_name: String,
}

impl BenchmarkResults {
    fn new(config_name: String) -> Self {
        Self {
            sessions: Vec::new(),
            config_name,
        }
    }

    fn add_session(&mut self, session: GameSession) {
        self.sessions.push(session);
    }

    fn highest_score(&self) -> u64 {
        self.sessions.iter().map(|s| s.final_score).max().unwrap_or(0)
    }

    fn average_score(&self) -> f64 {
        if self.sessions.is_empty() {
            return 0.0;
        }
        let sum: u64 = self.sessions.iter().map(|s| s.final_score).sum();
        sum as f64 / self.sessions.len() as f64
    }

    fn median_score(&self) -> u64 {
        if self.sessions.is_empty() {
            return 0;
        }
        let mut scores: Vec<u64> = self.sessions.iter().map(|s| s.final_score).collect();
        scores.sort_unstable();
        scores[scores.len() / 2]
    }

    fn top_n_sessions(&self, n: usize) -> Vec<&GameSession> {
        let mut sessions: Vec<&GameSession> = self.sessions.iter().collect();
        sessions.sort_by_key(|s| std::cmp::Reverse(s.final_score));
        sessions.into_iter().take(n).collect()
    }

    fn print_summary(&self) {
        println!("\n╔═══════════════════════════════════════════════════╗");
        println!("║  {} RESULTS", self.config_name.to_uppercase());
        println!("╚═══════════════════════════════════════════════════╝");
        println!("Sessions completed: {}", self.sessions.len());
        println!("Highest score:      {}", self.highest_score());
        println!("Average score:      {:.2}", self.average_score());
        println!("Median score:       {}", self.median_score());

        println!("\n┌─ Top 5 Sessions ────────────────────────────────┐");
        for (i, session) in self.top_n_sessions(5).iter().enumerate() {
            println!("{}. Score: {:<6} | Moves: {:<4} | Highest Atom: {:<3} | Ring Size: {}",
                     i + 1, session.final_score, session.moves, session.highest_atom, session.final_ring_size);
        }
        println!("└─────────────────────────────────────────────────┘");
    }
}

/// Spawn a new random atom based on game progression
fn spawn_atom(moves: u32) -> Atom {
    use rand::Rng;
    let mut rng = rand::thread_rng();

    // Difficulty progression: higher atoms appear as game progresses
    let max_value = if moves < 10 {
        3  // Early game: 1-3
    } else if moves < 30 {
        5  // Mid game: 1-5
    } else if moves < 60 {
        7  // Late game: 1-7
    } else {
        10 // End game: 1-10
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

    // Initialize game with a small ring
    let mut game_state = GameState::new(
        vec![Atom::new(1), Atom::new(2), Atom::new(1)],
        Atom::new(2),
    );

    let mut moves_without_growth = 0;

    loop {
        // Check game over conditions
        if game_state.ring_size() >= max_ring_size {
            break; // Ring too large - game over
        }

        if game_state.moves >= max_moves {
            break; // Move limit reached
        }

        if moves_without_growth > 20 {
            break; // Stuck without progress
        }

        // Get solver's best action
        let result = match solver.solve(&game_state) {
            Ok(r) => r,
            Err(_) => break, // No legal moves - game over
        };

        // Apply the action
        let prev_size = game_state.ring_size();
        game_state = match game_state.apply_action(&result.best_action) {
            Ok(state) => state,
            Err(_) => break, // Action failed - game over
        };

        // Spawn next atom
        game_state.player_atom = spawn_atom(game_state.moves);

        // Track progress
        if game_state.ring_size() < prev_size {
            moves_without_growth = 0; // Made progress
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
    println!("║  EXTENDED BENCHMARK - FIND HIGHEST SCORES         ║");
    println!("╚════════════════════════════════════════════════════╝");

    let configurations = vec![
        ("Default Solver (Depth=2)", Solver::default()),
        ("Thorough Solver (Depth=3)", Solver::thorough()),
    ];

    let num_sessions = 50; // Run 50 sessions per configuration
    let max_ring_size = 18;
    let max_moves = 300; // Allow more moves

    let mut all_time_high_score = 0u64;
    let mut all_time_best_config = String::new();
    let mut all_time_best_session: Option<GameSession> = None;

    for (config_name, solver) in configurations {
        println!("\n🎮 Running {} sessions with {}...", num_sessions, config_name);

        let mut results = BenchmarkResults::new(config_name.to_string());

        for session_num in 1..=num_sessions {
            let session = run_game_session(&solver, max_ring_size, max_moves);

            if session_num % 10 == 0 {
                print!("  Progress: {}/{} sessions | Current high: {}\r",
                       session_num, num_sessions, results.highest_score());
            }

            if session.final_score > all_time_high_score {
                all_time_high_score = session.final_score;
                all_time_best_config = config_name.to_string();
                all_time_best_session = Some(session.clone());
            }

            results.add_session(session);
        }
        println!(); // New line after progress

        results.print_summary();
    }

    println!("\n╔════════════════════════════════════════════════════╗");
    println!("║  🏆 ALL-TIME HIGHEST SCORE ACHIEVED 🏆             ║");
    println!("╚════════════════════════════════════════════════════╝");

    if let Some(best) = all_time_best_session {
        println!("Configuration:   {}", all_time_best_config);
        println!("Score:           {}", best.final_score);
        println!("Moves:           {}", best.moves);
        println!("Highest Atom:    {}", best.highest_atom);
        println!("Final Ring Size: {}", best.final_ring_size);
        println!("Duration:        {}ms", best.duration_ms);
    }

    println!("\n╔════════════════════════════════════════════════════╗");
    println!("║  BENCHMARK COMPLETE ✓                              ║");
    println!("╚════════════════════════════════════════════════════╝\n");
}
