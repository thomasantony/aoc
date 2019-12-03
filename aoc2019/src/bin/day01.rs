use ::aoc2019::*;

fn calc_fuel(mass: i32) -> i32 {
    mass / 3 - 2
}

fn calc_fuel_cumulative(mass: i32) -> i32 {
    let mut fuel = 0;
    let mut input_mass = mass;
    loop
    {
        let new_fuel = calc_fuel(input_mass);
        if new_fuel <= 0
        {
            break;
        }
        fuel += new_fuel;
        input_mass = new_fuel;
    }
    fuel
}

fn main()
{
    let input = read_stdin();
    let masses = parse_numbers(&input);

    let mut total_a = 0;
    let mut total_b = 0;
    
    for mass in masses {
        total_a += calc_fuel(mass);
        total_b += calc_fuel_cumulative(mass);
    }
    println!("Part A, answer = {}", total_a);
    println!("Part B, answer = {}", total_b);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn unit_tests_day_01() {
        assert_eq!(calc_fuel(12), 2);
        assert_eq!(calc_fuel(14), 2);
        assert_eq!(calc_fuel(1969), 654);
        assert_eq!(calc_fuel(100756), 33583);

        assert_eq!(calc_fuel_cumulative(14), 2);
        assert_eq!(calc_fuel_cumulative(1969), 966);
        assert_eq!(calc_fuel_cumulative(100756), 50346);
    }
}