use itertools::Itertools;

type Position = (i32, i32, i32);
type Velocity = (i32, i32, i32);
#[derive(Debug)]
pub struct Moon {
    pos: Position,
    vel: Velocity
}

impl Moon {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self {
            pos: (x, y, z),
            vel: (0, 0, 0)
        }
    }
    pub fn from(pos: Position, vel: Velocity) -> Self {
        Self {
            pos,
            vel,
        }
    }
}
fn apply_gravity(moon: &Moon, moon_2: &Moon) -> (Moon, Moon)
{
    let (p1, v1) = (moon.pos, moon.vel);
    let (p2, v2) = (moon_2.pos, moon_2.vel);
    let sign_vel12_x = num::signum(p2.0 - p1.0);
    let sign_vel12_y = num::signum(p2.1 - p1.1);
    let sign_vel12_z = num::signum(p2.2 - p1.2);

    let v1 = (v1.0 + sign_vel12_x*1, v1.1 + sign_vel12_y*1, v1.2 + sign_vel12_z*1);
    let v2 = (v2.0 - sign_vel12_x*1, v2.1 - sign_vel12_y*1, v2.2 - sign_vel12_z*1);

    (Moon::from(p1, v1), Moon::from(p2, v2))
}

fn apply_velocity(moon: &Moon) -> Moon
{
    let (p1, v1) = (moon.pos, moon.vel);
    let p1 = (p1.0 + v1.0, p1.1 + v1.1, p1.2 + v1.2);
    
    Moon::from(p1, v1)
}

fn potential_energy(moon: &Moon) -> i32
{
    moon.pos.0.abs() + moon.pos.1.abs() + moon.pos.2.abs()
}
fn kinetic_energy(moon: &Moon) -> i32
{
    moon.vel.0.abs() + moon.vel.1.abs() + moon.vel.2.abs()
}
fn total_energy(moons: &Vec<Moon>) -> i32
{
    moons.iter().map(|m| potential_energy(m) * kinetic_energy(m)).sum()
}

fn simulate(mut moons: Vec<Moon>, num_iterations: i32) -> Vec<Moon>
{
    for _ in 0..num_iterations
    {
        for m12 in (0..4).combinations(2)
        {
            let i = m12[0];
            let j = m12[1];
            let m1 = &moons[i];
            let m2 = &moons[j];

            let (m1, m2) = apply_gravity(m1, m2);
            
            moons[i] = m1;
            moons[j] = m2;
        }
        
        for m in moons.iter_mut()
        {
            *m = apply_velocity(m);
        }
    }
    moons
}
fn main()
{
    let mut moons = Vec::new();


    moons.push(Moon::new(16, -8, 13));
    moons.push(Moon::new(4, 10,10));
    moons.push(Moon::new(17, -5, 6));
    moons.push(Moon::new(13, -3, 0));

    let moons = simulate(moons, 1000);
    let part_a = total_energy(&moons);
    println!("Part A: {}", part_a);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_day12_part_a()
    {
        let mut moons = Vec::new();
        moons.push(Moon::new(-1, 0, 2));
        moons.push(Moon::new(2, -10,-7));
        moons.push(Moon::new(4, -8, 8));
        moons.push(Moon::new(3, 5, -1));

        let moons = simulate(moons, 1);
        assert_eq!(moons[0].pos, (2, -1,  1));
        assert_eq!(moons[0].vel, (3, -1, -1));

        assert_eq!(moons[1].pos, (3, -7, -4));
        assert_eq!(moons[1].vel, (1,  3,  3));

        assert_eq!(moons[2].pos, (1, -7,  5));
        assert_eq!(moons[2].vel, (-3,  1, -3));

        assert_eq!(moons[3].pos, (2,  2,  0));
        assert_eq!(moons[3].vel, (-1, -3,  1));

        let moons = simulate(moons, 9);
        assert_eq!(total_energy(&moons), 179);


        let mut moons = Vec::new();
        moons.push(Moon::new(-8, -10, 0));
        moons.push(Moon::new(5, 5, 10));
        moons.push(Moon::new(2, -7, 3));
        moons.push(Moon::new(9, -8, -3));
        let moons = simulate(moons, 100);
        assert_eq!(total_energy(&moons), 1940);
    }   
}