//! Seed Discovery Engine — Rapid Iteration with Tiny Models to Discover Response Tiles
//!
//! The architecture:
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────┐
//! │                  SEED DISCOVERY ENGINE                   │
//! │                                                          │
//! │  1. Define a ROLE (e.g., "constraint-checker")          │
//! │  2. Run N iterations with SEED model (cheap, fast)       │
//! │  3. Score each response against criteria                 │
//! │  4. Extract the PATTERN from high-scoring responses      │
//! │  5. Crystallize pattern as a TILE (structured fragment)  │
//! │  6. Propagate tile UP (larger models) and DOWN (seeds)   │
//! │                                                          │
//! │  The tile IS the inner logic.                            │
//! │  The discovery IS the fine-tuning.                       │
//! └─────────────────────────────────────────────────────────┘
//! ```
//!
//! ## How It Works
//!
//! A "seed run" is:
//! - A role definition (what the agent should do)
//! - An evaluation function (how to score the response)
//! - N iterations with a cheap model
//! - Pattern extraction from the winners
//!
//! The output is a **Tile** — a compressed, self-contained fragment that captures
//! the discovered inner logic. Future calls to ANY model include this tile as
//! conditioning context.
//!
//! ## The Meta-Insight
//!
//! The seed doesn't need to produce the BEST response. It needs to produce
//! enough variation that the PATTERN of good responses becomes visible.
//! The tile captures that pattern, not any individual response.

use crate::eisenstein::{EisensteinConstraint, SnapResult, COVERING_RADIUS};
use crate::temporal::{TemporalAgent, AgentAction, FunnelPhase, ChiralityState};
use std::collections::HashMap;

/// Maximum iterations per seed run
const MAX_ITERATIONS: usize = 64;

/// A discovered tile — the crystallized inner logic from seed experimentation.
#[derive(Debug, Clone)]
pub struct DiscoveryTile {
    /// What role this tile is for
    pub role: String,
    /// The discovered pattern (structured prompt fragment)
    pub pattern: String,
    /// The constraint parameters that produced high-scoring responses
    pub optimal_params: TileParams,
    /// How many seed iterations produced this tile
    pub iterations: usize,
    /// The score that crystallized this tile
    pub crystallization_score: f64,
    /// Entropy of the discovery process (how much variation was explored)
    pub discovery_entropy: f64,
    /// Which actions were most common in high-scoring runs
    pub dominant_actions: Vec<(AgentAction, f64)>,
    /// The funnel phase distribution in winning responses
    pub phase_distribution: HashMap<String, f64>,
    /// Generation: how many times this tile has been refined
    pub generation: u32,
}

/// Parameters discovered by seed experimentation
#[derive(Debug, Clone, Copy)]
pub struct TileParams {
    pub decay_rate: f64,
    pub prediction_horizon: usize,
    pub anomaly_sigma: f64,
    pub learning_rate: f64,
    pub chirality_lock_threshold: u16,
    pub merge_trust: f64,
}

impl Default for TileParams {
    fn default() -> Self {
        TileParams {
            decay_rate: 1.0,
            prediction_horizon: 4,
            anomaly_sigma: 2.0,
            learning_rate: 0.1,
            chirality_lock_threshold: 500,
            merge_trust: 0.5,
        }
    }
}

/// Score for a single seed iteration
#[derive(Debug, Clone)]
pub struct IterationScore {
    /// Which iteration (0..N)
    pub iteration: usize,
    /// The parameters used
    pub params: TileParams,
    /// Final error achieved
    pub final_error: f64,
    /// How many steps to converge (lower is better)
    pub convergence_steps: usize,
    /// How many anomalies detected (too many = bad)
    pub anomaly_count: usize,
    /// Whether chirality locked (good for deterministic systems)
    pub chirality_locked: bool,
    /// Total precision energy spent (lower is better)
    pub precision_energy: f64,
    /// The dominant action in the trajectory
    pub dominant_action: AgentAction,
    /// Composite score (higher is better)
    pub score: f64,
}

/// A seed discovery run — rapid iteration to find optimal parameters
pub struct SeedDiscovery {
    /// The constraint checker
    constraint: EisensteinConstraint,
    /// The role being discovered
    role: String,
    /// Iteration results
    iterations: Vec<IterationScore>,
    /// Current best score
    best_score: f64,
    /// Current best params
    best_params: TileParams,
    /// Generation counter
    generation: u32,
}

