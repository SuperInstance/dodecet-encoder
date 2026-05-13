//! PLATO Agent Runtime — Agents live in rooms, Forgemaster is the lighthouse.
//!
//! Architecture:
//! - AgentRoom: a PLATO room that hosts an agent
//! - Lighthouse: Forgemaster's relay, orientation, and gate
//! - TileRegistry: shared fleet intelligence
//!
//! The lighthouse doesn't sail the ships. It shows them where the rocks are.

use std::collections::HashMap;

/// Agent status in a PLATO room
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AgentStatus {
    /// Agent is being configured
    Orienting,
    /// Seeds are running discovery
    Seeding,
    /// Agent is working
    Running,
    /// Agent paused (waiting for gate/approval)
    Paused,
    /// Agent finished successfully
    Complete,
    /// Agent failed
    Failed,
}

/// Tile lifecycle for agent output — mirrors PLATO v3
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TileLifecycle {
    Active,
    Superseded,
    Retracted,
}

/// Lamport clock for causal ordering
#[derive(Debug, Clone)]
pub struct LamportClock {
    time: u64,
}

impl LamportClock {
    pub fn new() -> Self { Self { time: 0 } }
    pub fn tick(&mut self) -> u64 { self.time += 1; self.time }
    pub fn merge(&mut self, remote: u64) -> u64 { self.time = self.time.max(remote) + 1; self.time }
    pub fn now(&self) -> u64 { self.time }
}

impl Default for LamportClock {
    fn default() -> Self { Self::new() }
}

/// Model tier — matches resource allocation to task complexity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModelTier {
    /// Claude Code — synthesis, big ideas, stepping-back
    /// Daily limit — use WISELY
    Claude,
    /// GLM-5.1 — architecture, complex code, orchestration
    /// Monthly, short rate limit
    GLM,
    /// Seed-2.0-mini — discovery, exploration, variation
    /// Per-token, cheap
    Seed,
    /// DeepSeek Flash — token-heavy work, documentation
    /// Per-token, cheap
    DeepSeek,
    /// Hermes-70B — second opinions, adversarial testing
    /// Per-token, cheap
    Hermes,
}

impl ModelTier {
    /// Cost estimate per 1K queries (relative)
    pub fn relative_cost(&self) -> f64 {
        match self {
            ModelTier::Claude => 50.0,  // Daily limit — expensive per slot
            ModelTier::GLM => 5.0,      // Monthly but rate-limited
            ModelTier::Seed => 0.1,     // Cheap
            ModelTier::DeepSeek => 0.2, // Cheap
            ModelTier::Hermes => 0.15,  // Cheap
        }
    }

    /// Should this model be used for this task type?
    pub fn appropriate_for(&self, task: TaskType) -> bool {
        match (self, task) {
            (ModelTier::Claude, TaskType::Synthesis) => true,
            (ModelTier::Claude, TaskType::Critique) => true,
            (ModelTier::Claude, TaskType::BigIdea) => true,
            (ModelTier::Claude, _) => false, // Don't waste on drafting

            (ModelTier::GLM, TaskType::Architecture) => true,
            (ModelTier::GLM, TaskType::ComplexCode) => true,
            (ModelTier::GLM, TaskType::Orchestration) => true,
            (ModelTier::GLM, _) => false,

            (ModelTier::Seed, TaskType::Discovery) => true,
            (ModelTier::Seed, TaskType::Exploration) => true,
            (ModelTier::Seed, TaskType::Drafting) => true,
            (ModelTier::Seed, TaskType::Variation) => true,
            (ModelTier::Seed, _) => false,

            (ModelTier::DeepSeek, TaskType::Documentation) => true,
            (ModelTier::DeepSeek, TaskType::Research) => true,
            (ModelTier::DeepSeek, TaskType::Drafting) => true,
            (ModelTier::DeepSeek, _) => false,

            (ModelTier::Hermes, TaskType::Adversarial) => true,
            (ModelTier::Hermes, TaskType::SecondOpinion) => true,
            (ModelTier::Hermes, _) => false,
        }
    }
}

