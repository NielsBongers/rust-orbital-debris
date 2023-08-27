
#[allow(dead_code)]
#[allow(unused_variables)]

use std::fs::File; 
use std::fs::OpenOptions; 
use std::io::Write; 

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

        // if self.name == "ISS" {
        //     println!("Acceleration: {}, {}, {}", a_x, a_y, a_z); 
        // }

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

    fn update(&mut self, acceleration: (f64, f64, f64), dt: f64) { 
        let (a_x, a_y, a_z) = acceleration; 

        self.v_x += a_x * dt; 
        self.v_y += a_y * dt; 
        self.v_z += a_z * dt; 

        self.x += self.v_x * dt; 
        self.y += self.v_y * dt; 
        self.z += self.v_z * dt; 

        // self.describe(); 
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

fn euler_integration(p1: &mut Particle, p2: &mut Particle, dt: f64) { 
    let (p1_force, p2_force)= p1.force(&p2); 

    let p1_acceleration = p1.acceleration(p1_force); 
    let p2_acceleration = p2.acceleration(p2_force); 

    p1.update(p1_acceleration, dt); 
    p2.update(p2_acceleration, dt); 
}

fn main(){
    let mut p1 = Particle { 
        name: String::from("Earth"), 
        x: 0.0,
        y: 0.0,
        z: 0.0,
        v_x: 0.0,
        v_y: 0.0,
        v_z: 0.0,
        mass: 5.972e24,
    }; 

    let mut p2 = Particle { 
        name: String::from("ISS"), 

        x: 6371.0*1000.0 + 413000.0, 
        y: 0.0,
        z: 0.0,
        v_x: 0.0,
        v_y: 7700.0,
        v_z: 0.0,
        mass: 100.0,
    }; 

    p1.init(); 
    p2.init(); 

    let mut t = 0.0; 
    let t_end = 5400.; 
    let dt = 10e-2; 

    while t < t_end {
        euler_integration(&mut p1, &mut p2, dt); 
        p1.write(t); 
        p2.write(t); 

        t += dt; 
    } 

    let mut cmd = std::process::Command::new("C:\\Program Files\\Python310\\python.exe");
    cmd.arg("D:\\Google Drive\\Rust\\velocity_verlet\\results\\post_processing.py"); 
    
    cmd.output().unwrap(); 
    
}