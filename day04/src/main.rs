use digits_iterator::*;
use itertools::Itertools;
use rayon::prelude::*;

fn main() {
    let range = || (254032..=789860u32).into_par_iter();

    let part1_ans = range()
        .filter(has_monotonically_increasing_digits)
        .filter(has_double_digit)
        .count();

    println!("{}", part1_ans);

    let part2_ans = range()
        .filter(has_monotonically_increasing_digits)
        .filter(has_strict_double_digit)
        .count();

    println!("{}", part2_ans);
}

fn has_double_digit(x: &u32) -> bool {
    digit_pairs(x).any(|(a, b)| a == b)
}

fn has_monotonically_increasing_digits(x: &u32) -> bool {
    digit_pairs(x).all(|(a, b)| a <= b)
}

fn digit_pairs(x: &u32) -> impl Iterator<Item = (u8, u8)> {
    x.digits().zip(x.digits().skip(1))
}

fn has_strict_double_digit(x: &u32) -> bool {
    x.digits()
        .group_by(|&x| x)
        .into_iter()
        .any(|(_key, grp)| grp.count() == 2)
}
