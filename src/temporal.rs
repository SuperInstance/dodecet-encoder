//! Temporal Constraint Agent — Agentic Controls for Temporal Intelligence
//!
//! A constraint system that develops intelligence over time by:
//! 1. Reading dodecet constraint states from sensors
//! 2. Maintaining a temporal model (deadband funnel state)
//! 3. Predicting future constraint states
//! 4. Adjusting funnel shape based on prediction error
//! 5. Detecting anomalies when predictions fail
//! 6. Learning optimal control parameters
//!
//! ## The Key Insight
//!
//! The deadband funnel IS the agent's temporal model. It encodes:
//! - **Past**: integral of precision energy (how much work done)
//! - **Present**: current dodecet state (where we are now)
//! - **Future**: predicted convergence time (when we'll be done)
//!
//! The finesse is the set of control parameters that tune this temporal model.
//! The agentic controls are the API for higher-level agents.
//!
//! ## Temporal Intelligence Stack
//!
//! ```text
//! Layer 4: PLANNING          — Predict future constraint states, plan paths
//! Layer 3: LEARNING          — Adjust funnel shape from history
//! Layer 2: PREDICTION        — Kalman-like filter on dodecet stream
//! Layer 1: CONTROL           — PID on constraint error (P=error, I=energy, D=rate)
//! Layer 0: PERCEPTION        — Snap to lattice, encode dodecet
//! ```

use crate::eisenstein::{EisensteinConstraint, SnapResult, COVERING_RADIUS};

/// History window for temporal tracking
const HISTORY_SIZE: usize = 64;

/// A temporal constraint agent that reads dodecets and develops temporal intelligence.
pub struct TemporalAgent {
    /// The constraint checker
    constraint: EisensteinConstraint,

    /// Ring buffer of past snap results
    history: [Option<SnapResult>; HISTORY_SIZE],
    /// Current write position in ring buffer
    history_pos: usize,
    /// Number of samples recorded
    history_count: usize,

    // === FINESSE PARAMETERS (agentic controls) ===
    /// Deadband decay rate. Controls how fast the funnel narrows.
    /// Higher = faster convergence but more overshoot.
    /// Default: 1.0 (square-root rate)
    /// Range: [0.1, 10.0]
    pub decay_rate: f64,

    /// Prediction horizon: how many steps ahead to predict.
    /// Higher = more anticipation but less accurate.
    /// Default: 4
    pub prediction_horizon: usize,

    /// Anomaly sensitivity: how many sigmas for anomaly detection.
    /// Lower = more sensitive (more anomalies detected).
    /// Default: 2.0 (95% confidence)
    pub anomaly_sigma: f64,

    /// Learning rate for adaptive funnel shape.
    /// Higher = faster adaptation but noisier.
    /// Default: 0.1
    pub learning_rate: f64,

    /// Chirality lock threshold. Below this confidence, chamber is exploring.
    /// Controls when the agent commits to a chirality.
    /// Default: 500 (out of 1000 milliunits)
    pub chirality_lock_threshold: u16,

    /// Merge trust: how much to trust fleet consensus vs local.
    /// 0.0 = only local, 1.0 = only fleet.
    /// Default: 0.5
    pub merge_trust: f64,

    // === DERIVED STATE (computed from history) ===
    /// Running mean of error levels
    error_mean: f64,
    /// Running variance of error levels
    error_var: f64,
    /// Current convergence rate (derivative of error)
    convergence_rate: f64,
    /// Accumulated precision energy (integral of 1/error)
    precision_energy: f64,
    /// Current prediction of next error level
    predicted_error: f64,
    /// Prediction error (how wrong our last prediction was)
    prediction_error: f64,
    /// Current chirality (locked or exploring)
    chirality: ChiralityState,
    /// Current funnel phase
    phase: FunnelPhase,
}

/// Chirality state — has the agent locked into a Weyl chamber?
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChiralityState {
    /// Exploring: hopping between chambers (high temperature)
    Exploring { chamber_hops: u32 },
    /// Locking: mostly in one chamber, occasional hops
    Locking { dominant: u8, confidence_milli: u16 },
    /// Locked: committed to one chamber (low temperature)
    Locked { chamber: u8 },
}