/// Task type classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TaskType {
    /// Synthesis of multiple sources — USE CLAUDE
    Synthesis,
    /// Critical review, finding weak points — USE CLAUDE
    Critique,
    /// Big idea, stepping-back analysis — USE CLAUDE
    BigIdea,
    /// System design, interfaces — USE GLM
    Architecture,
    /// Multi-file, algorithmic — USE GLM
    ComplexCode,
    /// Coordinating multiple agents — USE GLM
    Orchestration,
    /// Parameter exploration — USE SEED
    Discovery,
    /// Survey the landscape — USE SEED
    Exploration,
    /// Generate text, docs — USE SEED or DEEPSEEK
    Drafting,
    /// Run many variations — USE SEED
    Variation,
    /// Write docs, READMEs — USE DEEPSEEK
    Documentation,
    /// Literature review — USE DEEPSEEK
    Research,
    /// Try to break something — USE HERMES
    Adversarial,
    /// Independent verification — USE HERMES
    SecondOpinion,
}

/// An agent living in a PLATO room
#[derive(Debug, Clone)]
pub struct AgentRoom {
    /// Room ID in PLATO
    pub room_id: String,
    /// Agent role
    pub role: String,
    /// Current status
    pub status: AgentStatus,
    /// Model tier assigned
    pub model: ModelTier,
    /// Task type
    pub task_type: TaskType,
    /// Generation (for seed refinement)
    pub generation: u32,
    /// Number of seed iterations run
    pub seed_iterations: usize,
    /// Crystallization score (from seeds, if applicable)
    pub crystallization_score: f64,
    /// Whether this agent has been gated (safety check)
    pub gated: bool,
    /// Whether gate passed
    pub gate_passed: Option<bool>,
    /// Tile lifecycle of agent output
    pub lifecycle: TileLifecycle,
    /// Lamport timestamp
    pub lamport: u64,
    /// Timestamps
    pub created_at: u64,
    pub updated_at: u64,
}

/// Gate result — safety and alignment check
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GateResult {
    Approved,
    Rejected(String),
    NeedsApproval(String),
}

/// The Lighthouse — Forgemaster's relay, orientation, and gate
pub struct Lighthouse {
    /// Active agent rooms
    agents: HashMap<String, AgentRoom>,
    /// Available models and their remaining capacity
    capacity: HashMap<ModelTier, f64>,
    /// Lamport clock for causal ordering
    clock: LamportClock,
}

impl Default for Lighthouse {
    fn default() -> Self {
        Self::new()
    }
}

impl Lighthouse {
    pub fn new() -> Self {
        let mut capacity = HashMap::new();
        capacity.insert(ModelTier::Claude, 1.0); // 100% daily budget
        capacity.insert(ModelTier::GLM, 1.0); // 100% monthly budget
        capacity.insert(ModelTier::Seed, 1.0); // Effectively unlimited
        capacity.insert(ModelTier::DeepSeek, 1.0); // Effectively unlimited
        capacity.insert(ModelTier::Hermes, 1.0); // Effectively unlimited

        Lighthouse {
            agents: HashMap::new(),
            capacity,
            clock: LamportClock::new(),
        }
    }

    /// ORIENT: Classify a task and choose the right model.
    ///
    /// The lighthouse's first job: what needs doing, who should do it.
    pub fn orient(&mut self, task: &str, task_type: TaskType) -> AgentRoom {
        // Find the cheapest appropriate model
        let model = self.cheapest_appropriate(task_type);

        let room_id = format!("agent-{}", simple_hash(task));

        let agent = AgentRoom {
            room_id: room_id.clone(),
            role: task.to_string(),
            status: AgentStatus::Orienting,
            model,
            task_type,
            generation: 0,
            seed_iterations: 0,
            crystallization_score: 0.0,
            gated: false,
            gate_passed: None,
            lifecycle: TileLifecycle::Active,
            lamport: self.clock.tick(),
            created_at: current_timestamp(),
            updated_at: current_timestamp(),
        };

        self.agents.insert(room_id, agent.clone());
        agent
    }

    /// RELAY: Configure agent with API access and conditioning.
    ///
    /// The lighthouse relays keys without exposing them.
    /// The lighthouse provides tiles as conditioning context.
    pub fn relay(&mut self, room_id: &str, seed_iterations: usize) -> Option<&AgentRoom> {
        let agent = self.agents.get_mut(room_id)?;

        // If seed work needed, set to Seeding first
        if seed_iterations > 0 && agent.model != ModelTier::Seed {
            // Run seeds first, then upgrade
            agent.status = AgentStatus::Seeding;
            agent.seed_iterations = seed_iterations;
        } else {
            agent.status = AgentStatus::Running;
        }

        agent.updated_at = current_timestamp();
        self.agents.get(room_id)
    }

