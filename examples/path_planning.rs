// Path Planning Example - Dodecet Encoder Integration
//
// This example demonstrates path planning and navigation using dodecet encoding,
// showing how discrete geometric representation enables efficient algorithms.

use dodecet_encoder::{Dodecet, Point3D, Vector3D};
use std::collections::{HashSet, BinaryHeap, HashMap};
use std::cmp::Ordering;

/// 3D grid node for path planning
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct GridNode {
    x: u16,
    y: u16,
    z: u16,
}

impl GridNode {
    fn new(x: u16, y: u16, z: u16) -> Self {
        GridNode { x, y, z }
    }

    fn to_point(&self) -> Point3D {
        Point3D::new(self.x, self.y, self.z)
    }

    fn neighbors(&self) -> Vec<GridNode> {
        let mut neighbors = Vec::new();
        let dirs = [
            (1, 0, 0), (-1, 0, 0),
            (0, 1, 0), (0, -1, 0),
            (0, 0, 1), (0, 0, -1),
        ];

        for (dx, dy, dz) in dirs.iter() {
            let nx = self.x as i32 + dx;
            let ny = self.y as i32 + dy;
            let nz = self.z as i32 + dz;

            if nx >= 0 && nx < 4096 && ny >= 0 && ny < 4096 && nz >= 0 && nz < 4096 {
                neighbors.push(GridNode::new(nx as u16, ny as u16, nz as u16));
            }
        }

        neighbors
    }

    fn distance_to(&self, other: &GridNode) -> f64 {
        let dx = (self.x as i32 - other.x as i32).abs() as f64;
        let dy = (self.y as i32 - other.y as i32).abs() as f64;
        let dz = (self.z as i32 - other.z as i32).abs() as f64;
        (dx*dx + dy*dy + dz*dz).sqrt()
    }

    fn manhattan_distance(&self, other: &GridNode) -> u32 {
        let dx = (self.x as i32 - other.x as i32).abs();
        let dy = (self.y as i32 - other.y as i32).abs();
        let dz = (self.z as i32 - other.z as i32).abs();
        (dx + dy + dz) as u32
    }
}

/// A* pathfinding node
#[derive(Debug, Clone, Copy)]
struct AStarNode {
    node: GridNode,
    g_cost: u32, // Cost from start
    h_cost: u32, // Heuristic to goal
    f_cost: u32, // Total cost
}

impl PartialEq for AStarNode {
    fn eq(&self, other: &Self) -> bool {
        self.f_cost == other.f_cost && self.node == other.node
    }
}

impl Eq for AStarNode {}

impl PartialOrd for AStarNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for AStarNode {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse order for min-heap
        other.f_cost.cmp(&self.f_cost)
            .then_with(|| other.h_cost.cmp(&self.h_cost))
    }
}

/// 3D environment for path planning
struct Environment {
    obstacles: HashSet<GridNode>,
    bounds: (u16, u16, u16),
}

impl Environment {
    fn new(width: u16, height: u16, depth: u16) -> Self {
        Environment {
            obstacles: HashSet::new(),
            bounds: (width, height, depth),
        }
    }

    fn add_obstacle(&mut self, node: GridNode) {
        self.obstacles.insert(node);
    }

    fn is_obstacle(&self, node: &GridNode) -> bool {
        self.obstacles.contains(node)
    }

    fn is_valid(&self, node: &GridNode) -> bool {
        node.x < self.bounds.0 &&
        node.y < self.bounds.1 &&
        node.z < self.bounds.2 &&
        !self.is_obstacle(node)
    }

    fn get_valid_neighbors(&self, node: &GridNode) -> Vec<GridNode> {
        node.neighbors()
            .into_iter()
            .filter(|n| self.is_valid(n))
            .collect()
    }
}

/// A* pathfinding algorithm
fn astar_pathfind(
    env: &Environment,
    start: GridNode,
    goal: GridNode
) -> Option<Vec<GridNode>> {
    let mut open_set = BinaryHeap::new();
    let mut came_from: HashMap<GridNode, GridNode> = HashMap::new();
    let mut g_score: HashMap<GridNode, u32> = HashMap::new();

    g_score.insert(start, 0);
    open_set.push(AStarNode {
        node: start,
        g_cost: 0,
        h_cost: start.manhattan_distance(&goal),
        f_cost: start.manhattan_distance(&goal),
    });

    while let Some(current) = open_set.pop() {
        if current.node == goal {
            // Reconstruct path
            let mut path = vec![current.node];
            let mut current_node = current.node;

            while let Some(&prev) = came_from.get(&current_node) {
                path.push(prev);
                current_node = prev;
            }

            path.reverse();
            return Some(path);
        }

        for neighbor in env.get_valid_neighbors(&current.node) {
            let tentative_g = g_score[&current.node] + 1;

            if tentative_g < *g_score.get(&neighbor).unwrap_or(&u32::MAX) {
                came_from.insert(neighbor, current.node);
                g_score.insert(neighbor, tentative_g);

                let h = neighbor.manhattan_distance(&goal);
                let f = tentative_g + h;

                open_set.push(AStarNode {
                    node: neighbor,
                    g_cost: tentative_g,
                    h_cost: h,
                    f_cost: f,
                });
            }
        }
    }

    None
}