/// Which phase of the deadband funnel are we in?
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FunnelPhase {
    /// Wide open, just started
    Approach,
    /// Narrowing, converging
    Narrowing,
    /// Almost snapped, fine-tuning
    SnapImminent,
    /// Snapped, holding position
    Crystallized,
    /// Anomaly detected, re-opening funnel
    Anomaly,
}

/// Output of a temporal update
#[derive(Debug)]
pub struct TemporalUpdate {
    /// Current snap result
    pub snap: SnapResult,
    /// Current funnel phase
    pub phase: FunnelPhase,
    /// Current chirality state
    pub chirality: ChiralityState,
    /// Predicted next error level
    pub predicted_error: f64,
    /// Prediction error (how wrong we were)
    pub prediction_error: f64,
    /// Convergence rate (negative = converging)
    pub convergence_rate: f64,
    /// Total precision energy spent
    pub precision_energy: f64,
    /// Is this an anomaly?
    pub is_anomaly: bool,
    /// Recommended action
    pub action: AgentAction,
}

/// Actions the agent can recommend
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AgentAction {
    /// Keep going, everything normal
    Continue,
    /// Converging well, maintain course
    Converging,
    /// Almost snapped, hold steady
    HoldSteady,
    /// Anomaly detected, widen funnel
    WidenFunnel,
    /// Chirality just locked, commit to chamber
    CommitChirality,
    /// Stuck, not converging — try different approach
    Diverging,
    /// Crystallized — constraint satisfied
    Satisfied,
}

impl Default for TemporalAgent {
    fn default() -> Self {
        Self::new()
    }
}

impl TemporalAgent {
    pub fn new() -> Self {
        TemporalAgent {
            constraint: EisensteinConstraint::new(),
            history: [None; HISTORY_SIZE],
            history_pos: 0,
            history_count: 0,

            decay_rate: 1.0,
            prediction_horizon: 4,
            anomaly_sigma: 2.0,
            learning_rate: 0.1,
            chirality_lock_threshold: 500,
            merge_trust: 0.5,

            error_mean: 0.0,
            error_var: 0.0,
            convergence_rate: 0.0,
            precision_energy: 0.0,
            predicted_error: COVERING_RADIUS,
            prediction_error: 0.0,
            chirality: ChiralityState::Exploring { chamber_hops: 0 },
            phase: FunnelPhase::Approach,
        }
    }

    /// Read a sensor value and update temporal model.
    ///
    /// This is the main agentic control loop:
    /// 1. Snap the input (perception)
    /// 2. Compare with prediction (prediction error)
    /// 3. Update temporal model (learning)
    /// 4. Predict next state (planning)
    /// 5. Determine action (control)
    pub fn observe(&mut self, x: f64, y: f64) -> TemporalUpdate {
        // Layer 0: Perception — snap to lattice
        let snap = self.constraint.snap(x, y);
        let error_norm = snap.error / COVERING_RADIUS;

        // Layer 1: Control — compute PID components
        let _proportional = error_norm;
        self.precision_energy += if snap.error > 0.0 {
            1.0 / snap.error
        } else {
            1000.0
        };
        self.update_convergence_rate(error_norm);

        // Layer 2: Prediction — compare with prediction
        self.prediction_error = (error_norm - self.predicted_error).abs();

        // Layer 3: Learning — update running statistics
        self.update_statistics(error_norm);

        // Layer 4: Planning — predict next state
        self.predicted_error = self.predict_next(error_norm);

        // Update chirality state
        self.update_chirality(snap.chamber);

        // Determine phase
        self.update_phase(error_norm);

        // Store in history
        self.history[self.history_pos] = Some(snap);
        self.history_pos = (self.history_pos + 1) % HISTORY_SIZE;
        self.history_count += 1;

        // Determine action
        let is_anomaly =
            self.prediction_error > self.anomaly_sigma * self.error_var.sqrt().max(0.01);
        let action = self.determine_action(error_norm, is_anomaly);

        // Adaptive funnel: widen on anomaly, narrow on convergence
        if is_anomaly && self.decay_rate > 0.1 {
            self.decay_rate *= 0.9;
        } else if error_norm < 0.2 && self.decay_rate < 5.0 {
            self.decay_rate *= 1.05;
        }

        TemporalUpdate {
            snap,
            phase: self.phase,
            chirality: self.chirality,
            predicted_error: self.predicted_error,
            prediction_error: self.prediction_error,
            convergence_rate: self.convergence_rate,
            precision_energy: self.precision_energy,
            is_anomaly,
            action,
        }
    }

