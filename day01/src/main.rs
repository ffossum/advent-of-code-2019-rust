use std::error::Error;
use std::iter;

fn main() -> Result<(), Box<dyn Error>> {
    let input = std::fs::read_to_string("input.txt")?;
    let input = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let part1_ans: u32 = input
        .iter()
        .map(|mass| simple_fuel_required(mass).unwrap_or(0))
        .sum();
    println!("{}", part1_ans);

    let part2_ans: u32 = input.iter().map(fuel_required).sum();
    println!("{}", part2_ans);

    Ok(())
}

fn simple_fuel_required(mass: &u32) -> Option<u32> {
    (mass / 3).checked_sub(2)
}
fn fuel_required(mass: &u32) -> u32 {
    iter::successors(simple_fuel_required(&mass), simple_fuel_required).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fuel_required_test() {
        assert_eq!(fuel_required(&0), 0);
        assert_eq!(fuel_required(&14), 2);
        assert_eq!(fuel_required(&1969), 966);
        assert_eq!(fuel_required(&100756), 50346);
    }
}
