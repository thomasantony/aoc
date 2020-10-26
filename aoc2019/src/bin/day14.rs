use ::aoc2019::*;
use std::collections::HashMap;
use counter::Counter;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Component<'a> (&'a str, usize);

impl<'a> From<&'a str> for Component<'a>
{
    fn from(s: &'a str) -> Self {
        let parts = s.split(" ").collect::<Vec<&str>>();
        let count: usize = parts[0].parse().expect("Error parsing count");
        let name = parts[1];
        Self(name, count)
    }
}
fn main()
{
    let input = include_str!("../../inputs/day14.txt").to_string();
    let lines = input.lines();

    let mut recipes: HashMap<&str, (usize, Vec<Component>)> = HashMap::new();
    for line in lines
    {
        let parts = line.split(" => ").collect::<Vec<&str>>();
        let ingredients: Vec<Component> = parts[0].split(", ").map(Component::from).collect();
        let element = Component::from(parts[1]);
        recipes.insert(element.0, (element.1, ingredients));
    }
   
    let mut stack = vec![ Component("FUEL", 1)];
    let mut primary_materials: Counter<&str> = Counter::new();
    let mut materials: Counter<&str> = Counter::new();

    while let Some(item) = stack.pop()
    {
        let recipe = recipes.get(item.0)
                                                .expect("Ingredient not found in recipe");
        
        
        let ingredients = &recipe.1;

        // This one is made directly from ore
        if ingredients.len() == 1 && ingredients[0].0 == "ORE"
        {
            primary_materials[&item.0] += item.1;
        }
        else{
            for ingr in ingredients.iter() {
                let count = ((ingr.1 * (materials[&item.0] + item.1)as f32 / recipe.0 as f32).ceil();
                println!("Need {} x {} for making {} x {}", count, ingr.0, item.1, item.0);
                let comp = Component(ingr.0, count as usize);
                materials[&item.0] += item.1;
                stack.push(comp);
            }
        }
        println!();
    }
    let total: f32 = primary_materials.iter().map(|(item_name, &count)|
    {
        let recipe = recipes.get(item_name).expect("Item not found");
        let ingredients = &recipe.1;
        let item_created_per_reaction = recipe.0 as f32;
        let ore_needed_per_reaction = ingredients[0].1 as f32;
        
        let num_reactions = ((count as f32) / item_created_per_reaction).ceil();
        let ore_needed = ore_needed_per_reaction * num_reactions;
        println!("Ore needed for {} x {} is {}", count, item_name, ore_needed);
        ore_needed
    }).sum();
    println!("total ore used: {:?}", total);
}