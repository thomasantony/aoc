use ::aoc2019::vec3::Vec3;
use num::signum;

fn compute_gravity(positions: &[Vec3]) -> Vec<Vec3> {
    let mut output = Vec::new();
    for p1 in positions.iter() {
        let mut d_v = Vec3::default();
        for p2 in positions.iter() {
            let p12 = p2 - p1;
            let sgn_x = signum(p12.x);
            let sgn_y = signum(p12.y);
            let sgn_z = signum(p12.z);

            d_v = d_v + Vec3::new(sgn_x, sgn_y, sgn_z);
        }
        output.push(d_v);
    }
    output
}

// One time step for all bodies
fn single_step(pos: &mut [Vec3], vel: &mut [Vec3]) {
    let d_v = compute_gravity(pos);
    for i in 0..pos.len() {
        vel[i] = vel[i] + d_v[i];
        pos[i] = pos[i] + vel[i];
    }
}

fn simulate(start_pos: &[Vec3], num_iterations: i64) -> (Vec<Vec3>, Vec<Vec3>) {
    let mut pos: Vec<Vec3> = start_pos.iter().cloned().collect();
    let mut vel = vec![Vec3::default(); pos.len()];
    for _ in 0..(num_iterations) {
        single_step(&mut pos, &mut vel);
    }
    (pos, vel)
}
// Assumption:
// Transition steps are reversible.
// => Each step has a single predecessor
// => If there is a cycle, it *has* to hit the initial state
//     => If cycle starts elsewhere, this would mean there is a state with two predecessors
//        which is a contradiction
// Therefore it is enough to check if the initial state repeats
// Additionally, since initial vel is zero and all 3 states propagate independently
// It is enough to find cycle lengths for vx, vy and vz
// and then find the LCM as half the cycle length

fn detect_cycle_in_component(start_pos: &[Vec3], component_index: usize) -> usize {
    let mut pos: Vec<Vec3> = start_pos.iter().cloned().collect();
    let mut vel = vec![Vec3::default(); pos.len()];
    let mut i = 1;
    single_step(&mut pos, &mut vel);
    while !vel.iter().all(|v| v.get(component_index) == 0) {
        single_step(&mut pos, &mut vel);
        i += 1;
    }
    return 2 * i;
}
fn solve_part_a(moons: &[Vec3]) -> i64 {
    let (pos, vel) = simulate(&moons, 1000);

    let potential_energy = pos.iter().map(|p| p.l1_norm());
    let kinetic_energy = vel.iter().map(|v| v.l1_norm());

    potential_energy
        .zip(kinetic_energy)
        .fold(0, |acc, (ke, pe)| acc + ke * pe)
}
fn solve_part_b(moons: &[Vec3]) -> usize {
    use num::Integer;
    let x_cycle = detect_cycle_in_component(moons, 0);
    let y_cycle = detect_cycle_in_component(moons, 1);
    let z_cycle = detect_cycle_in_component(moons, 2);

    x_cycle.lcm(&y_cycle).lcm(&z_cycle)
}
fn main() {
    let mut moons = Vec::new();

    moons.push(Vec3::new(16, -8, 13));
    moons.push(Vec3::new(4, 10, 10));
    moons.push(Vec3::new(17, -5, 6));
    moons.push(Vec3::new(13, -3, 0));

    let part_a = solve_part_a(&moons);
    println!("Part A: {}", part_a);

    let part_b = solve_part_b(&moons);
    println!("Part B: {}", part_b);
}

#[cfg(test)]
mod tests {
    use super::*;
    fn sample_data_1() -> Vec<Vec3> {
        let mut moons = Vec::new();
        moons.push(Vec3::new(-1, 0, 2));
        moons.push(Vec3::new(2, -10, -7));
        moons.push(Vec3::new(4, -8, 8));
        moons.push(Vec3::new(3, 5, -1));
        moons
    }
    fn sample_data_2() -> Vec<Vec3> {
        let mut moons = Vec::new();
        moons.push(Vec3::new(-8, -10, 0));
        moons.push(Vec3::new(5, 5, 10));
        moons.push(Vec3::new(2, -7, 3));
        moons.push(Vec3::new(9, -8, -3));
        moons
    }
    #[test]
    fn test_day12_part_a() {
        let moons = sample_data_1();

        let (moon_pos, moon_vel) = simulate(&moons, 1);
        assert_eq!(moon_pos[0], Vec3::new(2, -1, 1));
        assert_eq!(moon_vel[0], Vec3::new(3, -1, -1));
        assert_eq!(moon_pos[1], Vec3::new(3, -7, -4));
        assert_eq!(moon_vel[1], Vec3::new(1, 3, 3));
        assert_eq!(moon_pos[2], Vec3::new(1, -7, 5));
        assert_eq!(moon_vel[2], Vec3::new(-3, 1, -3));
        assert_eq!(moon_pos[3], Vec3::new(2, 2, 0));
        assert_eq!(moon_vel[3], Vec3::new(-1, -3, 1));
    }
    #[test]
    fn test_day12_part_b() {
        let moons = sample_data_1();
        assert_eq!(solve_part_b(&moons), 2772);

        let moons = sample_data_2();
        assert_eq!(solve_part_b(&moons), 4686774924);
    }
}