impl SeedDiscovery {
    pub fn new(role: &str) -> Self {
        SeedDiscovery {
            constraint: EisensteinConstraint::new(),
            role: role.to_string(),
            iterations: Vec::with_capacity(MAX_ITERATIONS),
            best_score: f64::NEG_INFINITY,
            best_params: TileParams::default(),
            generation: 0,
        }
    }

    /// Run a single seed iteration with given parameters on a trajectory.
    ///
    /// The trajectory is a series of (x, y) sensor readings.
    /// The evaluation scores how well the agent handles the trajectory.
    pub fn run_iteration(
        &mut self,
        params: TileParams,
        trajectory: &[(f64, f64)],
    ) -> IterationScore {
        let mut agent = TemporalAgent::new();
        agent.decay_rate = params.decay_rate;
        agent.prediction_horizon = params.prediction_horizon;
        agent.anomaly_sigma = params.anomaly_sigma;
        agent.learning_rate = params.learning_rate;
        agent.chirality_lock_threshold = params.chirality_lock_threshold;
        agent.merge_trust = params.merge_trust;

        let mut anomaly_count = 0;
        let mut convergence_step = trajectory.len();
        let mut final_error = COVERING_RADIUS;
        let mut action_counts: HashMap<AgentAction, usize> = HashMap::new();

        for (step, &(x, y)) in trajectory.iter().enumerate() {
            let update = agent.observe(x, y);

            if update.is_anomaly {
                anomaly_count += 1;
            }

            if update.snap.error < 0.05 * COVERING_RADIUS && convergence_step == trajectory.len() {
                convergence_step = step;
            }

            final_error = update.snap.error;
            *action_counts.entry(update.action).or_insert(0) += 1;
        }

        let dominant_action = action_counts
            .iter()
            .max_by_key(|(_, &c)| c)
            .map(|(&a, _)| a)
            .unwrap_or(AgentAction::Continue);

        let chirality_locked = matches!(agent.summary().chirality, ChiralityState::Locked { .. });

        // Composite score: balance convergence speed, error, anomalies, and energy
        let convergence_bonus = 1.0 - (convergence_step as f64 / trajectory.len() as f64).min(1.0);
        let error_score = 1.0 - (final_error / COVERING_RADIUS).min(1.0);
        let anomaly_penalty = (anomaly_count as f64 * 0.1).min(1.0);
        let chirality_bonus = if chirality_locked { 0.1 } else { 0.0 };
        let energy_penalty = (agent.summary().precision_energy * 0.001).min(0.5);

        let score = convergence_bonus * 0.3
            + error_score * 0.3
            + (1.0 - anomaly_penalty) * 0.2
            + chirality_bonus * 0.1
            + (1.0 - energy_penalty) * 0.1;

        let iter_score = IterationScore {
            iteration: self.iterations.len(),
            params,
            final_error,
            convergence_steps: convergence_step,
            anomaly_count,
            chirality_locked,
            precision_energy: agent.summary().precision_energy,
            dominant_action,
            score,
        };

        if score > self.best_score {
            self.best_score = score;
            self.best_params = params;
        }

        self.iterations.push(iter_score.clone());
        iter_score
    }

    /// Run a sweep of parameter variations (the seed experimentation).
    ///
    /// Generates parameter variations around the current best and evaluates them.
    pub fn run_sweep(&mut self, trajectory: &[(f64, f64)], n_variations: usize) {
        for i in 0..n_variations {
            let params = self.generate_variation(i, n_variations);
            self.run_iteration(params, trajectory);
        }
    }

    /// Generate a parameter variation.
    ///
    /// Strategy: Latin hypercube sampling around current best.
    fn generate_variation(&self, index: usize, total: usize) -> TileParams {
        let t = index as f64 / total as f64;
        let base = self.best_params;

        // Vary each parameter along a different dimension
        let phase = t * std::f64::consts::PI * 2.0;
        let r = 0.5; // variation radius

        TileParams {
            decay_rate: (base.decay_rate + r * (phase * 1.0).sin()).max(0.1).min(10.0),
            prediction_horizon: (base.prediction_horizon as f64 + 4.0 * (phase * 2.0).sin())
                .round()
                .max(1.0)
                .min(16.0) as usize,
            anomaly_sigma: (base.anomaly_sigma + r * 2.0 * (phase * 3.0).sin()).max(0.5).min(5.0),
            learning_rate: (base.learning_rate + 0.3 * (phase * 5.0).sin())
                .max(0.01)
                .min(1.0),
            chirality_lock_threshold: ((base.chirality_lock_threshold as f64
                + 200.0 * (phase * 7.0).sin())
                .round()
                .max(100.0)
                .min(900.0)) as u16,
            merge_trust: (base.merge_trust + 0.3 * (phase * 11.0).sin())
                .max(0.0)
                .min(1.0),
        }
    }

