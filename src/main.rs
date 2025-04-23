// src/main.rs

use hylaean_path::ecs::{World, Position, Velocity, gravity_system, propagate_system, proximity_detection_system};
use rand::Rng;
use std::f64::consts::TAU;

fn main() {
    let gravitational_parameter = 3.986004418e14; // Earth's gravitational parameter in m³/s²
    let dt = 10.0; // time step in seconds
    let proximity_threshold = 10_000.0; // 100 km in meters
    let n_satellites = 200;

    let mut world = World::new();
    let mut rng = rand::thread_rng();

    // Create n random satellites with positions in full 3D space.
    for _ in 0..n_satellites {
        // Generate a random orbital radius between 6.5e6 and 7.0e6 meters.
        let r: f64 = rng.gen_range(6.5e6..7.0e6);
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
        let e: f64 = rng.gen_range(0.0..0.4);

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

    println!("Simulating {} satellites...", n_satellites);

    // Simulation loop.
    for step in 0..10_000 {
        gravity_system(&mut world, dt, gravitational_parameter);
        propagate_system(&mut world, dt);
        proximity_detection_system(&world, proximity_threshold);

        if step % 100 == 0 {
            println!("Step {}:", step);
            // for (i, pos) in world.positions.iter().enumerate() {
            //     let vel = &world.velocities[i];
            //     println!(
            //         "Satellite {}: Position = ({:.2}, {:.2}, {:.2}) m, Velocity = ({:.2}, {:.2}, {:.2}) m/s",
            //         i, pos.x, pos.y, pos.z, vel.dx, vel.dy, vel.dz
            //     );
            // }
        }
    }
}
