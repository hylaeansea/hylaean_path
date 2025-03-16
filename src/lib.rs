// src/lib.rs

/// This module contains the core ECS implementation.
pub mod ecs {
    use std::collections::HashMap;

    #[derive(Debug, Clone)]
    pub struct Position {
        pub x: f64,
        pub y: f64,
        pub z: f64,
    }

    #[derive(Debug, Clone)]
    pub struct Velocity {
        pub dx: f64,
        pub dy: f64,
        pub dz: f64,
    }

    pub type EntityId = usize;

    pub struct World {
        pub positions: HashMap<EntityId, Position>,
        pub velocities: HashMap<EntityId, Velocity>,
        pub next_entity: EntityId,
    }

    impl World {
        /// Creates a new, empty world.
        pub fn new() -> Self {
            Self {
                positions: HashMap::new(),
                velocities: HashMap::new(),
                next_entity: 0,
            }
        }

        /// Creates a new entity and returns its ID.
        pub fn create_entity(&mut self) -> EntityId {
            let entity = self.next_entity;
            self.next_entity += 1;
            entity
        }

        /// Attaches a position component to an entity.
        pub fn add_position(&mut self, entity: EntityId, position: Position) {
            self.positions.insert(entity, position);
        }

        /// Attaches a velocity component to an entity.
        pub fn add_velocity(&mut self, entity: EntityId, velocity: Velocity) {
            self.velocities.insert(entity, velocity);
        }
    }

    /// The gravity system updates velocities based on Earth's gravitational pull.
    ///
    /// It uses Euler integration: v += a * dt, where acceleration
    /// a = -μ * (r / |r|³), with μ being Earth's gravitational parameter.
    pub fn gravity_system(world: &mut World, dt: f64, gravitational_parameter: f64) {
        for (entity, pos) in world.positions.iter() {
            if let Some(vel) = world.velocities.get_mut(entity) {
                let r = (pos.x * pos.x + pos.y * pos.y + pos.z * pos.z).sqrt();
                if r > 0.0 {
                    let accel_factor = -gravitational_parameter / (r * r * r);
                    vel.dx += accel_factor * pos.x * dt;
                    vel.dy += accel_factor * pos.y * dt;
                    vel.dz += accel_factor * pos.z * dt;
                }
            }
        }
    }

    /// The propagation system updates positions based on their velocities.
    /// new_position = old_position + velocity * dt
    pub fn propagate_system(world: &mut World, dt: f64) {
        for (entity, pos) in world.positions.iter_mut() {
            if let Some(vel) = world.velocities.get(entity) {
                pos.x += vel.dx * dt;
                pos.y += vel.dy * dt;
                pos.z += vel.dz * dt;
            }
        }
    }

    /// The proximity detection system checks for any two satellites that are within a specified threshold.
    ///
    /// If the distance between any two satellites is less than `threshold` (in meters),
    /// it prints a warning.
    pub fn proximity_detection_system(world: &World, threshold: f64) {
        let entities: Vec<_> = world.positions.keys().copied().collect();
        for i in 0..entities.len() {
            for j in (i + 1)..entities.len() {
                let e1 = entities[i];
                let e2 = entities[j];
                let pos1 = world.positions.get(&e1).unwrap();
                let pos2 = world.positions.get(&e2).unwrap();
                let dx = pos1.x - pos2.x;
                let dy = pos1.y - pos2.y;
                let dz = pos1.z - pos2.z;
                let distance = (dx * dx + dy * dy + dz * dz).sqrt();
                if distance < threshold {
                    println!(
                        "Warning: Satellites {} and {} are within {:.2} m (distance = {:.2} m)",
                        e1, e2, threshold, distance
                    );
                }
            }
        }
    }
}