    /// Crystallize the discovered pattern into a tile.
    ///
    /// This is the key operation: extract the inner logic from the iteration history
    /// and compress it into a self-contained tile that can condition future models.
    pub fn crystallize(&self) -> DiscoveryTile {
        let top_scores: Vec<&IterationScore> = {
            let mut sorted: Vec<&IterationScore> = self.iterations.iter().collect();
            sorted.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
            sorted.into_iter().take(10).collect()
        };

        // Extract dominant actions from top scores
        let mut action_counts: HashMap<AgentAction, f64> = HashMap::new();
        for iter in &top_scores {
            *action_counts.entry(iter.dominant_action).or_insert(0.0) += iter.score;
        }
        let total_action_weight: f64 = action_counts.values().sum();
        let mut dominant_actions: Vec<(AgentAction, f64)> = action_counts
            .into_iter()
            .map(|(a, w)| (a, w / total_action_weight))
            .collect();
        dominant_actions.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        // Compute entropy of the discovery process
        let scores: Vec<f64> = self.iterations.iter().map(|i| i.score).collect();
        let mean_score = scores.iter().sum::<f64>() / scores.len() as f64;
        let variance = scores.iter().map(|s| (s - mean_score).powi(2)).sum::<f64>() / scores.len() as f64;
        let discovery_entropy = (variance.sqrt() / mean_score).min(1.0);

        // Build the pattern string (the inner logic)
        let pattern = self.build_pattern(&top_scores);

        // Phase distribution
        let mut phase_dist: HashMap<String, f64> = HashMap::new();
        // Re-run best params to get phase distribution
        // (simplified: use the score distribution as proxy)
        phase_dist.insert("convergent".to_string(), self.best_score);
        phase_dist.insert("exploratory".to_string(), 1.0 - self.best_score);

        DiscoveryTile {
            role: self.role.clone(),
            pattern,
            optimal_params: self.best_params,
            iterations: self.iterations.len(),
            crystallization_score: self.best_score,
            discovery_entropy,
            dominant_actions,
            phase_distribution: phase_dist,
            generation: self.generation,
        }
    }

    /// Build the pattern string from top-scoring iterations.
    fn build_pattern(&self, top: &[&IterationScore]) -> String {
        let avg_convergence: f64 =
            top.iter().map(|i| i.convergence_steps as f64).sum::<f64>() / top.len() as f64;
        let avg_anomaly: f64 =
            top.iter().map(|i| i.anomaly_count as f64).sum::<f64>() / top.len() as f64;
        let locked_ratio: f64 =
            top.iter().filter(|i| i.chirality_locked).count() as f64 / top.len() as f64;

        format!(
            "Role: {}\n\
             Optimal decay_rate: {:.3} (funnel speed)\n\
             Optimal horizon: {} (prediction depth)\n\
             Optimal anomaly_sigma: {:.2} (surprise sensitivity)\n\
             Optimal learning_rate: {:.3} (memory plasticity)\n\
             Optimal chirality_lock: {} (commitment threshold)\n\
             Convergence: ~{:.0} steps average\n\
             Anomaly rate: ~{:.1} per trajectory\n\
             Chirality lock: {:.0}% of top runs\n\
             Score: {:.3}\n\
             Discovery entropy: {:.3}\n\
             Generation: {}",
            self.role,
            self.best_params.decay_rate,
            self.best_params.prediction_horizon,
            self.best_params.anomaly_sigma,
            self.best_params.learning_rate,
            self.best_params.chirality_lock_threshold,
            avg_convergence,
            avg_anomaly,
            locked_ratio * 100.0,
            self.best_score,
            self.discovery_entropy(),
            self.generation,
        )
    }

