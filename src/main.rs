// src/main.rs

use hylaean_path::ecs::{
    World, Position, Velocity, propagate_system, gravity_system, proximity_detection_system,
};
use rand::Rng;

fn main() {
    let mut world = World::new();

    // Constants
    let gravitational_parameter: f64 = 3.986004418e14; // Earth's gravitational parameter in m³/s²
    let n_satellites = 100; // Number of random satellites to generate
    let proximity_threshold: f64 = 100_000.0; // 100 km expressed in meters

    let mut rng = rand::thread_rng();

    // Create n random satellites with positions in full 3D space.
    for _ in 0..n_satellites {
        // Random orbital radius between 6.5e6 and 7.0e6 meters.
        let r: f64 = rng.gen_range(6.5e6..7.0e6);
        // Random azimuth angle (θ) in [0, 2π)
        let theta = rng.gen_range(0.0..std::f64::consts::TAU);
        // Random cosine of inclination (u) uniformly distributed in [-1, 1]
        let u: f64 = rng.gen_range(-1.0..1.0);
        // Compute the inclination (φ) as acos(u). φ ∈ [0, π]
        let phi = u.acos();
        // sin(φ) can be computed as sqrt(1 - cos²(φ))
        let sin_phi = (1.0 - u * u).sqrt();

        // Convert spherical coordinates (r, θ, φ) to Cartesian (x, y, z)
        let pos_x = r * sin_phi * theta.cos();
        let pos_y = r * sin_phi * theta.sin();
        let pos_z = r * u; // since cos(φ) = u

        let pos = Position {
            x: pos_x,
            y: pos_y,
            z: pos_z,
        };

        // Compute orbital velocity for a circular orbit: v = √(μ / r)
        let orbital_velocity = (gravitational_parameter / r).sqrt();

        // Compute a velocity vector perpendicular to the position vector.
        // We choose a reference vector that is not parallel to the position.
        let (ref_x, ref_y, ref_z) = if pos_x.abs() < 1e-6 && pos_y.abs() < 1e-6 {
            (1.0, 0.0, 0.0)
        } else {
            (0.0, 0.0, 1.0)
        };

        // Compute cross product: velocity direction = pos x ref.
        let vx = pos_y * ref_z - pos_z * ref_y;
        let vy = pos_z * ref_x - pos_x * ref_z;
        let vz = pos_x * ref_y - pos_y * ref_x;
        let norm = (vx * vx + vy * vy + vz * vz).sqrt();
        let vdx = (vx / norm) * orbital_velocity;
        let vdy = (vy / norm) * orbital_velocity;
        let vdz = (vz / norm) * orbital_velocity;

        let vel = Velocity {
            dx: vdx,
            dy: vdy,
            dz: vdz,
        };

        let entity = world.create_entity();
        world.add_position(entity, pos);
        world.add_velocity(entity, vel);
    }

    let dt = 10.0; // time step in seconds
    println!("Simulating {} satellites with Earth's gravity in SI units:", n_satellites);

    // Run the simulation loop for 1000 steps.
    for step in 0..10000 {
        gravity_system(&mut world, dt, gravitational_parameter);
        propagate_system(&mut world, dt);
        proximity_detection_system(&world, proximity_threshold);

        // Print state every 100 steps.
        if step % 100 == 0 {
            println!("Step {}:", step);
            // for (entity, pos) in &world.positions {
            //     let vel = world.velocities.get(entity).unwrap();
            //     println!(
            //         "  Satellite {}: Position = ({:.2}, {:.2}, {:.2}) m, Velocity = ({:.2}, {:.2}, {:.2}) m/s",
            //         entity, pos.x, pos.y, pos.z, vel.dx, vel.dy, vel.dz
            //     );
            // }
        }
    }
}
