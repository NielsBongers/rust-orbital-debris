
use std::fs::File; 
use std::fs::OpenOptions; 
use std::io::Write; 
use rand::Rng; 

struct Particle {
    name: String,

    x: f64, // position in x-axis
    y: f64, // position in y-axis
    z: f64, // position in z-axis
    v_x: f64, // velocity in x-axis
    v_y: f64, // velocity in y-axis
    v_z: f64, // velocity in z-axis
    mass: f64, // mass of the particle
}

#[allow(dead_code)]
impl Particle { 
    fn acceleration(&self, f: (f64, f64, f64)) -> (f64, f64, f64) { 
        let a_x = f.0 / self.mass; 
        let a_y = f.1 / self.mass; 
        let a_z = f.2 / self.mass; 

        return (a_x, a_y, a_z) 
    }

    #[allow(non_snake_case)]
    fn force(&self, p2: &Particle) -> ((f64, f64, f64), (f64, f64, f64)){ 
        let G = 6.674e-11; 
        
        let r = ((p2.x - self.x).powf(2.0) + (p2.y - self.y).powf(2.0) + (p2.z - self.z).powf(2.0)).sqrt(); 
        let F = G * self.mass * p2.mass / r.powf(2.0); 

        let (F_x, F_y, F_z): (f64, f64, f64) = (F * (p2.x - self.x)/r, F * (p2.y - self.y)/r, F * (p2.z - self.z)/r); 
        return ((F_x, F_y, F_z), (-F_x, -F_y, -F_z)); 
    }

    fn update_position(&mut self, dt: f64) { 
        self.x += self.v_x * dt; 
        self.y += self.v_y * dt; 
        self.z += self.v_z * dt; 
    }

    fn update_velocity(&mut self, acceleration: (f64, f64, f64), dt: f64) { 
        let (a_x, a_y, a_z) = acceleration; 

        self.v_x += 0.5 * a_x * dt; 
        self.v_y += 0.5 * a_y * dt; 
        self.v_z += 0.5 * a_z * dt; 
    }

    fn describe(&self) { 
        println!(
                r"Particle name: {}, Position: {}, {}, {}, Velocity: {}, {}, {}, Mass: {}\n", 
                self.name, self.x, self.y, self.z, self.v_x, self.v_y, self.v_z, self.mass); 
    }

    fn init(&self) { 
        // Emptying the file before writing and adding a header. 
        let file_name = format!("results/data/{}.csv", self.name); 
        let mut file = File::create(file_name).unwrap();
        write!(file, "t,x,y,z,v_x,v_y,v_z,mass\n").unwrap(); 
    }

    fn write(&self, t: f64) { 
        let file_name = format!("results/data/{}.csv", self.name); 
        let mut file = OpenOptions::new().append(true).write(true).create(true).open(file_name).unwrap(); 

        write!(file, "{}, {},{},{},{},{},{},{}\n", 
                t, self.x, self.y, self.z, self.v_x, self.v_y, self.v_z, self.mass).unwrap(); 
    }

}

fn velocity_verlet(p1: &mut Particle, p2: &mut Particle, dt: f64) { 
    let (p1_force_t, p2_force_t)= p1.force(&p2); 

    let p1_acceleration_t = p1.acceleration(p1_force_t); 
    let p2_acceleration_t = p2.acceleration(p2_force_t); 

    p1.update_velocity(p1_acceleration_t, dt); 
    p2.update_velocity(p2_acceleration_t, dt); 

    p1.update_position(dt); 
    p2.update_position(dt); 

    let (p1_force_tdt, p2_force_tdt)= p1.force(&p2); 

    let p1_acceleration_tdt = p1.acceleration(p1_force_tdt); 
    let p2_acceleration_tdt = p2.acceleration(p2_force_tdt); 

    p1.update_velocity(p1_acceleration_tdt, dt); 
    p2.update_velocity(p2_acceleration_tdt, dt); 
}

fn main(){

    // Parameters 
    let number_particles: i64 = 50; 
    let maximum_velocity = 300.0; 

    let mut t = 0.0; 
    let t_end = 2.0 * 4865. + 500.; 
    let dt = 10e-0; 

    // Initializing all the objects. 
    let mut earth = Particle { 
        name: String::from("Earth"), 
        x: 0.0,
        y: 0.0,
        z: 0.0,
        v_x: 0.0,
        v_y: 0.0,
        v_z: 0.0,
        mass: 5.972e24,
    }; 

    let mut object_vector: Vec<Particle> = Vec::new(); 
    let mut rng = rand::thread_rng();

    for i in 0..number_particles { 

        let default_particle: Particle = Particle { 
            name: format!("particle {}", i.to_string()), 

            x: 6371000.0 + 413000.0, 
            y: 0.0,
            z: 0.0,
            v_x: 0.0 + 2.0 * (rng.gen::<f64>() - 0.5) * maximum_velocity,
            v_y: 7700.0 + 2.0 * (rng.gen::<f64>() - 0.5) * maximum_velocity, 
            v_z: 0.0,
            mass: 100.0,
            }; 

            default_particle.init(); 
            object_vector.push(default_particle)
    } 

    // Running the simulation. 
    while t < t_end {

        for mut particle in object_vector.iter_mut() { 
            velocity_verlet(&mut earth, &mut particle, dt); 
            particle.write(t); 
        }

        t += dt; 
    } 

    // Visualizing using Python. 
    let mut cmd = std::process::Command::new("C:\\Program Files\\Python310\\python.exe");
    cmd.arg("D:\\Google Drive\\Rust\\velocity_verlet\\results\\post_processing.py"); 
    cmd.output().unwrap(); 

    println!("Done!"); 
    
}