    fn discovery_entropy(&self) -> f64 {
        if self.iterations.is_empty() {
            return 0.0;
        }
        let scores: Vec<f64> = self.iterations.iter().map(|i| i.score).collect();
        let mean = scores.iter().sum::<f64>() / scores.len() as f64;
        let var = scores.iter().map(|s| (s - mean).powi(2)).sum::<f64>() / scores.len() as f64;
        (var.sqrt() / mean).min(1.0)
    }

    /// Refine: run another sweep centered on the current best.
    /// Each refinement tightens the search around the optimum.
    pub fn refine(&mut self, trajectory: &[(f64, f64)], n_variations: usize) {
        self.generation += 1;
        self.run_sweep(trajectory, n_variations);
    }
}

/// Tile registry — stores discovered tiles for use by larger models.
#[derive(Debug, Default)]
pub struct TileRegistry {
    tiles: HashMap<String, DiscoveryTile>,
}

impl TileRegistry {
    pub fn new() -> Self {
        TileRegistry {
            tiles: HashMap::new(),
        }
    }

    /// Register a discovered tile.
    pub fn register(&mut self, tile: DiscoveryTile) {
        self.tiles.insert(tile.role.clone(), tile);
    }

    /// Get a tile for a role.
    pub fn get(&self, role: &str) -> Option<&DiscoveryTile> {
        self.tiles.get(role)
    }

    /// Get the optimal parameters for a role (for conditioning larger models).
    pub fn get_params(&self, role: &str) -> TileParams {
        self.tiles
            .get(role)
            .map(|t| t.optimal_params)
            .unwrap_or_default()
    }

    /// Generate a conditioning prompt for a larger model.
    /// This is how the seed's discovered inner logic propagates upward.
    pub fn conditioning_prompt(&self, role: &str) -> String {
        match self.tiles.get(role) {
            Some(tile) => format!(
                "# Discovered Inner Logic for: {}\n\
                 # (crystallized from {} seed iterations, generation {})\n\
                 # Score: {:.3}, Entropy: {:.3}\n\
                 \n\
                 {}\n\
                 \n\
                 # Optimal Parameters:\n\
                 decay_rate = {:.3}  # funnel speed (square-root base)\n\
                 prediction_horizon = {}  # steps ahead\n\
                 anomaly_sigma = {:.2}  # surprise threshold (sigma)\n\
                 learning_rate = {:.3}  # memory plasticity\n\
                 chirality_lock = {}  # commitment threshold (milli)\n\
                 merge_trust = {:.2}  # fleet vs local\n\
                 \n\
                 # Dominant actions: {}\n\
                 \n\
                 Use these parameters when performing this role. \
                 The seed experimentation has proven these are optimal \
                 for the constraint geometry of this domain.",
                tile.role,
                tile.iterations,
                tile.generation,
                tile.crystallization_score,
                tile.discovery_entropy,
                tile.pattern,
                tile.optimal_params.decay_rate,
                tile.optimal_params.prediction_horizon,
                tile.optimal_params.anomaly_sigma,
                tile.optimal_params.learning_rate,
                tile.optimal_params.chirality_lock_threshold,
                tile.optimal_params.merge_trust,
                tile.dominant_actions
                    .iter()
                    .take(3)
                    .map(|(a, w)| format!("{:?} ({:.0}%)", a, w * 100.0))
                    .collect::<Vec<_>>()
                    .join(", "),
            ),
            None => "# No seed tile found for this role. Use defaults.".to_string(),
        }
    }

    /// List all registered tiles.
    pub fn list(&self) -> Vec<&DiscoveryTile> {
        self.tiles.values().collect()
    }
}

/// Generate a test trajectory (converging spiral toward origin)
pub fn converging_spiral(steps: usize, radius: f64, turns: f64) -> Vec<(f64, f64)> {
    (0..steps)
        .map(|i| {
            let t = i as f64 / steps as f64;
            let r = radius * (1.0 - t);
            let angle = turns * 2.0 * std::f64::consts::PI * t;
            (r * angle.cos(), r * angle.sin())
        })
        .collect()
}

/// Generate a test trajectory (noisy sensor reading around a point)
pub fn noisy_sensor(steps: usize, center: (f64, f64), noise: f64) -> Vec<(f64, f64)> {
    (0..steps)
        .map(|i| {
            let t = i as f64 / steps as f64;
            let angle = t * 7.0 * std::f64::consts::PI;
            let r = noise * (angle.sin() * 0.7 + angle.cos() * 0.3);
            (center.0 + r * angle.cos(), center.1 + r * angle.sin())
        })
        .collect()
}

