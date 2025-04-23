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
            // Generate a random orbital radius between 6.5e6 and 7.0e6 meters.
            let r: f64 = rng.gen_range(7.6e6..7.601e6);

            // Random azimuth angle (θ) in [0, 2π)
            let theta = rng.gen_range(0.0..TAU);
            // Random cosine of inclination (u) in [-1, 1]
            let u: f64 = rng.gen_range(-1.0..1.0);
            // Inclination φ = acos(u); sin(φ) = sqrt(1 - u²)
            // let phi = u.acos();
            let sin_phi = (1.0 - u * u).sqrt();
    
            // Convert spherical coordinates to Cartesian coordinates.
            let pos = Position {
                x: r * sin_phi * theta.cos(),
                y: r * sin_phi * theta.sin(),
                z: r * u,
            };
    
            // 1. Pick a small eccentricity (e.g. up to 0.1 for “nearly” circular)
            let e: f64 = rng.gen_range(0.0..0.001);
    
            // 2. Pick a random true anomaly ν in [0, 2π)
            let nu: f64 = rng.gen_range(0.0..TAU);
    
            // 3. Compute the semi‑latus rectum p = r * (1 + e cos ν)
            let p = r * (1.0 + e * nu.cos());
    
            // 4. Compute radial & tangential speeds for an ellipse
            //    Vᵣ = √(μ/p) · e · sin ν
            //    Vₜ = √(μ/p) · (1 + e cos ν)
            let mu = gravitational_parameter;
            let sqrt_mu_p = (mu / p).sqrt();
            let vr = sqrt_mu_p * e * nu.sin();
            let vt = sqrt_mu_p * (1.0 + e * nu.cos());
    
            // 5. Unit radial vector r̂ = pos / r
            let r_hat = (pos.x / r, pos.y / r, pos.z / r);
    
            // 6. Pick a random “reference” vector and orthogonalize to r̂ to get the orbital plane normal
            let n_hat = loop {
                let candidate = (
                    rng.gen_range(-1.0..1.0),
                    rng.gen_range(-1.0..1.0),
                    rng.gen_range(-1.0..1.0),
                );
                // cross(r̂, candidate)
                let cross = (
                    r_hat.1 * candidate.2 - r_hat.2 * candidate.1,
                    r_hat.2 * candidate.0 - r_hat.0 * candidate.2,
                    r_hat.0 * candidate.1 - r_hat.1 * candidate.0,
                );
                let norm = (cross.0*cross.0 + cross.1*cross.1 + cross.2*cross.2).sqrt();
                if norm > 1e-6 {
                    break (cross.0 / norm, cross.1 / norm, cross.2 / norm);
                }
            };
    
            // 7. Tangential unit vector θ̂ = cross(n̂, r̂)
            let theta_hat = (
                n_hat.1 * r_hat.2 - n_hat.2 * r_hat.1,
                n_hat.2 * r_hat.0 - n_hat.0 * r_hat.2,
                n_hat.0 * r_hat.1 - n_hat.1 * r_hat.0,
            );
    
            // 8. Combine radial + tangential components
            let vel = Velocity {
                dx: vr * r_hat.0 + vt * theta_hat.0,
                dy: vr * r_hat.1 + vt * theta_hat.1,
                dz: vr * r_hat.2 + vt * theta_hat.2,
            };
    
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
