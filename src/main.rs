// src/main.rs

use hylaean_path::ecs::{World, Position, Velocity, gravity_system, propagate_system, proximity_detection_system};
use rand::Rng;
use std::f64::consts::TAU;

fn main() {
    let gravitational_parameter = 3.986004418e14; // Earth's gravitational parameter in m³/s²
    let dt = 10.0; // time step in seconds
    let proximity_threshold = 10_000.0; // 100 km in meters
    let n_satellites = 10_000;

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

        // Choose a reference vector that is not parallel to pos.
        let (ref_x, ref_y, ref_z) = if pos.x.abs() < 1e-6 && pos.y.abs() < 1e-6 {
            (1.0, 0.0, 0.0)
        } else {
            (0.0, 0.0, 1.0)
        };

        // Compute cross product: velocity direction = pos × ref.
        let vx = pos.y * ref_z - pos.z * ref_y;
        let vy = pos.z * ref_x - pos.x * ref_z;
        let vz = pos.x * ref_y - pos.y * ref_x;
        let norm = (vx * vx + vy * vy + vz * vz).sqrt();
        let vel = Velocity {
            dx: (vx / norm) * orbital_velocity,
            dy: (vy / norm) * orbital_velocity,
            dz: (vz / norm) * orbital_velocity,
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
