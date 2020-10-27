use ::aoc2019::*;
use std::collections::HashMap;
use std::collections::VecDeque;
use counter::Counter;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Component<'a> {
    name : &'a str, 
    count: i32,
}

#[derive(Debug)]
pub struct Recipe<'a> {
    output_count: i32,
    ingredients: Vec<Component<'a>>
}

impl<'a> From<&'a str> for Component<'a>
{
    fn from(s: &'a str) -> Self {
        let parts = s.split(" ").collect::<Vec<&str>>();
        let count = parts[0].parse().expect("Error parsing count");
        let name = parts[1];
        Self{name, count}
    }
}

type RecipeBook<'a> =  HashMap<&'a str, Recipe<'a>>;
fn get_raw_material_for<'a>(product_name: &'a str, 
    product_count: i32, 
    recipes: &'a RecipeBook, 
    extra_feed_stock: &mut Counter<&'a str>) -> Vec<(&'a str, i32)>
{
    let recipe = recipes.get(product_name).unwrap();
    let recipe_output_count = recipe.output_count;

    let extra_material_available = extra_feed_stock[&product_name] as i32;
    
    let product_needed = (product_count - extra_material_available).max(0);
    let extra_material_remaining = (extra_material_available-product_count).max(0);
    extra_feed_stock[&product_name] = extra_material_remaining as usize;

    if extra_material_available > 0
    {
        println!("Used {} x {} from extra stock instead of making more", product_count - product_needed, &product_name);
    }

    let mut output = Vec::new();
    for raw_material in recipe.ingredients.iter() 
    {
        let raw_material_per_unit = raw_material.count;
        let raw_material_needed = product_needed * raw_material_per_unit;

        let extra_material_available = extra_feed_stock[&raw_material.name] as i32;
        let raw_material_still_needed = (raw_material_needed - extra_material_available).max(0);
        let extra_material_remaining = (extra_material_available-raw_material_needed).max(0);
        
        if extra_material_available > 0
        {
            println!("Used {} from extra stock", &raw_material.name);
        }
        if raw_material_still_needed > 0
        {
            output.push((raw_material.name, raw_material_needed));
        }
        extra_feed_stock[&raw_material.name] = extra_material_remaining as usize;
    }
    let num_reactions = (product_needed as f32/recipe_output_count as f32).ceil() as i32;
    let total_output = recipe_output_count * num_reactions;
    
    let extra_output = total_output - product_needed;
    println!("{}: recipe creates {}, we need {}, we make {}, extra {} with {} rns",
        product_name,
        recipe_output_count,
        product_needed,
        total_output,
        extra_output,
        num_reactions
    );
    extra_feed_stock[&product_name] += extra_output as usize;
    output
}
fn main()
{
    let input = include_str!("../../inputs/day14.txt").to_string();
    let lines = input.lines();

    let mut recipes: HashMap<&str, Recipe> = HashMap::new();
    for line in lines
    {
        let parts = line.split(" => ").collect::<Vec<&str>>();
        let ingredients: Vec<Component> = parts[0].split(", ").map(Component::from).collect();
        let recipe_item = Component::from(parts[1]);
        let recipe = Recipe {
            output_count: recipe_item.count,
            ingredients
        };
        recipes.insert(recipe_item.name, recipe);
    }
   
    // let mut stack = VecDeque::from(vec![ ("FUEL", 1.0f32)]);
    let output:Vec<i32> = Vec::new();
    let mut extra = Counter::new();
    let mut q = VecDeque::from(vec![("FUEL", 1)]);
    let mut total_ore = 0;
    while let Some((product_name, product_count)) = q.pop_front()
    {
        println!("Getting {} recipe", &product_name);
        let feedstock = get_raw_material_for(product_name, product_count, &recipes, &mut extra);

        
        let ore_count: i32 = feedstock.iter()
                                .filter_map(|(name, count)| {
                                if name == &"ORE"{
                                    Some(count)
                                }else{None}}).sum();
        println!("Added {} ore for {}", ore_count, product_name);
        total_ore += ore_count;
        q.extend(feedstock.into_iter().filter(|(name, count)| name != &"ORE"));
        
        println!("Q after: {:?}", &q);
        println!("Extra after: {:?}\n", &extra);
    }
    println!("Total: {}", total_ore);


    // let mut primary_materials: HashMap<&str, f32> = HashMap::new();
    // let mut materials: HashMap<&str, f32> = HashMap::new();

    // while let Some(item) = stack.pop_front()
    // {
    //     let recipe = recipes.get(item.0)
    //                                             .expect("Ingredient not found in recipe");
        
        
    //     let ingredients = &recipe.1;

    //     // This one is made directly from ore
    //     if ingredients.len() == 1 && ingredients[0].0 == "ORE"
    //     {
    //         // primary_materials[&item.0] += item.1;
    //         let entry = primary_materials.entry(&item.0).or_default();
    //         *entry += item.1;
    //     }
    //     else{
    //         for ingr in ingredients.iter() 
    //         {
    //             let count = (ingr.1 as f32 * (item.1)as f32 / recipe.0  as f32);
    //             println!("Need {} x {} for making {} x {}", count, ingr.0, item.1, item.0);
    //             // let comp = Component(ingr.0, count as usize);

    //             let entry = materials.entry(&ingr.0).or_default();
    //             *entry += count;
    //             // materials[&item.0] += count;
    //             let comp = (ingr.0, count);
    //             stack.push_back(comp);
    //         }
    //     }
    //     println!();
    // }

    // println!("BOM: {:?}", &primary_materials);
    // let total: f32 = primary_materials.iter().map(|(item_name, &count)|
    // {
    //     let recipe = recipes.get(item_name).expect("Item not found");
    //     let ingredients = &recipe.1;
    //     let item_created_per_reaction = recipe.0 as f32;
    //     let ore_needed_per_reaction = ingredients[0].1 as f32;
        
    //     let num_reactions = (count.ceil() / item_created_per_reaction).ceil();
    //     let ore_needed = ore_needed_per_reaction * num_reactions;
    //     println!("Ore needed for {} ({}) x {} is {} ({} reactions)", count, count.ceil(), item_name, ore_needed, num_reactions);
    //     ore_needed
    // }).sum();
    // println!("total ore used: {:?}", total);
}