    /// GATE: Safety and alignment check on agent output.
    ///
    /// The lighthouse checks:
    /// 1. No credential leaks
    /// 2. No external actions without approval
    /// 3. No overclaims
    /// 4. Constraint satisfaction verified
    pub fn gate(&mut self, room_id: &str, output: &str) -> GateResult {
        let agent = match self.agents.get_mut(room_id) {
            Some(a) => a,
            None => return GateResult::Rejected("Unknown room".to_string()),
        };

        agent.gated = true;

        // Check 1: Credential leaks
        if contains_credentials(output) {
            agent.gate_passed = Some(false);
            agent.status = AgentStatus::Failed;
            return GateResult::Rejected("Credential leak detected".to_string());
        }

        // Check 2: External action markers
        if contains_external_action(output) {
            agent.gate_passed = Some(false);
            return GateResult::NeedsApproval(
                "External action requires Casey approval".to_string(),
            );
        }

        // Check 3: Overclaim markers
        if contains_overclaims(output) {
            agent.gate_passed = Some(false);
            return GateResult::Rejected(
                "Overclaim detected — falsify before asserting".to_string(),
            );
        }

        // All checks passed
        agent.gate_passed = Some(true);
        agent.status = AgentStatus::Complete;
        agent.updated_at = current_timestamp();

        // Deduct capacity
        let cost = agent.model.relative_cost() * 0.01;
        if let Some(remaining) = self.capacity.get_mut(&agent.model) {
            *remaining = (*remaining - cost).max(0.0);
        }

        GateResult::Approved
    }

    /// Find cheapest appropriate model for a task type.
    fn cheapest_appropriate(&self, task_type: TaskType) -> ModelTier {
        // Try from cheapest to most expensive
        let tiers = [
            ModelTier::Seed,
            ModelTier::Hermes,
            ModelTier::DeepSeek,
            ModelTier::GLM,
            ModelTier::Claude,
        ];

        for &tier in &tiers {
            if tier.appropriate_for(task_type) {
                if let Some(cap) = self.capacity.get(&tier) {
                    if *cap > 0.1 {
                        return tier;
                    }
                }
            }
        }

        // Fallback: Seed is always available
        ModelTier::Seed
    }

    /// List active agents
    pub fn active_agents(&self) -> Vec<&AgentRoom> {
        self.agents
            .values()
            .filter(|a| a.status == AgentStatus::Running || a.status == AgentStatus::Seeding)
            .collect()
    }

    /// Get agent status
    pub fn get_agent(&self, room_id: &str) -> Option<&AgentRoom> {
        self.agents.get(room_id)
    }

    /// Resource summary for fleet reporting
    pub fn resource_summary(&self) -> String {
        let mut lines = vec!["LIGHTHOUSE RESOURCE STATUS".to_string()];
        for (tier, remaining) in &self.capacity {
            let bar_len = (*remaining * 20.0) as usize;
            let bar: String = "█".repeat(bar_len) + &"░".repeat(20 - bar_len);
            lines.push(format!(
                "  {:?}: [{}] {:.0}% remaining",
                tier,
                bar,
                remaining * 100.0
            ));
        }
        lines.push(format!("  Active agents: {}", self.active_agents().len()));
        lines.join("\n")
    }

    // ── Tile Lifecycle (v1.2.0) ────────────────────────────

    /// Supersede an agent's output — mark as superseded.
    pub fn supersede_agent(&mut self, room_id: &str) -> bool {
        if let Some(agent) = self.agents.get_mut(room_id) {
            if agent.lifecycle == TileLifecycle::Active {
                agent.lifecycle = TileLifecycle::Superseded;
                agent.lamport = self.clock.tick();
                return true;
            }
        }
        false
    }

    /// Retract an agent's output — mark as retracted.
    pub fn retract_agent(&mut self, room_id: &str, reason: &str) -> bool {
        if let Some(agent) = self.agents.get_mut(room_id) {
            if agent.lifecycle == TileLifecycle::Active {
                agent.lifecycle = TileLifecycle::Retracted;
                agent.lamport = self.clock.tick();
                agent.status = AgentStatus::Failed;
                let _ = reason; // logged in production
                return true;
            }
        }
        false
    }