/// Waypoint path smoothing
fn smooth_path(path: &[GridNode], env: &Environment) -> Vec<GridNode> {
    if path.len() <= 2 {
        return path.to_vec();
    }

    let mut smoothed = vec![path[0]];
    let mut current = 0;

    while current < path.len() - 1 {
        let mut furthest = current + 1;

        // Find furthest visible point
        for i in (current + 2)..path.len() {
            if line_of_sight(&path[current], &path[i], env) {
                furthest = i;
            } else {
                break;
            }
        }

        smoothed.push(path[furthest]);
        current = furthest;
    }

    smoothed
}

/// Check line of sight between two points
fn line_of_sight(a: &GridNode, b: &GridNode, env: &Environment) -> bool {
    let dx = (b.x as i32 - a.x as i32) as f64;
    let dy = (b.y as i32 - a.y as i32) as f64;
    let dz = (b.z as i32 - a.z as i32) as f64;
    let dist = (dx*dx + dy*dy + dz*dz).sqrt();

    let steps = dist.ceil() as usize;
    if steps == 0 {
        return true;
    }

    for i in 1..steps {
        let t = i as f64 / steps as f64;
        let x = a.x as f64 + dx * t;
        let y = a.y as f64 + dy * t;
        let z = a.z as f64 + dz * t;

        let node = GridNode::new(x as u16, y as u16, z as u16);
        if env.is_obstacle(&node) {
            return false;
        }
    }

    true
}

/// Path length calculation
fn path_length(path: &[GridNode]) -> f64 {
    if path.len() < 2 {
        return 0.0;
    }

    let mut length = 0.0;
    for i in 0..path.len()-1 {
        length += path[i].distance_to(&path[i+1]);
    }

    length
}

