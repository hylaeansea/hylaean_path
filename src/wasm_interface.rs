// src/wasm_interface.rs

use wasm_bindgen::prelude::*;
use crate::ecs;
use crate::ecs::{World, Position, Velocity, gravity_system, propagate_system, proximity_detection_system};
use rand::Rng;
use std::f64::consts::TAU;

#[wasm_bindgen]
pub struct Simulation {
    world: World,
    gravitational_parameter: f64,
    dt: f64,
}

#[wasm_bindgen]
impl Simulation {
    /// Creates a new simulation with `n_satellites` randomly generated.
    #[wasm_bindgen(constructor)]
    pub fn new(n_satellites: usize) -> Simulation {
        let gravitational_parameter = 3.986004418e14; // Earth's gravitational parameter (m³/s²)
        let dt = 10.0; // time step in seconds
        let mut world = World::new();
        let mut rng = rand::thread_rng();

        // Create n random satellites in full 3D space.
        for _ in 0..n_satellites {
            // Random orbital radius between 6.5e6 and 7.0e6 meters.
            let r: f64 = rng.gen_range(6.7e6..7.7e6);
            // Random azimuth angle (θ) in [0, 2π)
            let theta: f64 = rng.gen_range(0.0..TAU);
            // Random cosine of inclination (u) in [-1, 1]
            let u: f64 = rng.gen_range(-1.0..1.0);
            // Compute inclination φ = acos(u) and sin(φ)
            let phi = u.acos();
            let sin_phi = (1.0 - u * u).sqrt();

            // Convert spherical coordinates to Cartesian coordinates.
            let pos = Position {
                x: r * sin_phi * theta.cos(),
                y: r * sin_phi * theta.sin(),
                z: r * u,
            };

            // Compute orbital velocity for a circular orbit: v = √(μ / r)
            let orbital_velocity = (gravitational_parameter / r).sqrt();

            // Choose a reference vector that's not parallel to the position.
            let (ref_x, ref_y, ref_z) = if pos.x.abs() < 1e-6 && pos.y.abs() < 1e-6 {
                (1.0, 0.0, 0.0)
            } else {
                (0.0, 0.0, 1.0)
            };

            // Compute cross product to get a perpendicular vector: vel = pos × ref.
            let vx = pos.y * ref_z - pos.z * ref_y;
            let vy = pos.z * ref_x - pos.x * ref_z;
            let vz = pos.x * ref_y - pos.y * ref_x;
            let norm = (vx * vx + vy * vy + vz * vz).sqrt();

            let vel = Velocity {
                dx: (vx / norm) * orbital_velocity,
                dy: (vy / norm) * orbital_velocity,
                dz: (vz / norm) * orbital_velocity,
            };

            // Add the new satellite to the world.
            world.add_entity(pos, vel);
        }

        Simulation {
            world,
            gravitational_parameter,
            dt,
        }
    }

    /// Advances the simulation by one time step.
    #[wasm_bindgen]
    pub fn step(&mut self) {
        gravity_system(&mut self.world, self.dt, self.gravitational_parameter);
        propagate_system(&mut self.world, self.dt);
        
        // Set a reasonable threshold for proximity detection
        let proximity_threshold = 100000.0;
        
        // Get new warnings
        let new_warnings = proximity_detection_system(&self.world, proximity_threshold);
        
        // Clear and update warnings
        self.world.proximity_warnings.clear();
        self.world.proximity_warnings.extend(new_warnings);
    }

    /// Returns the positions of all satellites as a JS array of [x, y, z] values.
    #[wasm_bindgen]
    pub fn get_positions(&self) -> JsValue {
        let positions: Vec<[f64; 3]> = self.world.positions
            .iter()
            .map(|p| [p.x, p.y, p.z])
            .collect();
        JsValue::from_serde(&positions).unwrap()
    }

    /// Returns the IDs of satellites currently in proximity warning state
    #[wasm_bindgen]
    pub fn get_proximity_warnings(&self) -> JsValue {
        JsValue::from_serde(&self.world.proximity_warnings).unwrap()
    }
}
