use std::collections::HashMap;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("day06/input.txt")?;
    let mut orbit_counts: HashMap<&str, u32> = HashMap::new();
    let mut children: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut parents: HashMap<&str, &str> = HashMap::new();
    input
        .lines()
        .map(|line| {
            let mut obj_iter = line.split(')');
            let parent: &str = obj_iter.next().unwrap();
            let child: &str = obj_iter.next().unwrap();

            (parent, child)
        })
        .for_each(|(parent, child)| {
            let children = children.entry(parent).or_insert(Vec::new());
            children.push(child);

            let parent_orbits = orbit_counts.get(parent).copied().unwrap_or(0);
            orbit_counts.insert(child, parent_orbits + 1);

            parents.insert(child, parent);
        });

    let total_orbits: usize = sum_depths(0, "COM", &children);

    println!("{}", total_orbits);

    let my_parents = get_ancestors("YOU", &parents);
    let santas_parents = get_ancestors("SAN", &parents);

    let common_ancestor: &str = my_parents
        .iter()
        .rev()
        .zip(santas_parents.iter().rev())
        .filter(|(a, b)| a == b)
        .last()
        .map(|(a, _)| a)
        .unwrap();

    let ancestor_to_me = my_parents
        .iter()
        .position(|&x| x == common_ancestor)
        .unwrap();
    let ancestor_to_santa = santas_parents
        .iter()
        .position(|&x| x == common_ancestor)
        .unwrap();
    let distance_to_santa = ancestor_to_me + ancestor_to_santa;

    println!("{}", distance_to_santa);

    Ok(())
}

fn get_ancestors<'a>(mut child: &'a str, parents: &'a HashMap<&str, &str>) -> Vec<&'a str> {
    let mut result = Vec::new();

    while let Some(&parent) = parents.get(child) {
        result.push(parent);
        child = parent;
    }

    result
}

fn sum_depths(depth: usize, root: &str, children: &HashMap<&str, Vec<&str>>) -> usize {
    let empty = Vec::new();
    let direct_children = children.get(root).unwrap_or(&empty);
    direct_children.len() * (depth + 1)
        + direct_children
            .iter()
            .map(|child| sum_depths(depth + 1, child, children))
            .sum::<usize>()
}