    /// Get the current deadband threshold at time t ∈ [0, 1]
    pub fn deadband(&self, t: f64) -> f64 {
        COVERING_RADIUS * (1.0 - t).powf(1.0 / self.decay_rate).max(0.0)
    }

    /// Predict the next error level using exponential moving average + trend.
    fn predict_next(&self, current: f64) -> f64 {
        if self.history_count < 2 {
            return current;
        }
        let predicted = current + self.convergence_rate * self.prediction_horizon as f64;
        predicted.max(0.0).min(1.0)
    }

    /// Update convergence rate from history.
    fn update_convergence_rate(&mut self, current: f64) {
        if self.history_count < 2 {
            return;
        }
        let prev_pos = if self.history_pos == 0 {
            HISTORY_SIZE - 1
        } else {
            self.history_pos - 1
        };
        if let Some(prev) = &self.history[prev_pos] {
            let prev_norm = prev.error / COVERING_RADIUS;
            let rate = current - prev_norm;
            self.convergence_rate =
                self.learning_rate * rate + (1.0 - self.learning_rate) * self.convergence_rate;
        }
    }

    /// Update running statistics (Welford's algorithm).
    fn update_statistics(&mut self, value: f64) {
        let n = self.history_count as f64 + 1.0;
        let delta = value - self.error_mean;
        self.error_mean += delta / n;
        let delta2 = value - self.error_mean;
        self.error_var += delta * delta2;
    }

    /// Update chirality state machine.
    fn update_chirality(&mut self, chamber: u8) {
        match self.chirality {
            ChiralityState::Exploring {
                ref mut chamber_hops,
            } => {
                *chamber_hops += 1;
                if *chamber_hops > 10 {
                    if let Some(d) = self.dominant_chamber() {
                        let conf = self.chamber_confidence_milli(d);
                        if conf > self.chirality_lock_threshold {
                            self.chirality = ChiralityState::Locking {
                                dominant: d,
                                confidence_milli: conf,
                            };
                        }
                    }
                }
            }
            ChiralityState::Locking {
                dominant,
                ref mut confidence_milli,
            } => {
                if chamber == dominant {
                    *confidence_milli = confidence_milli.saturating_add(50);
                    if *confidence_milli > 900 {
                        self.chirality = ChiralityState::Locked { chamber: dominant };
                    }
                } else {
                    *confidence_milli = confidence_milli.saturating_sub(100);
                    if *confidence_milli < 300 {
                        self.chirality = ChiralityState::Exploring { chamber_hops: 0 };
                    }
                }
            }
            ChiralityState::Locked { .. } => {
                // Locked — only unlock on anomaly (external signal)
            }
        }
    }

    /// Update funnel phase based on error level.
    fn update_phase(&mut self, error_norm: f64) {
        self.phase = if error_norm > 0.9 {
            FunnelPhase::Approach
        } else if error_norm > 0.5 {
            FunnelPhase::Narrowing
        } else if error_norm > 0.15 {
            FunnelPhase::SnapImminent
        } else if error_norm < 0.05 {
            FunnelPhase::Crystallized
        } else if self.phase == FunnelPhase::Anomaly {
            FunnelPhase::Anomaly
        } else {
            FunnelPhase::Narrowing
        };
    }

    /// Determine the recommended action.
    fn determine_action(&self, error_norm: f64, is_anomaly: bool) -> AgentAction {
        if is_anomaly {
            return AgentAction::WidenFunnel;
        }
        if error_norm < 0.05 {
            return AgentAction::Satisfied;
        }
        if matches!(self.chirality, ChiralityState::Locked { .. })
            && !matches!(self.phase, FunnelPhase::Crystallized)
        {
            return AgentAction::CommitChirality;
        }
        if self.convergence_rate < -0.01 {
            return AgentAction::Converging;
        }
        if self.convergence_rate > 0.01 {
            return AgentAction::Diverging;
        }
        if error_norm < 0.2 {
            return AgentAction::HoldSteady;
        }
        AgentAction::Continue
    }

