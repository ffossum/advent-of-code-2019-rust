use std::collections::HashMap;

use std::error::Error;
#[derive(Debug, Clone)]
struct Materials {
    name: String,
    quantity: u64,
}
impl Materials {
    fn parse(s: &str) -> Option<Materials> {
        let mut iter = s.split(" ");
        let quantity: u64 = iter.next()?.parse().ok()?;
        let name = iter.next().unwrap().to_string();
        Some(Materials { name, quantity })
    }
}

#[derive(Debug)]
struct Reaction {
    input: Vec<Materials>,
    output: Materials,
}

fn parse_reactions(s: &str) -> HashMap<String, Reaction> {
    let mut reactions: HashMap<String, Reaction> = HashMap::new();
    for line in s.lines() {
        let mut iter = line.split(" => ");
        let lhs = iter.next().unwrap();
        let lhs = lhs
            .split(", ")
            .map(|x| Materials::parse(x).unwrap())
            .collect::<Vec<Materials>>();
        let rhs = iter.next().unwrap();
        let rhs = Materials::parse(rhs).unwrap();
        let reaction = Reaction {
            input: lhs,
            output: rhs,
        };
        reactions.insert(reaction.output.name.clone(), reaction);
    }
    reactions
}

pub fn main() -> Result<(), Box<dyn Error>> {
    let input = std::fs::read_to_string("day14/input.txt")?;
    // let input =
    // "171 ORE => 8 CNZTR
    // 7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
    // 114 ORE => 4 BHXH
    // 14 VRPVC => 6 BMBT
    // 6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
    // 6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
    // 15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
    // 13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
    // 5 BMBT => 4 WPTQ
    // 189 ORE => 9 KTJDG
    // 1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
    // 12 VRPVC, 27 CNZTR => 2 XDBXC
    // 15 KTJDG, 12 BHXH => 5 XCVML
    // 3 BHXH, 2 VRPVC => 7 MZWV
    // 121 ORE => 7 VRPVC
    // 7 XCVML => 6 RJRHP
    // 5 BHXH, 4 VRPVC => 5 LTCX"
    //         .to_string();

    let mut reactions: HashMap<String, Reaction> = HashMap::new();
    for line in input.lines() {
        let mut iter = line.split(" => ");
        let lhs = iter.next().unwrap();
        let lhs = lhs
            .split(", ")
            .map(|x| Materials::parse(x).unwrap())
            .collect::<Vec<Materials>>();
        let rhs = iter.next().unwrap();
        let rhs = Materials::parse(rhs).unwrap();
        let reaction = Reaction {
            input: lhs,
            output: rhs,
        };
        reactions.insert(reaction.output.name.clone(), reaction);
    }

    // assert_eq!(reactions.len(), input.lines().count());
    // for r in reactions.iter() {
    //     println!("{:?}", r);
    // }

    let mut required_materials = HashMap::new();
    let mut spare_materials = HashMap::new();
    get_required_materials(
        &reactions,
        &Materials {
            name: "FUEL".to_string(),
            quantity: 4215655,
        },
        &mut required_materials,
        &mut spare_materials,
    );
    println!("{:?}", required_materials);
    let part1_ans = get_required_ore(&reactions, required_materials);
    println!("{:?}", part1_ans);

    println!("{}", part1_ans > 1000000000000);
    Ok(())
}

fn get_required_ore(reactions: &HashMap<String, Reaction>, materials: HashMap<String, u64>) -> u64 {
    let mut sum = 0;
    for (material, material_needed) in materials {
        let material_per_reaction = reactions.get(&material).unwrap().output.quantity;

        let required_times = if material_needed % material_per_reaction == 0 {
            material_needed / material_per_reaction
        } else {
            (material_needed / material_per_reaction) + 1
        };


        let ore_per_reaction = reactions
            .get(&material)
            .unwrap()
            .input
            .get(0)
            .unwrap()
            .quantity;

        sum += ore_per_reaction * required_times
    }
    sum
}

fn get_required_materials(
    reactions: &HashMap<String, Reaction>,
    material: &Materials,
    required_materials: &mut HashMap<String, u64>,
    spare_materials: &mut HashMap<String, u64>,
) {
    let reaction = reactions.get(&material.name).unwrap();
    if reaction.input.get(0).unwrap().name != "ORE" {
        required_materials.remove(&material.name);

        let mut required_quantity = material.quantity;
        let spare = spare_materials.entry(material.name.clone()).or_insert(0);
        if required_quantity >= *spare {
            required_quantity -= *spare;
            spare_materials.remove(&material.name);
        } else {
            required_quantity = 0;
            *spare -= material.quantity;
        }

        let required_times = if required_quantity % reaction.output.quantity == 0 {
            required_quantity / reaction.output.quantity
        } else {
            (required_quantity / reaction.output.quantity) + 1
        };

        let to_spare = (reaction.output.quantity * required_times) - required_quantity;
        let spare = spare_materials.entry(material.name.clone()).or_insert(0);
        *spare += to_spare;

        for required_material in reaction.input.iter() {
            let mut required_material = required_material.clone();
            required_material.quantity *= required_times;

            let req = required_materials
                .entry(required_material.name.clone())
                .or_insert(0);
            *req += required_material.quantity;

            get_required_materials(
                &reactions,
                &required_material,
                required_materials,
                spare_materials,
            );
        }
    }
}
