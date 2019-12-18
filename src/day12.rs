use std::collections::HashMap;
use std::error::Error;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}
impl Point3D {
    fn zero() -> Self {
        Point3D { x: 0, y: 0, z: 0 }
    }
}

impl std::ops::AddAssign for Point3D {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Moon {
    position: Point3D,
    velocity: Point3D,
}
impl Moon {
    fn new(position: Point3D) -> Self {
        let velocity = Point3D::zero();
        Moon { position, velocity }
    }

    fn update_position(&mut self) {
        self.position += self.velocity;
    }

    fn potential_energy(&self) -> i32 {
        self.position.x.abs() + self.position.y.abs() + self.position.z.abs()
    }

    fn kinetic_energy(&self) -> i32 {
        self.velocity.x.abs() + self.velocity.y.abs() + self.velocity.z.abs()
    }

    fn total_energy(&self) -> i32 {
        self.potential_energy() * self.kinetic_energy()
    }

    fn print(&self) {
        let p = self.position;
        let v = self.velocity;
        print!("pos=<x={}, y={}, z={}>, ", p.x, p.y, p.z);
        println!("vel=<x={}, y={}, z={}>, ", v.x, v.y, v.z);
    }
}

pub fn main() -> Result<(), Box<dyn Error>> {
    let part1_ans = {
        let input = get_input();
        let mut system = MoonSystem { moons: input };

        for _ in 0..1000 {
            system.step();
        }

        system.total_energy()
    };
    println!("{}", part1_ans);

    let input = get_input();
    let mut system = MoonSystem { moons: input };

    let mut seen_x_states = HashMap::new();
    seen_x_states.insert(system.get_x_state(), 0);

    let mut seen_y_states = HashMap::new();
    seen_y_states.insert(system.get_y_state(), 0);

    let mut seen_z_states = HashMap::new();
    seen_z_states.insert(system.get_z_state(), 0);

    let mut x_period_start = None;
    let mut y_period_start = None;
    let mut z_period_start = None;

    let mut x_period = None;
    let mut y_period = None;
    let mut z_period = None;

    system.print(0);

    let mut step: usize = 1;

    while x_period.is_none() || y_period.is_none() || z_period.is_none()
    {
        system.step();

        let x_state = system.get_x_state();
        if let Some(&previous_step) = seen_x_states.get(&x_state) {
            if x_period.is_none() {
                x_period_start.replace(previous_step);
                x_period.replace(step - previous_step);
                println!("found x period! {} -> {}", previous_step, step);
                system.print(step);
            }
        }
        seen_x_states.insert(x_state, step);

        let y_state = system.get_y_state();
        if let Some(&previous_step) = seen_y_states.get(&y_state) {
            if y_period.is_none() {

                y_period_start.replace(previous_step);
                y_period.replace(step - previous_step);
                println!("found y period! {} -> {}", previous_step, step);
                system.print(step);
            }
        }
        seen_y_states.insert(y_state, step);

        let z_state = system.get_z_state();
        if let Some(&previous_step) = seen_z_states.get(&z_state) {
            if z_period.is_none(){

                z_period_start.replace(previous_step);
                z_period.replace(step - previous_step);
                println!("found z period! {} -> {}", previous_step, step);
                system.print(step);
            }
        }

        seen_z_states.insert(z_state, step);

        step += 1;
    }

    let x_period: usize = x_period.unwrap();
    let y_period = y_period.unwrap();
    let z_period = z_period.unwrap();

    println!(
        "{}, {}, {}",
        x_period, y_period, z_period
    );
    use num::Integer;
    let lcm_period = x_period.lcm(&y_period).lcm(&z_period);
    println!("lcm: {}", lcm_period);

    Ok(())
}

struct MoonSystem {
    moons: Vec<Moon>,
}
impl MoonSystem {
    pub fn step(&mut self) {
        self.apply_gravity();
        self.update_positions();
    }

    fn print(&self, n_steps: usize) {
        println!("after {} steps", n_steps);
        for moon in self.moons.iter() {
            moon.print();
        }
        println!("")
    }

    fn get_x_state(&self) -> Vec<(i32,i32)> {
        self.moons.iter().map(|m| (m.position.x, m.velocity.x)).collect()
    }
    fn get_y_state(&self) -> Vec<(i32,i32)>  {
        self.moons.iter().map(|m| (m.position.y, m.velocity.y)).collect()
    }
    fn get_z_state(&self) -> Vec<(i32,i32)> {
        self.moons.iter().map(|m| (m.position.z, m.velocity.z)).collect()
    }

    fn apply_gravity(&mut self) {
        let moons_len = self.moons.len();

        for i in 0..moons_len {
            for j in 0..moons_len {
                if i != j {
                    let other_moon = self.moons.get(j).copied().unwrap();
                    let moon = self.moons.get_mut(i).unwrap();

                    let x_change = (other_moon.position.x - moon.position.x).signum();
                    let y_change = (other_moon.position.y - moon.position.y).signum();
                    let z_change = (other_moon.position.z - moon.position.z).signum();

                    moon.velocity.x += x_change;
                    moon.velocity.y += y_change;
                    moon.velocity.z += z_change;
                }
            }
        }
    }

    fn update_positions(&mut self) {
        for moon in self.moons.iter_mut() {
            moon.update_position()
        }
    }

    fn total_energy(&self) -> i32 {
        self.moons.iter().map(|m| m.total_energy()).sum()
    }
}

fn get_test_input() -> Vec<Moon> {
    vec![
        Moon::new(Point3D { x: -1, y: 0, z: 2 }),
        Moon::new(Point3D {
            x: 2,
            y: -10,
            z: -7,
        }),
        Moon::new(Point3D { x: 4, y: -8, z: 8 }),
        Moon::new(Point3D { x: 3, y: 5, z: -1 }),
    ]
}

fn get_input() -> Vec<Moon> {
    vec![
        Moon::new(Point3D {
            x: 13,
            y: -13,
            z: -2,
        }),
        Moon::new(Point3D {
            x: 16,
            y: 2,
            z: -15,
        }),
        Moon::new(Point3D {
            x: 7,
            y: -18,
            z: -12,
        }),
        Moon::new(Point3D {
            x: -3,
            y: -8,
            z: -8,
        }),
    ]
}
