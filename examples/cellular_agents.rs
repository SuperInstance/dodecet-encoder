// Cellular Agents Example - Dodecet Encoder Integration
//
// This example demonstrates how dodecet encoding provides efficient state
// representation for cellular agents (Claw agents) in the SuperInstance ecosystem.

use dodecet_encoder::{Dodecet, DodecetArray, Point3D, Vector3D};

/// Represents the state of a cellular agent
#[derive(Debug, Clone)]
struct AgentState {
    position: Point3D,      // 3 dodecets (6 bytes)
    velocity: Vector3D,     // 3 dodecets (6 bytes)
    status: Dodecet,        // 1 dodecet (2 bytes)
    energy: Dodecet,        // 1 dodecet (2 bytes)
    equipment: DodecetArray<4>, // 4 dodecets (8 bytes)
}

impl AgentState {
    /// Create a new agent state
    fn new(x: u16, y: u16, z: u16) -> Self {
        AgentState {
            position: Point3D::new(x, y, z),
            velocity: Vector3D::new(0, 0, 0),
            status: Dodecet::from_hex(0x001), // Active
            energy: Dodecet::from_hex(0xFFF), // Full energy
            equipment: DodecetArray::from_slice(&[0x000, 0x000, 0x000, 0x000]),
        }
    }

    /// Serialize to compact dodecet array (8 dodecets = 16 bytes)
    fn serialize(&self) -> [Dodecet; 8] {
        [
            Dodecet::new(self.position.x()).unwrap(),
            Dodecet::new(self.position.y()).unwrap(),
            Dodecet::new(self.position.z()).unwrap(),
            Dodecet::from_signed(self.velocity.x()),
            Dodecet::from_signed(self.velocity.y()),
            Dodecet::from_signed(self.velocity.z()),
            self.status,
            self.energy,
        ]
    }

    /// Convert serialized state to hex string for hashing
    fn to_hex_string(&self) -> String {
        self.serialize()
            .iter()
            .map(|d| format!("{:03X}", d.value()))
            .collect()
    }

    /// Calculate memory efficiency vs traditional struct
    fn memory_savings(&self) -> (usize, usize, f64) {
        let dodecet_size = 16; // 8 dodecets * 2 bytes
        let traditional_size = std::mem::size_of::<Point3D>() +
                              std::mem::size_of::<Vector3D>() +
                              std::mem::size_of::<Dodecet>() * 2 +
                              std::mem::size_of::<DodecetArray<4>>();
        let savings = 100.0 * (1.0 - (dodecet_size as f64 / traditional_size as f64));
        (dodecet_size, traditional_size, savings)
    }
}

/// Agent status codes
#[allow(dead_code)]
mod status {
    use dodecet_encoder::Dodecet;
    pub const INACTIVE: Dodecet = Dodecet::from_hex(0x000);
    pub const ACTIVE: Dodecet = Dodecet::from_hex(0x001);
    pub const THINKING: Dodecet = Dodecet::from_hex(0x002);
    pub const MOVING: Dodecet = Dodecet::from_hex(0x003);
    pub const EQUIPPING: Dodecet = Dodecet::from_hex(0x004);
}

/// Equipment types
#[allow(dead_code)]
mod equipment {
    use dodecet_encoder::Dodecet;
    pub const NONE: Dodecet = Dodecet::from_hex(0x000);
    pub const MEMORY: Dodecet = Dodecet::from_hex(0x001);
    pub const REASONING: Dodecet = Dodecet::from_hex(0x002);
    pub const CONSENSUS: Dodecet = Dodecet::from_hex(0x003);
    pub const SPREADSHEET: Dodecet = Dodecet::from_hex(0x004);
    pub const DISTILLATION: Dodecet = Dodecet::from_hex(0x005);
    pub const COORDINATION: Dodecet = Dodecet::from_hex(0x006);
}