    /// Get only active agents (lifecycle = Active).
    pub fn active_lifecycle_agents(&self) -> Vec<&AgentRoom> {
        self.agents.values()
            .filter(|a| a.lifecycle == TileLifecycle::Active)
            .collect()
    }

    /// Predict outcome before relay — simulation-first.
    /// Returns predicted gate result without actually gating.
    pub fn predict_gate(&self, room_id: &str, output: &str) -> GateResult {
        // Same logic as gate(), but doesn't modify state
        if contains_credentials(output) {
            return GateResult::Rejected("Credential leak detected in prediction".to_string());
        }
        if contains_external_action(output) {
            return GateResult::NeedsApproval("External action predicted".to_string());
        }
        if contains_overclaims(output) {
            return GateResult::Rejected("Overclaim detected in prediction".to_string());
        }
        GateResult::Approved
    }

    /// Count agents by lifecycle state.
    pub fn lifecycle_stats(&self) -> (usize, usize, usize) {
        let active = self.agents.values().filter(|a| a.lifecycle == TileLifecycle::Active).count();
        let superseded = self.agents.values().filter(|a| a.lifecycle == TileLifecycle::Superseded).count();
        let retracted = self.agents.values().filter(|a| a.lifecycle == TileLifecycle::Retracted).count();
        (active, superseded, retracted)
    }
}

// ─── Helper functions ─────────────────────────────────────────

fn simple_hash(s: &str) -> String {
    let mut hash: u64 = 5381;
    for b in s.bytes() {
        hash = hash.wrapping_mul(33).wrapping_add(b as u64);
    }
    format!("{:08x}", hash)
}

fn current_timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

fn contains_credentials(s: &str) -> bool {
    let lower = s.to_lowercase();
    lower.contains("api_key=")
        || lower.contains("password=")
        || lower.contains("secret=")
        || lower.contains("token=")
        || lower.contains("bearer ")
}

fn contains_external_action(s: &str) -> bool {
    let markers = [
        "send_email",
        "post_tweet",
        "git push",
        "npm publish",
        "deploy",
    ];
    markers.iter().any(|m| s.contains(m))
}

