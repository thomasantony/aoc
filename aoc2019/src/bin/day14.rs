/// Day 14 - Space Stoichiometry
///
/// Need to parse recipe book and figure out number of ORE needed to make 1 unit of fuel
///
/// Part 1
///
/// 1. Keep track of the recipes in a recipebook hashmap
/// 2. A "need to make" hashmap is used. The main loop runs as long as this is not empty.
/// 3. Fetch item to make. Find out how many to make.
/// 4. See if we have enough in "leftovers" pile and subtract from that
/// 5. Figure out recipe and find out how many of the recipe to make based on requirement after leftovers.
/// 6. Push ingredients into need-to-make map, unless it is ORE, in which case we increment counter
///
/// Part 2
/// First attempt:
/// Use leftovers as well to figure out how many total FUEL can be made from 1 trillion units of ORE
/// Also, found that this would take too long. We can stop once the "leftovers" is empty.
/// At this point, the fuel counter has the amount of fuel produced by exactly the amount of ore used so far.
/// total_fuel = 1e12 / (ore_used) * fuel_counter
///
/// Second attempt:
/// Rather than back-calculating, use hint from https://0xdf.gitlab.io/adventofcode2019/14
/// Give in arbitrary count of FUEL to computation function, starting with estimate and find when it exceeds 1e12
/// Use binary search here?
///
/// Easier if all i32 are converted to i64. Ensure f32 is converted to f64 as well!!
///
/// Done!
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Component<'a> {
    name: &'a str,
    count: i64,
}

#[derive(Debug)]
pub struct Recipe<'a> {
    output_count: i64,
    ingredients: Vec<Component<'a>>,
}

impl<'a> From<&'a str> for Component<'a> {
    fn from(s: &'a str) -> Self {
        let parts = s.split(" ").collect::<Vec<&str>>();
        let count = parts[0].parse().expect("Error parsing count");
        let name = parts[1];
        Self { name, count }
    }
}

type RecipeBook<'a> = HashMap<&'a str, Recipe<'a>>;

fn parse_recipe_book(input: &str) -> RecipeBook {
    let lines = input.lines();

    let mut recipes: RecipeBook = HashMap::new();
    for line in lines {
        let parts = line.split(" => ").collect::<Vec<&str>>();
        let ingredients: Vec<Component> = parts[0].split(", ").map(Component::from).collect();
        let recipe_item = Component::from(parts[1]);
        let recipe = Recipe {
            output_count: recipe_item.count,
            ingredients,
        };
        recipes.insert(recipe_item.name, recipe);
    }
    recipes
}

fn compute_ore_required<'a>(recipes: &'a RecipeBook, required_qty: i64) -> i64 {
    let mut leftovers: HashMap<&str, i64> = HashMap::new();
    // Says how many of a certain item we need
    let mut need_to_make: HashMap<&str, i64> = HashMap::new();
    need_to_make.insert("FUEL", required_qty);

    let mut ore_counter = 0;
    // Still have things we need to make
    while !need_to_make.is_empty() {
        let first_key = need_to_make.keys().next().cloned().unwrap();
        let (item_name, mut item_qty) = need_to_make.remove_entry(&first_key).unwrap();

        // First check leftovers
        if let Some(leftover_qty) = leftovers.get(item_name).cloned() {
            let qty_from_leftovers = leftover_qty.min(item_qty);
            // Subtract the lower of leftover qty or item_qty
            item_qty -= qty_from_leftovers;
            *leftovers.entry(&item_name).or_default() -= qty_from_leftovers;
        }

        // Still need more, so we pull from recipe book
        if item_qty >= 0 {
            let recipe = recipes.get(&item_name).expect("Recipe not found");

            // Number of multiples of recipe item required to be produced
            let recipe_qty = (item_qty as f64 / recipe.output_count as f64).ceil() as i64;

            // Add all ingredients to required list or update ore counter if ore is needed
            // Multiply by item_qty to account for needing multiple units
            for ingredient in recipe.ingredients.iter() {
                let ingredient_qty = ingredient.count * recipe_qty;
                if ingredient.name == "ORE" {
                    ore_counter += ingredient_qty;
                } else {
                    *need_to_make.entry(ingredient.name).or_insert(0) += ingredient_qty;
                }
            }
            // Add any extras to leftovers
            let extra_output = recipe_qty * recipe.output_count - item_qty;
            *leftovers.entry(item_name.into()).or_insert(0) += extra_output;
        }
    }
    ore_counter
}

fn compute_max_fuel(recipes: &RecipeBook, ore_available: i64) -> i64 {
    let ore_for_one_fuel = compute_ore_required(&recipes, 1);

    let mut fuel_total_estimate = ore_available / ore_for_one_fuel as i64;
    // Start with doubling
    let mut delta = fuel_total_estimate;
    loop {
        let ore_req = compute_ore_required(&recipes, fuel_total_estimate);

        if ore_req > ore_available {
            fuel_total_estimate -= delta;
            delta /= 2;
            fuel_total_estimate += delta;
        } else {
            if delta == 0 {
                break;
            }
            fuel_total_estimate += delta;
        }
    }
    fuel_total_estimate
}
fn main() {
    let input = include_str!("../../inputs/day14.txt").to_string();
    let recipes = parse_recipe_book(&input);

    let part01_solution = compute_ore_required(&recipes, 1);
    println!("Part 1: {}", part01_solution);

    let ore_available = 1000000000000i64;
    println!("Part 2: {}", compute_max_fuel(&recipes, ore_available))
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUTS: [&str; 5] = [
        "10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL",
        "9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL",
        "157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT",
        "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
17 NVRVD, 3 JNWZP => 8 VPVL
53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
22 VJHF, 37 MNCFX => 5 FWMGM
139 ORE => 4 NVRVD
144 ORE => 7 JNWZP
5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
145 ORE => 6 MNCFX
1 NVRVD => 8 CXFTF
1 VJHF, 6 MNCFX => 4 RFSQX
176 ORE => 6 VJHF",
        "171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX",
    ];
    #[test]
    fn day_14_test_part_01() {
        let outputs = vec![31, 165, 13312, 180697, 2210736];

        for (input, &expected_output) in INPUTS.iter().zip(outputs.iter()) {
            let recipes = parse_recipe_book(&input);
            let output = compute_ore_required(&recipes, 1);
            assert_eq!(output, expected_output);
        }
    }
    #[test]
    fn day_14_test_part_02() {
        let outputs = vec![82892753, 5586022, 460664];

        for (input, &expected_output) in INPUTS[2..].iter().zip(outputs.iter()) {
            let recipes = parse_recipe_book(input);
            let output = compute_max_fuel(&recipes, 1_000_000_000_000i64);
            assert_eq!(output, expected_output);
            break;
        }
    }
}