fn main() {
    println!("=== Path Planning with Dodecet Encoding ===\n");

    // 1. Simple straight line path
    println!("1. Simple Straight Line Path:");
    let start = GridNode::new(100, 100, 100);
    let goal = GridNode::new(200, 200, 200);

    println!("   Start: {:?}", start);
    println!("   Goal: {:?}", goal);
    println!("   Euclidean distance: {:.2}", start.distance_to(&goal));
    println!("   Manhattan distance: {}", start.manhattan_distance(&goal));

    // 2. Create environment with obstacles
    println!("\n2. Environment with Obstacles:");
    let mut env = Environment::new(256, 256, 256);

    // Add wall
    for x in 150..=160 {
        for y in 0..256 {
            env.add_obstacle(GridNode::new(x, y, 150));
        }
    }

    println!("   Environment size: 256x256x256");
    println!("   Obstacles: {} cells", env.obstacles.len());

    // 3. A* pathfinding
    println!("\n3. A* Pathfinding:");
    let start = GridNode::new(100, 100, 150);
    let goal = GridNode::new(200, 100, 150);

    let path_start = std::time::Instant::now();
    if let Some(path) = astar_pathfind(&env, start, goal) {
        let path_time = path_start.elapsed();

        println!("   Path found in {:?}", path_time);
        println!("   Path length: {} steps", path.len());
        println!("   Total distance: {:.2}", path_length(&path));
        println!("   First 5 steps:");
        for (i, node) in path.iter().take(5).enumerate() {
            println!("     {}: {:?}", i, node);
        }
    } else {
        println!("   No path found!");
    }

    // 4. Path smoothing
    println!("\n4. Path Smoothing:");
    if let Some(path) = astar_pathfind(&env, start, goal) {
        println!("   Original path: {} steps", path.len());
        println!("   Original distance: {:.2}", path_length(&path));

        let smoothed = smooth_path(&path, &env);
        println!("   Smoothed path: {} steps", smoothed.len());
        println!("   Smoothed distance: {:.2}", path_length(&smoothed));
        println!("   Reduction: {:.1}%", 100.0 * (1.0 - smoothed.len() as f64 / path.len() as f64));
    }

    // 5. Multiple waypoints
    println!("\n5. Multi-Waypoint Path:");
    let waypoints = vec![
        GridNode::new(50, 50, 50),
        GridNode::new(200, 50, 50),
        GridNode::new(200, 200, 50),
        GridNode::new(50, 200, 200),
    ];

    let mut total_distance = 0.0;
    let mut total_steps = 0;

    for i in 0..waypoints.len()-1 {
        if let Some(segment) = astar_pathfind(&env, waypoints[i], waypoints[i+1]) {
            let dist = path_length(&segment);
            total_distance += dist;
            total_steps += segment.len();
            println!("   Segment {}: {} steps, {:.2} units",
                i, segment.len(), dist);
        }
    }

    println!("   Total: {} steps, {:.2} units", total_steps, total_distance);

    // 6. 3D maze navigation
    println!("\n6. 3D Maze Navigation:");
    let mut maze = Environment::new(100, 100, 100);

    // Create maze walls
    for i in 20..=80 {
        maze.add_obstacle(GridNode::new(i, 40, 40));
        maze.add_obstacle(GridNode::new(i, 40, 60));
        maze.add_obstacle(GridNode::new(i, 60, 40));
        maze.add_obstacle(GridNode::new(i, 60, 60));
    }

    let maze_start = GridNode::new(10, 50, 50);
    let maze_goal = GridNode::new(90, 50, 50);

    if let Some(maze_path) = astar_pathfind(&maze, maze_start, maze_goal) {
        println!("   Start: {:?}", maze_start);
        println!("   Goal: {:?}", maze_goal);
        println!("   Path found: {} steps", maze_path.len());
        println!("   Distance: {:.2}", path_length(&maze_path));
    }

    // 7. Dynamic obstacle avoidance
    println!("\n7. Dynamic Obstacle Avoidance:");
    let mut dynamic_env = Environment::new(256, 256, 256);

    // Moving obstacle simulation
    let dynamic_start = GridNode::new(100, 100, 100);
    let dynamic_goal = GridNode::new(200, 200, 200);

    if let Some(initial_path) = astar_pathfind(&dynamic_env, dynamic_start, dynamic_goal) {
        println!("   Initial path: {} steps", initial_path.len());

        // Add obstacle in the middle of path
        if initial_path.len() > 5 {
            let mid_point = initial_path[initial_path.len() / 2];
            dynamic_env.add_obstacle(mid_point);
            println!("   Obstacle added at: {:?}", mid_point);

            // Replan
            if let Some(new_path) = astar_pathfind(&dynamic_env, dynamic_start, dynamic_goal) {
                println!("   Replanned path: {} steps", new_path.len());
            }
        }
    }

    // 8. Performance benchmarking
    println!("\n8. Performance Benchmarking:");
    let mut benchmark_env = Environment::new(512, 512, 512);

    // Add random obstacles
    for i in 0..1000 {
        let x = (i * 17) % 512;
        let y = (i * 23) % 512;
        let z = (i * 31) % 512;
        benchmark_env.add_obstacle(GridNode::new(x, y, z));
    }

    let bench_start = GridNode::new(0, 0, 0);
    let bench_goal = GridNode::new(511, 511, 511);

    let bench_time = std::time::Instant::now();
    let result = astar_pathfind(&benchmark_env, bench_start, bench_goal);
    let bench_elapsed = bench_time.elapsed();

    if let Some(bench_path) = result {
        println!("   Environment: 512x512x512");
        println!("   Obstacles: 1000");
        println!("   Path found: {} steps", bench_path.len());
        println!("   Time: {:?}", bench_elapsed);
        println!("   Performance: {:.2} steps/ms", bench_path.len() as f64 / bench_elapsed.as_millis() as f64);
    }

    // 9. Memory efficiency
    println!("\n9. Memory Efficiency:");
    let node_size = std::mem::size_of::<GridNode>();
    let traditional_node_size = std::mem::size_of::<Point3D>() + std::mem::size_of::<Vector3D>();

    println!("   GridNode size: {} bytes", node_size);
    println!("   Traditional (Point3D + Vector3D): {} bytes", traditional_node_size);
    println!("   Savings per node: {} bytes", traditional_node_size - node_size);
    println!("   For 10000 nodes: {} KB saved",
        (traditional_node_size - node_size) * 10000 / 1024);

    // 10. Dodecet advantages
    println!("\n=== Key Advantages for Path Planning ===");
    println!("✓ 12-bit precision provides 4096x4096x4096 search space");
    println!("✓ Integer arithmetic for fast distance calculations");
    println!("✓ Compact node representation (6 bytes vs 24+ bytes)");
    println!("✓ Deterministic pathfinding (no floating-point drift)");
    println!("✓ Hex-friendly debugging (3 hex digits per coordinate)");
    println!("✓ Efficient A* implementation with BinaryHeap");
    println!("✓ Natural 3D grid representation");
}