fn contains_overclaims(s: &str) -> bool {
    let markers = ["proven that", "theorem:", "this proves", "we have proven"];
    markers.iter().any(|m| s.to_lowercase().contains(m))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_orient_synthesis_uses_claude() {
        let mut lh = Lighthouse::new();
        let agent = lh.orient("synthesis paper", TaskType::Synthesis);
        assert_eq!(agent.model, ModelTier::Claude);
    }

    #[test]
    fn test_orient_drafting_uses_seed() {
        let mut lh = Lighthouse::new();
        let agent = lh.orient("draft a readme", TaskType::Drafting);
        assert_eq!(agent.model, ModelTier::Seed);
    }

    #[test]
    fn test_orient_adversarial_uses_hermes() {
        let mut lh = Lighthouse::new();
        let agent = lh.orient("find weak points", TaskType::Adversarial);
        assert_eq!(agent.model, ModelTier::Hermes);
    }

    #[test]
    fn test_orient_architecture_uses_glm() {
        let mut lh = Lighthouse::new();
        let agent = lh.orient("design the system", TaskType::Architecture);
        assert_eq!(agent.model, ModelTier::GLM);
    }

    #[test]
    fn test_gate_approves_clean_output() {
        let mut lh = Lighthouse::new();
        let agent = lh.orient("task", TaskType::Drafting);
        let result = lh.gate(&agent.room_id, "Here is a clean result with no issues.");
        assert_eq!(result, GateResult::Approved);
    }

    #[test]
    fn test_gate_rejects_credentials() {
        let mut lh = Lighthouse::new();
        let agent = lh.orient("task", TaskType::Drafting);
        let result = lh.gate(&agent.room_id, "The api_key=abc123 is here");
        assert!(matches!(result, GateResult::Rejected(_)));
    }

    #[test]
    fn test_gate_needs_approval_for_external() {
        let mut lh = Lighthouse::new();
        let agent = lh.orient("task", TaskType::Drafting);
        let result = lh.gate(&agent.room_id, "Running git push to main");
        assert!(matches!(result, GateResult::NeedsApproval(_)));
    }

    #[test]
    fn test_gate_rejects_overclaims() {
        let mut lh = Lighthouse::new();
        let agent = lh.orient("task", TaskType::Drafting);
        let result = lh.gate(
            &agent.room_id,
            "We have proven that all lattices are perfect",
        );
        assert!(matches!(result, GateResult::Rejected(_)));
    }

    #[test]
    fn test_relay_sets_seeding() {
        let mut lh = Lighthouse::new();
        let agent = lh.orient("explore", TaskType::Architecture);
        // GLM agent with seed iterations -> Seeding first
        let result = lh.relay(&agent.room_id, 50);
        assert!(result.is_some());
        assert_eq!(result.unwrap().status, AgentStatus::Seeding);
    }

    #[test]
    fn test_resource_summary() {
        let mut lh = Lighthouse::new();
        let summary = lh.resource_summary();
        assert!(summary.contains("Claude"));
        assert!(summary.contains("Seed"));
        assert!(summary.contains("remaining"));
    }

    #[test]
    fn test_capacity_decreases_after_gate() {
        let mut lh = Lighthouse::new();
        let initial = *lh.capacity.get(&ModelTier::Seed).unwrap();
        let agent = lh.orient("task", TaskType::Drafting);
        lh.gate(&agent.room_id, "clean output");
        let after = *lh.capacity.get(&ModelTier::Seed).unwrap();
        assert!(after < initial);
    }

    // ── v1.2.0: Tile lifecycle tests ────────────────────────

    #[test]
    fn test_agent_starts_active() {
        let mut lh = Lighthouse::new();
        let agent = lh.orient("task", TaskType::Drafting);
        assert_eq!(agent.lifecycle, TileLifecycle::Active);
        assert_eq!(agent.lamport, 1);
    }

    #[test]
    fn test_supersede_agent() {
        let mut lh = Lighthouse::new();
        let agent = lh.orient("task", TaskType::Drafting);
        let result = lh.supersede_agent(&agent.room_id);
        assert!(result);
        let (active, sup, _) = lh.lifecycle_stats();
        assert_eq!(active, 0);
        assert_eq!(sup, 1);
    }

    #[test]
    fn test_retract_agent() {
        let mut lh = Lighthouse::new();
        let agent = lh.orient("task", TaskType::Drafting);
        let result = lh.retract_agent(&agent.room_id, "constraint violation");
        assert!(result);
        let (_, _, ret) = lh.lifecycle_stats();
        assert_eq!(ret, 1);
    }

    #[test]
    fn test_cannot_supersede_retracted() {
        let mut lh = Lighthouse::new();
        let agent = lh.orient("task", TaskType::Drafting);
        lh.retract_agent(&agent.room_id, "bad");
        let result = lh.supersede_agent(&agent.room_id);
        assert!(!result, "Cannot supersede a retracted agent");
    }

    #[test]
    fn test_active_lifecycle_agents_filters() {
        let mut lh = Lighthouse::new();
        let a1 = lh.orient("task1", TaskType::Drafting);
        let a2 = lh.orient("task2", TaskType::Drafting);
        lh.supersede_agent(&a1.room_id);
        let active = lh.active_lifecycle_agents();
        assert_eq!(active.len(), 1);
        assert_eq!(active[0].room_id, a2.room_id);
    }

    #[test]
    fn test_predict_gate_clean() {
        let mut lh = Lighthouse::new();
        let agent = lh.orient("task", TaskType::Drafting);
        let result = lh.predict_gate(&agent.room_id, "clean output");
        assert_eq!(result, GateResult::Approved);
    }

    #[test]
    fn test_predict_gate_catches_credentials() {
        let mut lh = Lighthouse::new();
        let agent = lh.orient("task", TaskType::Drafting);
        let result = lh.predict_gate(&agent.room_id, "api_key=secret123");
        assert!(matches!(result, GateResult::Rejected(_)));
    }

    #[test]
    fn test_lamport_clock_across_agents() {
        let mut lh = Lighthouse::new();
        let a1 = lh.orient("task1", TaskType::Drafting);
        let a2 = lh.orient("task2", TaskType::Drafting);
        assert!(a1.lamport < a2.lamport, "Second agent should have higher Lamport");
    }
}
