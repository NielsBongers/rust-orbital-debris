use rand::Rng;

mod particles;

fn velocity_verlet(p1: &mut particles::Particle, p2: &mut particles::Particle, dt: f64) {
    if p1.has_deorbited() {
        println!("{}", p1.name);
        p1.deorbited = true;
    }

    let p1_force_t = p1.forces(&p2);
    let p1_acceleration_t = p1.acceleration(p1_force_t);

    p1.update_velocity(p1_acceleration_t, dt);
    p1.update_position(dt);

    let p1_force_tdt = p1.gravity(&p2);
    let p1_acceleration_tdt = p1.acceleration(p1_force_tdt);

    p1.update_velocity(p1_acceleration_tdt, dt);
}

fn sample_characteristic_size() -> i64 {
    let mut rng = rand::thread_rng();

    let random_number: f64 = rng.gen::<f64>();

    let L_C_min: f64 = 0.05;
    let beta: f64 = -1.71;
    let M = 1000.0;

    let item_count: i64 = (((1.0 - random_number) * L_C_min.powf(beta)).powf(1.0 / beta)
        * L_C_min.powf(beta))
    .round() as i64;

    return item_count;
}

fn main() {

    

    let item_count = sample_characteristic_size();

    println!("Item count: {}", item_count);

    // // Parameters
    // let number_particles: i64 = 50;
    // let maximum_velocity = 300.;

    // let mut t = 0.0;
    // let t_end = 50000.0 * 95. * 60. + 500.;
    // // let t_end = 95.0 * 60.0;
    // let dt = 10e-0;

    // // Initializing all the objects.
    // let mut earth = particles::Particle {
    //     name: String::from("Earth"),
    //     x: 0.0,
    //     y: 0.0,
    //     z: 0.0,
    //     v_x: 0.0,
    //     v_y: 0.0,
    //     v_z: 0.0,
    //     mass: 5.972e24,
    //     deorbited: false,
    // };

    // let mut object_vector: Vec<particles::Particle> = Vec::new();
    // let mut rng = rand::thread_rng();

    // for i in 0..number_particles {
    //     let default_particle: particles::Particle = particles::Particle {
    //         name: format!("particle {}", i.to_string()),

    //         x: 6371000.0 + 413000.0,
    //         y: 0.0,
    //         z: 0.0,
    //         v_x: 0.0 + 2.0 * (rng.gen::<f64>() - 0.5) * maximum_velocity,
    //         v_y: 7700.0 + 2.0 * (rng.gen::<f64>() - 0.5) * maximum_velocity,
    //         v_z: 0.0,
    //         mass: 1.0,
    //         deorbited: false,
    //     };

    //     default_particle.init();
    //     object_vector.push(default_particle)
    // }

    // // Running the simulation.
    // while t < t_end {
    //     for mut particle in object_vector.iter_mut() {
    //         if particle.deorbited {
    //             continue;
    //         }
    //         velocity_verlet(&mut particle, &mut earth, dt);
    //         particle.write(t);
    //     }

    //     t += dt;
    // }

    // Visualizing using Python.
    // let mut cmd = std::process::Command::new("C:\\Program Files\\Python310\\python.exe");
    // cmd.arg("D:\\Google Drive\\Rust\\velocity_verlet\\results\\post_processing.py");
    // cmd.output().unwrap();

    println!("Done!");
}
