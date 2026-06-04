use crate::Atom;
use serde::{Deserialize, Serialize};

/// Configuration for atom spawn probabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpawnConfig {
    /// Probability weights for regular atoms (value 1-10)
    pub regular_weights: Vec<f64>,
    /// Probability of spawning a Plus atom
    pub plus_probability: f64,
    /// Probability of spawning a Minus atom
    pub minus_probability: f64,
}

impl Default for SpawnConfig {
    fn default() -> Self {
        // Default spawn distribution based on typical Atomas gameplay
        // Lower atoms are more common, higher atoms are rarer
        Self {
            regular_weights: vec![
                0.25, // 1 (H)  - 25%
                0.20, // 2 (He) - 20%
                0.15, // 3 (Li) - 15%
                0.12, // 4 (Be) - 12%
                0.10, // 5 (B)  - 10%
                0.08, // 6 (C)  - 8%
                0.05, // 7 (N)  - 5%
                0.03, // 8 (O)  - 3%
                0.01, // 9 (F)  - 1%
                0.01, // 10 (Ne) - 1%
            ],
            plus_probability: 0.05,  // 5% chance of Plus
            minus_probability: 0.03, // 3% chance of Minus
        }
    }
}

impl SpawnConfig {
    /// Create a simple uniform distribution for testing
    pub fn uniform(max_value: usize) -> Self {
        let weight = 1.0 / max_value as f64;
        Self {
            regular_weights: vec![weight; max_value],
            plus_probability: 0.0,
            minus_probability: 0.0,
        }
    }

    /// Create a distribution with no special atoms
    pub fn regular_only() -> Self {
        let mut config = Self::default();
        config.plus_probability = 0.0;
        config.minus_probability = 0.0;
        config
    }

    /// Get all possible spawns with their probabilities
    pub fn get_spawn_distribution(&self) -> Vec<(Atom, f64)> {
        let mut distribution = Vec::new();

        // Normalize regular weights
        let regular_total: f64 = self.regular_weights.iter().sum();
        let special_total = self.plus_probability + self.minus_probability;
        let total = regular_total + special_total;

        // Add regular atoms
        for (i, &weight) in self.regular_weights.iter().enumerate() {
            let value = (i + 1) as i16;
            let probability = weight / total;
            if probability > 0.0 {
                distribution.push((Atom::new(value), probability));
            }
        }

        // Add Plus atom
        if self.plus_probability > 0.0 {
            distribution.push((Atom::new(-1), self.plus_probability / total));
        }

        // Add Minus atom
        if self.minus_probability > 0.0 {
            distribution.push((Atom::new(-2), self.minus_probability / total));
        }

        distribution
    }

    /// Get the probability of spawning a specific atom
    pub fn get_probability(&self, atom: &Atom) -> f64 {
        if atom.is_plus() {
            return self.plus_probability;
        }
        if atom.is_minus() {
            return self.minus_probability;
        }

        if atom.value > 0 && (atom.value as usize) <= self.regular_weights.len() {
            return self.regular_weights[(atom.value - 1) as usize];
        }

        0.0
    }

    /// Sample the most likely spawns up to a count limit
    pub fn sample_likely_spawns(&self, max_samples: usize) -> Vec<(Atom, f64)> {
        let mut distribution = self.get_spawn_distribution();
        distribution.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        distribution.truncate(max_samples);
        distribution
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_spawn_config() {
        let config = SpawnConfig::default();
        assert_eq!(config.regular_weights.len(), 10);
        assert!(config.plus_probability > 0.0);
        assert!(config.minus_probability > 0.0);
    }

    #[test]
    fn test_spawn_distribution_sums_to_one() {
        let config = SpawnConfig::default();
        let distribution = config.get_spawn_distribution();
        let total: f64 = distribution.iter().map(|(_, p)| p).sum();
        assert!(
            (total - 1.0).abs() < 1e-6,
            "Total probability should be ~1.0, got {}",
            total
        );
    }

    #[test]
    fn test_uniform_spawn() {
        let config = SpawnConfig::uniform(5);
        let distribution = config.get_spawn_distribution();
        assert_eq!(distribution.len(), 5);
        for (_, prob) in distribution {
            assert!((prob - 0.2).abs() < 1e-6);
        }
    }

    #[test]
    fn test_regular_only() {
        let config = SpawnConfig::regular_only();
        assert_eq!(config.plus_probability, 0.0);
        assert_eq!(config.minus_probability, 0.0);
    }

    #[test]
    fn test_sample_likely_spawns() {
        let config = SpawnConfig::default();
        let samples = config.sample_likely_spawns(3);
        assert_eq!(samples.len(), 3);
        // Should be sorted by probability descending
        assert!(samples[0].1 >= samples[1].1);
        assert!(samples[1].1 >= samples[2].1);
    }
}
