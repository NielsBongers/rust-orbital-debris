use std::fs; 
use std::fs::File; 
use std::fs::OpenOptions; 
use std::io::Write; 

pub struct Particle {
    pub name: String,

    pub x: f64, // position in x-axis
    pub y: f64, // position in y-axis
    pub z: f64, // position in z-axis
    pub v_x: f64, // velocity in x-axis
    pub v_y: f64, // velocity in y-axis
    pub v_z: f64, // velocity in z-axis
    pub mass: f64, // mass of the particle

    pub deorbited: bool
}

#[allow(dead_code)]
impl Particle { 
    pub fn acceleration(&self, f: (f64, f64, f64)) -> (f64, f64, f64) { 
        let a_x = f.0 / self.mass; 
        let a_y = f.1 / self.mass; 
        let a_z = f.2 / self.mass; 

        return (a_x, a_y, a_z); 
    }

    pub fn particle_distance(&self, p2: &Particle) -> f64 { 
        let r = ((p2.x - self.x).powf(2.0) + (p2.y - self.y).powf(2.0) + (p2.z - self.z).powf(2.0)).sqrt(); 
        return r; 
    }

    pub fn distance_to_origin(&self) -> f64 { 
        let r = (self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0)).sqrt(); 
        return r; 
    }

    pub fn has_deorbited(&self) -> bool { 
        let r: f64 = self.distance_to_origin(); 

        let earth_surface_radius = 6371.*1000.; 

        if r <= earth_surface_radius { 
            return true; 
        }
        else { 
            return false; 
        }
    }

    #[allow(non_snake_case)]
    pub fn gravity(&self, p2: &Particle) -> (f64, f64, f64){ 
        let G = 6.674e-11; 
        
        let r = self.particle_distance(p2); 
        let F = G * self.mass * p2.mass / r.powf(2.0); 

        let (F_x, F_y, F_z): (f64, f64, f64) = (F * (p2.x - self.x)/r, F * (p2.y - self.y)/r, F * (p2.z - self.z)/r); 
        return (F_x, F_y, F_z); 
    }

    #[allow(non_snake_case)]
    pub fn drag(&self) -> (f64, f64, f64){ 

        let earth_surface_radius = 6371.*1000.; 
        let h: f64 = (self.distance_to_origin() - earth_surface_radius)/1000.; 

        let T = 900. + 2.5 * (129.0 - 70.0) + 1.5 * 26.0; 
        let m: f64 = 27. - 0.012 * (h - 200.0); 
        let rho = 6e-10 * ((-h-175.)*m/T).exp(); 

        let C_D = 0.5;  
        let A = 0.5; 

        let F_x = 0.5 * C_D * rho * A * self.v_x.powf(2.0); 
        let F_y = 0.5 * C_D * rho * A * self.v_y.powf(2.0); 
        let F_z = 0.5 * C_D * rho * A * self.v_z.powf(2.0); 

        return (F_x, F_y, F_z); 
    }

    #[allow(non_snake_case)]
    pub fn forces(&self, p2: &Particle) -> (f64, f64, f64){ 
        let (F_x_grav, F_y_grav, F_z_grav) = self.gravity(p2);
        let (F_x_drag, F_y_drag, F_z_drag) = self.drag(); 

        let F_x = F_x_grav + F_x_drag; 
        let F_y = F_y_grav + F_y_drag; 
        let F_z = F_z_grav + F_z_drag; 

        return (F_x, F_y, F_z); 
    }

    pub fn update_position(&mut self, dt: f64) { 
        self.x += self.v_x * dt; 
        self.y += self.v_y * dt; 
        self.z += self.v_z * dt; 
    }

    pub fn update_velocity(&mut self, acceleration: (f64, f64, f64), dt: f64) { 
        let (a_x, a_y, a_z) = acceleration; 

        self.v_x += 0.5 * a_x * dt; 
        self.v_y += 0.5 * a_y * dt; 
        self.v_z += 0.5 * a_z * dt; 
    }

    pub fn describe(&self) { 
        println!(
                r"Particle name: {}, Position: {}, {}, {}, Velocity: {}, {}, {}, Mass: {}\n", 
                self.name, self.x, self.y, self.z, self.v_x, self.v_y, self.v_z, self.mass); 
    }

    pub fn init(&self) { 
        // Emptying the file before writing and adding a header. 
        fs::create_dir_all("results/data").unwrap(); 
        let file_name = format!("results/data/{}.csv", self.name); 
        let mut file = File::create(file_name).unwrap();
        write!(file, "t,x,y,z,v_x,v_y,v_z,mass\n").unwrap(); 
    }

    pub fn write(&self, t: f64) { 
        let file_name = format!("results/data/{}.csv", self.name); 
        let mut file = OpenOptions::new().append(true).write(true).create(true).open(file_name).unwrap(); 

        write!(file, "{}, {},{},{},{},{},{},{}\n", 
                t, self.x, self.y, self.z, self.v_x, self.v_y, self.v_z, self.mass).unwrap(); 
    }

}