fn main() {
    println!("=== Cellular Agents with Dodecet Encoding ===\n");

    // 1. Create agent states
    println!("1. Creating Agent States:");
    let agent1 = AgentState::new(0x100, 0x200, 0x300);
    let agent2 = AgentState::new(0x400, 0x500, 0x600);
    let agent3 = AgentState::new(0x700, 0x800, 0x900);

    println!("   Agent 1: {:?}", agent1.position);
    println!("   Agent 2: {:?}", agent2.position);
    println!("   Agent 3: {:?}", agent3.position);

    // 2. Serialize agents
    println!("\n2. Serialization:");
    let _state1 = agent1.serialize();
    println!("   Agent 1 serialized: {}", agent1.to_hex_string());
    println!("   Size: 16 bytes (8 dodecets)");

    // 3. Memory efficiency
    println!("\n3. Memory Efficiency:");
    let (dodecet_size, traditional_size, savings) = agent1.memory_savings();
    println!("   Dodecet encoding: {} bytes", dodecet_size);
    println!("   Traditional struct: {} bytes", traditional_size);
    println!("   Savings: {:.1}%", savings);

    // 4. Batch operations (simulating 1000 agents)
    println!("\n4. Batch Operations (1000 agents):");
    let start = std::time::Instant::now();
    let agents: Vec<AgentState> = (0..1000)
        .map(|i| AgentState::new(i % 4096, (i * 2) % 4096, (i * 3) % 4096))
        .collect();
    let creation_time = start.elapsed();

    let start = std::time::Instant::now();
    let _states: Vec<[Dodecet; 8]> = agents.iter()
        .map(|a| a.serialize())
        .collect();
    let serial_time = start.elapsed();

    println!("   Creation time: {:?}", creation_time);
    println!("   Serialization time: {:?}", serial_time);
    println!("   Total memory: {} bytes ({} KB)",
        1000 * dodecet_size,
        (1000 * dodecet_size) / 1024
    );

    // 5. Spatial queries (neighbor detection)
    println!("\n5. Spatial Queries (Neighbor Detection):");
    let query_point = Point3D::new(0x105, 0x205, 0x305);
    let radius = 0x50;

    let neighbors: Vec<&AgentState> = agents.iter()
        .filter(|a| a.position.distance_to(&query_point) < radius as f64)
        .take(5)
        .collect();

    println!("   Query point: {:?}", query_point);
    println!("   Radius: {}", radius);
    println!("   Found {} neighbors (showing first 5):", neighbors.len());
    for (i, agent) in neighbors.iter().enumerate() {
        println!("     {}: {:?} (dist: {:.2})",
            i + 1,
            agent.position,
            agent.position.distance_to(&query_point)
        );
    }

    // 6. Equipment management
    println!("\n6. Equipment Management:");
    let mut equipped_agent = AgentState::new(0x200, 0x300, 0x400);
    equipped_agent.status = status::EQUIPPING;
    equipped_agent.equipment = DodecetArray::from_dodecets([
        equipment::MEMORY,
        equipment::REASONING,
        equipment::CONSENSUS,
        equipment::COORDINATION,
    ]);

    println!("   Agent status: {}", equipped_agent.status.value());
    println!("   Equipment slots:");
    for (i, eq) in equipped_agent.equipment.iter().enumerate() {
        println!("     Slot {}: 0x{:03X}", i, eq.value());
    }

    // 7. Movement simulation
    println!("\n7. Movement Simulation:");
    let mut moving_agent = AgentState::new(0x100, 0x100, 0x100);
    moving_agent.status = status::MOVING;
    moving_agent.velocity = Vector3D::new(10, 15, 20);

    println!("   Initial position: {:?}", moving_agent.position);
    println!("   Velocity: {:?}", moving_agent.velocity);

    for step in 1..=5 {
        let new_x = (moving_agent.position.x() as i32 + moving_agent.velocity.x() as i32) as u16 % 4096;
        let new_y = (moving_agent.position.y() as i32 + moving_agent.velocity.y() as i32) as u16 % 4096;
        let new_z = (moving_agent.position.z() as i32 + moving_agent.velocity.z() as i32) as u16 % 4096;
        moving_agent.position = Point3D::new(new_x, new_y, new_z);
        println!("   Step {}: {:?}", step, moving_agent.position);
    }

    // 8. Energy consumption
    println!("\n8. Energy Consumption:");
    let mut working_agent = AgentState::new(0x300, 0x300, 0x300);
    println!("   Initial energy: {}", working_agent.energy.value());

    for cycle in 1..=10 {
        let consumption = match working_agent.status {
            _ if working_agent.status.value() == status::THINKING.value() => 5,
            _ if working_agent.status.value() == status::MOVING.value() => 10,
            _ if working_agent.status.value() == status::EQUIPPING.value() => 15,
            _ => 1,
        };

        let current = working_agent.energy.value();
        let new_energy = current.saturating_sub(consumption);
        working_agent.energy = Dodecet::new(new_energy).unwrap();

        println!("   Cycle {}: consumed {}, remaining: {}",
            cycle, consumption, working_agent.energy.value());

        if working_agent.energy.value() == 0 {
            println!("   Agent depleted at cycle {}", cycle);
            break;
        }
    }

    // 9. State comparison (deterministic reasoning)
    println!("\n9. State Comparison (Deterministic Reasoning):");
    let reference = AgentState::new(0x100, 0x200, 0x300);
    let test1 = AgentState::new(0x100, 0x200, 0x300);
    let test2 = AgentState::new(0x100, 0x200, 0x301);

    let ref_hash = reference.to_hex_string();
    let test1_hash = test1.to_hex_string();
    let test2_hash = test2.to_hex_string();

    println!("   Reference state hash: {}", ref_hash);
    println!("   Test 1 hash: {}", test1_hash);
    println!("   Test 2 hash: {}", test2_hash);
    println!("   Test 1 == Reference: {}", ref_hash == test1_hash);
    println!("   Test 2 == Reference: {}", ref_hash == test2_hash);
    println!("   → Dodecet encoding ensures deterministic state comparison");

    // 10. Large-scale simulation summary
    println!("\n10. Large-Scale Simulation Summary:");
    println!("   Agents simulated: 1000");
    println!("   Total state memory: {} KB", (1000 * dodecet_size) / 1024);
    println!("   Average serialization time: {:.2} ns/agent",
        serial_time.as_nanos() / 1000
    );
    println!("   Memory vs traditional: {:.1}% reduction", savings);
    println!("   Deterministic: Yes (integer-based)");
    println!("   Hex-editor friendly: Yes (3 hex digits per dodecet)");

    println!("\n=== Key Advantages for Cellular Agents ===");
    println!("✓ Compact state representation (16 bytes vs 48+ bytes)");
    println!("✓ Fast serialization and comparison");
    println!("✓ Deterministic reasoning (no floating-point drift)");
    println!("✓ Hex-friendly debugging and inspection");
    println!("✓ Efficient spatial queries with integer math");
    println!("✓ Natural equipment encoding (12-bit equipment IDs)");
}
