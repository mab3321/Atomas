use super::heuristic::{HeuristicWeights, evaluate_state};
use super::movegen::generate_legal_actions;
use super::spawn::SpawnConfig;
use crate::{Action, GameState};

/// Expectimax search result
#[derive(Debug, Clone)]
pub struct ExpectimaxResult {
    pub best_action: Action,
    pub expected_value: f64,
    pub nodes_evaluated: usize,
}

/// Expectimax search configuration
#[derive(Debug, Clone)]
pub struct ExpectimaxConfig {
    pub max_depth: usize,
    pub max_spawns_per_node: usize,
}

impl Default for ExpectimaxConfig {
    fn default() -> Self {
        Self {
            max_depth: 2,
            max_spawns_per_node: 5, // Consider top 5 most likely spawns
        }
    }
}

/// Perform expectimax search to find the best action
pub fn expectimax_search(
    state: &GameState,
    spawn_config: &SpawnConfig,
    heuristic_weights: &HeuristicWeights,
    config: &ExpectimaxConfig,
) -> Result<ExpectimaxResult, String> {
    let actions = generate_legal_actions(state);

    if actions.is_empty() {
        return Err("No legal actions available".to_string());
    }

    let mut best_action = actions[0];
    let mut best_value = f64::NEG_INFINITY;
    let mut nodes_evaluated = 0;

    // Evaluate each action
    for action in &actions {
        let (value, nodes) =
            expectimax_max_node(state, action, spawn_config, heuristic_weights, config, 0)?;

        nodes_evaluated += nodes;

        if value > best_value {
            best_value = value;
            best_action = *action;
        }
    }

    Ok(ExpectimaxResult {
        best_action,
        expected_value: best_value,
        nodes_evaluated,
    })
}

/// Max node: evaluate the expected value of taking an action
fn expectimax_max_node(
    state: &GameState,
    action: &Action,
    spawn_config: &SpawnConfig,
    heuristic_weights: &HeuristicWeights,
    config: &ExpectimaxConfig,
    depth: usize,
) -> Result<(f64, usize), String> {
    // Apply the action
    let next_state = state.apply_action(action)?;
    let mut nodes_evaluated = 1;

    // If we've reached max depth, evaluate with heuristic
    if depth >= config.max_depth {
        let value = evaluate_state(&next_state, heuristic_weights);
        return Ok((value, nodes_evaluated));
    }

    // Get immediate score gain
    let immediate_value = (next_state.score - state.score) as f64;

    // Chance node: evaluate expected value over possible spawns
    let (expected_future_value, spawn_nodes) = expectimax_chance_node(
        &next_state,
        spawn_config,
        heuristic_weights,
        config,
        depth + 1,
    )?;

    nodes_evaluated += spawn_nodes;

    // Combine immediate and expected future value
    let total_value = immediate_value * 2.0 + expected_future_value * 0.8;

    Ok((total_value, nodes_evaluated))
}

/// Chance node: compute expected value over possible spawns
fn expectimax_chance_node(
    state: &GameState,
    spawn_config: &SpawnConfig,
    heuristic_weights: &HeuristicWeights,
    config: &ExpectimaxConfig,
    depth: usize,
) -> Result<(f64, usize), String> {
    // Sample likely spawns
    let spawns = spawn_config.sample_likely_spawns(config.max_spawns_per_node);

    if spawns.is_empty() {
        // No spawns configured, just evaluate current state
        let value = evaluate_state(state, heuristic_weights);
        return Ok((value, 0));
    }

    let mut expected_value = 0.0;
    let mut nodes_evaluated = 0;

    // Evaluate each possible spawn
    for (spawn_atom, probability) in spawns {
        let spawn_state = GameState {
            ring: state.ring.clone(),
            player_atom: spawn_atom,
            score: state.score,
            moves: state.moves,
        };

        // Recursively evaluate best action from this spawn state
        let (value, nodes) = evaluate_best_action_value(
            &spawn_state,
            spawn_config,
            heuristic_weights,
            config,
            depth,
        )?;

        nodes_evaluated += nodes;
        expected_value += probability * value;
    }

    Ok((expected_value, nodes_evaluated))
}

