use digits_iterator::*;

fn main() {
    let range = || 254032..=789860;

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
    let digits: Vec<u8> = x.digits().collect();

    let starts_with_double = {
        let first = digits.get(0);
        let second = digits.get(1);
        let third = digits.get(2);

        first.is_some() && first == second && first != third
    };

    let ends_with_double = {
        let last_i = digits.len() - 1;

        let last = digits.get(last_i);
        let second_last = digits.get(last_i - 1);
        let third_last = digits.get(last_i - 2);

        last.is_some() && last == second_last && last != third_last
    };

    let double_in_middle = {
        digits.windows(4).any(|window| {
            (window[0] != window[1]) && (window[1] == window[2]) && (window[2] != window[3])
        })
    };

    starts_with_double || ends_with_double || double_in_middle
}