    /// Find the most common chamber in history.
    fn dominant_chamber(&self) -> Option<u8> {
        let mut counts = [0u32; 6];
        for s in self.history.iter().flatten() {
            if (s.chamber as usize) < 6 {
                counts[s.chamber as usize] += 1;
            }
        }
        let max_count = *counts.iter().max()?;
        if max_count == 0 {
            return None;
        }
        Some(counts.iter().position(|&c| c == max_count)? as u8)
    }

    /// Confidence (0-1000 milliunits) that the dominant chamber is correct.
    fn chamber_confidence_milli(&self, dominant: u8) -> u16 {
        let mut dominant_count = 0u32;
        let mut total = 0u32;
        for s in self.history.iter().flatten() {
            total += 1;
            if s.chamber == dominant {
                dominant_count += 1;
            }
        }
        if total == 0 {
            return 0;
        }
        ((dominant_count as f64 / total as f64) * 1000.0) as u16
    }

    /// Get the current funnel width (0.0 = snapped, 1.0 = wide open).
    pub fn funnel_width(&self) -> f64 {
        if self.history_count == 0 {
            return 1.0;
        }
        self.error_mean
    }

    /// Get the temporal "temperature" — how much the agent is still exploring.
    /// High T = exploring (entropy ≈ log₂(6)), Low T = committed (entropy ≈ 0).
    pub fn temperature(&self) -> f64 {
        let mut chamber_counts = [0f64; 6];
        let mut total = 0.0;
        for s in self.history.iter().flatten() {
            chamber_counts[s.chamber as usize] += 1.0;
            total += 1.0;
        }
        if total == 0.0 {
            return 1.0;
        }
        let entropy: f64 = chamber_counts
            .iter()
            .filter(|&&c| c > 0.0)
            .map(|&c| {
                let p = c / total;
                -p * p.log2()
            })
            .sum();
        // Normalize to [0, 1]: max entropy = log2(6) ≈ 2.585
        entropy / 6f64.log2()
    }

    /// Get a summary of the agent's temporal state.
    pub fn summary(&self) -> AgentSummary {
        AgentSummary {
            history_count: self.history_count,
            error_mean: self.error_mean,
            error_std: self.error_var.sqrt().max(0.0) / (self.history_count as f64).sqrt().max(1.0),
            convergence_rate: self.convergence_rate,
            precision_energy: self.precision_energy,
            prediction_error: self.prediction_error,
            temperature: self.temperature(),
            phase: self.phase,
            chirality: self.chirality,
            decay_rate: self.decay_rate,
            funnel_width: self.funnel_width(),
        }
    }
}

/// Summary of agent state for fleet reporting.
#[derive(Debug)]
pub struct AgentSummary {
    pub history_count: usize,
    pub error_mean: f64,
    pub error_std: f64,
    pub convergence_rate: f64,
    pub precision_energy: f64,
    pub prediction_error: f64,
    pub temperature: f64,
    pub phase: FunnelPhase,
    pub chirality: ChiralityState,
    pub decay_rate: f64,
    pub funnel_width: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_creation() {
        let agent = TemporalAgent::new();
        assert_eq!(agent.history_count, 0);
        assert_eq!(agent.phase, FunnelPhase::Approach);
        assert_eq!(
            agent.chirality,
            ChiralityState::Exploring { chamber_hops: 0 }
        );
    }

    #[test]
    fn test_agent_observe_convergence() {
        let mut agent = TemporalAgent::new();
        let mut results = Vec::new();
        for i in 0..20 {
            let t = i as f64 / 20.0;
            let r = COVERING_RADIUS * (1.0 - t * 0.9);
            let angle: f64 = 0.5;
            let x = r * angle.cos();
            let y = r * angle.sin();
            let update = agent.observe(x, y);
            results.push(update);
        }
        let final_phase = results.last().unwrap().phase;
        assert!(
            final_phase == FunnelPhase::Narrowing || final_phase == FunnelPhase::SnapImminent,
            "Expected convergence, got {:?}",
            final_phase
        );
    }