/// Evaluate the value of the best action from a state
fn evaluate_best_action_value(
    state: &GameState,
    spawn_config: &SpawnConfig,
    heuristic_weights: &HeuristicWeights,
    config: &ExpectimaxConfig,
    depth: usize,
) -> Result<(f64, usize), String> {
    // If at max depth, just use heuristic
    if depth >= config.max_depth {
        let value = evaluate_state(state, heuristic_weights);
        return Ok((value, 1));
    }

    let actions = generate_legal_actions(state);

    if actions.is_empty() {
        // No legal actions, evaluate current state
        let value = evaluate_state(state, heuristic_weights);
        return Ok((value, 0));
    }

    let mut best_value = f64::NEG_INFINITY;
    let mut nodes_evaluated = 0;

    // Find the best action value
    for action in &actions {
        let (value, nodes) = expectimax_max_node(
            state,
            action,
            spawn_config,
            heuristic_weights,
            config,
            depth,
        )?;

        nodes_evaluated += nodes;
        best_value = best_value.max(value);
    }

    Ok((best_value, nodes_evaluated))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Atom;

    #[test]
    fn test_expectimax_returns_legal_action() {
        let state = GameState::new(vec![Atom::new(1), Atom::new(2), Atom::new(3)], Atom::new(1));

        let spawn_config = SpawnConfig::uniform(3);
        let heuristic_weights = HeuristicWeights::default();
        let config = ExpectimaxConfig::default();

        let result = expectimax_search(&state, &spawn_config, &heuristic_weights, &config).unwrap();

        assert!(state.is_legal_action(&result.best_action));
        assert!(result.nodes_evaluated > 0);
    }

    #[test]
    fn test_expectimax_depth_1() {
        let state = GameState::new(vec![Atom::new(2), Atom::new(2), Atom::new(1)], Atom::new(2));

        let spawn_config = SpawnConfig::uniform(3);
        let heuristic_weights = HeuristicWeights::default();
        let config = ExpectimaxConfig {
            max_depth: 1,
            max_spawns_per_node: 3,
        };

        let result = expectimax_search(&state, &spawn_config, &heuristic_weights, &config).unwrap();

        assert!(state.is_legal_action(&result.best_action));
    }

    #[test]
    fn test_expectimax_with_plus_atom() {
        let state = GameState::new(
            vec![Atom::new(3), Atom::new(2), Atom::new(3)],
            Atom::new(-1), // Plus
        );

        let spawn_config = SpawnConfig::uniform(3);
        let heuristic_weights = HeuristicWeights::default();
        let config = ExpectimaxConfig::default();

        let result = expectimax_search(&state, &spawn_config, &heuristic_weights, &config).unwrap();

        assert!(state.is_legal_action(&result.best_action));
        assert!(matches!(result.best_action, Action::UsePlus { .. }));
    }

    #[test]
    fn test_expectimax_with_minus_atom() {
        let state = GameState::new(
            vec![Atom::new(1), Atom::new(2), Atom::new(3)],
            Atom::new(-2), // Minus
        );

        let spawn_config = SpawnConfig::uniform(3);
        let heuristic_weights = HeuristicWeights::default();
        let config = ExpectimaxConfig::default();

        let result = expectimax_search(&state, &spawn_config, &heuristic_weights, &config).unwrap();

        assert!(state.is_legal_action(&result.best_action));
        assert!(matches!(result.best_action, Action::UseMinus { .. }));
    }

    #[test]
    fn test_expectimax_deterministic() {
        let state = GameState::new(vec![Atom::new(1), Atom::new(2), Atom::new(3)], Atom::new(1));

        let spawn_config = SpawnConfig::uniform(3);
        let heuristic_weights = HeuristicWeights::default();
        let config = ExpectimaxConfig::default();

        let result1 =
            expectimax_search(&state, &spawn_config, &heuristic_weights, &config).unwrap();
        let result2 =
            expectimax_search(&state, &spawn_config, &heuristic_weights, &config).unwrap();

        assert_eq!(result1.best_action, result2.best_action);
        assert_eq!(result1.expected_value, result2.expected_value);
    }

    #[test]
    fn test_expectimax_handles_small_ring() {
        let state = GameState::new(vec![Atom::new(1), Atom::new(2)], Atom::new(1));

        let spawn_config = SpawnConfig::uniform(2);
        let heuristic_weights = HeuristicWeights::default();
        let config = ExpectimaxConfig::default();

        let result = expectimax_search(&state, &spawn_config, &heuristic_weights, &config).unwrap();

        assert!(state.is_legal_action(&result.best_action));
    }
}