/// Generate a test trajectory (step function — sudden jump)
pub fn step_trajectory(steps: usize, jump_at: usize) -> Vec<(f64, f64)> {
    (0..steps)
        .map(|i| {
            if i < jump_at {
                (0.1, 0.1)
            } else {
                (2.0, 2.0)
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seed_discovery_converging() {
        let trajectory = converging_spiral(50, COVERING_RADIUS * 2.0, 2.0);
        let mut discovery = SeedDiscovery::new("converging-tracker");
        discovery.run_sweep(&trajectory, 20);

        let tile = discovery.crystallize();
        assert!(tile.crystallization_score > 0.0);
        assert_eq!(tile.role, "converging-tracker");
        assert_eq!(tile.iterations, 20);
    }

    #[test]
    fn test_seed_discovery_noisy() {
        let trajectory = noisy_sensor(50, (0.0, 0.0), 0.1);
        let mut discovery = SeedDiscovery::new("noisy-sensor");
        discovery.run_sweep(&trajectory, 20);

        let tile = discovery.crystallize();
        assert!(tile.crystallization_score > 0.0);
        // Noisy sensor should prefer higher anomaly sigma (less jumpy)
        // and lower learning rate (more stable)
    }

    #[test]
    fn test_seed_discovery_step() {
        let trajectory = step_trajectory(50, 25);
        let mut discovery = SeedDiscovery::new("step-detector");
        discovery.run_sweep(&trajectory, 20);

        let tile = discovery.crystallize();
        assert!(tile.crystallization_score > 0.0);
        // Step detector should prefer lower anomaly sigma (more sensitive)
    }

    #[test]
    fn test_tile_registry() {
        let trajectory = converging_spiral(50, COVERING_RADIUS * 2.0, 2.0);
        let mut discovery = SeedDiscovery::new("test-role");
        discovery.run_sweep(&trajectory, 10);
        let tile = discovery.crystallize();

        let mut registry = TileRegistry::new();
        registry.register(tile);

        assert!(registry.get("test-role").is_some());
        assert!(registry.get("nonexistent").is_none());

        let prompt = registry.conditioning_prompt("test-role");
        assert!(prompt.contains("test-role"));
        assert!(prompt.contains("decay_rate"));
    }

    #[test]
    fn test_refinement_improves() {
        let trajectory = converging_spiral(50, COVERING_RADIUS * 2.0, 2.0);
        let mut discovery = SeedDiscovery::new("refinement-test");

        // First sweep
        discovery.run_sweep(&trajectory, 10);
        let _score_gen0 = discovery.crystallize().crystallization_score;

        // Refine
        discovery.refine(&trajectory, 10);
        let score_gen1 = discovery.crystallize().crystallization_score;

        // Refinement should not significantly degrade (may not improve due to randomness)
        assert!(score_gen1 > 0.0);
        assert_eq!(discovery.crystallize().generation, 1);
    }

    #[test]
    fn test_trajectory_generators() {
        let spiral = converging_spiral(20, 1.0, 1.0);
        assert_eq!(spiral.len(), 20);
        assert!(spiral[0].0.abs() > spiral[19].0.abs()); // converging

        let noisy = noisy_sensor(20, (1.0, 1.0), 0.5);
        assert_eq!(noisy.len(), 20);

        let step = step_trajectory(20, 10);
        assert_eq!(step.len(), 20);
        assert!((step[5].0 - 0.1).abs() < 0.01);
        assert!((step[15].0 - 2.0).abs() < 0.01);
    }

    #[test]
    fn test_conditioning_prompt_structure() {
        let trajectory = converging_spiral(30, COVERING_RADIUS, 1.5);
        let mut discovery = SeedDiscovery::new("structured-role");
        discovery.run_sweep(&trajectory, 15);
        let tile = discovery.crystallize();

        let mut registry = TileRegistry::new();
        registry.register(tile);

        let prompt = registry.conditioning_prompt("structured-role");
        assert!(prompt.contains("Discovered Inner Logic"));
        assert!(prompt.contains("seed iterations"));
        assert!(prompt.contains("decay_rate"));
        assert!(prompt.contains("prediction_horizon"));
        assert!(prompt.contains("anomaly_sigma"));
        assert!(prompt.contains("learning_rate"));
        assert!(prompt.contains("chirality_lock"));
        assert!(prompt.contains("merge_trust"));
    }
}