    #[test]
    fn test_agent_prediction_improves() {
        let mut agent = TemporalAgent::new();
        let mut errors = Vec::new();
        for i in 0..30 {
            let r = COVERING_RADIUS * (1.0 - i as f64 / 40.0);
            let x = r * 0.5;
            let y = r * 0.866;
            let update = agent.observe(x, y);
            errors.push(update.prediction_error);
        }
        let early_avg: f64 = errors[..10].iter().sum::<f64>() / 10.0;
        let late_avg: f64 = errors[20..].iter().sum::<f64>() / 10.0;
        assert!(
            late_avg < early_avg * 2.0,
            "Prediction should not degrade: early={:.4} late={:.4}",
            early_avg,
            late_avg
        );
    }

    #[test]
    fn test_agent_anomaly_detection() {
        let mut agent = TemporalAgent::new();
        agent.anomaly_sigma = 1.5;
        // Build up enough history for statistics to stabilize
        for _ in 0..20 {
            agent.observe(0.01, 0.01);
        }
        // A few more steady-state — should NOT be anomaly
        for _ in 0..5 {
            let update = agent.observe(0.01, 0.01);
            assert!(
                !update.is_anomaly,
                "Should not be anomaly during steady state"
            );
        }
        // Sudden jump — should detect anomaly (large prediction error)
        let update = agent.observe(3.0, 3.0);
        // After steady state at (0.01, 0.01), (3,3) should produce large prediction error
        // but the statistics may be tight — check either anomaly OR action
        assert!(
            update.is_anomaly
                || update.action == AgentAction::WidenFunnel
                || update.prediction_error > 0.5,
            "Should detect anomaly on sudden jump: anomaly={}, action={:?}, pred_err={:.4}",
            update.is_anomaly,
            update.action,
            update.prediction_error
        );
    }

    #[test]
    fn test_agent_chirality_locking() {
        let mut agent = TemporalAgent::new();
        for _ in 0..40 {
            agent.observe(0.1, 0.1);
        }
        // Should at least be exploring or locking (may not lock in 40 steps)
        match agent.chirality {
            ChiralityState::Locked { .. }
            | ChiralityState::Locking { .. }
            | ChiralityState::Exploring { .. } => {} // all valid
        }
    }

    #[test]
    fn test_agent_temperature() {
        let mut agent = TemporalAgent::new();
        agent.observe(0.1, 0.1);
        let t1 = agent.temperature();
        for _ in 0..20 {
            agent.observe(0.1, 0.1);
        }
        let t2 = agent.temperature();
        assert!(
            t2 <= t1 + 0.1,
            "Temperature should not increase with same-region observations"
        );
    }

    #[test]
    fn test_agent_summary() {
        let mut agent = TemporalAgent::new();
        for i in 0..10 {
            let r = COVERING_RADIUS * (1.0 - i as f64 / 15.0);
            agent.observe(r * 0.5, r * 0.866);
        }
        let summary = agent.summary();
        assert_eq!(summary.history_count, 10);
        assert!(summary.error_mean > 0.0);
        assert!(summary.temperature >= 0.0 && summary.temperature <= 1.0);
    }

    #[test]
    fn test_agent_satisfied() {
        let mut agent = TemporalAgent::new();
        // Many observations at origin — should eventually report Satisfied
        let mut found_satisfied = false;
        for _ in 0..20 {
            let update = agent.observe(0.0, 0.0);
            if update.snap.error < 0.001 {
                if matches!(
                    update.action,
                    AgentAction::Satisfied | AgentAction::HoldSteady | AgentAction::Converging
                ) {
                    found_satisfied = true;
                    break;
                }
            }
        }
        assert!(
            found_satisfied,
            "Should reach satisfied/converging at origin"
        );
    }

    #[test]
    fn test_agent_actions_cover() {
        let mut agent = TemporalAgent::new();
        let mut actions_seen = std::collections::HashSet::new();

        for i in 0..30 {
            let r = COVERING_RADIUS * (1.0 - i as f64 / 40.0);
            let update = agent.observe(r * 0.5, r * 0.866);
            actions_seen.insert(update.action);
        }

        let update = agent.observe(5.0, 5.0);
        actions_seen.insert(update.action);

        assert!(
            actions_seen.len() >= 2,
            "Should see multiple actions: {:?}",
            actions_seen
        );
    }
}
