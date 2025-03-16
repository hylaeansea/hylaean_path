// src/lib.rs

/// This module contains the core ECS implementation using Vec storage and Rayon for parallelism.
pub mod ecs {
    use rayon::prelude::*;

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
        pub positions: Vec<Position>,
        pub velocities: Vec<Velocity>,
    }

    impl World {
        /// Creates a new, empty world.
        pub fn new() -> Self {
            Self {
                positions: Vec::new(),
                velocities: Vec::new(),
            }
        }

        /// Adds a new entity with a position and velocity, returning its entity id.
        pub fn add_entity(&mut self, position: Position, velocity: Velocity) -> EntityId {
            self.positions.push(position);
            self.velocities.push(velocity);
            self.positions.len() - 1 // entity id is the last index
        }
    }

    /// The gravity system updates velocities based on Earth's gravitational pull.
    ///
    /// It uses Euler integration: v += a * dt, where acceleration
    /// a = -μ * (r / |r|³), with μ being Earth's gravitational parameter.
    pub fn gravity_system(world: &mut World, dt: f64, gravitational_parameter: f64) {
        world.positions
            .par_iter()
            .zip(world.velocities.par_iter_mut())
            .for_each(|(pos, vel)| {
                let r = (pos.x * pos.x + pos.y * pos.y + pos.z * pos.z).sqrt();
                if r > 0.0 {
                    let accel_factor = -gravitational_parameter / (r * r * r);
                    vel.dx += accel_factor * pos.x * dt;
                    vel.dy += accel_factor * pos.y * dt;
                    vel.dz += accel_factor * pos.z * dt;
                }
            });
    }

    /// The propagation system updates positions based on their velocities.
    /// new_position = old_position + velocity * dt
    pub fn propagate_system(world: &mut World, dt: f64) {
        world.positions
            .par_iter_mut()
            .zip(world.velocities.par_iter())
            .for_each(|(pos, vel)| {
                pos.x += vel.dx * dt;
                pos.y += vel.dy * dt;
                pos.z += vel.dz * dt;
            });
    }

    /// The proximity detection system checks for any two satellites that are within a specified threshold.
    ///
    /// If the distance between any two satellites is less than `threshold` (in meters),
    /// it prints a warning.
    pub fn proximity_detection_system(world: &World, threshold: f64) {
        let positions = &world.positions;
        let len = positions.len();
        (0..len).into_par_iter().for_each(|i| {
            for j in (i + 1)..len {
                let pos1 = &positions[i];
                let pos2 = &positions[j];
                let dx = pos1.x - pos2.x;
                let dy = pos1.y - pos2.y;
                let dz = pos1.z - pos2.z;
                let distance = (dx * dx + dy * dy + dz * dz).sqrt();
                if distance < threshold {
                    println!(
                        "Warning: Satellites {} and {} are within {:.2} m (distance = {:.2} m)",
                        i, j, threshold, distance
                    );
                }
            }
        });
    }
}
